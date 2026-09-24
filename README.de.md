<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>ClarityDesk</h1>
</div>

[🇬🇧 English Version](README.md)

**Erklärt, was auf deinem Bildschirm steht, ohne dass dieser Bildschirm je das Gerät verlässt.**

Ein Stacktrace in einer Sprache, die du nicht liest. Ein Konfigurationsdialog
in einer VM ohne Zwischenablage. Ein Fehler auf einem Screenshot, den dir
jemand in den Chat geworfen hat. Immer dasselbe Muster: der Text steht direkt
vor dir und ist trotzdem unerreichbar, weil du ihn nicht markieren und damit
nirgends einfügen kannst.

ClarityDesk greift den Ausschnitt ab, liest ihn per OCR und stellt Übersetzung,
Erklärung oder Diagnose neben das Original. Hotkey oder Button, mehr ist die
Bedienung nicht.

Es läuft gegen ein lokales Ollama-Modell. Nichts wird hochgeladen, und
Aufnahmen und Ergebnisse landen nie auf der Festplatte; genau darum geht es,
wenn auf dem Schirm
ein Kundensystem oder ein Produktivlog steht.

**Nichts für dich, wenn** sich der Text markieren lässt. Dann kopierst du ihn
und fügst ihn in das Modell ein, das du ohnehin nutzt; OCR fügt da nur die
Chance hinzu, sich zu verlesen.

