use std::{path::Path, process::Command};

use serde::{Deserialize, Serialize};

use crate::NyvraxError;

use super::{ChangedFile, ChangedLine, DiffHunk, DiffLine, DiffLineKind};

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
        .args(["diff", "--no-ext-diff", "--unified=5"]);

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

    let mut current_hunk: Option<DiffHunk> = None;

    let mut old_line = None;
    let mut new_line = None;

    for line in raw.lines() {
        /*
         * New file section
         */
        if line.starts_with("diff --git ") {
            flush_hunk(&mut current_file, &mut current_hunk);

            if let Some(file) = current_file.take() {
                files.push(file);
            }

            let path = line
                .split_whitespace()
                .nth(3)
                .ok_or_else(|| NyvraxError::InvalidDiff(line.to_owned()))?
                .strip_prefix("b/")
                .unwrap_or_default();

            current_file = Some(ChangedFile::new(path));

            old_line = None;
            new_line = None;

            continue;
        }

        /*
         * New hunk
         *
         * Example:
         *
         * @@ -10,5 +10,4 @@
         */
        if line.starts_with("@@") {
            flush_hunk(&mut current_file, &mut current_hunk);

            let (old_start, old_count, new_start, new_count) = parse_hunk_header(line)?;

            old_line = Some(old_start);
            new_line = Some(new_start);

            current_hunk = Some(DiffHunk::new(old_start, old_count, new_start, new_count));

            continue;
        }

        let Some(file) = current_file.as_mut() else {
            continue;
        };

        /*
         * Git metadata, not code.
         */
        if line.starts_with("+++ ")
            || line.starts_with("--- ")
            || line.starts_with("index ")
            || line.starts_with("new file mode ")
            || line.starts_with("deleted file mode ")
        {
            continue;
        }

        /*
         * Added line
         */
        if let Some(content) = line.strip_prefix('+') {
            file.additions.push(ChangedLine {
                line_number: new_line,
                content: content.to_owned(),
            });

            if let Some(hunk) = current_hunk.as_mut() {
                hunk.lines.push(DiffLine {
                    kind: DiffLineKind::Addition,

                    old_line_number: None,
                    new_line_number: new_line,

                    content: content.to_owned(),
                });
            }

            if let Some(value) = new_line.as_mut() {
                *value += 1;
            }

            continue;
        }

        /*
         * Deleted line
         */
        if let Some(content) = line.strip_prefix('-') {
            file.deletions.push(ChangedLine {
                line_number: old_line,
                content: content.to_owned(),
            });

            if let Some(hunk) = current_hunk.as_mut() {
                hunk.lines.push(DiffLine {
                    kind: DiffLineKind::Deletion,

                    old_line_number: old_line,
                    new_line_number: None,

                    content: content.to_owned(),
                });
            }

            if let Some(value) = old_line.as_mut() {
                *value += 1;
            }

            continue;
        }

        /*
         * "\ No newline at end of file"
         */
        if line.starts_with('\\') {
            continue;
        }

        /*
         * Context line.
         *
         * Git prefixes context lines with one space.
         */
        if let Some(hunk) = current_hunk.as_mut() {
            let content = line.strip_prefix(' ').unwrap_or(line);

            hunk.lines.push(DiffLine {
                kind: DiffLineKind::Context,

                old_line_number: old_line,
                new_line_number: new_line,

                content: content.to_owned(),
            });
        }

        if let Some(value) = old_line.as_mut() {
            *value += 1;
        }

        if let Some(value) = new_line.as_mut() {
            *value += 1;
        }
    }

    flush_hunk(&mut current_file, &mut current_hunk);

    if let Some(file) = current_file {
        files.push(file);
    }

    Ok(files)
}

fn flush_hunk(current_file: &mut Option<ChangedFile>, current_hunk: &mut Option<DiffHunk>) {
    let Some(hunk) = current_hunk.take() else {
        return;
    };

    if let Some(file) = current_file.as_mut() {
        file.hunks.push(hunk);
    }
}

fn parse_hunk_header(line: &str) -> Result<(usize, usize, usize, usize), NyvraxError> {
    let mut parts = line.split_whitespace();

    let opening = parts.next();

    if opening != Some("@@") {
        return Err(NyvraxError::InvalidDiff(line.to_owned()));
    }

    let old_range = parts
        .next()
        .ok_or_else(|| NyvraxError::InvalidDiff(line.to_owned()))?;

    let new_range = parts
        .next()
        .ok_or_else(|| NyvraxError::InvalidDiff(line.to_owned()))?;

    let (old_start, old_count) = parse_range(old_range, '-')?;

    let (new_start, new_count) = parse_range(new_range, '+')?;

    Ok((old_start, old_count, new_start, new_count))
}

fn parse_range(value: &str, prefix: char) -> Result<(usize, usize), NyvraxError> {
    let value = value
        .strip_prefix(prefix)
        .ok_or_else(|| NyvraxError::InvalidDiff(value.to_owned()))?;

    let (start, count) = match value.split_once(',') {
        Some((start, count)) => (start, count),

        None => (value, "1"),
    };

    let start = start
        .parse::<usize>()
        .map_err(|_| NyvraxError::InvalidDiff(value.to_owned()))?;

    let count = count
        .parse::<usize>()
        .map_err(|_| NyvraxError::InvalidDiff(value.to_owned()))?;

    Ok((start, count))
}
