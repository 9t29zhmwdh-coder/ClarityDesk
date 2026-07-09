# Getting Started with ClarityDesk

This guide walks you through setting up and running ClarityDesk from scratch, even if you have never used a terminal or built a Rust/Tauri app before. ClarityDesk runs on Windows, Linux, and macOS, so pick the section below that matches your computer. Because it does local OCR and local AI analysis, it needs two extra tools besides Rust and Node.js: **Ollama** (for local AI) and **Tesseract OCR** (for text extraction).

<!-- TODO: Screenshot -->

---

## Windows

### 1. Open a terminal

Right-click the Start button and choose **Terminal** (or **Windows PowerShell** on older versions of Windows).

### 2. Check prerequisites

```powershell
rustc --version
cargo --version
node --version
cargo tauri --version
```

If any command prints something like `'rustc' is not recognized as an internal or external command`, that tool is missing:

- **Rust / Cargo missing** → install from [rustup.rs](https://rustup.rs)
- **Node.js missing** → install from [nodejs.org](https://nodejs.org)
- **Tauri CLI missing** → once Rust is installed, run `cargo install tauri-cli`

You'll also need:
- **Ollama** → install from [ollama.ai](https://ollama.ai), then pull a model: `ollama pull llama3.2`
- **Tesseract OCR** → follow the [Windows install instructions](https://tesseract-ocr.github.io/tessdoc/Installation.html)

Close and reopen your terminal after installing so new PATH entries take effect.

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [ClarityDesk GitHub page](https://github.com/9t29zhmwdh-coder/ClarityDesk)
2. Click the green **Code** button → **Download ZIP**
3. Extract the ZIP file somewhere convenient, e.g. `C:\Projects\ClarityDesk`

**If you already use git:**
```powershell
git clone https://github.com/9t29zhmwdh-coder/ClarityDesk.git
```

### 4. Build & run

```powershell
cd ClarityDesk
cd frontend
npm install
cd ..
cargo tauri dev
```

To build a production version instead: `cargo tauri build`.

The first run takes a few minutes while dependencies install and Rust compiles. Once it finishes, the ClarityDesk window opens; trigger a capture with the hotkey or the on-screen button to see OCR + AI analysis in action.

---

## Linux

### 1. Open a terminal

This depends on your desktop environment: try **Ctrl+Alt+T**, or look for "Terminal" in your application menu.

### 2. Check prerequisites

```bash
rustc --version
cargo --version
node --version
cargo tauri --version
```

If you see `command not found` for any of these:

- **Rust / Cargo missing** → install from [rustup.rs](https://rustup.rs)
- **Node.js missing** → install from [nodejs.org](https://nodejs.org)
- **Tauri CLI missing** → once Rust is installed, run `cargo install tauri-cli`

You'll also need:
- **Ollama** → install from [ollama.ai](https://ollama.ai), then pull a model: `ollama pull llama3.2`
- **Tesseract OCR** → install via your package manager, e.g. `sudo apt install tesseract-ocr` on Debian/Ubuntu, or see the [official install docs](https://tesseract-ocr.github.io/tessdoc/Installation.html)

Restart your terminal (or run `source ~/.bashrc`) after installing Rust so `cargo` is on your PATH.

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [ClarityDesk GitHub page](https://github.com/9t29zhmwdh-coder/ClarityDesk)
2. Click the green **Code** button → **Download ZIP**
3. Extract the ZIP file, e.g. into `~/Projects/ClarityDesk`

**If you already use git:**
```bash
git clone https://github.com/9t29zhmwdh-coder/ClarityDesk.git
```

### 4. Build & run

```bash
cd ClarityDesk
cd frontend && npm install && cd ..
cargo tauri dev
```

To build a production version instead: `cargo tauri build`.

Tauri needs some system libraries to open its window on Linux (see Troubleshooting below). Once the build succeeds, the ClarityDesk window opens and you can trigger a capture to see OCR + local AI analysis.

---

## macOS

### 1. Open a terminal

Press **Cmd+Space** to open Spotlight, type "Terminal", and press Enter.

### 2. Check prerequisites

```bash
rustc --version
cargo --version
node --version
cargo tauri --version
```

If any command says `command not found`:

- **Rust / Cargo missing** → install from [rustup.rs](https://rustup.rs)
- **Node.js missing** → install from [nodejs.org](https://nodejs.org)
- **Tauri CLI missing** → once Rust is installed, run `cargo install tauri-cli`

You'll also need:
- **Ollama** → install from [ollama.ai](https://ollama.ai), then pull a model: `ollama pull llama3.2`
- **Tesseract OCR** → `brew install tesseract` (requires [Homebrew](https://brew.sh))

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [ClarityDesk GitHub page](https://github.com/9t29zhmwdh-coder/ClarityDesk)
2. Click the green **Code** button → **Download ZIP**
3. Extract the ZIP file, e.g. into `~/Projects/ClarityDesk`

**If you already use git:**
```bash
git clone https://github.com/9t29zhmwdh-coder/ClarityDesk.git
```

### 4. Build & run

```bash
cd ClarityDesk
cd frontend && npm install && cd ..
cargo tauri dev
```

To build a production version instead: `cargo tauri build`.

The first time you trigger a screen capture, macOS will ask you to grant **Screen Recording** permission in System Settings → Privacy & Security; enable it for your terminal or the ClarityDesk app, then trigger the capture again.

---

## CLI usage (all platforms)

ClarityDesk also ships a command-line tool for quick checks without opening the full app window:

```bash
# Capture & analyze current screen
cargo run -p cd-cli -- capture --mode smart --lang Deutsch

# Translate a text string
cargo run -p cd-cli -- translate "Hello, world" --lang Deutsch

# Check Ollama connection
cargo run -p cd-cli -- status
```

---

### Troubleshooting

| Issue | Cause | Fix |
|---|---|---|
| `'cargo' is not recognized` / `command not found: cargo` | Rust is not installed, or your terminal was opened before installing it | Install via [rustup.rs](https://rustup.rs), then open a **new** terminal window |
| `'npm' is not recognized` / `command not found: npm` | Node.js is not installed or not on PATH | Install via [nodejs.org](https://nodejs.org), then reopen your terminal |
| PowerShell says a `.ps1` script "cannot be loaded because running scripts is disabled" | Windows execution policy blocks local scripts | Run `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` in an elevated PowerShell, then retry |
| Build fails with linker errors mentioning `link.exe` or MSVC (Windows) | Missing C++ build tools required by Rust on Windows | Install "Desktop development with C++" via the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) installer |
| `error: failed to run custom build command for glib-sys` or missing `webkit2gtk` (Linux) | Tauri needs WebKitGTK and related system libraries | Install them, e.g. on Debian/Ubuntu: `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev` |
| App shows "Ollama not reachable" / CLI `status` command fails | Ollama isn't installed, running, or has no model pulled | Install from [ollama.ai](https://ollama.ai), start it, then run `ollama pull llama3.2` |
| OCR returns empty text / "Tesseract not found" | Tesseract OCR isn't installed or not on PATH | Install via the [official docs](https://tesseract-ocr.github.io/tessdoc/Installation.html) (`brew install tesseract` on macOS, `apt install tesseract-ocr` on Debian/Ubuntu) |
