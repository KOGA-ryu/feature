use ai_intent_model::{
    AIIntentState, FEATURE_ID, IntentCommitment, IntentKind, sample_fixture, sample_state,
};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "ai_intent_model");
    assert_eq!(manifest.kind.to_string(), "logic_pattern");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["current_intent"].as_str(), Some("quick_strike"));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.current_intent, IntentKind::QuickStrike);
}

#[test]
fn intent_advances_through_all_phases() {
    let mut state = AIIntentState::new(IntentKind::GuardBreakWindup, 1, 2, 1);
    assert_eq!(state.commitment, IntentCommitment::Telegraphing);

    state.tick(1);
    assert_eq!(state.commitment, IntentCommitment::Committed);
    assert!(state.is_committed());

    state.tick(2);
    assert_eq!(state.commitment, IntentCommitment::Recovering);

    state.tick(1);
    assert_eq!(state.commitment, IntentCommitment::Cancelled);
}

#[test]
fn choosing_or_cancelling_intent_is_deterministic() {
    let mut state = AIIntentState::default();
    state.choose_intent(IntentKind::OvercommitLunge, 2, 3, 2);
    assert_eq!(state.current_intent, IntentKind::OvercommitLunge);
    assert_eq!(state.telegraph_ticks_remaining, 2);

    state.cancel();
    assert_eq!(state.commitment, IntentCommitment::Cancelled);
    assert_eq!(state.commit_ticks_remaining, 0);
}
