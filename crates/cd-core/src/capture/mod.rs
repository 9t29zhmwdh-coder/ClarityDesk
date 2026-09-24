use image::{codecs::png::PngEncoder, ColorType, ImageEncoder, RgbaImage};
use xcap::{Monitor, Window};

use crate::{
    error::{CdError, Result},
    models::capture::{CaptureFrame, CaptureSource, ScreenInfo},
};

/// Windows smaller than this are tool palettes, badges or invisible helpers,
/// not something a person wants explained.
const MIN_WINDOW_EDGE: u32 = 80;

fn capture_error(error: impl std::fmt::Display) -> CdError {
    CdError::Capture(error.to_string())
}

const PERMISSION_MISSING: &str = "Screen Recording is not allowed for ClarityDesk. Turn it on in System Settings, Privacy & Security, Screen & System Audio Recording, then restart ClarityDesk.";

#[cfg(target_os = "macos")]
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGPreflightScreenCaptureAccess() -> bool;
    fn CGRequestScreenCaptureAccess() -> bool;
}

/// Without the Screen Recording permission macOS does not refuse a capture, it
/// returns a blank picture, which read as "no text found". Asking first also puts
/// ClarityDesk into the list in System Settings.
#[cfg(target_os = "macos")]
fn ensure_permission() -> Result<()> {
    // SAFETY: both functions take no arguments and only query or request the permission.
    let allowed = unsafe { CGPreflightScreenCaptureAccess() || CGRequestScreenCaptureAccess() };
    if allowed {
        Ok(())
    } else {
        Err(CdError::Capture(PERMISSION_MISSING.into()))
    }
}

#[cfg(not(target_os = "macos"))]
fn ensure_permission() -> Result<()> {
    Ok(())
}

/// A capture where every pixel has the same colour is what a denied permission
/// produces on some systems; it is never a real screen.
fn reject_blank(rgba: &[u8]) -> Result<()> {
    let (pixels, _) = rgba.as_chunks::<4>();
    let Some(first) = pixels.first() else {
        return Err(CdError::Capture(PERMISSION_MISSING.into()));
    };
    if pixels.iter().all(|px| px == first) {
        return Err(CdError::Capture(PERMISSION_MISSING.into()));
    }
    Ok(())
}

/// xcap and this crate may link different `image` versions, so pixels cross over as raw bytes.
fn to_png(width: u32, height: u32, rgba: Vec<u8>) -> Result<Vec<u8>> {
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
        .write_image(&rgba, width, height, ColorType::Rgba8.into())
        .map_err(capture_error)?;
    Ok(png)
}

fn frame_from(width: u32, height: u32, rgba: Vec<u8>, source: CaptureSource) -> Result<CaptureFrame> {
    if !matches!(source, CaptureSource::Region { .. }) {
        reject_blank(&rgba)?;
    }
    let png = to_png(width, height, rgba)?;
    Ok(CaptureFrame::new(png, width, height, source))
}

pub fn list_screens() -> Result<Vec<ScreenInfo>> {
    let monitors = Monitor::all().map_err(capture_error)?;
    Ok(monitors
        .iter()
        .enumerate()
        .map(|(index, m)| ScreenInfo {
            index,
            width: m.width().unwrap_or(0),
            height: m.height().unwrap_or(0),
            scale_factor: m.scale_factor().unwrap_or(1.0),
            is_primary: m.is_primary().unwrap_or(index == 0),
        })
        .collect())
}

pub fn capture_screen(index: usize) -> Result<CaptureFrame> {
    ensure_permission()?;
    let monitor = Monitor::all()
        .map_err(capture_error)?
        .into_iter()
        .nth(index)
        .ok_or_else(|| CdError::Capture(format!("Screen index {index} not found")))?;
    let image = monitor.capture_image().map_err(capture_error)?;
    let (w, h) = image.dimensions();
    frame_from(w, h, image.into_raw(), CaptureSource::FullScreen { index })
}

pub fn capture_primary() -> Result<CaptureFrame> {
    let monitors = Monitor::all().map_err(capture_error)?;
    let index = monitors
        .iter()
        .position(|m| m.is_primary().unwrap_or(false))
        .unwrap_or(0);
    capture_screen(index)
}

