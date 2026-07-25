use std::collections::HashSet;

use warp_core::channel::ProductProfile;

use super::{FeatureFlag, filter_features_for_product_profile};

#[test]
fn full_profile_preserves_every_collected_feature() {
    let collected_features = HashSet::from([
        FeatureFlag::ResizeFix,
        FeatureFlag::AgentMode,
        FeatureFlag::CloudEnvironments,
        FeatureFlag::McpServer,
        FeatureFlag::RuntimeFeatureFlags,
        FeatureFlag::SkipFirebaseAnonymousUser,
        FeatureFlag::AccountFirstOnboarding,
        FeatureFlag::OpenWarpNewSettingsModes,
        FeatureFlag::AgentOnboarding,
        FeatureFlag::BackgroundComputerUse,
    ]);

    assert_eq!(
        filter_features_for_product_profile(collected_features.clone(), ProductProfile::Full),
        collected_features
    );
}

#[test]
fn terminal_only_profile_filters_features_by_product_domain() {
    for (feature, should_be_retained) in [
        (FeatureFlag::ResizeFix, true),
        (FeatureFlag::TerminalLifecycleRecovery, true),
        (FeatureFlag::Ligatures, true),
        (FeatureFlag::MinimalistUI, true),
        (FeatureFlag::FullScreenZenMode, true),
        (FeatureFlag::TabCloseButtonOnLeft, true),
        (FeatureFlag::AllowIgnoringInputSuggestions, true),
        (FeatureFlag::VerticalTabsSummaryMode, true),
        (FeatureFlag::SkipFirebaseAnonymousUser, true),
        (FeatureFlag::AccountFirstOnboarding, false),
        (FeatureFlag::OpenWarpNewSettingsModes, false),
        (FeatureFlag::AgentOnboarding, false),
        (FeatureFlag::BackgroundComputerUse, false),
        (FeatureFlag::AgentMode, false),
        (FeatureFlag::CloudEnvironments, false),
        (FeatureFlag::McpServer, false),
        (FeatureFlag::GlobalSearch, false),
        (FeatureFlag::Autoupdate, false),
        (FeatureFlag::Changelog, false),
        (FeatureFlag::CrashReporting, false),
        (FeatureFlag::RuntimeFeatureFlags, false),
    ] {
        let filtered = filter_features_for_product_profile(
            HashSet::from([feature]),
            ProductProfile::TerminalOnly,
        );

        assert_eq!(
            filtered.contains(&feature),
            should_be_retained,
            "{feature:?}"
        );
    }
}

#[test]
fn terminal_only_profile_filters_additional_and_release_style_inputs() {
    let additional_features = HashSet::from([
        FeatureFlag::ResizeFix,
        FeatureFlag::McpServer,
        FeatureFlag::RuntimeFeatureFlags,
    ]);
    let release_features = HashSet::from([
        FeatureFlag::Autoupdate,
        FeatureFlag::Changelog,
        FeatureFlag::CrashReporting,
        FeatureFlag::DragTabsToWindows,
    ]);
    let collected_features = additional_features
        .into_iter()
        .chain(release_features)
        .collect();

    assert_eq!(
        filter_features_for_product_profile(collected_features, ProductProfile::TerminalOnly),
        HashSet::from([FeatureFlag::ResizeFix, FeatureFlag::DragTabsToWindows,])
    );
}
