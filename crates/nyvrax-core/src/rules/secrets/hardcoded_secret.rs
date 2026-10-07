use std::{path::Path, sync::LazyLock};

use regex::Regex;

use crate::{Confidence, Finding, FindingCategory, NyvraxError, NyvraxRule, ScanContext, Severity};

pub struct HardcodedSecretRule;

struct SecretPattern {
    name: &'static str,
    regex: Regex,
    severity: Severity,
    confidence: Confidence,
}

static PATTERNS: LazyLock<Vec<SecretPattern>> = LazyLock::new(|| {
    vec![
        SecretPattern {
            name: "AWS access key",
            regex: Regex::new(r"\bAKIA[0-9A-Z]{16}\b").expect("valid AWS regex"),
            severity: Severity::High,
            confidence: Confidence::High,
        },
        SecretPattern {
            name: "GitHub token",
            regex: Regex::new(r"\bgh[pousr]_[A-Za-z0-9_]{30,255}\b")
                .expect("valid GitHub token regex"),
            severity: Severity::High,
            confidence: Confidence::High,
        },
        SecretPattern {
            name: "Private key",
            regex: Regex::new(r"-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----")
                .expect("valid private key regex"),
            severity: Severity::Critical,
            confidence: Confidence::High,
        },
        SecretPattern {
            name: "Live secret key",
            regex: Regex::new(r"\bsk_live_[A-Za-z0-9]{16,}\b").expect("valid live key regex"),
            severity: Severity::Critical,
            confidence: Confidence::High,
        },
        SecretPattern {
            name: "Hardcoded credential",
            regex: Regex::new(
                r#"(?i)\b(api[_-]?key|secret|token|password)\b\s*[:=]\s*["'][^"'\s]{8,}["']"#,
            )
            .expect("valid generic credential regex"),
            severity: Severity::Medium,
            confidence: Confidence::Medium,
        },
    ]
});

impl NyvraxRule for HardcodedSecretRule {
    fn id(&self) -> &'static str {
        "NYX-SECRET-001"
    }

    fn name(&self) -> &'static str {
        "Hardcoded secret"
    }

    fn scan(&self, context: &ScanContext) -> Result<Vec<Finding>, NyvraxError> {
        let mut findings = Vec::new();

        for file in &context.changed_files {
            if !should_scan(&file.path) {
                continue;
            }

            for line in &file.additions {
                for pattern in PATTERNS.iter() {
                    if !pattern.regex.is_match(&line.content) {
                        continue;
                    }

                    let line_number = line.line_number;

                    let finding_id = format!(
                        "{}:{}:{}",
                        self.id(),
                        file.path.display(),
                        line_number.unwrap_or_default(),
                    );

                    findings.push(
                        Finding::new(
                            finding_id,
                            self.id(),
                            pattern.name,
                            format!(
                                "{} appears to have been introduced in the current change.",
                                pattern.name,
                            ),
                            pattern.severity,
                            pattern.confidence,
                            FindingCategory::Secrets,
                        )
                        .at_location(
                            file.path.clone(),
                            line_number,
                            None,
                        )
                        .with_evidence(
                            "Potential secret value detected. The value has been redacted by Nyvrax.",
                        )
                        .with_remediation(
                            "Remove the credential from source code, rotate it if it was real, and load it from an appropriate secret store or environment variable.",
                        ),
                    );
                }
            }
        }

        Ok(findings)
    }
}

fn should_scan(path: &Path) -> bool {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    if file_name.starts_with(".env") {
        return true;
    }

    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some(
            "rs" | "ts"
                | "tsx"
                | "js"
                | "jsx"
                | "mjs"
                | "cjs"
                | "py"
                | "go"
                | "java"
                | "kt"
                | "kts"
                | "cs"
                | "rb"
                | "php"
                | "json"
                | "yaml"
                | "yml"
                | "toml"
        )
    )
}
