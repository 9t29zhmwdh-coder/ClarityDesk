use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, State};

use cd_core::{
    models::settings::{OllamaStatus, Settings},
    ocr::OcrEngine,
    semantic::SemanticEngine,
};
use crate::{error::Result, hotkeys, state::AppState};

#[tauri::command]
pub async fn get_settings(state: State<'_, Arc<AppState>>) -> Result<Settings> {
    Ok(state.settings.lock().await.clone())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveOutcome {
    /// Shortcuts that could not be registered: invalid, or taken by another app.
    pub failed_hotkeys: Vec<String>,
}

/// Applies the settings, writes them to disk and re-registers the shortcuts.
#[tauri::command]
pub async fn save_settings(
    settings: Settings,
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<SaveOutcome> {
    *state.ocr.lock().await = OcrEngine::new(&settings.ocr_language);
    *state.engine.lock().await = SemanticEngine::new(&settings.ollama_host, &settings.ollama_model);
    settings.save(&state.settings_path)?;
    let failed_hotkeys = hotkeys::register_all(&app, &settings.hotkeys);
    *state.settings.lock().await = settings;
    Ok(SaveOutcome { failed_hotkeys })
}

/// Asks with a client of its own, so a long analysis holding the engine does not freeze the status.
#[tauri::command]
pub async fn check_ollama(state: State<'_, Arc<AppState>>) -> Result<OllamaStatus> {
    let settings = state.settings.lock().await.clone();
    let probe = SemanticEngine::new(&settings.ollama_host, &settings.ollama_model);
    Ok(probe.status().await)
}

#[tauri::command]
pub async fn get_default_settings() -> Result<Settings> {
    Ok(Settings::default())
}

/// Where a person can drop their own app profiles.
#[tauri::command]
pub async fn get_profiles_dir(state: State<'_, Arc<AppState>>) -> Result<String> {
    Ok(state.profiles_dir.display().to_string())
}
