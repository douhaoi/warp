use warp_core::channel::ProductProfile;

use super::{can_restore_persisted_user, should_initialize_test_user};

#[test]
fn terminal_only_does_not_restore_or_persist_user_credentials() {
    assert!(can_restore_persisted_user(ProductProfile::Full));
    assert!(!can_restore_persisted_user(ProductProfile::TerminalOnly));
}

#[test]
fn terminal_only_rejects_test_user_initialization_before_test_shortcuts() {
    for test_shortcut_enabled in [false, true] {
        assert!(!should_initialize_test_user(
            ProductProfile::TerminalOnly,
            test_shortcut_enabled
        ));
    }

    assert!(should_initialize_test_user(ProductProfile::Full, true));
    assert!(!should_initialize_test_user(ProductProfile::Full, false));
}
