use checkpoint_flow::{FEATURE_ID, StageMode, sample_fixture, sample_state};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "checkpoint_flow");
    assert_eq!(manifest.kind.to_string(), "simulation_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["checkpoints"].as_array().map(Vec::len), Some(3));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.current_mode, StageMode::Brawler);
    assert_eq!(
        state.current_checkpoint().map(|checkpoint| checkpoint.id),
        Some(1)
    );
}

#[test]
fn ordered_progression_and_mode_transition_work() {
    let mut state = sample_state().expect("sample state should load");
    assert!(state.complete_current_checkpoint());
    assert_eq!(state.completed_checkpoint_ids, vec![1]);
    assert!(state.pending_transition.is_some());
    assert_eq!(state.current_mode, StageMode::Brawler);

    assert!(state.finish_transition());
    assert_eq!(state.current_mode, StageMode::Racing);
    assert_eq!(
        state.current_checkpoint().map(|checkpoint| checkpoint.id),
        Some(2)
    );

    assert!(state.complete_current_checkpoint());
    assert!(state.pending_transition.is_some());
    assert!(state.finish_transition());
    assert_eq!(state.current_mode, StageMode::Brawler);
    assert_eq!(
        state.current_checkpoint().map(|checkpoint| checkpoint.id),
        Some(3)
    );
}

#[test]
fn restart_clears_pending_transition_and_restores_current_mode() {
    let mut state = sample_state().expect("sample state should load");
    state.complete_current_checkpoint();
    assert!(state.pending_transition.is_some());
    assert!(state.restart_from_checkpoint());
    assert!(state.pending_transition.is_none());
    assert_eq!(state.current_mode, StageMode::Brawler);
}
