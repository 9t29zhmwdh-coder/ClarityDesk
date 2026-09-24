use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Manager, State};

use cd_core::{
    capture as cap,
    models::capture::{CaptureFrame, ScreenInfo},
};
use crate::{error::Result, state::AppState};

/// macOS fades a hidden window out; capturing sooner would catch it half transparent.
const MIN_HIDE_DELAY_MS: u64 = 250;

#[tauri::command]
pub async fn list_screens() -> Result<Vec<ScreenInfo>> {
    Ok(cap::list_screens()?)
}

/// Captures the primary screen with ClarityDesk itself out of the way: the window
/// hides, the screen is taken, the window comes back. Without this the button
/// photographed ClarityDesk instead of what the person wanted explained.
#[tauri::command]
pub async fn capture_primary(app: AppHandle, state: State<'_, Arc<AppState>>) -> Result<CaptureFrame> {
    let delay = state.settings.lock().await.capture.delay_ms.max(MIN_HIDE_DELAY_MS);
    let window = app.get_webview_window("main");
    if let Some(w) = &window {
        w.hide().ok();
    }
    tokio::time::sleep(Duration::from_millis(delay)).await;
    let frame = tauri::async_runtime::spawn_blocking(cap::capture_primary)
        .await
        .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
    if let Some(w) = &window {
        w.show().ok();
        w.set_focus().ok();
    }
    let frame = frame?;
    *state.last_frame.lock().await = Some(frame.clone());
    Ok(frame)
}

/// Replaces the last capture by the rectangle the person drew on it.
#[tauri::command]
pub async fn crop_last_frame(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    state: State<'_, Arc<AppState>>,
) -> Result<CaptureFrame> {
    let mut last = state.last_frame.lock().await;
    let source = last
        .as_ref()
        .ok_or_else(|| crate::error::AppError::Other("No frame captured yet".into()))?;
    let cropped = cap::crop_frame(source, x, y, width, height)?;
    *last = Some(cropped.clone());
    Ok(cropped)
}

#[tauri::command]
pub async fn get_last_frame(state: State<'_, Arc<AppState>>) -> Result<Option<CaptureFrame>> {
    Ok(state.last_frame.lock().await.clone())
}
