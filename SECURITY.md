# Security Policy — ClarityDesk

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅        |

## Reporting a Vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Please report security issues via:
1. [GitHub Security Advisory](https://github.com/9t29zhmwdh-coder/ClarityDesk/security/advisories/new)
2. Or contact the maintainer directly via the GitHub profile

Include: description, steps to reproduce, potential impact, suggested fix (if known).
You can expect a response within 7 days.

## Security Design

- **No external network calls** except `localhost:11434` (Ollama) — configurable
- **No file writes** during analysis (RAM-only processing)
- **macOS entitlements** limited to screen recording and shell (Tesseract only)
- **No third-party analytics SDKs**
- **All IPC commands** explicitly allowlisted in `capabilities/default.json`

## Known Limitations

- Screen capture requires OS-level permission (macOS: Screen Recording)
- The Ollama endpoint URL is configurable — do not point it at an untrusted server
