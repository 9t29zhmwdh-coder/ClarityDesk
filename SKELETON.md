# ClarityDesk — Professional Repo Skeleton

**Generated:** 2026-06-16 | **Earliest commit:** 2026-06-14T18:55:28Z | **Release:** v0.1.0

This file is the canonical skeleton artifact for ClarityDesk. It documents the complete
professional repo structure, contains all key file contents, and provides a migration
checklist for integrating existing code.

---

## Canonical File Tree

```
ClarityDesk/
├── README.md                          # English (header + footer preserved)
├── README.de.md                       # German (header + footer preserved)
├── LICENSE                            # MIT — Rafael Yilmaz 2026
├── SKELETON.md                        # This file
├── ARCHITECTURE.md                    # System design overview
├── PRIVACY.md                         # Privacy-first policy
├── ROADMAP.md                         # Feature roadmap
├── CONTRIBUTING.md                    # Contribution guide
├── CODE_OF_CONDUCT.md                 # Contributor Covenant 2.1
├── SECURITY.md                        # Security policy + reporting
├── CHANGELOG.md                       # Version history (v0.1.0)
├── .cargo/
│   └── config.toml                    # macOS dynamic linking
├── .github/
│   ├── workflows/
│   │   └── ci.yml                     # Cross-compile CI (see content below)
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── PULL_REQUEST_TEMPLATE.md
├── docs/
│   └── scan-report.json               # Full account scan summary
├── config/
│   ├── app-profiles/
│   │   ├── browser.json
│   │   ├── terminal.json
│   │   └── vscode.json
│   └── model-config.toml
├── crates/
│   ├── cd-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── error.rs
│   │       ├── analyzer/mod.rs
│   │       ├── capture/mod.rs
│   │       ├── ocr/mod.rs
│   │       ├── semantic/
│   │       │   ├── mod.rs
│   │       │   ├── ollama.rs
│   │       │   └── prompts.rs
│   │       └── models/
│   │           ├── mod.rs
│   │           ├── analysis.rs
│   │           ├── capture.rs
│   │           └── settings.rs
│   └── cd-cli/
│       ├── Cargo.toml
│       └── src/main.rs
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── entitlements.plist
│   ├── capabilities/default.json
│   └── src/
│       ├── main.rs
│       ├── error.rs
│       ├── state.rs
│       └── commands/
│           ├── mod.rs
│           ├── capture.rs
│           ├── analyze.rs
│           └── settings.rs
└── frontend/
    ├── package.json
    ├── tsconfig.json
    ├── vite.config.ts
    ├── tailwind.config.ts
    ├── index.html
    └── src/
        ├── main.tsx
        ├── App.tsx
        ├── index.css
        ├── components/
        │   ├── Layout/Sidebar.tsx
        │   ├── Dashboard/DashboardView.tsx
        │   ├── Analysis/AnalysisView.tsx
        │   └── Settings/SettingsView.tsx
        ├── stores/clarityStore.ts
        └── lib/tauri.ts
```

---

## README.md (English) — updated with CI badge

> Header and footer preserved exactly. Added CI badge after existing badges.

```markdown
<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>ClarityDesk</h1>
</div>

[🇩🇪 Deutsche Version](README.de.md)

**Universal Display Interpreter — OCR + local AI, fully offline. Rust + Tauri.**

ClarityDesk captures your screen, extracts text with OCR and analyzes it with a local AI
model: translation, code explanation, log analysis and terminal diagnostics — all without
cloud, without data storage, without account.

[![CI](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/Rust-1.77+-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

[... existing content ...]

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Framework Preview · **Last Updated:** Juni 2026
```

---

## README.de.md (German) — updated with CI badge

> Header and footer preserved exactly. CI badge added to match EN version.

```markdown
<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>ClarityDesk</h1>
</div>

[🇬🇧 English Version](README.md)

**Universeller Display-Interpreter — OCR + lokale KI, vollständig offline. Rust + Tauri.**

[![CI](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/Rust-1.77+-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

[... existing content ...]

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Framework Preview · **Last Updated:** Juni 2026
```

---

## CI Workflow: `.github/workflows/ci.yml`

> Note: Pushing to `.github/workflows/` requires the `workflows` OAuth scope.
> To enable: `gh auth refresh -s workflows`, then push this file manually.

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    name: Build & Test (${{ matrix.os }})
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Install system dependencies (Linux)
        if: runner.os == 'Linux'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev \
            libayatana-appindicator3-dev librsvg2-dev tesseract-ocr

      - name: Install Tesseract (macOS)
        if: runner.os == 'macOS'
        run: brew install tesseract

      - name: Cache Rust
        uses: Swatinem/rust-cache@v2

      - name: Check
        run: cargo check --workspace

      - name: Test
        run: cargo test --workspace

      - name: Clippy
        run: cargo clippy --workspace -- -D warnings

      - name: Format check
        run: cargo fmt --all -- --check
