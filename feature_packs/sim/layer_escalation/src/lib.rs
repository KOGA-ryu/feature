use ai_interaction_grader::{
    AIInteractionGradeCard, GradeConfidence, SkillBand, sample_grade_card,
};
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "sim.layer_escalation";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoomLayer {
    DuelArena,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationReason {
    HighSkillRead,
    MasteryPressure,
    LowConfidence,
    CompetentBaseline,
    PoorReadStreak,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerEscalationDecision {
    pub stay_coarse: bool,
    pub offer_zoom: bool,
    pub force_zoom: bool,
    pub next_layer: Option<ZoomLayer>,
    pub reward_modifier: i32,
    pub reason: EscalationReason,
}

impl LayerEscalationDecision {
    pub fn from_grade_card(card: &AIInteractionGradeCard) -> Self {
        if card.poor_read_streak >= 2 {
            return Self {
                stay_coarse: true,
                offer_zoom: false,
                force_zoom: false,
                next_layer: None,
                reward_modifier: -1,
                reason: EscalationReason::PoorReadStreak,
            };
        }

        match (card.overall_band, card.confidence) {
            (SkillBand::Mastery, GradeConfidence::High) => Self {
                stay_coarse: false,
                offer_zoom: false,
                force_zoom: true,
                next_layer: Some(ZoomLayer::DuelArena),
                reward_modifier: 2,
                reason: EscalationReason::MasteryPressure,
            },
            (
                SkillBand::Advanced | SkillBand::Mastery,
                GradeConfidence::Medium | GradeConfidence::High,
            ) => Self {
                stay_coarse: false,
                offer_zoom: true,
                force_zoom: false,
                next_layer: Some(ZoomLayer::DuelArena),
                reward_modifier: 1,
                reason: EscalationReason::HighSkillRead,
            },
            (_, GradeConfidence::Low) => Self {
                stay_coarse: true,
                offer_zoom: false,
                force_zoom: false,
                next_layer: None,
                reward_modifier: 0,
                reason: EscalationReason::LowConfidence,
            },
            _ => Self {
                stay_coarse: true,
                offer_zoom: false,
                force_zoom: false,
                next_layer: None,
                reward_modifier: 0,
                reason: EscalationReason::CompetentBaseline,
            },
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw).map_err(|error| error.to_string())
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_input.json")
}

pub fn sample_state() -> Result<LayerEscalationDecision, String> {
    let card = sample_grade_card()?;
    Ok(LayerEscalationDecision::from_grade_card(&card))
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
