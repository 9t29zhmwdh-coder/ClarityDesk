// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod error;
mod state;

use state::AppState;
use commands::*;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("claritydesk=debug".parse().unwrap()),
        )
        .init();

    let state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            // Capture
            list_screens,
            capture_primary,
            capture_screen,
            capture_region,
            get_last_frame,
            // Analysis
            analyze_last_frame,
            analyze_blocks,
            extract_text,
            get_last_result,
            is_analyzing,
            // Settings
            get_settings,
            save_settings,
            check_ollama,
            get_default_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ClarityDesk");
}
