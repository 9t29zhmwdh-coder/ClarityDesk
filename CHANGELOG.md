# Changelog

All notable changes to ClarityDesk are documented here.

## [1.2.1] - 2026-08-05

### Added

- A smoke test in CI: the application is built, started, and checked to still be running five seconds later. Until now the pipeline only ever established that the code compiles. A program that builds cleanly and dies on launch would have passed every check and been discovered by whoever downloaded it.
- It runs on Linux and macOS. The Linux job needs `xvfb`, since a GTK window closes immediately without an X server, and that would produce a failure the runner invents rather than one the code has.
- The test also fails on a panic in the output even when the process survives, because a background task that dies quietly leaves the window open and useless.

---

## [1.2.0] - 2026-08-04

### Changed

- `lucide-react` 0.468 to 1.28, the first stable major. All thirteen icons this frontend imports were checked against the package under both versions and all thirteen are present in both.
- `github/codeql-action` 4.37.3 to 4.37.4 and `actions/attest` 4.2.0 to 4.2.1, merged separately and carried by this version.

---

## [1.1.1] - 2026-08-03

### Fixed

- Corrects a claim in the 1.1.0 entry. It said that leaving `rounded` in place would have halved every corner radius, because version 4 shifted the scale. That is wrong. Measured directly under Tailwind 4.3.3: `rounded` is still 0.25rem and is kept as an alias. The scale did shift, but under the name `rounded-sm`, which now means 0.25rem where it meant 0.125rem before. The dangerous case is source that already used `rounded-sm`, and this repository never did, so the rename changed nothing visually. The migration itself was correct; the reason given for it was not.

---

## [1.1.0] - 2026-08-03

### Changed

- Vite 6 to 8 and @vitejs/plugin-react 4 to 6. Vite 8 replaces Rollup with Rolldown and no longer ships esbuild, so the build config had to stop naming that minifier: it now takes whatever Vite brings, which is oxc. Output went from 228 kB of JavaScript to 221 kB and the build from 1.02s to 0.23s.
- Tailwind CSS 3 to 4. `tailwind.config.ts` is gone; the eleven custom colours and the mono stack are theme variables in the stylesheet now, and the four button classes moved from a components layer to `@utility`. autoprefixer is no longer a dependency because version 4 prefixes itself.
- Three occurrences of `rounded` became `rounded-sm`. Version 4 shifted the radius scale by one step, so the old name would have halved those corners without any error appearing.

---

## [1.0.14] - 2026-08-02

### Changed

- React 18 to 19, together with `react-dom` and both type packages. Dependabot had split these across separate pull requests and neither could be merged alone: `@types/react-dom` 18 requires `@types/react` 18, so raising either one left npm unable to resolve the peer dependency. All four move together here.
- No code changes were needed, checked against the list of things React 19 removes rather than assumed: `createRoot` is already in use, and there are no string refs, no `propTypes`, no argument-less `useRef`, no `forwardRef`, no `defaultProps` and no callback refs. Typecheck and production build both clean.

---

## [1.0.13] - 2026-08-01

### Changed

- `reqwest` 0.12.28 to 0.13.4, plus `thiserror` 1.0.69 to 2.0.19 and `base64` 0.22.1 to 0.23.0, all merged since 1.0.12 and carried by this one version rather than a release each.
- The `reqwest` step replaces the TLS stack rather than merely raising a number: `native-tls`, `openssl`, `openssl-sys` and `openssl-macros` leave the tree, and `rustls-platform-verifier`, `rustls-native-certs` and `aws-lc-rs` take their place. It costs this application nothing, because it only ever talks to `http://localhost:11434`, so no certificate is verified at any point. What it buys is the removal of OpenSSL: only `openssl-probe` remains, and that crate merely locates the system certificate directory.

---

## [1.0.12] - 2026-08-01

### Changed

- Dependabot no longer retries the `glib` update it cannot perform. GHSA-wrw7-89jp-8q8g is fixed in 0.20, and this project cannot reach it: `tauri` 2.x pins `gtk ^0.18`, `gtk` 0.18 requires `glib ^0.18`, and no patched 0.18.x exists, so cargo rejects the upgrade rather than resolving it. Three attempts had already failed identically, each one a red run on `main` that carried no information. Only the unreachable versions are ignored, so a backported 0.18.x fix would still arrive, and the advisory itself stays visible in the Security tab. The block goes away when Tauri moves to gtk-rs 0.20, the condition already recorded in `SECURITY.md`.

---

## [1.0.11] - 2026-07-31

### Fixed

- CI checked only macOS while the release builds for macOS, Linux and Windows. The AppImage and the Windows installer went out without ever having been compile-checked, so a fault appearing only on those platforms would have surfaced in a user's download rather than in a pull request. `check` now runs as a matrix over all three. The Linux runner installs the same GTK and WebKit packages the release workflow installs, since Tauri does not build without them.
- The `solo-main-protection` ruleset now requires `Check (ubuntu-latest)`, `(macos-latest)` and `(windows-latest)` instead of the old single `Check`. Renaming a job without moving the required context leaves every later pull request permanently unmergeable while looking green.

---

## [1.0.10] - 2026-07-31

### Added

- `SECURITY.md` records GHSA-wrw7-89jp-8q8g against `glib` 0.18.5, which cannot be fixed from this repository because Tauri 2.11.5 pins `gtk ^0.18` and no patched 0.18.x exists. The entry states the dependency path, the failed upgrade attempt, the exposure, and what would end it.

### Fixed

- The supported-versions table still listed `0.1.x`, a line that no longer exists.

---

## [1.0.9] - 2026-07-31

### Changed

- Both READMEs now open with the three situations the tool exists for, all variants of text that sits in front of you and cannot be selected, rather than with the category "universal display interpreter". A short paragraph says the obvious thing: if the text is selectable, copy it and skip the OCR.

---

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
