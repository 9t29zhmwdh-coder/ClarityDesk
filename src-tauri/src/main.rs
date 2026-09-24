// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod error;
mod hotkeys;
mod state;

use state::AppState;
use commands::*;
use tauri::Manager;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("claritydesk=debug".parse().unwrap()),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Regular);
            let config_dir = app.path().app_config_dir()?;
            let state = AppState::new(config_dir);
            let hotkeys = state.settings.try_lock().map(|s| s.hotkeys.clone()).unwrap_or_default();
            app.manage(state);
            for keys in hotkeys::register_all(app.handle(), &hotkeys) {
                tracing::warn!("shortcut {keys} is unavailable, change it in the settings");
            }
            if let Some(window) = app.get_webview_window("main") {
                window.show()?;
                window.set_focus()?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Capture
            list_screens,
            capture_primary,
            crop_last_frame,
            get_last_frame,
            // Analysis
            analyze_last_frame,
            extract_text,
            get_last_result,
            is_analyzing,
            // Settings
            get_settings,
            save_settings,
            check_ollama,
            get_default_settings,
            get_profiles_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ClarityDesk");
}
