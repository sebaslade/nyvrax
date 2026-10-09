use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Head,
}

impl HttpMethod {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "get" => Some(Self::Get),
            "post" => Some(Self::Post),
            "put" => Some(Self::Put),
            "patch" => Some(Self::Patch),
            "delete" => Some(Self::Delete),
            "options" => Some(Self::Options),
            "head" => Some(Self::Head),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Options => "OPTIONS",
            Self::Head => "HEAD",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteState {
    pub handlers: Vec<String>,
}

impl RouteState {
    pub fn new(handlers: Vec<String>) -> Self {
        Self { handlers }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteSurface {
    pub file: PathBuf,

    pub method: HttpMethod,
    pub path: String,

    pub before: Option<RouteState>,
    pub after: Option<RouteState>,

    pub before_line: Option<usize>,
    pub after_line: Option<usize>,
}

impl RouteSurface {
    pub fn existed_before(&self) -> bool {
        self.before.is_some()
    }

    pub fn exists_after(&self) -> bool {
        self.after.is_some()
    }

    pub fn was_modified(&self) -> bool {
        self.before.is_some() && self.after.is_some() && self.before != self.after
    }
}
