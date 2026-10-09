use nyvrax_core::rules::auth::is_auth_guard;

#[test]
fn recognizes_common_auth_guards() {
    assert!(is_auth_guard("requireAuth"));

    assert!(is_auth_guard("ensureAuthenticated"));

    assert!(is_auth_guard("authMiddleware"));

    assert!(is_auth_guard(r#"passport.authenticate("jwt")"#));

    assert!(is_auth_guard("verifyToken"));

    assert!(is_auth_guard("requireSession"));
}

#[test]
fn rejects_unrelated_handlers() {
    assert!(!is_auth_guard("getOrder"));

    assert!(!is_auth_guard("validateOrder"));

    assert!(!is_auth_guard("getAuthor"));

    assert!(!is_auth_guard("authorService"));

    assert!(!is_auth_guard("createAuthenticationLog"));
}
