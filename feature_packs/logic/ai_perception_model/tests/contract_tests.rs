use ai_perception_model::{
    AIPerceptionSnapshot, FEATURE_ID, PerceptionEvent, sample_fixture, sample_state,
};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "ai_perception_model");
    assert_eq!(manifest.kind.to_string(), "logic_pattern");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["certainty"].as_u64(), Some(65));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.noticed_count, 2);
}

#[test]
fn noticed_events_raise_certainty_and_missed_events_reduce_it() {
    let mut snapshot = AIPerceptionSnapshot::new();
    snapshot.apply(PerceptionEvent::NoticedAdvance);
    assert_eq!(snapshot.noticed_count, 1);
    assert_eq!(snapshot.certainty, 65);

    snapshot.apply(PerceptionEvent::MissedTelegraph);
    assert_eq!(snapshot.missed_count, 1);
    assert_eq!(snapshot.certainty, 55);
}

#[test]
fn misreads_surface_pressure_hint() {
    let mut snapshot = AIPerceptionSnapshot::new();
    assert!(!snapshot.has_recent_misread_pressure());
    snapshot.apply(PerceptionEvent::MisreadFeint);
    assert!(snapshot.has_recent_misread_pressure());
}
