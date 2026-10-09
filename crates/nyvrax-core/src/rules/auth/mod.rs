//! Authentication regression rules.
//!
//! Planned for Nyvrax v0.1 after the initial
//! scanning pipeline is stable.
mod guard_classifier;
mod removed_auth_guard;

pub use guard_classifier::{auth_guard_name, is_auth_guard};

pub use removed_auth_guard::RemovedAuthGuardRule;
