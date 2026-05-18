use crash_recovery::{CrashRecoveryState, CrashState, FEATURE_ID, sample_fixture, sample_state};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "crash_recovery");
    assert_eq!(manifest.kind.to_string(), "simulation_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["state"].as_str(), Some("crashed"));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.state, CrashState::Crashed);
    assert_eq!(state.locked_ticks_remaining, 1);
}

#[test]
fn crash_trigger_and_recovery_ticks_follow_rules() {
    let mut state = CrashRecoveryState::new();
    assert!(state.trigger_crash(2, 3));
    assert_eq!(state.state, CrashState::Crashed);
    assert!(!state.can_steer());
    assert!(!state.can_collide());

    state.tick(2);
    assert_eq!(state.state, CrashState::Recovering);
    assert_eq!(state.recovery_ticks_remaining, 3);

    state.tick(2);
    assert_eq!(state.state, CrashState::Recovering);
    assert_eq!(state.recovery_ticks_remaining, 1);

    state.tick(1);
    assert!(state.is_stable());
    assert!(state.can_steer());
}

#[test]
fn cannot_retrigger_until_stable() {
    let mut state = CrashRecoveryState::new();
    assert!(state.trigger_crash(1, 1));
    assert!(!state.trigger_crash(1, 1));
}
