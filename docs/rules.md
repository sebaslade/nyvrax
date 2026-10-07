
## `docs/rules.md`

```markdown
# Nyvrax Rules

Nyvrax rules implement the `NyvraxRule` trait.

Each rule receives a `ScanContext` and returns zero or more structured findings.

## Initial namespaces

- `NYX-SECRET-*`
- `NYX-AUTH-*`
- `NYX-AUTHZ-*`
- `NYX-INPUT-*`
- `NYX-WEBHOOK-*`
- `NYX-DEP-*`

## Severity

- Critical
- High
- Medium
- Low
- Info

## Confidence

- High
- Medium
- Low

Severity describes impact.

Confidence describes how certain Nyvrax is that the finding represents a real problem.