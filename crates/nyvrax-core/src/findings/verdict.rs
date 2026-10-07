use serde::{Deserialize, Serialize};

use super::{Finding, Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    Pass,
    Warn,
    Block,
}

impl Verdict {
    pub fn from_findings(findings: &[Finding]) -> Self {
        if findings
            .iter()
            .any(|finding| matches!(finding.severity, Severity::Critical | Severity::High))
        {
            return Self::Block;
        }

        if findings
            .iter()
            .any(|finding| matches!(finding.severity, Severity::Medium | Severity::Low))
        {
            return Self::Warn;
        }

        Self::Pass
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Warn => "WARN",
            Self::Block => "BLOCK",
        }
    }
}
