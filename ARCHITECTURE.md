# ClarityDesk — Architecture

## Overview

ClarityDesk is a privacy-first, offline-only desktop application that captures screen content,
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

### `cd-core` — Core Engine

| Module | Responsibility |
|--------|----------------|
| `capture/` | Platform screen capture via `screenshots` crate |
| `ocr/` | Tesseract OCR + HOCR block parser |
| `analyzer/` | Block classifier: Code / Terminal / Log / Text / Table / UI |
| `semantic/` | OllamaClient REST adapter + mode-specific prompt templates |
| `models/` | Shared data types (CaptureFrame, AnalysisBlock, Settings) |

### `cd-cli` — Command-Line Interface

Standalone binary for scripted use: `claritydesk capture|translate|status`.

### `src-tauri` — Desktop Shell

Tauri v2 application bridging the React frontend to `cd-core` via IPC commands.
Active plugins: `shell`, `fs`, `global-shortcut`, `notification`.

### `frontend` — React UI

React 18 + TypeScript + Tailwind CSS + Zustand. Three views: Dashboard, Analysis, Settings.

## Data Flow

```
User triggers capture
       │
       ▼
screenshots::capture() → CaptureFrame (PNG, RAM only)
       │
       ▼
tesseract::ocr() → Vec<HocrBlock>
       │
       ▼
BlockClassifier::classify() → Vec<AnalysisBlock { kind, text }>
       │
       ▼
OllamaClient::analyze(mode, blocks) → AnalysisResult
       │
       ▼
Frontend renders side-by-side view
       │
       ▼
User closes/copies → all data discarded from RAM
```

## Privacy Architecture

All processing is RAM-only. No data is written to disk during any analysis pass.
See [PRIVACY.md](PRIVACY.md) for the full policy.

## External Dependencies (local only)

| Dependency | Purpose | Network |
|------------|---------|---------|
| Tesseract OCR | Text extraction | none — local binary |
| Ollama | AI inference | localhost:11434 only |
| `screenshots` crate | Screen capture | none |

No cloud endpoints. No telemetry. No accounts required.
