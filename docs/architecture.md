# Nyvrax Architecture

Nyvrax separates security analysis from its delivery mechanisms.

```text
nyvrax-cli
    |
    v
nyvrax-core
    |
    +-- Git diff
    +-- ScanContext
    +-- Analysis
    +-- Rules
    +-- Findings
    +-- Verdict
    |
    v
nyvrax-reporters