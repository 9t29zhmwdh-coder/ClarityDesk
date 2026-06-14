use tauri::State;
use std::sync::Arc;

use cd_core::{
    models::settings::{OllamaStatus, Settings},
    ocr::OcrEngine,
    semantic::SemanticEngine,
};
use crate::{error::Result, state::AppState};

#[tauri::command]
pub async fn get_settings(state: State<'_, Arc<AppState>>) -> Result<Settings> {
    Ok(state.settings.lock().await.clone())
}

#[tauri::command]
pub async fn save_settings(
    settings: Settings,
    state: State<'_, Arc<AppState>>,
) -> Result<()> {
    // Update dependent components
    {
        let mut ocr = state.ocr.lock().await;
        *ocr = OcrEngine::new(&settings.ocr_language);
    }
    {
        let mut engine = state.engine.lock().await;
        *engine = SemanticEngine::new(&settings.ollama_host, &settings.ollama_model);
    }
    *state.settings.lock().await = settings;
    Ok(())
}

#[tauri::command]
pub async fn check_ollama(state: State<'_, Arc<AppState>>) -> Result<OllamaStatus> {
    let engine = state.engine.lock().await;
    Ok(engine.status().await)
}

#[tauri::command]
pub async fn get_default_settings() -> Result<Settings> {
    Ok(Settings::default())
}
