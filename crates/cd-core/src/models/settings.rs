use serde::{Deserialize, Serialize};

use super::analysis::AnalysisMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub ollama_host: String,
    pub ollama_model: String,
    pub ocr_language: String,
    pub default_mode: AnalysisMode,
    pub target_language: String,
    pub hotkeys: HotkeyConfig,
    pub privacy: PrivacyConfig,
    pub capture: CaptureSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            ollama_host: "http://localhost:11434".into(),
            ollama_model: "llama3.2".into(),
            ocr_language: "eng+deu".into(),
            default_mode: AnalysisMode::Smart,
            target_language: "Deutsch".into(),
            hotkeys: HotkeyConfig::default(),
            privacy: PrivacyConfig::default(),
            capture: CaptureSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyConfig {
    pub start_stop:  String,
    pub dev_mode:    String,
    pub smart_mode:  String,
    pub re_analyze:  String,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            start_stop: "Alt+Shift+C".into(),
            dev_mode:   "Alt+Shift+D".into(),
            smart_mode: "Alt+Shift+S".into(),
            re_analyze: "Alt+Shift+E".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyConfig {
    pub store_captures: bool,
    pub store_results: bool,
    pub app_whitelist: Vec<String>,
    pub show_consent_on_start: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            store_captures: false,
            store_results: false,
            app_whitelist: vec![],
            show_consent_on_start: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureSettings {
    pub scale_factor: f32,
    pub include_cursor: bool,
    pub delay_ms: u64,
}

impl Default for CaptureSettings {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            include_cursor: false,
            delay_ms: 200,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OllamaStatus {
    pub connected: bool,
    pub version: Option<String>,
    pub available_models: Vec<String>,
    pub host: String,
}
