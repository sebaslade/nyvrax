use std::{path::Path, process::Command};

use serde::{Deserialize, Serialize};

use crate::{ChangedFile, ChangedLine, NyvraxError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitDiff {
    pub raw: String,
    pub files: Vec<ChangedFile>,
}

pub fn load_changed_diff(root: &Path, base: Option<&str>) -> Result<GitDiff, NyvraxError> {
    let mut command = Command::new("git");

    command
        .arg("-C")
        .arg(root)
        .args(["diff", "--no-ext-diff", "--unified=0"]);

    match base {
        Some(base) => {
            command.arg(format!("{base}...HEAD"));
        }
        None => {
            command.arg("HEAD");
        }
    }

    let output = command.output()?;

    if !output.status.success() {
        return Err(NyvraxError::GitCommand {
            command: format!("{command:?}"),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        });
    }

    let raw = String::from_utf8_lossy(&output.stdout).into_owned();

    let files = parse_diff(&raw)?;

    Ok(GitDiff { raw, files })
}

pub fn parse_diff(raw: &str) -> Result<Vec<ChangedFile>, NyvraxError> {
    let mut files = Vec::new();
    let mut current_file: Option<ChangedFile> = None;

    let mut old_line = None;
    let mut new_line = None;

    for line in raw.lines() {
        if line.starts_with("diff --git ") {
            if let Some(file) = current_file.take() {
                files.push(file);
            }

            let path = line
                .split_whitespace()
                .nth(3)
                .ok_or_else(|| NyvraxError::InvalidDiff(line.to_owned()))?
                .strip_prefix("b/")
                .unwrap_or_else(|| line.split_whitespace().nth(3).unwrap_or_default());

            current_file = Some(ChangedFile::new(path));

            old_line = None;
            new_line = None;

            continue;
        }

        if line.starts_with("@@") {
            let mut parts = line.split_whitespace();

            let _opening = parts.next();

            let old_range = parts
                .next()
                .ok_or_else(|| NyvraxError::InvalidDiff(line.to_owned()))?;

            let new_range = parts
                .next()
                .ok_or_else(|| NyvraxError::InvalidDiff(line.to_owned()))?;

            old_line = Some(parse_range_start(old_range, '-')?);

            new_line = Some(parse_range_start(new_range, '+')?);

            continue;
        }

        let Some(file) = current_file.as_mut() else {
            continue;
        };

        if line.starts_with("+++") || line.starts_with("---") {
            continue;
        }

        if let Some(content) = line.strip_prefix('+') {
            file.additions.push(ChangedLine {
                line_number: new_line,
                content: content.to_owned(),
            });

            if let Some(value) = new_line.as_mut() {
                *value += 1;
            }

            continue;
        }

        if let Some(content) = line.strip_prefix('-') {
            file.deletions.push(ChangedLine {
                line_number: old_line,
                content: content.to_owned(),
            });

            if let Some(value) = old_line.as_mut() {
                *value += 1;
            }

            continue;
        }

        if !line.starts_with('\\') {
            if let Some(value) = old_line.as_mut() {
                *value += 1;
            }

            if let Some(value) = new_line.as_mut() {
                *value += 1;
            }
        }
    }

    if let Some(file) = current_file {
        files.push(file);
    }

    Ok(files)
}

fn parse_range_start(value: &str, prefix: char) -> Result<usize, NyvraxError> {
    let value = value
        .strip_prefix(prefix)
        .ok_or_else(|| NyvraxError::InvalidDiff(value.to_owned()))?;

    let start = value.split(',').next().unwrap_or(value);

    start
        .parse::<usize>()
        .map_err(|_| NyvraxError::InvalidDiff(value.to_owned()))
}
