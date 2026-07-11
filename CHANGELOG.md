# Changelog

All notable changes to ClarityDesk are documented here.

## [0.2.4] - 2026-07-11

### Fixed

- SemVer correction: v0.1.1 added a genuine new feature (full English/German UI translation) but was versioned as a patch. Renumbered v0.1.1 through v0.1.4 to v0.2.0 through v0.2.3 (same commits, tags and releases recreated at identical SHAs), per the portfolio's SemVer discipline (patch = fix, minor = feature, major = finished product).

## [0.2.3] - 2026-07-11

### Added

- Documented Dual-Licensing assessment (Community-only) in ROADMAP.md.

### Fixed

- Removed em-dashes from ROADMAP.md and SECURITY.md headings and body text.

## [0.2.2] - 2026-07-11

### Fixed

- Updated actions/setup-node to its latest major version in CI, since GitHub is deprecating the Node.js 20 runtime and the previous version was being forced onto Node 24 and crashing during post-run cleanup.

## [0.2.1] - 2026-07-10

### Added

- Added the "New here?" beginner guide callout to README.md (moved above Features) and README.de.md (was missing)

## [0.2.0] - 2026-07-08

### Fixed

- Missing `src-tauri/capabilities/` permissions were blocking core Tauri APIs at runtime
- Wrong notification permission identifiers (`allow-notify`, `allow-check-permissions`)
- `tauri.conf.json` used an inline `infoPlist` object where a file path was expected; replaced with a proper `src-tauri/Info.plist`
- Invalid `plugins.shell` config caused a startup panic; the shell plugin was unused dead code and has been removed entirely
- Missing `thiserror` and `tracing-subscriber` (`env-filter`) dependencies
- A local `Result<T>` alias was shadowing `std::result::Result` in the `Serialize` impl for `AppError`
- Main window did not reliably appear on launch; added explicit activation policy and `show()`/`set_focus()` in the setup hook
- CI previously excluded the Tauri app crate from checks, hiding all of the above

### Added

- Full English/German UI translation (`useLangStore`, language toggle)
- README onboarding sections: how it runs, screenshot, in practice, uninstall/cleanup

## [0.1.0] - 2026-06-14

### Added

- Initial release, framework preview
- Screen capture: full screen, active window, custom region
- Tesseract OCR integration with HOCR block parsing
- Block classifier: Code, Terminal, Log, Paragraph, Table, UI
- Ollama REST client with mode-specific prompt templates
- Modes: Language (translation), Dev (code/log/terminal), Smart (auto-detect)
- App profiles: per-application mode presets (VS Code, Browser, Terminal)
- Side-panel view: original vs. analyzed content
- Tauri v2 desktop shell (macOS, Windows, Linux)
- CLI tool: `claritydesk capture|translate|status`
- System-wide hotkeys for capture and mode switching
- Bilingual README (English + German)
- MIT License
