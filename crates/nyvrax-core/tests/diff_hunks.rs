use nyvrax_core::{DiffLineKind, git::parse_diff};

#[test]
fn reconstructs_before_and_after_from_diff_hunk() {
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

    assert_eq!(files.len(), 1);

    let file = &files[0];

    assert_eq!(file.path.to_string_lossy(), "src/routes.ts",);

    assert_eq!(file.hunks.len(), 1,);

    let hunk = &file.hunks[0];

    assert_eq!(hunk.old_start, 10,);

    assert_eq!(hunk.old_count, 5,);

    assert_eq!(hunk.new_start, 10,);

    assert_eq!(hunk.new_count, 4,);

    assert_eq!(hunk.deletions().count(), 1,);

    assert_eq!(hunk.additions().count(), 0,);

    assert!(hunk.lines.iter().any(|line| {
        line.kind == DiffLineKind::Deletion && line.content.contains("requireAuth")
    }));

    let before = hunk.before_text();

    let after = hunk.after_text();

    assert!(before.contains("requireAuth"));

    assert!(!after.contains("requireAuth"));

    assert!(before.contains(r#""/orders/:id""#));

    assert!(after.contains(r#""/orders/:id""#));
}

#[test]
fn reconstructs_added_security_guard() {
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

    let hunk = &files[0].hunks[0];

    let before = hunk.before_text();

    let after = hunk.after_text();

    assert!(!before.contains("requireAuth"));

    assert!(after.contains("requireAuth"));
}
