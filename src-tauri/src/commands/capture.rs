use tauri::State;
use std::sync::Arc;

use cd_core::{
    capture as cap,
    models::capture::{CaptureFrame, ScreenInfo},
};
use crate::{error::Result, state::AppState};

#[tauri::command]
pub async fn list_screens(state: State<'_, Arc<AppState>>) -> Result<Vec<ScreenInfo>> {
    let _ = state;
    Ok(cap::list_screens()?)
}

#[tauri::command]
pub async fn capture_primary(state: State<'_, Arc<AppState>>) -> Result<CaptureFrame> {
    let frame = cap::capture_primary()?;
    *state.last_frame.lock().await = Some(frame.clone());
    Ok(frame)
}

#[tauri::command]
pub async fn capture_screen(
    index: usize,
    state: State<'_, Arc<AppState>>,
) -> Result<CaptureFrame> {
    let frame = cap::capture_screen(index)?;
    *state.last_frame.lock().await = Some(frame.clone());
    Ok(frame)
}

#[tauri::command]
pub async fn capture_region(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    state: State<'_, Arc<AppState>>,
) -> Result<CaptureFrame> {
    let frame = cap::capture_region(x, y, width, height)?;
    *state.last_frame.lock().await = Some(frame.clone());
    Ok(frame)
}

#[tauri::command]
pub async fn get_last_frame(state: State<'_, Arc<AppState>>) -> Result<Option<CaptureFrame>> {
    Ok(state.last_frame.lock().await.clone())
}
