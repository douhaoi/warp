use warp_core::channel::ProductProfile;

use super::{InputClassifierInitialization, input_classifier_initialization};

#[test]
fn terminal_only_uses_heuristic_input_classifier_without_loading_onnx() {
    assert_eq!(
        input_classifier_initialization(ProductProfile::TerminalOnly),
        InputClassifierInitialization::HeuristicOnly
    );
}

#[test]
fn full_profile_keeps_onnx_input_classifier_initialization() {
    assert_eq!(
        input_classifier_initialization(ProductProfile::Full),
        InputClassifierInitialization::LoadOnnxWhenAvailable
    );
}
