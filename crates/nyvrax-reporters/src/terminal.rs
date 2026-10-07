use std::fmt::Write;

use nyvrax_core::ScanResult;

use crate::{Reporter, ReporterError};

pub struct TerminalReporter;

impl Reporter for TerminalReporter {
    fn render(&self, result: &ScanResult) -> Result<String, ReporterError> {
        let mut output = String::new();

        writeln!(output, "NYVRAX").unwrap();
        writeln!(output).unwrap();

        writeln!(output, "Scanned files     {}", result.scanned_files).unwrap();

        writeln!(output, "Security checks   {}", result.rules_executed).unwrap();

        writeln!(output, "Findings          {}", result.summary.total).unwrap();

        writeln!(output).unwrap();

        if result.findings.is_empty() {
            writeln!(output, "✓ No security findings detected.").unwrap();
        }

        for finding in &result.findings {
            writeln!(output, "{}  {}", finding.severity.as_str(), finding.rule_id,).unwrap();

            writeln!(output, "{}", finding.title).unwrap();

            if let Some(location) = &finding.location {
                match location.line {
                    Some(line) => {
                        writeln!(output, "{}:{}", location.file.display(), line,).unwrap();
                    }
                    None => {
                        writeln!(output, "{}", location.file.display(),).unwrap();
                    }
                }
            }

            writeln!(output, "{}", finding.description).unwrap();

            if let Some(remediation) = &finding.remediation {
                writeln!(output, "Remediation: {remediation}").unwrap();
            }

            writeln!(output).unwrap();
        }

        writeln!(output, "Verdict: {}", result.verdict.as_str()).unwrap();

        Ok(output)
    }
}
