pub mod analysis;
pub mod context;
pub mod engine;
pub mod error;
pub mod findings;
pub mod git;
pub mod rules;

pub use context::{ProjectInfo, ScanContext, ScanMode};
pub use engine::{Engine, ScanResult, ScanSummary};
pub use error::NyvraxError;
pub use findings::{Confidence, Finding, FindingCategory, FindingLocation, Severity, Verdict};
pub use git::{ChangedFile, ChangedLine, GitDiff};
pub use rules::NyvraxRule;
