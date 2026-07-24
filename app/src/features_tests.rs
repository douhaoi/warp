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
    ]);

    assert_eq!(
        filter_features_for_product_profile(collected_features.clone(), ProductProfile::Full),
        collected_features
    );
}

#[test]
fn terminal_only_profile_keeps_allowlisted_features_and_removes_disallowed_domains() {
    let collected_features = HashSet::from([
        FeatureFlag::ResizeFix,
        FeatureFlag::TerminalLifecycleRecovery,
        FeatureFlag::Ligatures,
        FeatureFlag::MinimalistUI,
        FeatureFlag::FullScreenZenMode,
        FeatureFlag::TabCloseButtonOnLeft,
        FeatureFlag::AllowIgnoringInputSuggestions,
        FeatureFlag::VerticalTabsSummaryMode,
        FeatureFlag::AgentMode,
        FeatureFlag::CloudEnvironments,
        FeatureFlag::McpServer,
        FeatureFlag::GlobalSearch,
        FeatureFlag::Autoupdate,
        FeatureFlag::Changelog,
        FeatureFlag::CrashReporting,
        FeatureFlag::RuntimeFeatureFlags,
    ]);

    assert_eq!(
        filter_features_for_product_profile(collected_features, ProductProfile::TerminalOnly),
        HashSet::from([
            FeatureFlag::ResizeFix,
            FeatureFlag::TerminalLifecycleRecovery,
            FeatureFlag::Ligatures,
            FeatureFlag::MinimalistUI,
            FeatureFlag::FullScreenZenMode,
            FeatureFlag::TabCloseButtonOnLeft,
            FeatureFlag::AllowIgnoringInputSuggestions,
            FeatureFlag::VerticalTabsSummaryMode,
        ])
    );
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
