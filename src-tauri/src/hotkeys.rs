//! System-wide shortcuts. They work while another app is in front, which is the
//! point: the window being explained is the one that has focus when the key is pressed.

use std::sync::Arc;

use cd_core::{
    capture as cap,
    models::{analysis::AnalysisMode, settings::HotkeyConfig},
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::{commands::run_analysis, state::AppState};

pub const RESULT_EVENT: &str = "claritydesk://result";
pub const ERROR_EVENT: &str = "claritydesk://error";

#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    /// Capture the active window; a forced mode, or its app profile, or the default.
    Capture(Option<AnalysisMode>),
    /// Run the last capture through the model again.
    Reanalyze,
}

fn actions(config: &HotkeyConfig) -> Vec<(String, Action)> {
    vec![
        (config.start_stop.clone(), Action::Capture(None)),
        (config.dev_mode.clone(), Action::Capture(Some(AnalysisMode::Dev))),
        (config.smart_mode.clone(), Action::Capture(Some(AnalysisMode::Smart))),
        (config.re_analyze.clone(), Action::Reanalyze),
    ]
}

/// Replaces all registered shortcuts with the configured ones. Returns the ones that
/// could not be registered (bad syntax, or taken by another app) so the settings can say so.
pub fn register_all(app: &AppHandle, config: &HotkeyConfig) -> Vec<String> {
    let shortcuts = app.global_shortcut();
    shortcuts.unregister_all().ok();
    let mut failed = Vec::new();
    for (keys, action) in actions(config) {
        if keys.trim().is_empty() {
            continue;
        }
        let registered = shortcuts.on_shortcut(keys.as_str(), move |app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                let app = app.clone();
                tauri::async_runtime::spawn(async move { run(app, action).await });
            }
        });
        if let Err(error) = registered {
            tracing::warn!("shortcut {keys} not registered: {error}");
            failed.push(keys);
        }
    }
    failed
}

async fn run(app: AppHandle, action: Action) {
    let state = app.state::<Arc<AppState>>().inner().clone();
    let outcome = match action {
        Action::Capture(mode) => capture_and_analyze(&state, mode).await,
        Action::Reanalyze => reanalyze(&state).await,
    };
    bring_to_front(&app);
    match outcome {
        Ok(payload) => app.emit(RESULT_EVENT, payload).ok(),
        Err(message) => app.emit(ERROR_EVENT, message).ok(),
    };
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct HotkeyResult {
    frame: cd_core::models::capture::CaptureFrame,
    result: cd_core::models::analysis::AnalysisResult,
}

async fn capture_and_analyze(state: &AppState, mode: Option<AnalysisMode>) -> Result<HotkeyResult, String> {
    let own_pid = std::process::id();
    let (frame, _app) = tauri::async_runtime::spawn_blocking(move || cap::capture_active_window(own_pid))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    *state.last_frame.lock().await = Some(frame.clone());
    let result = run_analysis(state, &frame, mode).await.map_err(|e| e.to_string())?;
    Ok(HotkeyResult { frame, result })
}

async fn reanalyze(state: &AppState) -> Result<HotkeyResult, String> {
    let frame = state
        .last_frame
        .lock()
        .await
        .clone()
        .ok_or_else(|| "No frame captured yet".to_string())?;
    let result = run_analysis(state, &frame, None).await.map_err(|e| e.to_string())?;
    Ok(HotkeyResult { frame, result })
}

fn bring_to_front(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.show().ok();
        window.unminimize().ok();
        window.set_focus().ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_configured_shortcut_has_an_action() {
        let config = HotkeyConfig::default();
        let mapped = actions(&config);
        assert_eq!(mapped.len(), 4);
        assert_eq!(mapped[1], (config.dev_mode.clone(), Action::Capture(Some(AnalysisMode::Dev))));
        assert_eq!(mapped[3].1, Action::Reanalyze);
    }
}
