use super::*;

#[test]
fn terminal_only_profile_disables_harness_refresh() {
    assert!(!harness_refresh_is_enabled_for_profile(
        ProductProfile::TerminalOnly
    ));
}

#[test]
fn full_profile_allows_harness_refresh() {
    assert!(harness_refresh_is_enabled_for_profile(ProductProfile::Full));
}
