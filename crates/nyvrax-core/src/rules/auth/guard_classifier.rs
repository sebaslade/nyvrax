pub fn is_auth_guard(handler: &str) -> bool {
    auth_guard_name(handler).is_some()
}

pub fn auth_guard_name(handler: &str) -> Option<String> {
    let handler = handler.trim();

    if handler.is_empty() {
        return None;
    }

    let head = handler
        .strip_prefix("await ")
        .unwrap_or(handler)
        .split('(')
        .next()
        .unwrap_or(handler)
        .trim();

    let compact = head
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();

    const EXACT_GUARDS: &[&str] = &[
        "requireauth",
        "requireauthentication",
        "ensureauthenticated",
        "isauthenticated",
        "authenticate",
        "authmiddleware",
        "authenticationmiddleware",
        "withauth",
        "requireuser",
        "requiresession",
        "verifytoken",
        "verifyjwt",
        "requirejwt",
    ];

    if EXACT_GUARDS.contains(&compact.as_str()) {
        return Some(handler.to_owned());
    }

    let words = identifier_words(head);

    /*
     * authenticate(...)
     * passport.authenticate(...)
     */
    if words.iter().any(|word| word == "authenticate") {
        return Some(handler.to_owned());
    }

    let has_auth_marker = words
        .iter()
        .any(|word| matches!(word.as_str(), "auth" | "authentication" | "authenticated"));

    let has_guard_action = words.iter().any(|word| {
        matches!(
            word.as_str(),
            "require"
                | "ensure"
                | "verify"
                | "check"
                | "guard"
                | "middleware"
                | "protect"
                | "with"
                | "is"
        )
    });

    if has_auth_marker && has_guard_action {
        return Some(handler.to_owned());
    }

    /*
     * Token/session/JWT checks are also common
     * authentication boundaries.
     */
    let has_identity_marker = words
        .iter()
        .any(|word| matches!(word.as_str(), "token" | "jwt" | "session"));

    let has_verification_action = words.iter().any(|word| {
        matches!(
            word.as_str(),
            "require" | "ensure" | "verify" | "validate" | "check"
        )
    });

    if has_identity_marker && has_verification_action {
        return Some(handler.to_owned());
    }

    None
}

fn identifier_words(value: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();

    let mut previous_was_lowercase = false;

    for character in value.chars() {
        if !character.is_ascii_alphanumeric() {
            flush_word(&mut words, &mut current);

            previous_was_lowercase = false;

            continue;
        }

        if character.is_ascii_uppercase() && previous_was_lowercase {
            flush_word(&mut words, &mut current);
        }

        current.push(character.to_ascii_lowercase());

        previous_was_lowercase = character.is_ascii_lowercase();
    }

    flush_word(&mut words, &mut current);

    words
}

fn flush_word(words: &mut Vec<String>, current: &mut String) {
    if current.is_empty() {
        return;
    }

    words.push(std::mem::take(current));
}
