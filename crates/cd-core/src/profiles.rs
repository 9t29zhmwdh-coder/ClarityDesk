//! App profiles: which analysis mode fits the app a capture came from.
//!
//! Three profiles ship inside the binary (browsers, terminals, code editors).
//! JSON files in the user's profile folder with the same shape are added on top
//! and win when they name the same app.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::models::analysis::AnalysisMode;

const BUILT_IN: [&str; 3] = [
    include_str!("../../../config/app-profiles/browser.json"),
    include_str!("../../../config/app-profiles/terminal.json"),
    include_str!("../../../config/app-profiles/vscode.json"),
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppProfile {
    pub app: String,
    pub match_process_names: Vec<String>,
    pub mode: AnalysisMode,
}

pub fn built_in() -> Vec<AppProfile> {
    BUILT_IN
        .iter()
        .map(|json| serde_json::from_str(json).expect("bundled profile is valid JSON"))
        .collect()
}

/// User profiles first, so they are found before a built-in one for the same app.
/// A broken file is skipped and logged instead of breaking every capture.
pub fn load(user_dir: &Path) -> Vec<AppProfile> {
    let mut profiles = read_dir(user_dir);
    profiles.extend(built_in());
    profiles
}

fn read_dir(dir: &Path) -> Vec<AppProfile> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .filter_map(|path| {
            let text = std::fs::read_to_string(&path).ok()?;
            serde_json::from_str(&text)
                .inspect_err(|e| tracing::warn!("skipping profile {}: {e}", path.display()))
                .ok()
        })
        .collect()
}

/// Case-insensitive, and tolerant of a trailing ".exe" on Windows process names.
pub fn mode_for_app(profiles: &[AppProfile], app_name: &str) -> Option<AnalysisMode> {
    let wanted = normalize(app_name);
    profiles
        .iter()
        .find(|p| p.match_process_names.iter().any(|name| normalize(name) == wanted))
        .map(|p| p.mode)
}

/// Which mode an analysis runs in: an explicit choice wins, then the profile of
/// the app the capture came from, then the default from the settings.
pub fn resolve_mode(
    requested: Option<AnalysisMode>,
    source_app: Option<&str>,
    profiles: &[AppProfile],
    default: &AnalysisMode,
) -> AnalysisMode {
    requested
        .or_else(|| source_app.and_then(|app| mode_for_app(profiles, app)))
        .unwrap_or(*default)
}

fn normalize(name: &str) -> String {
    let lower = name.trim().to_lowercase();
    lower.strip_suffix(".exe").unwrap_or(&lower).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_profiles_match_common_apps() {
        let profiles = built_in();
        assert_eq!(mode_for_app(&profiles, "Safari"), Some(AnalysisMode::Language));
        assert_eq!(mode_for_app(&profiles, "iTerm2"), Some(AnalysisMode::Dev));
        assert_eq!(mode_for_app(&profiles, "Code"), Some(AnalysisMode::Dev));
        assert_eq!(mode_for_app(&profiles, "code.exe"), Some(AnalysisMode::Dev));
        assert_eq!(mode_for_app(&profiles, "Finder"), None);
    }

    #[test]
    fn explicit_mode_then_profile_then_default() {
        let profiles = built_in();
        let default = AnalysisMode::Smart;
        let chosen = resolve_mode(Some(AnalysisMode::Language), Some("iTerm2"), &profiles, &default);
        assert_eq!(chosen, AnalysisMode::Language);
        assert_eq!(resolve_mode(None, Some("iTerm2"), &profiles, &default), AnalysisMode::Dev);
        assert_eq!(resolve_mode(None, Some("Finder"), &profiles, &default), AnalysisMode::Smart);
        assert_eq!(resolve_mode(None, None, &profiles, &default), AnalysisMode::Smart);
    }

    #[test]
    fn user_profiles_win_over_built_in_ones() {
        let dir = tempfile::tempdir().unwrap();
        let json = r#"{"app":"My Safari","matchProcessNames":["safari"],"mode":"dev"}"#;
        std::fs::write(dir.path().join("safari.json"), json).unwrap();
        std::fs::write(dir.path().join("broken.json"), "{not json").unwrap();
        let profiles = load(dir.path());
        assert_eq!(mode_for_app(&profiles, "Safari"), Some(AnalysisMode::Dev));
    }
}
