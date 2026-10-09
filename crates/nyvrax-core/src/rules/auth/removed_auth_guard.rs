use crate::{
    Confidence, Finding, FindingCategory, NyvraxError, NyvraxRule, ScanContext, Severity,
    analysis::detect_route_surfaces,
};

use super::{auth_guard_name, is_auth_guard};

pub struct RemovedAuthGuardRule;

impl NyvraxRule for RemovedAuthGuardRule {
    fn id(&self) -> &'static str {
        "NYX-AUTH-001"
    }

    fn name(&self) -> &'static str {
        "Authentication guard removed"
    }

    fn scan(&self, context: &ScanContext) -> Result<Vec<Finding>, NyvraxError> {
        let mut findings = Vec::new();

        let routes = detect_route_surfaces(context);

        for route in routes {
            let (Some(before), Some(after)) = (route.before.as_ref(), route.after.as_ref()) else {
                /*
                 * NYX-AUTH-001 is specifically
                 * about regressions on existing routes.
                 *
                 * New unprotected routes will eventually
                 * be handled by another rule.
                 */
                continue;
            };

            let before_guards: Vec<String> = before
                .handlers
                .iter()
                .filter_map(|handler| auth_guard_name(handler))
                .collect();

            if before_guards.is_empty() {
                continue;
            }

            let after_has_auth_guard = after.handlers.iter().any(|handler| is_auth_guard(handler));

            /*
             * Replacing one recognized auth guard
             * with another is not considered a
             * regression.
             */
            if after_has_auth_guard {
                continue;
            }

            let guard_names = before_guards.join(", ");

            let finding_id = format!(
                "{}:{}:{}:{}",
                self.id(),
                route.file.display(),
                route.method.as_str(),
                route.path,
            );

            let description = format!(
                "{} {} previously used an authentication guard, but no recognized authentication guard remains after the change.",
                route.method.as_str(),
                route.path,
            );

            let evidence = format!(
                "Authentication guard(s) present before the change: {guard_names}. No recognized authentication guard was found after the change."
            );

            let line = route.after_line.or(route.before_line);

            findings.push(
                Finding::new(
                    finding_id,
                    self.id(),
                    self.name(),
                    description,
                    Severity::High,
                    Confidence::High,
                    FindingCategory::Auth,
                )
                .at_location(
                    route.file,
                    line,
                    None,
                )
                .with_evidence(
                    evidence,
                )
                .with_remediation(
                    "Restore an authentication boundary for this route or replace the removed guard with an equivalent authenticated access control.",
                ),
            );
        }

        Ok(findings)
    }
}
