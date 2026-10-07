use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{ChangedFile, ProjectInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanMode {
    Repository,
    Changed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanContext {
    pub root: PathBuf,
    pub mode: ScanMode,
    pub changed_files: Vec<ChangedFile>,
    pub project: ProjectInfo,
}

impl ScanContext {
    pub fn changed(root: impl Into<PathBuf>, changed_files: Vec<ChangedFile>) -> Self {
        let root = root.into();

        let project = ProjectInfo::detect(&root, &changed_files);

        Self {
            root,
            mode: ScanMode::Changed,
            changed_files,
            project,
        }
    }
}
