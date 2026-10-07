# Nyvrax

Nyvrax is an AI-native security engine designed to detect security regressions introduced by software changes.

> Security verification for AI-written code.

## Status

Nyvrax is currently experimental.

The first development milestone focuses on deterministic, diff-aware security analysis.

## Current capabilities

- Git diff analysis
- Rule-based security engine
- Hardcoded-secret detection
- Security findings with severity and confidence
- PASS / WARN / BLOCK verdicts
- Terminal output
- JSON output
- SARIF output

## Run locally

```bash
cargo run -p nyvrax-cli -- scan --changed