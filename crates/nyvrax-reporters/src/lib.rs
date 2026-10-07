mod json;
mod sarif;
mod terminal;

use nyvrax_core::ScanResult;
use thiserror::Error;

pub use json::JsonReporter;
pub use sarif::SarifReporter;
pub use terminal::TerminalReporter;

#[derive(Debug, Error)]
pub enum ReporterError {
    #[error("Unable to serialize report: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub trait Reporter {
    fn render(&self, result: &ScanResult) -> Result<String, ReporterError>;
}
