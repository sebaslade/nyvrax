use std::path::PathBuf;

use nyvrax_core::{Engine, ScanContext, Verdict, git::parse_diff};

fn scan_diff(diff: &str) -> nyvrax_core::ScanResult {
    let files = parse_diff(diff).expect("diff should parse");

    let context = ScanContext::changed(PathBuf::from("."), files);

    Engine::default()
        .scan(&context)
        .expect("scan should succeed")
}

#[test]
fn blocks_when_auth_guard_is_removed() {
    let diff = r#"diff --git a/src/routes.ts b/src/routes.ts
index 1111111..2222222 100644
--- a/src/routes.ts
+++ b/src/routes.ts
@@ -10,5 +10,4 @@
 router.get(
   "/orders/:id",
-  requireAuth,
   getOrder,
 );
"#;

    let result = scan_diff(diff);

    assert_eq!(result.verdict, Verdict::Block,);

    let finding = result
        .findings
        .iter()
        .find(|finding| finding.rule_id == "NYX-AUTH-001")
        .expect("auth regression should be detected");

    assert_eq!(finding.rule_id, "NYX-AUTH-001",);
}

#[test]
fn passes_when_auth_guard_is_added() {
    let diff = r#"diff --git a/src/routes.ts b/src/routes.ts
index 1111111..2222222 100644
--- a/src/routes.ts
+++ b/src/routes.ts
@@ -10,4 +10,5 @@
 router.get(
   "/orders/:id",
+  requireAuth,
   getOrder,
 );
"#;

    let result = scan_diff(diff);

    assert!(
        result
            .findings
            .iter()
            .all(|finding| { finding.rule_id != "NYX-AUTH-001" })
    );
}

#[test]
fn passes_when_auth_guard_is_replaced() {
    let diff = r#"diff --git a/src/routes.ts b/src/routes.ts
index 1111111..2222222 100644
--- a/src/routes.ts
+++ b/src/routes.ts
@@ -10,5 +10,5 @@
 router.get(
   "/orders/:id",
-  requireAuth,
+  ensureAuthenticated,
   getOrder,
 );
"#;

    let result = scan_diff(diff);

    assert!(
        result
            .findings
            .iter()
            .all(|finding| { finding.rule_id != "NYX-AUTH-001" })
    );
}

#[test]
fn passes_when_unrelated_handler_changes() {
    let diff = r#"diff --git a/src/routes.ts b/src/routes.ts
index 1111111..2222222 100644
--- a/src/routes.ts
+++ b/src/routes.ts
@@ -10,5 +10,5 @@
 router.get(
   "/orders/:id",
   requireAuth,
-  getOrder,
+  getOrderV2,
 );
"#;

    let result = scan_diff(diff);

    assert!(
        result
            .findings
            .iter()
            .all(|finding| { finding.rule_id != "NYX-AUTH-001" })
    );
}

#[test]
fn does_not_invent_auth_regression_when_route_was_already_public() {
    let diff = r#"diff --git a/src/routes.ts b/src/routes.ts
index 1111111..2222222 100644
--- a/src/routes.ts
+++ b/src/routes.ts
@@ -10,4 +10,4 @@
 router.get(
   "/orders/:id",
-  getOrder,
+  getOrderV2,
 );
"#;

    let result = scan_diff(diff);

    assert!(
        result
            .findings
            .iter()
            .all(|finding| { finding.rule_id != "NYX-AUTH-001" })
    );
}

#[test]
fn does_not_treat_get_author_as_authentication() {
    let diff = r#"diff --git a/src/routes.ts b/src/routes.ts
index 1111111..2222222 100644
--- a/src/routes.ts
+++ b/src/routes.ts
@@ -10,4 +10,3 @@
 router.get(
   "/authors/:id",
-  getAuthor,
 );
"#;

    let result = scan_diff(diff);

    assert!(
        result
            .findings
            .iter()
            .all(|finding| { finding.rule_id != "NYX-AUTH-001" })
    );
}
