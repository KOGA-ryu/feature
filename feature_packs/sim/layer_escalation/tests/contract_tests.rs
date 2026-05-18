use ai_interaction_grader::{AIInteractionGradeCard, GradeConfidence, SkillBand};
use feature_core::parse_feature_manifest;
use layer_escalation::{
    EscalationReason, FEATURE_ID, LayerEscalationDecision, ZoomLayer, sample_state,
};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "layer_escalation");
    assert_eq!(manifest.kind.to_string(), "simulation_pattern");
}

#[test]
fn sample_state_loads() {
    let decision = sample_state().expect("sample decision should load");
    assert!(decision.offer_zoom);
    assert_eq!(decision.next_layer, Some(ZoomLayer::DuelArena));
}

#[test]
fn advanced_with_medium_confidence_offers_zoom() {
    let card = AIInteractionGradeCard {
        read_quality: 70,
        timing: 71,
        control: 69,
        adaptation: 70,
        efficiency: 68,
        style: 67,
        overall_band: SkillBand::Advanced,
        confidence: GradeConfidence::Medium,
        event_count: 4,
        poor_read_streak: 0,
    };

    let decision = LayerEscalationDecision::from_grade_card(&card);
    assert!(decision.offer_zoom);
    assert_eq!(decision.reason, EscalationReason::HighSkillRead);
}

#[test]
fn poor_read_streak_suppresses_zoom() {
    let card = AIInteractionGradeCard {
        read_quality: 75,
        timing: 62,
        control: 60,
        adaptation: 58,
        efficiency: 55,
        style: 54,
        overall_band: SkillBand::Competent,
        confidence: GradeConfidence::Medium,
        event_count: 5,
        poor_read_streak: 2,
    };

    let decision = LayerEscalationDecision::from_grade_card(&card);
    assert!(decision.stay_coarse);
    assert_eq!(decision.reason, EscalationReason::PoorReadStreak);
}