[![CI](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions) [![CodeQL](https://github.com/9t29zhmwdh-coder/ClarityDesk/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/9t29zhmwdh-coder/ClarityDesk/security/code-scanning) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/9t29zhmwdh-coder/ClarityDesk/badge)](https://securityscorecards.dev/viewer/?uri=github.com/9t29zhmwdh-coder/ClarityDesk) [![OpenSSF Best Practices](https://www.bestpractices.dev/projects/13715/badge)](https://www.bestpractices.dev/projects/13715)

![Platform](https://img.shields.io/badge/Platform-macOS_%7C_Windows-lightgrey) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![Tauri](https://img.shields.io/badge/Tauri-24C8D8?logo=tauri&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) ![AI | Ollama](https://img.shields.io/badge/AI-Ollama-black?logo=ollama&logoColor=white)

> **So läuft es:** ClarityDesk ist eine native Desktop-App, kein Server oder Browser-Tool. Sie öffnet sich als eigenes Fenster, ohne Tray-Icon oder Hintergrunddienst; sie erfasst und analysiert deinen Bildschirm nur, wenn du sie aktiv auslöst.

![ClarityDesk](docs/screenshot.de.png)

---

> 💾 **Download:** [macOS (DMG)](https://github.com/9t29zhmwdh-coder/ClarityDesk/releases/latest/download/ClarityDesk.dmg) · [Windows (Installer)](https://github.com/9t29zhmwdh-coder/ClarityDesk/releases/latest/download/ClarityDesk-Setup.exe) · [Linux (AppImage)](https://github.com/9t29zhmwdh-coder/ClarityDesk/releases/latest/download/ClarityDesk.AppImage): immer das neueste Release, nicht code-signiert/notarisiert (Gatekeeper/SmartScreen warnen beim ersten Start). Oder aus dem Quellcode bauen, siehe Erste Schritte unten.

---

Die Oberfläche von ClarityDesk ist auf Englisch (Standard) und Deutsch verfügbar; umschaltbar über den Sprachtoggle.

**In der Praxis:** du erteilst einmal die Zustimmung zur Bildschirmerfassung, löst dann per Hotkey oder Button eine Erfassung aus; ClarityDesk extrahiert den Text per OCR und zeigt eine übersetzte, erklärte oder diagnostizierte Version neben dem Original. Alles läuft lokal über Ollama; Aufnahmen und Ergebnisse bleiben im Arbeitsspeicher und sind weg, sobald die App schliesst.

---

> 🌱 Neu hier? → [Schritt-für-Schritt-Anleitung für Einsteiger](GETTING_STARTED.md)

---

## Funktionen

| Funktion | Beschreibung |
|---|---|
| **Aufnahme-Knopf** | Blendet ClarityDesk aus, erfasst den Hauptbildschirm und kommt mit dem Ergebnis zurück |
| **Hotkeys** | Systemweit: erfasst das vorderste Fenster aus jeder App und holt die Erklärung nach vorne |
| **Bereich** | Einen Rahmen über die Aufnahme ziehen und nur diesen Teil lesen lassen |
| **OCR** | Tesseract, Zeilen und Aufbau bleiben erhalten; nutzt von den eingestellten Sprachen die installierten |
| **Sprach-Modus** | Übersetzt den Text in deine Zielsprache |
| **Dev-Modus** | Erklärt Befehl, Code und Fehlermeldung als eine Geschichte |
| **Smart-Modus** | Entscheidet anhand des Inhalts zwischen beiden |
| **App-Profile** | Eine Hotkey-Aufnahme wählt den Modus nach der App im Vordergrund: Browser übersetzen, Terminals und Code-Editoren erklären; eigene Profile als JSON |
| **Lokale KI (Ollama)** | Standardmodell `qwen3.5:4b-mlx` auf dem Mac (`qwen3.5:4b` sonst), jedes Ollama-Modell funktioniert |
| **Antwortsprache** | Übersetzungen und Erklärungen kommen in der eingestellten Zielsprache |
| **Einstellungen** | Bleiben zwischen den Starts erhalten; Aufnahmen und Ergebnisse nicht |
| **CLI** | `claritydesk image <datei>` erklärt einen Screenshot, den du schon hast |

---

## Voraussetzungen

- [Ollama](https://ollama.com) mit einem Modell: `ollama pull qwen3.5:4b-mlx` auf dem Mac, `ollama pull qwen3.5:4b` sonst. Während es antwortet, braucht es etwa 4,5 GB Arbeitsspeicher.
- [Tesseract OCR](https://tesseract-ocr.github.io/tessdoc/Installation.html) mit den Sprachen, die du liest: `brew install tesseract tesseract-lang` auf macOS. Reines `brew install tesseract` bringt nur Englisch mit.
- macOS 12+ / Windows 10+ / Linux (Wayland oder X11)
- Zum Bauen aus dem Quellcode: [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/) 20+ und die Tauri-CLI (`npx @tauri-apps/cli@2` geht ohne Installation)

**macOS:** *Bildschirmaufnahme*-Berechtigung in Systemeinstellungen → Datenschutz & Sicherheit erteilen.

---

## Schnellstart

```bash
git clone https://github.com/9t29zhmwdh-coder/ClarityDesk
cd ClarityDesk

# Frontend-Abhängigkeiten installieren
cd frontend && npm install && cd ..

# Entwicklungsmodus starten
cargo tauri dev

# Produktions-Build erstellen
cargo tauri build
```

**CLI-Nutzung:**
```bash
# Einen vorhandenen Screenshot (PNG oder JPEG) erklären, Antwort auf Deutsch
cargo run -p cd-cli -- image fehler.png --lang Deutsch

# Hauptbildschirm erfassen und analysieren
cargo run -p cd-cli -- capture --mode smart --lang Deutsch

# Text übersetzen
cargo run -p cd-cli -- translate "Hello, world" --lang Deutsch

# Ollama prüfen und ob das Modell installiert ist
cargo run -p cd-cli -- status
```

---

## Deinstallation / Aufräumen

ClarityDesk schreibt eine Einstellungsdatei und sonst nichts. App entfernen, dann ihren Ordner:

- **macOS:** App-Bundle löschen (oder Build-Output aufräumen: `rm -rf target/`)
- **Windows:** Deinstallation über Einstellungen → Apps, oder Build-Output-Ordner löschen

- **Einstellungen:** `~/Library/Application Support/ch.raystudio.claritydesk` (macOS), `%APPDATA%\ch.raystudio.claritydesk` (Windows), `~/.config/ch.raystudio.claritydesk` (Linux). Eigene App-Profile liegen dort im Ordner `profiles`.
- **Bildschirmaufnahme**-Freigabe (macOS): ClarityDesk in den Systemeinstellungen unter Datenschutz & Sicherheit entfernen.
- Das Ollama-Modell: `ollama rm qwen3.5:4b-mlx`, falls nichts anderes es nutzt.

---

## Datenschutz

- Aufnahmen und Ergebnisse bleiben im Arbeitsspeicher und sind weg, sobald ClarityDesk schliesst. Tesseract liest das Bild aus einer Pipe, nicht aus einer Datei.
- Gespeichert werden nur die Einstellungen (Ollama-Adresse und Modell, Sprachen, Hotkeys, ob die Einwilligung erteilt wurde).
- OCR läuft lokal über Tesseract; der Text geht an die Ollama-Adresse aus den Einstellungen, also an diesen Rechner, solange du sie nicht änderst.
- Der Aufnahme-Knopf fragt einmal nach der Einwilligung; Hotkeys laufen nur, wenn du sie drückst.
- Die Seite in der App kann nur die eigenen Befehle von ClarityDesk aufrufen und hat keinen Dateizugriff.

---

## Architektur

```
ClarityDesk/
├── crates/
│   ├── cd-core/             # Core-Engine: Capture, OCR, Analyzer, Semantic
│   │   ├── capture/         # Bildschirm, aktives Fenster, Zuschnitt (xcap-Crate)
│   │   ├── ocr/             # Tesseract über Pipes, hOCR-Parser, Block-Klassifikation
│   │   ├── analyzer/        # Moduswahl, Gruppieren der Blöcke zu Prompts
│   │   ├── semantic/        # Ollama-Client + Prompt-Vorlagen
│   │   └── profiles.rs      # App-Profile: App-Name zu Modus
│   └── cd-cli/              # CLI-Tool (capture, image, translate, status)
├── src-tauri/               # Tauri v2 Shell, IPC-Commands, globale Hotkeys
├── frontend/                # React + TypeScript + Tailwind UI
│   └── src/components/
│       ├── Dashboard/       # Modus-Auswahl, Capture-Trigger, Ollama-Status
│       ├── Analysis/        # Block-Viewer, Original/Analysiert-Umschalter, Bereichsauswahl
│       └── Settings/        # Ollama, OCR, Hotkeys, Datenschutz-Konfiguration
└── config/
    └── app-profiles/        # Mitgelieferte App-Profile, in die App einkompiliert
```

---

**Autor:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Aktiv · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/ClarityDesk?color=6b7280&style=flat-square) · **Lizenz:** MIT