```

**To push the CI workflow (one-time):**
```bash
gh auth refresh -s workflows
# Then re-run the skeleton push for ci.yml
```

---

## Sample Rust Module with Unit Test

### `crates/cd-core/src/analyzer/mod.rs` (minimal working example)

```rust
//! Content block classifier — maps raw OCR lines to typed blocks.

use crate::models::analysis::{AnalysisBlock, BlockKind};

/// Classify a list of raw text lines into typed content blocks.
pub fn classify(lines: &[&str]) -> Vec<AnalysisBlock> {
    lines
        .iter()
        .map(|line| AnalysisBlock {
            kind: detect_kind(line),
            text: line.to_string(),
        })
        .collect()
}

fn detect_kind(line: &str) -> BlockKind {
    let t = line.trim();
    if t.starts_with("$ ") || t.starts_with("# ") || t.starts_with("> ") {
        BlockKind::Terminal
    } else if t.contains("ERROR") || t.contains("WARN") || t.contains("INFO") {
        BlockKind::Log
    } else if t.ends_with('{') || t.ends_with('}') || t.ends_with(';') {
        BlockKind::Code
    } else {
        BlockKind::Text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_terminal_line() {
        let blocks = classify(&["$ cargo build"]);
        assert_eq!(blocks[0].kind, BlockKind::Terminal);
    }

    #[test]
    fn classifies_log_line() {
        let blocks = classify(&["2026-06-14 ERROR: connection refused"]);
        assert_eq!(blocks[0].kind, BlockKind::Log);
    }

    #[test]
    fn classifies_code_line() {
        let blocks = classify(&["fn main() {"]);
        assert_eq!(blocks[0].kind, BlockKind::Code);
    }

    #[test]
    fn classifies_text_line() {
        let blocks = classify(&["Hello, world"]);
        assert_eq!(blocks[0].kind, BlockKind::Text);
    }
}
```

### `crates/cd-core/src/models/analysis.rs` (referenced types)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockKind {
    Code,
    Terminal,
    Log,
    Text,
    Table,
    Ui,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisBlock {
    pub kind: BlockKind,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub blocks: Vec<AnalysisBlock>,
    pub analyzed_text: String,
    pub mode: String,
}
```

---

## Migration Checklist

1. **ARCHITECTURE.md** — pushed ✅
2. **PRIVACY.md** — pushed ✅
3. **ROADMAP.md** — pushed ✅
4. **CONTRIBUTING.md** — pushed ✅
5. **CODE_OF_CONDUCT.md** — pushed ✅
6. **SECURITY.md** — pushed ✅
7. **CHANGELOG.md** — pushed ✅
8. **.github/ISSUE_TEMPLATE/bug_report.md** — pushed ✅
9. **.github/ISSUE_TEMPLATE/feature_request.md** — pushed ✅
10. **.github/PULL_REQUEST_TEMPLATE.md** — pushed ✅
11. **.github/workflows/ci.yml** — ⚠️ requires `workflows` OAuth scope
    - Run: `gh auth refresh -s workflows`, then push content from section above
12. **docs/scan-report.json** — pushed ✅
13. **README.md** — CI badge to add (content in section above)
14. **README.de.md** — CI badge to add (content in section above)
15. **GitHub Release v0.1.0** — dated 2026-06-14 (see release creation below)

### Create Release (run once)

```bash
# Tag the initial commit
gh api repos/9t29zhmwdh-coder/ClarityDesk/git/refs \
  --method POST \
  -f ref="refs/tags/v0.1.0" \
  -f sha="dfe90c31acabf257e84d60e2119714399d6be2d9"

# Create release with historical date
gh api repos/9t29zhmwdh-coder/ClarityDesk/releases \
  --method POST \
  -f tag_name="v0.1.0" \
  -f name="v0.1.0 — Initial import" \
  -f body="Initial import — earliest commit date: 2026-06-14

Universal Display Interpreter: screen capture, Tesseract OCR, local AI via Ollama.
Rust + Tauri v2 desktop shell. macOS / Windows / Linux." \
  -f published_at="2026-06-14T18:55:28Z" \
  -F prerelease=true
```

### Add CI Badge to README.md

After enabling the `workflows` scope and pushing `ci.yml`, add this line
**before** the existing `![Rust]` badge in both READMEs:

```markdown
[![CI](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions/workflows/ci.yml)
```

### Reusable Modules (cross-repo candidates)

| File | Rationale |
|------|-----------|
| `crates/cd-core/src/semantic/ollama.rs` | OllamaClient usable in any RayStudio Rust project |
| `crates/cd-core/src/error.rs` | Standard error type pattern, copy to other crates |
| `frontend/src/stores/clarityStore.ts` | Zustand pattern, adapt for other projects |
| `config/model-config.toml` | Shared model config format for all Ollama-based projects |
| `src-tauri/tauri.conf.json` | Template for new Tauri v2 projects (ch.raystudio.* identifier) |

---

*ClarityDesk — RayStudio · Rafael Yilmaz · MIT License · 2026*
