# Privacy Policy : ClarityDesk

## Summary

ClarityDesk reads text on your screen and explains it with a local AI model. Captures and results stay in memory; only the settings are saved.

## What I Collect

**Nothing.** ClarityDesk sends nothing to me or to anyone else. There is no telemetry.

## Screen Content

- A capture happens only when you press the capture button or one of the hotkeys.
- The latest capture and its result stay in memory, so you can pick a region or analyze again, and are replaced by the next capture or discarded when ClarityDesk closes.
- No capture is written to disk: Tesseract reads the image through a pipe.
- The capture button hides ClarityDesk first, so its own window is not in the picture; a hotkey captures the window in front of it.

## AI Processing

- The text found on screen is sent to the Ollama address in the settings, `http://localhost:11434` unless you change it. With the default, nothing leaves the computer.
- No external AI service (OpenAI, Anthropic, Google, and so on) is contacted.

## OCR Processing

- Text extraction runs locally via [Tesseract OCR](https://tesseract-ocr.github.io/).

## Settings and App Profiles

- Settings are saved in the application data folder (`~/Library/Application Support/ch.raystudio.claritydesk/` on macOS): Ollama address and model, OCR and target language, default mode, hotkeys, and whether consent was given.
- Your own app profiles, if you add any, sit in its `profiles` folder. They only name apps and a mode.
- Nothing is synced anywhere.

## Consent

The capture button asks for consent once and remembers the answer. On macOS the system additionally asks for the Screen Recording permission, which you can withdraw in System Settings at any time.

## Data Retention

Captures and results: until the next capture or until ClarityDesk closes. Settings: until you delete them (see the README, Uninstall).

## Contact

Security issues: see [SECURITY.md](SECURITY.md)

**Last updated: 2026-09-24**
