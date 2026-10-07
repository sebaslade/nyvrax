use std::collections::BTreeSet;

use crate::ScanContext;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttackSurface {
    Api,
    Authentication,
    Authorization,
    Webhook,
    Secrets,
}

pub fn detect_attack_surfaces(context: &ScanContext) -> Vec<AttackSurface> {
    let mut surfaces = BTreeSet::new();

    for file in &context.changed_files {
        let path = file.path.to_string_lossy().to_lowercase();

        if path.contains("/api/") || path.contains("route") || path.contains("controller") {
            surfaces.insert(AttackSurface::Api);
        }

        if path.contains("auth") || path.contains("session") {
            surfaces.insert(AttackSurface::Authentication);
        }

        if path.contains("permission") || path.contains("authorization") || path.contains("policy")
        {
            surfaces.insert(AttackSurface::Authorization);
        }

        if path.contains("webhook") {
            surfaces.insert(AttackSurface::Webhook);
        }

        if file.additions.iter().any(|line| {
            let line = line.content.to_lowercase();

            line.contains("secret") || line.contains("api_key") || line.contains("apikey")
        }) {
            surfaces.insert(AttackSurface::Secrets);
        }
    }

    surfaces.into_iter().collect()
}
