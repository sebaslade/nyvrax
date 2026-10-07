use std::collections::BTreeMap;

use nyvrax_core::{ScanResult, Severity};
use serde_json::{Value, json};

use crate::{Reporter, ReporterError};

pub struct SarifReporter;

impl Reporter for SarifReporter {
    fn render(&self, result: &ScanResult) -> Result<String, ReporterError> {
        let mut rule_map = BTreeMap::<String, String>::new();

        for finding in &result.findings {
            rule_map
                .entry(finding.rule_id.clone())
                .or_insert_with(|| finding.title.clone());
        }

        let rules: Vec<Value> = rule_map
            .into_iter()
            .map(|(id, title)| {
                json!({
                    "id": id,
                    "shortDescription": {
                        "text": title
                    }
                })
            })
            .collect();

        let results: Vec<Value> = result
            .findings
            .iter()
            .map(|finding| {
                let locations = match &finding.location {
                    Some(location) => {
                        let mut region = serde_json::Map::new();

                        if let Some(line) = location.line {
                            region.insert("startLine".into(), json!(line));
                        }

                        vec![json!({
                            "physicalLocation": {
                                "artifactLocation": {
                                    "uri": location.file.to_string_lossy()
                                },
                                "region": region
                            }
                        })]
                    }
                    None => Vec::new(),
                };

                json!({
                    "ruleId": finding.rule_id,
                    "level": sarif_level(
                        finding.severity
                    ),
                    "message": {
                        "text": finding.description
                    },
                    "locations": locations
                })
            })
            .collect();

        let document = json!({
            "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
            "version": "2.1.0",
            "runs": [
                {
                    "tool": {
                        "driver": {
                            "name": "Nyvrax",
                            "version": env!(
                                "CARGO_PKG_VERSION"
                            ),
                            "rules": rules
                        }
                    },
                    "results": results
                }
            ]
        });

        Ok(serde_json::to_string_pretty(&document)?)
    }
}

fn sarif_level(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical | Severity::High => "error",

        Severity::Medium | Severity::Low => "warning",

        Severity::Info => "note",
    }
}
