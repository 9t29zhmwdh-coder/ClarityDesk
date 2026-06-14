use tauri::State;
use std::sync::Arc;

use cd_core::models::analysis::{AnalysisMode, AnalysisResult, TextBlock};
use crate::{error::Result, state::AppState};

#[tauri::command]
pub async fn analyze_last_frame(
    mode: String,
    state: State<'_, Arc<AppState>>,
) -> Result<AnalysisResult> {
    let frame = {
        let lock = state.last_frame.lock().await;
        lock.clone().ok_or_else(|| crate::error::AppError::Other("No frame captured yet".into()))?
    };

    {
        let mut analyzing = state.is_analyzing.lock().await;
        if *analyzing {
            return Err(crate::error::AppError::Other("Analysis already running".into()));
        }
        *analyzing = true;
    }

    let settings = state.settings.lock().await.clone();

    let analysis_mode = match mode.as_str() {
        "language" => AnalysisMode::Language,
        "dev"      => AnalysisMode::Dev,
        _          => AnalysisMode::Smart,
    };

    let blocks = {
        let ocr = state.ocr.lock().await;
        ocr.extract_text(&frame.image_png_b64)
            .unwrap_or_default()
    };

    let result = {
        let engine = state.engine.lock().await;
        engine
            .analyze(&frame.id, blocks, analysis_mode, &settings.target_language)
            .await?
    };

    *state.last_result.lock().await = Some(result.clone());
    *state.is_analyzing.lock().await = false;

    Ok(result)
}

#[tauri::command]
pub async fn analyze_blocks(
    frame_id: String,
    blocks: Vec<TextBlock>,
    mode: String,
    state: State<'_, Arc<AppState>>,
) -> Result<AnalysisResult> {
    let settings = state.settings.lock().await.clone();
    let analysis_mode = match mode.as_str() {
        "language" => AnalysisMode::Language,
        "dev"      => AnalysisMode::Dev,
        _          => AnalysisMode::Smart,
    };

    let result = {
        let engine = state.engine.lock().await;
        engine
            .analyze(&frame_id, blocks, analysis_mode, &settings.target_language)
            .await?
    };

    *state.last_result.lock().await = Some(result.clone());
    Ok(result)
}

#[tauri::command]
pub async fn extract_text(
    png_b64: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<TextBlock>> {
    let ocr = state.ocr.lock().await;
    Ok(ocr.extract_text(&png_b64)?)
}

#[tauri::command]
pub async fn get_last_result(state: State<'_, Arc<AppState>>) -> Result<Option<AnalysisResult>> {
    Ok(state.last_result.lock().await.clone())
}

#[tauri::command]
pub async fn is_analyzing(state: State<'_, Arc<AppState>>) -> Result<bool> {
    Ok(*state.is_analyzing.lock().await)
}
