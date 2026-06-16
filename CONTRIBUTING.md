# Contributing to ClarityDesk

Thank you for your interest in contributing!

## Bug Reports

Use the [Bug Report](.github/ISSUE_TEMPLATE/bug_report.md) template.
Include your OS, Rust version, Tesseract version, Ollama version and model name.

## Feature Requests

Use the [Feature Request](.github/ISSUE_TEMPLATE/feature_request.md) template.
Check [ROADMAP.md](ROADMAP.md) first — it may already be planned.

## Code Contributions

1. Fork the repository
2. Create a branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run all checks:
   ```bash
   cargo fmt --all
   cargo check --workspace
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   ```
5. Commit: `git commit -m "feat: short description"`
6. Push and open a Pull Request using the [PR template](.github/PULL_REQUEST_TEMPLATE.md)

## Code Style

- Follow `rustfmt` defaults (`cargo fmt`)
- No `unwrap()` in production code — use `?` or handle errors explicitly
- No `println!()` in library code — use `tracing`
- Frontend: follow the existing Tailwind + Zustand patterns

## Privacy Rules

No contribution may:
- Add telemetry, analytics, or usage tracking
- Transmit screen content or OCR results to external services
- Store captures or analysis results to disk without explicit user export

## License

By contributing you agree your changes are licensed under the MIT License.
