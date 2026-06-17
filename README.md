<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>ClarityDesk</h1>
</div>

[🇩🇪 Deutsche Version](README.de.md)

**Universal Display Interpreter with OCR and local AI. Fully offline, built with Rust and Tauri.**

ClarityDesk captures your screen, extracts text with OCR and analyzes it with a local AI model: translation, code explanation, log analysis and terminal diagnostics; all without cloud, without data storage, without account.

[![CI](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions)
![Rust](https://img.shields.io/badge/Rust-1.77+-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

---

## Features

| Feature | Description |
|---|---|
| **Display Capture** | Capture full screen, active window or custom region |
| **OCR Extraction** | Tesseract-powered text extraction with layout detection |
| **Language Mode** | Translate any visible text to your target language |
| **Dev Mode** | Explain code, analyze logs, diagnose terminal output |
| **Smart Mode** | Auto-detects content type and applies the best analysis |
| **Block Classifier** | Identifies Code, Terminal, Log, Paragraph, Table, UI blocks |
| **Local AI (Ollama)** | Runs Llama, Mistral, CodeLlama or any compatible model |
| **App Profiles** | Per-application mode presets (VS Code → Dev, Browser → Language) |
| **Privacy-First** | No cloud, no storage, no telemetry: RAM-only processing |
| **Hotkeys** | System-wide shortcuts for capture and mode switching |
| **Side Panel** | Original vs. analyzed view with copy support |

---

## Requirements

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- [Tauri CLI v2](https://tauri.app/): `cargo install tauri-cli`
- [Ollama](https://ollama.ai) with at least one model (`ollama pull llama3.2`)
- [Tesseract OCR](https://tesseract-ocr.github.io/tessdoc/Installation.html) (`brew install tesseract` on macOS)
- macOS 12+ / Windows 10+ / Linux (Wayland or X11)

**macOS:** Grant *Screen Recording* permission in System Settings → Privacy & Security.

---

## Quick Start

```bash
git clone https://github.com/9t29zhmwdh-coder/ClarityDesk
cd ClarityDesk

# Install frontend dependencies
cd frontend && npm install && cd ..

# Run in development mode
cargo tauri dev

# Build for production
cargo tauri build
```

**CLI usage:**
```bash
# Capture & analyze current screen
cargo run -p cd-cli -- capture --mode smart --lang Deutsch

# Translate a text string
cargo run -p cd-cli -- translate "Hello, world" --lang Deutsch

# Check Ollama connection
cargo run -p cd-cli -- status
```

---

## Privacy

ClarityDesk is designed around explicit user consent:

- No screen content is stored, logged or transmitted
- All OCR runs locally via Tesseract
- All AI analysis runs locally via Ollama
- A consent dialog is shown on first use
- An app whitelist can restrict which applications ClarityDesk may analyze
- Everything is processed in RAM and discarded immediately after display

---

## Architecture

```
ClarityDesk/
├── crates/
│   ├── cd-core/             # Core engine: capture, OCR, analyzer, semantic
│   │   ├── capture/         # Platform screen capture (screenshots crate)
│   │   ├── ocr/             # Tesseract OCR + HOCR block parser
│   │   ├── analyzer/        # Content classifier (Code/Terminal/Log/Text)
│   │   └── semantic/        # Ollama REST client + prompt templates
│   └── cd-cli/              # CLI tool (capture, translate, status)
├── src-tauri/               # Tauri v2 desktop shell + IPC commands
├── frontend/                # React + TypeScript + Tailwind UI
│   └── src/components/
│       ├── Dashboard/       # Mode selector, capture trigger, Ollama status
│       ├── Analysis/        # Block viewer, original/analyzed toggle
│       └── Settings/        # Ollama, OCR, hotkeys, privacy config
└── config/
    ├── app-profiles/        # Per-app JSON presets
    └── model-config.toml    # Model and language defaults
```

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Framework Preview · **Last Updated:** Juni 2026
