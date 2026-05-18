use ai_interaction_event::{
    FEATURE_ID, InteractionOutcome, events_from_fixture_str, poor_read_streak, sample_events,
    sample_fixture,
};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "ai_interaction_event");
    assert_eq!(manifest.kind.to_string(), "logic_pattern");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture.as_array().map(Vec::len), Some(4));
}

#[test]
fn sample_events_load() {
    let events = sample_events().expect("sample events should load");
    assert_eq!(events[0].outcome, InteractionOutcome::BaitedCommit);
}

#[test]
fn poor_read_streak_only_counts_tail_failures() {
    let events = events_from_fixture_str(
        r#"
        [
          {"outcome":"correct_dodge","intent":"quick_strike","tick":1},
          {"outcome":"late_dodge","intent":"quick_strike","tick":2},
          {"outcome":"missed_telegraph","intent":"guard_break_windup","tick":3}
        ]
        "#,
    )
    .expect("events should parse");

    assert_eq!(poor_read_streak(&events), 2);
}
