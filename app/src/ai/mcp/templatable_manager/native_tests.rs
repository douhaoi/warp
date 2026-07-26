use super::*;

#[test]
fn terminal_only_startup_skips_persisted_mcp_state() {
    assert_eq!(
        startup_mcp_state_policy_for_profile(ProductProfile::TerminalOnly),
        StartupMcpStatePolicy {
            load_persisted_credentials: false,
            restore_running_servers: false,
            migrate_legacy_servers: false,
        }
    );
}

#[test]
fn full_profile_startup_keeps_persisted_mcp_state_behavior() {
    assert_eq!(
        startup_mcp_state_policy_for_profile(ProductProfile::Full),
        StartupMcpStatePolicy {
            load_persisted_credentials: true,
            restore_running_servers: true,
            migrate_legacy_servers: true,
        }
    );
}
