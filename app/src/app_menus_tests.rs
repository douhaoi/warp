use crate::channel::ProductProfile;
use crate::util::bindings::CustomAction;

use super::{
    ProfiledMenu, ProfiledMenuAction, menu_action_is_supported_for_profile,
    menu_is_supported_for_profile, supported_profiled_actions,
};

#[test]
fn full_profile_preserves_targeted_menu_and_action_policy() {
    for menu in [
        ProfiledMenu::File,
        ProfiledMenu::View,
        ProfiledMenu::Blocks,
        ProfiledMenu::Ai,
        ProfiledMenu::Drive,
    ] {
        assert!(menu_is_supported_for_profile(menu, ProductProfile::Full));
    }

    for (menu, action) in terminal_only_excluded_actions()
        .into_iter()
        .chain(terminal_only_retained_actions())
    {
        assert!(menu_action_is_supported_for_profile(
            menu,
            action,
            ProductProfile::Full,
        ));
    }
}

#[test]
fn terminal_only_profile_removes_cloud_and_ai_menus_and_actions() {
    assert!(menu_is_supported_for_profile(
        ProfiledMenu::File,
        ProductProfile::TerminalOnly,
    ));
    assert!(menu_is_supported_for_profile(
        ProfiledMenu::View,
        ProductProfile::TerminalOnly,
    ));
    assert!(menu_is_supported_for_profile(
        ProfiledMenu::Blocks,
        ProductProfile::TerminalOnly,
    ));
    assert!(!menu_is_supported_for_profile(
        ProfiledMenu::Ai,
        ProductProfile::TerminalOnly,
    ));
    assert!(!menu_is_supported_for_profile(
        ProfiledMenu::Drive,
        ProductProfile::TerminalOnly,
    ));

    for (menu, action) in terminal_only_excluded_actions() {
        assert!(!menu_action_is_supported_for_profile(
            menu,
            action,
            ProductProfile::TerminalOnly,
        ));
    }
}

#[test]
fn terminal_only_profile_keeps_local_terminal_menu_actions() {
    for (menu, action) in terminal_only_retained_actions() {
        assert!(menu_action_is_supported_for_profile(
            menu,
            action,
            ProductProfile::TerminalOnly,
        ));
    }

    let view_actions = [
        CustomAction::ToggleWarpDrive,
        CustomAction::CommandPalette,
        CustomAction::NavigationPalette,
        CustomAction::LaunchConfigPalette,
        CustomAction::FilesPalette,
        CustomAction::ToggleProjectExplorer,
        CustomAction::ToggleConversationListView,
        CustomAction::ToggleGlobalSearch,
        CustomAction::History,
        CustomAction::CommandSearch,
        CustomAction::Workflows,
    ];
    assert_eq!(
        supported_profiled_actions(
            ProfiledMenu::View,
            ProductProfile::TerminalOnly,
            &view_actions,
        )
        .collect::<Vec<_>>(),
        vec![
            CustomAction::CommandPalette,
            CustomAction::NavigationPalette,
            CustomAction::LaunchConfigPalette,
            CustomAction::History,
            CustomAction::CommandSearch,
        ],
    );
}

fn terminal_only_excluded_actions() -> Vec<(ProfiledMenu, ProfiledMenuAction)> {
    vec![
        (
            ProfiledMenu::File,
            ProfiledMenuAction::Custom(CustomAction::NewAgentTab),
        ),
        (
            ProfiledMenu::File,
            ProfiledMenuAction::Custom(CustomAction::NewFile),
        ),
        (
            ProfiledMenu::File,
            ProfiledMenuAction::Custom(CustomAction::OpenRepository),
        ),
        (ProfiledMenu::File, ProfiledMenuAction::OpenRecent),
        (
            ProfiledMenu::View,
            ProfiledMenuAction::Custom(CustomAction::ToggleWarpDrive),
        ),
        (
            ProfiledMenu::View,
            ProfiledMenuAction::Custom(CustomAction::FilesPalette),
        ),
        (
            ProfiledMenu::View,
            ProfiledMenuAction::Custom(CustomAction::ToggleProjectExplorer),
        ),
        (
            ProfiledMenu::View,
            ProfiledMenuAction::Custom(CustomAction::ToggleConversationListView),
        ),
        (
            ProfiledMenu::View,
            ProfiledMenuAction::Custom(CustomAction::ToggleGlobalSearch),
        ),
        (
            ProfiledMenu::View,
            ProfiledMenuAction::Custom(CustomAction::Workflows),
        ),
        (
            ProfiledMenu::Blocks,
            ProfiledMenuAction::Custom(CustomAction::CreateBlockPermalink),
        ),
        (
            ProfiledMenu::Blocks,
            ProfiledMenuAction::Custom(CustomAction::ViewSharedBlocks),
        ),
        (
            ProfiledMenu::Blocks,
            ProfiledMenuAction::Custom(CustomAction::ToggleBookmarkBlock),
        ),
    ]
}

fn terminal_only_retained_actions() -> Vec<(ProfiledMenu, ProfiledMenuAction)> {
    vec![
        (
            ProfiledMenu::File,
            ProfiledMenuAction::Custom(CustomAction::ReopenClosedSession),
        ),
        (
            ProfiledMenu::File,
            ProfiledMenuAction::Custom(CustomAction::CloseCurrentSession),
        ),
        (
            ProfiledMenu::File,
            ProfiledMenuAction::Custom(CustomAction::CloseWindow),
        ),
        (
            ProfiledMenu::Blocks,
            ProfiledMenuAction::Custom(CustomAction::ClearBlocks),
        ),
        (
            ProfiledMenu::Blocks,
            ProfiledMenuAction::Custom(CustomAction::SelectBlockAbove),
        ),
        (
            ProfiledMenu::Blocks,
            ProfiledMenuAction::Custom(CustomAction::ScrollToTopOfSelectedBlocks),
        ),
        (
            ProfiledMenu::Blocks,
            ProfiledMenuAction::Custom(CustomAction::FindWithinBlock),
        ),
        (
            ProfiledMenu::Blocks,
            ProfiledMenuAction::Custom(CustomAction::CopyBlock),
        ),
    ]
}
