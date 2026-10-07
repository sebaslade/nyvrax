use std::cmp::Reverse;

use serde::{Deserialize, Serialize};

use crate::{
    Finding, NyvraxError, NyvraxRule, ScanContext, Severity, Verdict,
    rules::secrets::HardcodedSecretRule,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScanSummary {
    pub total: usize,

    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub info: usize,
}

impl ScanSummary {
    pub fn from_findings(findings: &[Finding]) -> Self {
        let mut summary = Self {
            total: findings.len(),
            ..Self::default()
        };

        for finding in findings {
            match finding.severity {
                Severity::Critical => summary.critical += 1,
                Severity::High => summary.high += 1,
                Severity::Medium => summary.medium += 1,
                Severity::Low => summary.low += 1,
                Severity::Info => summary.info += 1,
            }
        }

        summary
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub engine: String,
    pub verdict: Verdict,

    pub findings: Vec<Finding>,
    pub summary: ScanSummary,

    pub scanned_files: usize,
    pub rules_executed: usize,
}

pub struct Engine {
    rules: Vec<Box<dyn NyvraxRule>>,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            rules: vec![Box::new(HardcodedSecretRule)],
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn with_rule<R>(mut self, rule: R) -> Self
    where
        R: NyvraxRule + 'static,
    {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn scan(&self, context: &ScanContext) -> Result<ScanResult, NyvraxError> {
        let mut findings = Vec::new();

        for rule in &self.rules {
            findings.extend(rule.scan(context)?);
        }

        findings.sort_by_key(|finding| Reverse(finding.severity.rank()));

        let verdict = Verdict::from_findings(&findings);

        let summary = ScanSummary::from_findings(&findings);

        Ok(ScanResult {
            engine: "nyvrax".to_owned(),
            verdict,
            findings,
            summary,
            scanned_files: context.changed_files.len(),
            rules_executed: self.rules.len(),
        })
    }
}
