use serde::Deserialize;

use super::ProductProfile;

#[test]
fn product_profile_defaults_to_full_when_omitted_from_config() {
    #[derive(Deserialize)]
    struct TestConfig {
        #[serde(default)]
        product_profile: ProductProfile,
    }

    let config: TestConfig = serde_json::from_str("{}").unwrap();

    assert_eq!(config.product_profile, ProductProfile::Full);
}

#[test]
fn product_profile_deserializes_terminal_only() {
    let profile: ProductProfile = serde_json::from_str("\"TerminalOnly\"").unwrap();

    assert_eq!(profile, ProductProfile::TerminalOnly);
}
