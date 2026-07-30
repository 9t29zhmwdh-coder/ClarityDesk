# Changelog

All notable changes to ClarityDesk are documented here.

## [1.0.8] - 2026-07-30

### Added

- `Cargo.lock` is committed. It was listed in `.gitignore`, so every build resolved dependencies afresh and no two builds were guaranteed to use the same versions. For an application rather than a library the lock file belongs in the repository: it is what makes a release reproducible and what lets a security advisory be checked against what actually shipped.

---

## [1.0.7] - 2026-07-29

### Added

- `frontend/src/vite-env.d.ts`, referencing `vite/client`. Vite has always declared modules for `*.css` and the other asset types it handles, but nothing in this project pulled that declaration in. TypeScript 5 accepts the untyped side-effect import of `index.css` regardless, so the gap stayed invisible; TypeScript 7 rejects it with `TS2882`. The file belongs to Vite's own project scaffold and was simply missing, so this closes an existing hole rather than preparing for a specific upgrade.
### Security

- The release workflow no longer grants `contents: write` for its whole run. The permission moves to the one job that publishes the release, and everything else runs with `contents: read`. OpenSSF Scorecard scores the Token-Permissions check 0 out of 10 whenever any workflow holds a top-level write permission, regardless of how little of the run needs it, so this single line was what held the check at zero.

---

## [1.0.6] - 2026-07-29

### Changed

Dependency and workflow updates merged since 1.0.5:

- chore(ci): bump the actions group across 1 directory with 3 updates
- chore(deps): bump autoprefixer

---

## [1.0.5] - 2026-07-28

### Fixed

- The CodeQL job requested `packages: read`, `actions: read` and `contents: read` at job level, repeating grants the workflow level already provides. OpenSSF Scorecard counts that as excessive token permissions and scores `Token-Permissions` at 0 out of 10 for it. The job now requests only `security-events: write`, which is the one grant that genuinely exceeds the workflow default.

## [1.0.4] - 2026-07-28

### Changed

- CodeQL moved from GitHub's default setup to an advanced setup with a committed `.github/workflows/codeql.yml`. The default setup skips pull requests that touch no code of a given language, so a dependency pull request changing only a lock file reported `skipping` on the required `Analyze (...)` checks forever and could never be merged. The workflow runs on every pull request regardless of what changed. It also uses the `security-extended` query suite, which the default setup does not allow choosing. Required checks are unchanged: verified on `BugRadar` that all eight, the generic `CodeQL` check included, turn green under this setup.
- Dependabot now groups only minor and patch updates per ecosystem; majors arrive as individual pull requests. The previous grouping put React 18 to 19, Tailwind 3 to 4 and similar breaking changes into one pull request together with urgently needed security patches, which made the whole batch unreviewable and unmergeable. Actions stay grouped wholesale. Follows `engineering-standards` v0.11.0.

## [1.0.3] - 2026-07-28

### Security

- `postcss` updated to 8.5.24, closing a high-severity path traversal in the source map auto-loading via `sourceMappingURL` that affects all versions up to and including 8.5.17.

Applied as a normal pull request rather than by merging Dependabot's, because Dependabot pull requests cannot currently pass this repository's required checks: CodeQL runs through GitHub's default setup, which does not trigger on a pull request that only touches a lock file, so its checks report `skipping` and never turn green. Bypassing a required check is not an option per `standards/ci-cd.md` section 7, so the fix takes the route that runs the full pipeline.

## [1.0.2] - 2026-07-28

### Added

- `.github/dependabot.yml`, covering GitHub Actions, the Cargo workspace and the frontend npm packages, with grouped weekly updates. The file was missing, and without it a repository receives no version updates at all: security alerts only fire for disclosed vulnerabilities. Follows `engineering-standards` v0.10.0.

### Fixed

- `actions/checkout` was pinned to two different SHAs: v6 in `ci.yml` and `release.yml`, v7 in `scorecard.yml`. All three now use v7.0.1 with the full version in the comment, per `engineering-standards` `standards/ci-cd.md` section 2. Drift like this is what happens when one workflow is updated by a PR that does not touch the others.

## [1.0.1] - 2026-07-20

### Changed

- OpenSSF Scorecard workflow and badge.
- `copilot-instructions.md` for consistent AI-assisted contributions.
- Coverage reporting in CI (cargo-tarpaulin, scoped to the non-GUI crates).
- Split the README's security/CI badges onto their own line, separate from the platform/tech/AI badges (they were rendering as a single merged line).

## [1.0.0] - 2026-07-17

First stable release: a real, packaged, installable distribution exists
for end users. Real macOS/Windows/Linux installers (DMG, NSIS, AppImage/deb/rpm).

## [0.2.9] - 2026-07-17

### Changed
- CI: added an explicit `permissions: contents: read` block to the workflow(s) that were missing one (CodeQL `actions/missing-workflow-permissions`), narrowing the default GITHUB_TOKEN scope.

## [0.2.8] - 2026-07-13

### Added

- Documented the EN/DE language toggle in README.md/README.de.md; it was already implemented and working but not mentioned.

## [0.2.7] - 2026-07-12

### Fixed

- `.cargo/config.toml` had an unscoped `[build]` section applying macOS-only linker flags (`-C link-arg=-undefined -C link-arg=dynamic_lookup`) to every target, including Windows and Linux. This broke the very first cross-platform release build (v0.2.6): the Windows build failed with `LINK1181: cannot open input file 'dynamic_lookup.obj'` because that flag is meaningless to MSVC's linker. Removed the unscoped entry; the target-scoped `[target.aarch64-apple-darwin]`/`[target.x86_64-apple-darwin]` entries (which are correct) are unchanged.

## [0.2.6] - 2026-07-12

### Added

- Release workflow (`.github/workflows/release.yml`): builds and attaches macOS (DMG), Windows (NSIS installer), and Linux (AppImage) bundles to a GitHub Release on every tag push. Previously, no release ever had an installer attached.
- README/README.de.md: Download section linking to the latest release's installers.

### Fixed

- All GitHub Actions in `ci.yml` pinned to a commit SHA, matching the portfolio's Action Pinning standard.

## [0.2.5] - 2026-07-11

### Fixed

- Removed an eszett and em-dashes from TEMPLATE_NOTES.md, ARCHITECTURE.md, CONTRIBUTING.md, SKELETON.md, docs/scan-report.json, and three Rust source files. The project uses Swiss German orthography (ss, not ß).

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
