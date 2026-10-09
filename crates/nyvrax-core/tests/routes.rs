use std::path::PathBuf;

use nyvrax_core::{
    ScanContext,
    analysis::{HttpMethod, detect_route_surfaces},
    git::parse_diff,
};

#[test]
fn detects_removed_handler_from_existing_route() {
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

    let files = parse_diff(diff).expect("diff should parse");

    let context = ScanContext::changed(PathBuf::from("."), files);

    let routes = detect_route_surfaces(&context);

    assert_eq!(routes.len(), 1,);

    let route = &routes[0];

    assert_eq!(route.method, HttpMethod::Get,);

    assert_eq!(route.path, "/orders/:id",);

    let before = route.before.as_ref().expect("route should exist before");

    let after = route.after.as_ref().expect("route should exist after");

    assert_eq!(before.handlers, vec!["requireAuth", "getOrder",],);

    assert_eq!(after.handlers, vec!["getOrder",],);

    assert!(route.was_modified());
}

#[test]
fn detects_single_line_route() {
    let diff = r#"diff --git a/src/routes.ts b/src/routes.ts
index 1111111..2222222 100644
--- a/src/routes.ts
+++ b/src/routes.ts
@@ -4 +4 @@
-router.post("/users", requireAuth, createUser);
+router.post("/users", createUser);
"#;

    let files = parse_diff(diff).expect("diff should parse");

    let context = ScanContext::changed(PathBuf::from("."), files);

    let routes = detect_route_surfaces(&context);

    assert_eq!(routes.len(), 1,);

    let route = &routes[0];

    assert_eq!(route.method, HttpMethod::Post,);

    assert_eq!(route.path, "/users",);

    assert_eq!(
        route.before.as_ref().unwrap().handlers,
        vec!["requireAuth", "createUser",],
    );

    assert_eq!(route.after.as_ref().unwrap().handlers, vec!["createUser",],);
}

#[test]
fn preserves_inline_handler_as_single_argument() {
    let diff = r#"diff --git a/src/routes.ts b/src/routes.ts
index 1111111..2222222 100644
--- a/src/routes.ts
+++ b/src/routes.ts
@@ -10,4 +10,5 @@
 router.get(
   "/health",
+  requireAuth,
   async (req, res) => {
     res.json({ ok: true });
   },
 );
"#;

    let files = parse_diff(diff).expect("diff should parse");

    let context = ScanContext::changed(PathBuf::from("."), files);

    let routes = detect_route_surfaces(&context);

    assert_eq!(routes.len(), 1,);

    let route = &routes[0];

    let after = route.after.as_ref().unwrap();

    assert_eq!(after.handlers.len(), 2,);

    assert_eq!(after.handlers[0], "requireAuth",);

    assert!(after.handlers[1].starts_with("async (req, res) =>",));
}

#[test]
fn detects_added_handler_to_existing_route() {
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

    let files = parse_diff(diff).expect("diff should parse");

    let context = ScanContext::changed(PathBuf::from("."), files);

    let routes = detect_route_surfaces(&context);

    assert_eq!(routes.len(), 1,);

    let route = &routes[0];

    let before = route.before.as_ref().expect("route should exist before");

    let after = route.after.as_ref().expect("route should exist after");

    assert_eq!(before.handlers, vec!["getOrder",],);

    assert_eq!(after.handlers, vec!["requireAuth", "getOrder",],);
}
