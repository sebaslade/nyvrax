use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Severity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingCategory {
    Auth,
    Authorization,
    Secrets,
    Injection,
    Validation,
    Dependencies,
    Webhook,
    Configuration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingLocation {
    pub file: PathBuf,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub rule_id: String,

    pub title: String,
    pub description: String,

    pub severity: Severity,
    pub confidence: Confidence,
    pub category: FindingCategory,

    pub location: Option<FindingLocation>,

    pub evidence: Option<String>,
    pub remediation: Option<String>,
    pub references: Vec<String>,
}

impl Finding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        rule_id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        severity: Severity,
        confidence: Confidence,
        category: FindingCategory,
    ) -> Self {
        Self {
            id: id.into(),
            rule_id: rule_id.into(),
            title: title.into(),
            description: description.into(),
            severity,
            confidence,
            category,
            location: None,
            evidence: None,
            remediation: None,
            references: Vec::new(),
        }
    }

    pub fn at_location(
        mut self,
        file: impl Into<PathBuf>,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Self {
        self.location = Some(FindingLocation {
            file: file.into(),
            line,
            column,
        });

        self
    }

    pub fn with_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.evidence = Some(evidence.into());
        self
    }

    pub fn with_remediation(mut self, remediation: impl Into<String>) -> Self {
        self.remediation = Some(remediation.into());
        self
    }

    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.references.push(reference.into());
        self
    }
}