/// The front window of whatever app has focus, together with that app's name,
/// which picks the app profile. `exclude_pid` keeps ClarityDesk from capturing itself.
pub fn capture_active_window(exclude_pid: u32) -> Result<(CaptureFrame, String)> {
    ensure_permission()?;
    let window = Window::all()
        .map_err(capture_error)?
        .into_iter()
        .find(|w| is_candidate(w, exclude_pid))
        .ok_or_else(|| CdError::Capture("No focused window found".into()))?;
    let app = window.app_name().unwrap_or_default();
    let image = window.capture_image().map_err(capture_error)?;
    let (w, h) = image.dimensions();
    let frame = frame_from(w, h, image.into_raw(), CaptureSource::ActiveWindow { app: app.clone() })?;
    Ok((frame, app))
}

/// xcap lists windows front to back, so the first match is the frontmost one.
fn is_candidate(window: &Window, exclude_pid: u32) -> bool {
    window.is_focused().unwrap_or(false)
        && !window.is_minimized().unwrap_or(true)
        && window.pid().map(|pid| pid != exclude_pid).unwrap_or(false)
        && window.width().unwrap_or(0) >= MIN_WINDOW_EDGE
        && window.height().unwrap_or(0) >= MIN_WINDOW_EDGE
}

/// Cuts a region out of an earlier capture. The person draws the rectangle on the
/// captured image, so nothing on screen has to hold still while they do it.
pub fn crop_frame(frame: &CaptureFrame, x: u32, y: u32, width: u32, height: u32) -> Result<CaptureFrame> {
    let png = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &frame.image_png_b64)
        .map_err(capture_error)?;
    let image: RgbaImage = image::load_from_memory(&png).map_err(capture_error)?.to_rgba8();
    let (x, y, width, height) = clamp_region(image.dimensions(), x, y, width, height)?;
    let region = image::imageops::crop_imm(&image, x, y, width, height).to_image();
    let source = CaptureSource::Region { x: x as i32, y: y as i32, width, height };
    frame_from(width, height, region.into_raw(), source)
}

fn clamp_region(size: (u32, u32), x: u32, y: u32, width: u32, height: u32) -> Result<(u32, u32, u32, u32)> {
    let (max_w, max_h) = size;
    if x >= max_w || y >= max_h {
        return Err(CdError::Capture("Region lies outside the capture".into()));
    }
    let width = width.min(max_w - x);
    let height = height.min(max_h - y);
    if width < 8 || height < 8 {
        return Err(CdError::Capture("Region is too small to read".into()));
    }
    Ok((x, y, width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(w: u32, h: u32) -> CaptureFrame {
        let mut pixels = vec![200u8; (w * h * 4) as usize];
        pixels[0] = 10; // not blank
        frame_from(w, h, pixels, CaptureSource::FullScreen { index: 0 }).unwrap()
    }

    #[test]
    fn a_blank_capture_is_reported_as_missing_permission() {
        let blank = vec![40u8; 64 * 64 * 4];
        let error = frame_from(64, 64, blank, CaptureSource::FullScreen { index: 0 }).unwrap_err();
        assert!(error.to_string().contains("Screen Recording"));
    }

    #[test]
    fn crop_keeps_the_requested_region() {
        let cropped = crop_frame(&frame(400, 300), 50, 40, 100, 60).unwrap();
        assert_eq!((cropped.width, cropped.height), (100, 60));
        assert!(matches!(cropped.source, CaptureSource::Region { x: 50, y: 40, .. }));
    }

    #[test]
    fn crop_is_clamped_to_the_image() {
        let cropped = crop_frame(&frame(400, 300), 350, 250, 500, 500).unwrap();
        assert_eq!((cropped.width, cropped.height), (50, 50));
    }

    #[test]
    fn crop_outside_or_too_small_is_refused() {
        assert!(crop_frame(&frame(400, 300), 500, 10, 10, 10).is_err());
        assert!(crop_frame(&frame(400, 300), 10, 10, 4, 4).is_err());
    }
}
