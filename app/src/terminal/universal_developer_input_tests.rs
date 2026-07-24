use super::*;

#[test]
fn full_profile_preserves_universal_developer_input_toggle_options_and_defaults() {
    assert_eq!(
        universal_developer_input_toggle_options_and_default(
            ProductProfile::Full,
            InputConfig {
                input_type: InputType::Shell,
                is_locked: true,
            },
            false,
        ),
        (
            vec![InputToggleMode::Terminal, InputToggleMode::AgentMode],
            InputToggleMode::Terminal,
        )
    );
    assert_eq!(
        universal_developer_input_toggle_options_and_default(
            ProductProfile::Full,
            InputConfig {
                input_type: InputType::AI,
                is_locked: true,
            },
            false,
        ),
        (
            vec![InputToggleMode::Terminal, InputToggleMode::AgentMode],
            InputToggleMode::AgentMode,
        )
    );
    assert_eq!(
        universal_developer_input_toggle_options_and_default(
            ProductProfile::Full,
            InputConfig {
                input_type: InputType::Shell,
                is_locked: false,
            },
            true,
        ),
        (
            vec![
                InputToggleMode::Terminal,
                InputToggleMode::AgentMode,
                InputToggleMode::AutoDetection,
            ],
            InputToggleMode::AutoDetection,
        )
    );
}

#[test]
fn terminal_only_profile_normalizes_legacy_universal_developer_input_states() {
    for input_config in [
        InputConfig {
            input_type: InputType::Shell,
            is_locked: true,
        },
        InputConfig {
            input_type: InputType::AI,
            is_locked: true,
        },
        InputConfig {
            input_type: InputType::Shell,
            is_locked: false,
        },
        InputConfig {
            input_type: InputType::AI,
            is_locked: false,
        },
    ] {
        assert_eq!(
            universal_developer_input_toggle_options_and_default(
                ProductProfile::TerminalOnly,
                input_config,
                true,
            ),
            (vec![InputToggleMode::Terminal], InputToggleMode::Terminal)
        );
    }
}
