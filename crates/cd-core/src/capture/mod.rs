use screenshots::Screen;
use screenshots::image::{ImageEncoder, codecs::png::PngEncoder, ColorType};

use crate::{
    error::{CdError, Result},
    models::capture::{CaptureFrame, CaptureSource, ScreenInfo},
};

fn rgba_to_png(img: screenshots::image::RgbaImage) -> Result<Vec<u8>> {
    let (width, height) = img.dimensions();
    let raw = img.into_raw();
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
        .write_image(&raw, width, height, ColorType::Rgba8)
        .map_err(|e| CdError::Capture(e.to_string()))?;
    Ok(png)
}

pub fn list_screens() -> Result<Vec<ScreenInfo>> {
    let screens = Screen::all().map_err(|e| CdError::Capture(e.to_string()))?;
    Ok(screens
        .into_iter()
        .enumerate()
        .map(|(i, s)| ScreenInfo {
            index: i,
            width: s.display_info.width,
            height: s.display_info.height,
            scale_factor: s.display_info.scale_factor,
            is_primary: i == 0,
        })
        .collect())
}

pub fn capture_screen(index: usize) -> Result<CaptureFrame> {
    let screens = Screen::all().map_err(|e| CdError::Capture(e.to_string()))?;
    let screen = screens
        .into_iter()
        .nth(index)
        .ok_or_else(|| CdError::Capture(format!("Screen index {index} not found")))?;

    let image = screen.capture().map_err(|e| CdError::Capture(e.to_string()))?;
    let width = image.width();
    let height = image.height();
    let png = rgba_to_png(image)?;

    Ok(CaptureFrame::new(png, width, height, CaptureSource::FullScreen { index }))
}

pub fn capture_region(x: i32, y: i32, width: u32, height: u32) -> Result<CaptureFrame> {
    let screens = Screen::all().map_err(|e| CdError::Capture(e.to_string()))?;
    let screen = screens
        .first()
        .ok_or_else(|| CdError::Capture("No screens found".into()))?;

    let image = screen
        .capture_area(x, y, width, height)
        .map_err(|e| CdError::Capture(e.to_string()))?;

    let w = image.width();
    let h = image.height();
    let png = rgba_to_png(image)?;

    Ok(CaptureFrame::new(png, w, h, CaptureSource::Region { x, y, width, height }))
}

pub fn capture_primary() -> Result<CaptureFrame> {
    capture_screen(0)
}