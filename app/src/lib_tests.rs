use super::*;

#[test]
fn tui_uses_distinct_secure_storage_service_name() {
    let launch_mode = LaunchMode::Tui {
        mount: Box::new(|_| {}),
        api_key: None,
    };

    assert_eq!(
        launch_mode
            .secure_storage_service_name("dev.warp.Warp-Dev", channel::ProductProfile::Full,),
        "dev.warp.Warp-Dev.tui"
    );
}

#[test]
fn app_keeps_default_secure_storage_service_name() {
    let launch_mode = LaunchMode::App {
        args: Default::default(),
        api_key: None,
    };

    assert_eq!(
        launch_mode
            .secure_storage_service_name("dev.warp.Warp-Dev", channel::ProductProfile::Full,),
        "dev.warp.Warp-Dev"
    );
}

#[test]
fn terminal_only_uses_distinct_secure_storage_and_persistence_scopes() {
    let launch_mode = LaunchMode::App {
        args: Default::default(),
        api_key: None,
    };

    assert_eq!(
        launch_mode.secure_storage_service_name(
            "dev.warp.WarpOss",
            channel::ProductProfile::TerminalOnly,
        ),
        "dev.warp.WarpOss.terminal-only"
    );
    assert!(matches!(
        launch_mode.persistence_scope(channel::ProductProfile::TerminalOnly),
        persistence::PersistenceScope::TerminalOnly
    ));
}

#[test]
fn terminal_only_initializes_execution_profiles_without_legacy_cloud_scanning() {
    assert_eq!(
        execution_profile_initialization(channel::ProductProfile::TerminalOnly),
        ExecutionProfileInitialization::LocalSettingsOnly
    );
    assert_eq!(
        execution_profile_initialization(channel::ProductProfile::Full),
        ExecutionProfileInitialization::CurrentLaunchMode
    );
}

#[test]
fn terminal_only_skips_logged_out_startup_reporting() {
    assert!(!logged_out_startup_reporting_is_enabled(
        channel::ProductProfile::TerminalOnly
    ));
}

#[test]
fn full_profile_keeps_logged_out_startup_reporting() {
    assert!(logged_out_startup_reporting_is_enabled(
        channel::ProductProfile::Full
    ));
}

#[test]
fn launch_modes_select_expected_logging_frontend() {
    let tui = LaunchMode::Tui {
        mount: Box::new(|_| {}),
        api_key: None,
    };
    let app = LaunchMode::App {
        args: Default::default(),
        api_key: None,
    };
    let test = LaunchMode::Test {
        driver: Box::new(None),
        is_integration_test: false,
    };

    assert_eq!(tui.log_frontend(), LogFrontend::Tui);
    assert_eq!(app.log_frontend(), LogFrontend::Gui);
    assert_eq!(test.log_frontend(), LogFrontend::Gui);
    assert_eq!(
        LaunchMode::RemoteServerProxy.log_frontend(),
        LogFrontend::Cli
    );
    assert_eq!(
        LaunchMode::RemoteServerDaemon {
            identity_key: "test".to_owned(),
        }
        .log_frontend(),
        LogFrontend::Cli
    );
}
