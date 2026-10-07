use std::{collections::BTreeSet, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::ChangedFile;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub package_manager: Option<String>,
}

impl ProjectInfo {
    pub fn detect(root: &Path, files: &[ChangedFile]) -> Self {
        let mut languages = BTreeSet::new();

        for file in files {
            let extension = file.path.extension().and_then(|value| value.to_str());

            let language = match extension {
                Some("rs") => Some("rust"),
                Some("ts") | Some("tsx") => Some("typescript"),
                Some("js") | Some("jsx") => Some("javascript"),
                Some("py") => Some("python"),
                Some("go") => Some("go"),
                Some("java") => Some("java"),
                Some("kt") | Some("kts") => Some("kotlin"),
                Some("cs") => Some("csharp"),
                Some("rb") => Some("ruby"),
                Some("php") => Some("php"),
                _ => None,
            };

            if let Some(language) = language {
                languages.insert(language.to_owned());
            }
        }

        let package_manager = detect_package_manager(root);

        let frameworks = detect_frameworks(root);

        Self {
            languages: languages.into_iter().collect(),
            frameworks,
            package_manager,
        }
    }
}

fn detect_package_manager(root: &Path) -> Option<String> {
    if root.join("pnpm-lock.yaml").exists() {
        return Some("pnpm".into());
    }

    if root.join("yarn.lock").exists() {
        return Some("yarn".into());
    }

    if root.join("package-lock.json").exists() {
        return Some("npm".into());
    }

    if root.join("bun.lock").exists() || root.join("bun.lockb").exists() {
        return Some("bun".into());
    }

    if root.join("Cargo.lock").exists() {
        return Some("cargo".into());
    }

    None
}

fn detect_frameworks(root: &Path) -> Vec<String> {
    let package_json = root.join("package.json");

    let Ok(contents) = fs::read_to_string(package_json) else {
        return Vec::new();
    };

    let candidates = [
        ("next", "nextjs"),
        ("express", "express"),
        ("@nestjs/core", "nestjs"),
        ("hono", "hono"),
        ("fastify", "fastify"),
    ];

    candidates
        .into_iter()
        .filter(|(needle, _)| contents.contains(&format!("\"{needle}\"")))
        .map(|(_, framework)| framework.to_owned())
        .collect()
}
