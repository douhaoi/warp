use super::*;

fn app_execution_mode(mode: ExecutionMode) -> AppExecutionMode {
    AppExecutionMode {
        mode,
        is_sandboxed: false,
    }
}

#[test]
fn terminal_only_profile_disables_mcp_server_autostart() {
    assert!(
        !app_execution_mode(ExecutionMode::App)
            .can_autostart_mcp_servers_for_profile(ProductProfile::TerminalOnly)
    );
}

#[test]
fn full_interactive_profiles_allow_mcp_server_autostart() {
    assert!(
        app_execution_mode(ExecutionMode::App)
            .can_autostart_mcp_servers_for_profile(ProductProfile::Full)
    );
    assert!(
        app_execution_mode(ExecutionMode::Tui)
            .can_autostart_mcp_servers_for_profile(ProductProfile::Full)
    );
}

#[test]
fn non_interactive_modes_do_not_autostart_mcp_servers() {
    assert!(
        !app_execution_mode(ExecutionMode::Sdk)
            .can_autostart_mcp_servers_for_profile(ProductProfile::Full)
    );
    assert!(
        !app_execution_mode(ExecutionMode::RemoteServerDaemon)
            .can_autostart_mcp_servers_for_profile(ProductProfile::Full)
    );
}
