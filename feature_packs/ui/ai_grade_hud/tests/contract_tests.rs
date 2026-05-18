use ai_grade_hud::{AIGradeHudState, FEATURE_ID, sample_fixture, sample_state};
use ai_interaction_grader::{AIInteractionGradeCard, GradeConfidence, SkillBand};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "ai_grade_hud");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["zoom_ready"].as_bool(), Some(true));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.visible_band, SkillBand::Advanced);
}

#[test]
fn grade_sync_surfaces_zoom_ready_feedback() {
    let card = AIInteractionGradeCard {
        read_quality: 72,
        timing: 68,
        control: 67,
        adaptation: 79,
        efficiency: 66,
        style: 64,
        overall_band: SkillBand::Advanced,
        confidence: GradeConfidence::Medium,
        event_count: 4,
        poor_read_streak: 0,
    };

    let hud = AIGradeHudState::from_grade(&card);
    assert!(hud.zoom_ready);
    assert_eq!(hud.emphasis_label, "best at adaptation");
}
