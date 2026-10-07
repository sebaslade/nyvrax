use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::NyvraxError;

pub fn repository_root(start: &Path) -> Result<PathBuf, NyvraxError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(start)
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if !output.status.success() {
        return Err(NyvraxError::NotGitRepository(start.to_path_buf()));
    }

    let root = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    if root.is_empty() {
        return Err(NyvraxError::NotGitRepository(start.to_path_buf()));
    }

    Ok(PathBuf::from(root))
}
