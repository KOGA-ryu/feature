use ai_interaction_event::{InteractionOutcome, events_from_fixture_str};
use ai_interaction_grader::{
    AIInteractionGrader, FEATURE_ID, GradeConfidence, SkillBand, sample_fixture, sample_grade_card,
};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "ai_interaction_grader");
    assert_eq!(manifest.kind.to_string(), "logic_pattern");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture.as_array().map(Vec::len), Some(4));
}

#[test]
fn sample_grade_card_is_stable() {
    let card = sample_grade_card().expect("sample grade card should load");
    assert_eq!(card.overall_band, SkillBand::Advanced);
    assert_eq!(card.confidence, GradeConfidence::Medium);
}

#[test]
fn positive_event_stream_grades_above_competent() {
    let events = events_from_fixture_str(
        r#"
        [
          {"outcome":"baited_commit","intent":"overcommit_lunge","tick":1},
          {"outcome":"correct_dodge","intent":"overcommit_lunge","tick":2},
          {"outcome":"punish_window_hit","intent":"overcommit_lunge","tick":3},
          {"outcome":"pattern_exploited","intent":"quick_strike","tick":4}
        ]
        "#,
    )
    .expect("events should parse");

    let card = AIInteractionGrader::grade(&events);
    assert!(matches!(
        card.overall_band,
        SkillBand::Advanced | SkillBand::Mastery
    ));
}

#[test]
fn negative_tail_events_raise_poor_read_streak() {
    let events = vec![
        ai_interaction_event::AIInteractionEvent::new(InteractionOutcome::CorrectDodge, None, 1),
        ai_interaction_event::AIInteractionEvent::new(InteractionOutcome::LateDodge, None, 2),
        ai_interaction_event::AIInteractionEvent::new(InteractionOutcome::MissedTelegraph, None, 3),
    ];

    let card = AIInteractionGrader::grade(&events);
    assert_eq!(card.poor_read_streak, 2);
    assert!(card.read_quality < 50);
}
