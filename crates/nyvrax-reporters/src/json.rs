use nyvrax_core::ScanResult;

use crate::{Reporter, ReporterError};

pub struct JsonReporter;

impl Reporter for JsonReporter {
    fn render(&self, result: &ScanResult) -> Result<String, ReporterError> {
        Ok(serde_json::to_string_pretty(result)?)
    }
}
