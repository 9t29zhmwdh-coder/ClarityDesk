use serde::{Deserialize, Serialize};

use super::analysis::AnalysisMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
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

/// The model LifeSort measured best for its size (qwen3.5 4B: 93 % on text, 4 GB).
/// On a Mac the MLX build runs on Apple's own engine and needs about a third less memory.
pub fn default_model() -> &'static str {
    if cfg!(target_os = "macos") {
        "qwen3.5:4b-mlx"
    } else {
        "qwen3.5:4b"
    }
}

impl Settings {
    /// Missing file or fields fall back to defaults, so an older or damaged
    /// settings file never keeps the app from starting.
    pub fn load(path: &std::path::Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &std::path::Path) -> crate::error::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, serde_json::to_vec_pretty(self)?)?;
        std::fs::rename(tmp, path)?;
        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            ollama_host: "http://localhost:11434".into(),
            ollama_model: default_model().into(),
            ocr_language: "eng+deu".into(),
            default_mode: AnalysisMode::Smart,
            target_language: "English".into(),
            hotkeys: HotkeyConfig::default(),
            privacy: PrivacyConfig::default(),
            capture: CaptureSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
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
#[serde(rename_all = "camelCase", default)]
pub struct PrivacyConfig {
    pub show_consent_on_start: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            show_consent_on_start: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CaptureSettings {
    /// How long ClarityDesk stays hidden before the button takes the screen.
    pub delay_ms: u64,
}

impl Default for CaptureSettings {
    fn default() -> Self {
        Self { delay_ms: 300 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OllamaStatus {
    pub connected: bool,
    pub model_installed: bool,
    pub model: String,
    pub version: Option<String>,
    pub available_models: Vec<String>,
    pub host: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_survive_a_save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("settings.json");
        let mut settings = Settings::default();
        settings.target_language = "Deutsch".into();
        settings.hotkeys.dev_mode = "Ctrl+Alt+D".into();
        settings.save(&path).unwrap();
        let loaded = Settings::load(&path);
        assert_eq!(loaded.target_language, "Deutsch");
        assert_eq!(loaded.hotkeys.dev_mode, "Ctrl+Alt+D");
    }

    #[test]
    fn a_damaged_or_partial_file_falls_back_to_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        std::fs::write(&path, r#"{"targetLanguage":"Français"}"#).unwrap();
        let partial = Settings::load(&path);
        assert_eq!(partial.target_language, "Français");
        assert_eq!(partial.ollama_model, default_model());
        std::fs::write(&path, "{broken").unwrap();
        assert_eq!(Settings::load(&path).target_language, "English");
    }
}
