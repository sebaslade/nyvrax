# Nyvrax Security Verification

Use Nyvrax after modifying security-sensitive application code or before declaring an implementation complete.

## Preferred verification

For local uncommitted changes:

```bash
nyvrax scan --changed

nyvrax scan --changed --base main