use ai_interaction_grader::{
    AIInteractionGradeCard, GradeConfidence, SkillBand, sample_grade_card,
};
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.ai_grade_hud";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AIGradeHudState {
    pub visible_band: SkillBand,
    pub confidence: GradeConfidence,
    pub summary_label: String,
    pub emphasis_label: String,
    pub zoom_ready: bool,
}

impl AIGradeHudState {
    pub fn from_grade(card: &AIInteractionGradeCard) -> Self {
        let emphasis_label = format!("best at {}", strongest_dimension(card));
        let summary_label = format!("{} read", band_label(card.overall_band));
        let zoom_ready = matches!(card.overall_band, SkillBand::Advanced | SkillBand::Mastery)
            && matches!(
                card.confidence,
                GradeConfidence::Medium | GradeConfidence::High
            );

        Self {
            visible_band: card.overall_band,
            confidence: card.confidence,
            summary_label,
            emphasis_label,
            zoom_ready,
        }
    }

    pub fn sync_from_grade(&mut self, card: &AIInteractionGradeCard) {
        *self = Self::from_grade(card);
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw).map_err(|error| error.to_string())
    }
}

fn strongest_dimension(card: &AIInteractionGradeCard) -> &'static str {
    let mut strongest = ("read_quality", card.read_quality);
    for candidate in [
        ("timing", card.timing),
        ("control", card.control),
        ("adaptation", card.adaptation),
        ("efficiency", card.efficiency),
        ("style", card.style),
    ] {
        if candidate.1 > strongest.1 {
            strongest = candidate;
        }
    }
    strongest.0
}

fn band_label(band: SkillBand) -> &'static str {
    match band {
        SkillBand::Novice => "novice",
        SkillBand::Competent => "competent",
        SkillBand::Advanced => "advanced",
        SkillBand::Mastery => "mastery",
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

pub fn sample_state() -> Result<AIGradeHudState, String> {
    let card = sample_grade_card()?;
    Ok(AIGradeHudState::from_grade(&card))
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
