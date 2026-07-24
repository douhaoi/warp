use std::sync::Arc;

use super::*;

#[test]
fn terminal_only_palette_modes_and_filters_are_fail_closed() {
    for mode in [
        PaletteMode::Command,
        PaletteMode::Navigation,
        PaletteMode::LaunchConfig,
    ] {
        assert!(is_palette_mode_supported_for_profile(
            mode,
            ProductProfile::TerminalOnly
        ));
    }

    for mode in [
        PaletteMode::WarpDrive,
        PaletteMode::Files,
        PaletteMode::Conversations,
    ] {
        assert!(!is_palette_mode_supported_for_profile(
            mode,
            ProductProfile::TerminalOnly
        ));
    }

    assert!(is_query_filter_supported_for_profile(
        QueryFilter::Actions,
        ProductProfile::TerminalOnly
    ));
    assert!(is_query_filter_supported_for_profile(
        QueryFilter::Sessions,
        ProductProfile::TerminalOnly
    ));
    assert!(is_query_filter_supported_for_profile(
        QueryFilter::LaunchConfigurations,
        ProductProfile::TerminalOnly
    ));
    assert!(!is_query_filter_supported_for_profile(
        QueryFilter::Files,
        ProductProfile::TerminalOnly
    ));
    assert!(!is_query_filter_supported_for_profile(
        QueryFilter::Conversations,
        ProductProfile::TerminalOnly
    ));
    assert!(!is_query_filter_supported_for_profile(
        QueryFilter::Drive,
        ProductProfile::TerminalOnly
    ));
}

#[test]
fn terminal_only_binding_allowlist_keeps_terminal_actions_and_rejects_product_actions() {
    let mut terminal_binding = CommandBinding::new("terminal:paste".into(), "Paste".into(), None);
    terminal_binding.action = Some(Arc::new(TerminalAction::Paste));
    assert!(terminal_only_binding_is_allowed(&terminal_binding));

    terminal_binding.group = Some(BindingGroup::WarpAi);
    assert!(!terminal_only_binding_is_allowed(&terminal_binding));

    let mut workspace_binding =
        CommandBinding::new("workspace:new_tab".into(), "New tab".into(), None);
    workspace_binding.action = Some(Arc::new(WorkspaceAction::AddDefaultTab));
    assert!(terminal_only_binding_is_allowed(&workspace_binding));

    workspace_binding.action = Some(Arc::new(WorkspaceAction::ToggleWarpDrive));
    assert!(!terminal_only_binding_is_allowed(&workspace_binding));

    workspace_binding.action = Some(Arc::new(WorkspaceAction::OpenSettingsFile));
    assert!(terminal_only_binding_is_allowed(&workspace_binding));
    workspace_binding.action = Some(Arc::new(WorkspaceAction::TerminateApp));
    assert!(terminal_only_binding_is_allowed(&workspace_binding));

    let mut fullscreen_binding = CommandBinding::new(
        "root_view:toggle_fullscreen".into(),
        "Toggle fullscreen".into(),
        None,
    );
    fullscreen_binding.action = Some(Arc::new(RootViewAction::ToggleFullscreen));
    fullscreen_binding.group = Some(BindingGroup::Navigation);
    assert!(terminal_only_binding_is_allowed(&fullscreen_binding));
}

#[test]
fn terminal_only_result_and_event_guards_reject_product_side_effects() {
    let unsupported_result = CommandPaletteItemAction::CreateFile {
        file_name: "blocked.txt".into(),
        current_directory: "/tmp".into(),
    };
    assert!(!terminal_only_command_palette_item_action_is_allowed(
        &unsupported_result
    ));
    assert!(!terminal_only_command_palette_event_is_allowed(
        &Event::OpenFile {
            path: "/tmp/blocked.txt".into(),
            line_and_column_arg: None,
        }
    ));
    assert!(terminal_only_command_palette_event_is_allowed(
        &Event::Close {
            accepted_action_type: None,
        }
    ));
}

#[test]
fn terminal_only_binding_filter_preserves_ctrl_tab_cycle_exclusion() {
    let mut cycle_binding = CommandBinding::new(
        "workspace:cycle_next_session".into(),
        "Cycle session".into(),
        None,
    );
    cycle_binding.action = Some(Arc::new(WorkspaceAction::CycleNextSession));

    assert!(terminal_only_binding_is_allowed(&cycle_binding));
    assert!(is_ctrl_tab_cycle_binding(&cycle_binding));
    assert!(terminal_only_binding_is_allowed_for_palette(
        &cycle_binding,
        false
    ));
    assert!(!terminal_only_binding_is_allowed_for_palette(
        &cycle_binding,
        true
    ));
}
