use feature_core::parse_feature_manifest;
use scratchpad::{FEATURE_ID, ScratchpadState, sample_fixture, sample_state};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "scratchpad");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(manifest.inputs.items, vec!["text"]);
    assert_eq!(
        manifest.outputs.items,
        vec!["text", "character_count", "line_count"]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(
        fixture["text"].as_str().unwrap_or_default().lines().count(),
        5
    );
}

#[test]
fn scratchpad_text_helpers_work() {
    let mut state = ScratchpadState::new("alpha");
    assert_eq!(state.character_count(), 5);
    assert_eq!(state.line_count(), 1);

    state.set_text("alpha\nbeta\n\ngamma");
    assert_eq!(state.line_count(), 4);
    assert_eq!(state.character_count(), 17);

    state.clear();
    assert_eq!(state.text, "");
    assert_eq!(state.character_count(), 0);
    assert_eq!(state.line_count(), 0);
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert!(state.text.contains("Capture card"));
    assert_eq!(state.line_count(), 5);
}
