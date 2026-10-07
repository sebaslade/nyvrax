use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NyvraxError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("'{0}' is not inside a Git repository")]
    NotGitRepository(PathBuf),

    #[error("Git command failed: {command}\n{stderr}")]
    GitCommand { command: String, stderr: String },

    #[error("Unable to parse Git diff: {0}")]
    InvalidDiff(String),
}
