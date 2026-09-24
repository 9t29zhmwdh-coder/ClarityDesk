# ClarityDesk: Roadmap

## Done in 1.3.0 (2026-09-24)

Several items were ticked here long before they worked. 1.3.0 made them real:

- [x] Screen capture that hides ClarityDesk first; focused-window capture through the hotkeys
- [x] Region: drag a rectangle over the capture
- [x] OCR that actually reads Tesseract 5 output, keeps lines and decodes entities
- [x] App profiles that are read and pick the mode from the app in front
- [x] System-wide hotkeys, customizable, with feedback when one is taken
- [x] Settings saved between runs
- [x] Clear errors when Tesseract, a language pack, Ollama or the model is missing
- [x] Answers in the target language; one prompt per story instead of one per paragraph
- [x] Installers: DMG, MSI/NSIS, AppImage, deb, rpm from the release workflow

## Next

- [ ] Session history panel (cleared on close, never persisted)
- [ ] Tray icon with quick-capture shortcut
- [ ] Choosing which screen the button captures on multi-monitor setups
- [ ] Render the model's Markdown (bold, lists, code blocks) instead of showing it as plain text
- [ ] Measuring OCR plus model quality on a fixed set of screenshots, like LifeSort does

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
