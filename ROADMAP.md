# ClarityDesk: Roadmap

## Current: v0.1.0, Framework Preview

- [x] Screen capture (full screen, active window, custom region)
- [x] Tesseract OCR + HOCR block parsing
- [x] Block classifier (Code / Terminal / Log / Text / Table / UI)
- [x] Ollama REST client + mode-specific prompt templates
- [x] Modes: Language (translation), Dev (code/log/terminal), Smart (auto-detect)
- [x] App profiles (per-application mode presets)
- [x] Side-panel view (original vs. analyzed)
- [x] Tauri v2 desktop shell (macOS, Windows, Linux)
- [x] CLI tool (`capture`, `translate`, `status`)
- [x] Bilingual UI (English / German)

## v0.2.0, UX & Stability

- [ ] Hotkey customization in Settings UI
- [ ] Copy individual blocks to clipboard
- [ ] Session history panel (cleared on close, never persisted)
- [ ] Drag-to-select region capture overlay
- [ ] Tray icon with quick-capture shortcut
- [ ] Improved error messages for Tesseract / Ollama connection failures

## v0.3.0, Language & Model Expansion

- [ ] Model switcher per mode in Settings
- [ ] Auto-detect source language (OCR language hint)
- [ ] Additional OCR language packs (FR, ES, IT, PT, ZH, JA, KO)
- [ ] Prompt template editor

## v0.4.0, Advanced Modes

- [ ] Table extraction → CSV / Markdown export
- [ ] Code diff mode (compare two consecutive captures)
- [ ] Batch capture session (multiple regions, one AI pass)
- [ ] Export analysis result as PDF / Markdown

## v1.0.0, Production Release

- [ ] Cross-platform installers: DMG (macOS), MSI (Windows), AppImage (Linux)
- [ ] Auto-update via GitHub Releases (local check, user-initiated download)
- [ ] Accessibility audit (VoiceOver / Narrator support)
- [ ] Full unit + integration test coverage
- [ ] Signed and notarized binaries

## Out of Scope (by design)

- Cloud AI backends or remote inference
- User accounts or subscription licensing
- Telemetry, analytics, or crash reporting
- Screenshots stored to disk without explicit user export

## Dual-Licensing Readiness

Assessed 2026-07-11: Community-only, not a Dual-Licensing candidate. Unlike governance/observability tools in this portfolio, ClarityDesk's own design philosophy above already rules out any licensing or subscription model by design (no accounts, no subscriptions). It is a single-user local productivity tool with no team, fleet or multi-tenant dimension on the roadmap, so there is no natural Enterprise-tier split point without contradicting the project's stated privacy-first intent. No further action planned unless the project's scope changes.
