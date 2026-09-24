use std::sync::atomic::Ordering;
use std::sync::Arc;
use tauri::State;

use cd_core::{
    models::{
        analysis::{AnalysisMode, AnalysisResult, TextBlock},
        capture::{CaptureFrame, CaptureSource},
    },
    profiles,
};
use crate::{
    error::{AppError, Result},
    state::AppState,
};

fn parse_mode(mode: &str) -> Option<AnalysisMode> {
    match mode {
        "language" => Some(AnalysisMode::Language),
        "dev" => Some(AnalysisMode::Dev),
        "smart" => Some(AnalysisMode::Smart),
        _ => None, // "auto": app profile, then the default from the settings
    }
}

/// OCR plus model for one frame. Shared by the analyze button and the hotkeys.
pub async fn run_analysis(
    state: &AppState,
    frame: &CaptureFrame,
    requested: Option<AnalysisMode>,
) -> Result<AnalysisResult> {
    let _slot = state
        .begin_analysis()
        .ok_or_else(|| AppError::Other("Analysis already running".into()))?;
    let settings = state.settings.lock().await.clone();
    let source_app = match &frame.source {
        CaptureSource::ActiveWindow { app } => Some(app.as_str()),
        _ => None,
    };
    let mode = profiles::resolve_mode(requested, source_app, &state.profiles, &settings.default_mode);
    let blocks = state.ocr.lock().await.extract_text(&frame.image_png_b64)?;
    let result = state
        .engine
        .lock()
        .await
        .analyze(&frame.id, blocks, mode, &settings.target_language)
        .await?;
    *state.last_result.lock().await = Some(result.clone());
    Ok(result)
}

#[tauri::command]
pub async fn analyze_last_frame(mode: String, state: State<'_, Arc<AppState>>) -> Result<AnalysisResult> {
    let frame = state
        .last_frame
        .lock()
        .await
        .clone()
        .ok_or_else(|| AppError::Other("No frame captured yet".into()))?;
    run_analysis(&state, &frame, parse_mode(&mode)).await
}

#[tauri::command]
pub async fn extract_text(png_b64: String, state: State<'_, Arc<AppState>>) -> Result<Vec<TextBlock>> {
    Ok(state.ocr.lock().await.extract_text(&png_b64)?)
}

#[tauri::command]
pub async fn get_last_result(state: State<'_, Arc<AppState>>) -> Result<Option<AnalysisResult>> {
    Ok(state.last_result.lock().await.clone())
}

#[tauri::command]
pub async fn is_analyzing(state: State<'_, Arc<AppState>>) -> Result<bool> {
    Ok(state.is_analyzing.load(Ordering::SeqCst))
}
