use super::*;

#[test]
fn terminal_only_hides_cloud_and_ai_keybinding_groups() {
    for group in [
        BindingGroup::WarpAi,
        BindingGroup::Workflow,
        BindingGroup::Notebooks,
        BindingGroup::Folders,
        BindingGroup::EnvVarCollection,
    ] {
        assert!(!keybinding_group_is_supported_for_profile(
            Some(group),
            ProductProfile::TerminalOnly,
        ));
    }
}

#[test]
fn terminal_only_keeps_terminal_keybinding_groups() {
    for group in [
        None,
        Some(BindingGroup::Navigation),
        Some(BindingGroup::Close),
        Some(BindingGroup::Settings),
    ] {
        assert!(keybinding_group_is_supported_for_profile(
            group,
            ProductProfile::TerminalOnly,
        ));
    }
}
