use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedLine {
    pub line_number: Option<usize>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedFile {
    pub path: PathBuf,
    pub additions: Vec<ChangedLine>,
    pub deletions: Vec<ChangedLine>,
}

impl ChangedFile {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            additions: Vec::new(),
            deletions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.additions.is_empty() && self.deletions.is_empty()
    }
}
