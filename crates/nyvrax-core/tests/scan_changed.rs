use std::path::PathBuf;

use nyvrax_core::{ChangedFile, ChangedLine, Engine, ScanContext, Verdict};

#[test]
fn detects_a_secret_in_added_code() {
    let mut file = ChangedFile::new("src/config.ts");

    file.additions.push(ChangedLine {
        line_number: Some(12),

        content: r#"const password = "super-secret-value";"#.to_owned(),
    });

    let context = ScanContext::changed(PathBuf::from("."), vec![file]);

    let result = Engine::default()
        .scan(&context)
        .expect("scan should succeed");

    assert_eq!(result.findings.len(), 1,);

    assert_eq!(result.findings[0].rule_id, "NYX-SECRET-001",);

    assert_eq!(result.verdict, Verdict::Warn,);
}

#[test]
fn passes_when_change_contains_no_secret() {
    let mut file = ChangedFile::new("src/main.ts");

    file.additions.push(ChangedLine {
        line_number: Some(1),
        content: r#"console.log("hello");"#.to_owned(),
    });

    let context = ScanContext::changed(PathBuf::from("."), vec![file]);

    let result = Engine::default()
        .scan(&context)
        .expect("scan should succeed");

    assert!(result.findings.is_empty());

    assert_eq!(result.verdict, Verdict::Pass,);
}
