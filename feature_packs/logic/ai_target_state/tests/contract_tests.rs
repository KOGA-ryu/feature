use ai_target_state::{
    AITargetState, FEATURE_ID, TargetAwareness, TargetPosture, TargetPressure, sample_fixture,
    sample_state,
};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "ai_target_state");
    assert_eq!(manifest.kind.to_string(), "logic_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["posture"].as_str(), Some("pressing"));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.label, "street bruiser");
    assert_eq!(state.awareness, TargetAwareness::Tracking);
}

#[test]
fn commitment_and_vulnerability_tick_down_deterministically() {
    let mut state = AITargetState::new("target");
    state.set_posture(TargetPosture::Overcommitted);
    state.start_commitment(3);
    state.expose_vulnerability(2);

    state.tick(1);
    assert!(state.is_committed());
    assert!(state.is_vulnerable());

    state.tick(2);
    assert!(!state.is_committed());
    assert!(!state.is_vulnerable());
    assert_eq!(state.posture, TargetPosture::Recovering);
}

#[test]
fn posture_awareness_and_pressure_are_mutable() {
    let mut state = AITargetState::default();
    state.set_posture(TargetPosture::Pressing);
    state.set_awareness(TargetAwareness::LockedOn);
    state.set_pressure(TargetPressure::Panicked);

    assert_eq!(state.posture, TargetPosture::Pressing);
    assert_eq!(state.awareness, TargetAwareness::LockedOn);
    assert_eq!(state.pressure, TargetPressure::Panicked);
}
