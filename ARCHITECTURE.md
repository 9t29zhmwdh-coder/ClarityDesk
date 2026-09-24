# ClarityDesk: Architecture

## Overview

ClarityDesk is a desktop application that captures screen content,
extracts text via OCR, and analyzes it with a local AI model. It is structured as a Rust
workspace with a Tauri v2 desktop shell and a React frontend.

## Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                       ClarityDesk                           │
│                                                             │
│  ┌────────────┐  IPC   ┌────────────┐   ┌──────────────┐  │
│  │  Frontend  │ ◄────► │ src-tauri  │ → │   cd-core    │  │
│  │ React/TS  │         │  Tauri v2  │   │ Rust Engine  │  │
│  └────────────┘         └────────────┘   └──────┬───────┘  │
│                                                  │          │
│                        ┌─────────────────────────┤          │
│                        │                         │          │
│                  ┌─────▼─────┐         ┌─────────▼──────┐  │
│                  │ capture/  │         │   semantic/    │  │
│                  │ ocr/      │         │   (Ollama)     │  │
│                  │ analyzer/ │         └────────────────┘  │
│                  └───────────┘                             │
└─────────────────────────────────────────────────────────────┘
```

## Crates

### `cd-core`: Core Engine

| Module | Responsibility |
|--------|----------------|
| `capture/` | Screen, focused window and crop, via the `xcap` crate |
| `ocr/` | Tesseract via stdin/stdout, hOCR parser keeping lines, block classifier |
| `analyzer/` | Mode inference and grouping of blocks into as few prompts as fit |
| `semantic/` | Ollama client (fixed context, no thinking, low temperature) and prompts |
| `profiles` | App profiles: built-in and user JSON, app name to analysis mode |
| `models/` | Shared data types (CaptureFrame, AnalysisBlock, Settings) |

### `cd-cli`: Command-Line Interface

Standalone binary: `claritydesk capture|image|translate|status`.

### `src-tauri`: Desktop Shell

Tauri v2 application bridging the React frontend to `cd-core` via IPC commands.
Plugin: `global-shortcut`, registered from Rust. The page gets only `core:default` and calls ClarityDesk's own commands; it has no file access.

### `frontend`: React UI

React 19 + TypeScript + Tailwind CSS + Zustand. Three views: Dashboard, Analysis (with region picker), Settings.

## Data Flow

```
button: hide window, capture primary screen, show window
hotkey: capture the focused window of another app (its app name picks the profile)
       │
       ▼
CaptureFrame (PNG in memory) ──► optional crop to a region drawn by the user
       │
       ▼
tesseract stdin stdout hocr ──► TextBlocks, lines kept, each classified
       │
       ▼
mode: explicit choice, else app profile, else default; Smart infers Language or Dev
       │
       ▼
blocks merged into at most 6 prompts ──► Ollama /api/generate
       │
       ▼
AnalysisResult shown next to the original; kept until the next capture
```

## Privacy Architecture

Captures and results live in memory only; the settings file is the one thing
written to disk. See [PRIVACY.md](PRIVACY.md).

## External Dependencies (local only)

| Dependency | Purpose | Network |
|------------|---------|---------|
| Tesseract OCR | Text extraction | none, local binary |
| Ollama | AI inference | the configured address, `localhost:11434` by default |
| `xcap` crate | Screen and window capture | none |

No telemetry. No accounts required.
