use nyvrax_core::{Confidence, Finding, FindingCategory, Severity, Verdict};

fn finding(severity: Severity) -> Finding {
    Finding::new(
        "test-finding",
        "NYX-TEST-001",
        "Test finding",
        "Finding created by test.",
        severity,
        Confidence::High,
        FindingCategory::Configuration,
    )
}

#[test]
fn blocks_on_high_severity() {
    let findings = vec![finding(Severity::High)];

    assert_eq!(Verdict::from_findings(&findings), Verdict::Block,);
}

#[test]
fn warns_on_medium_severity() {
    let findings = vec![finding(Severity::Medium)];

    assert_eq!(Verdict::from_findings(&findings), Verdict::Warn,);
}

#[test]
fn passes_without_findings() {
    assert_eq!(Verdict::from_findings(&[]), Verdict::Pass,);
}
