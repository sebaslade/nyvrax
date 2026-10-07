# Contributing Security Rules

A Nyvrax rule should answer one security question well.

Avoid broad rules that attempt to detect unrelated vulnerability classes.

Every new rule should include:

- a stable rule identifier;
- a clear title;
- severity;
- confidence;
- evidence without sensitive values;
- remediation guidance;
- positive tests;
- negative tests.

Rules should minimize false positives.

Rules must never print discovered secret values into logs or reports.