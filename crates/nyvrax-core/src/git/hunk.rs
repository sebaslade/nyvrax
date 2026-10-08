use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffLineKind {
    Context,
    Addition,
    Deletion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub kind: DiffLineKind,

    pub old_line_number: Option<usize>,
    pub new_line_number: Option<usize>,

    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_count: usize,

    pub new_start: usize,
    pub new_count: usize,

    pub lines: Vec<DiffLine>,
}

impl DiffHunk {
    pub fn new(old_start: usize, old_count: usize, new_start: usize, new_count: usize) -> Self {
        Self {
            old_start,
            old_count,
            new_start,
            new_count,
            lines: Vec::new(),
        }
    }

    /// Reconstructs the hunk as it existed before the change.
    pub fn before_text(&self) -> String {
        self.lines
            .iter()
            .filter(|line| line.kind != DiffLineKind::Addition)
            .map(|line| line.content.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Reconstructs the hunk after the change.
    pub fn after_text(&self) -> String {
        self.lines
            .iter()
            .filter(|line| line.kind != DiffLineKind::Deletion)
            .map(|line| line.content.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn additions(&self) -> impl Iterator<Item = &DiffLine> {
        self.lines
            .iter()
            .filter(|line| line.kind == DiffLineKind::Addition)
    }

    pub fn deletions(&self) -> impl Iterator<Item = &DiffLine> {
        self.lines
            .iter()
            .filter(|line| line.kind == DiffLineKind::Deletion)
    }
}
