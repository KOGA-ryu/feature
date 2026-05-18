use ai_interaction_event::{
    AIInteractionEvent, InteractionOutcome, poor_read_streak, sample_events,
};
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.ai_interaction_grader";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillBand {
    Novice,
    Competent,
    Advanced,
    Mastery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GradeConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AIInteractionGradeCard {
    pub read_quality: u32,
    pub timing: u32,
    pub control: u32,
    pub adaptation: u32,
    pub efficiency: u32,
    pub style: u32,
    pub overall_band: SkillBand,
    pub confidence: GradeConfidence,
    pub event_count: u32,
    pub poor_read_streak: u32,
}

pub struct AIInteractionGrader;

impl AIInteractionGrader {
    pub fn grade(events: &[AIInteractionEvent]) -> AIInteractionGradeCard {
        let mut read_quality = 50_i32;
        let mut timing = 50_i32;
        let mut control = 50_i32;
        let mut adaptation = 50_i32;
        let mut efficiency = 50_i32;
        let mut style = 50_i32;

        for event in events {
            match event.outcome {
                InteractionOutcome::BaitedCommit => {
                    read_quality += 12;
                    control += 8;
                    style += 6;
                    adaptation += 4;
                }
                InteractionOutcome::ForcedWhiff => {
                    read_quality += 10;
                    timing += 8;
                    control += 6;
                }
                InteractionOutcome::CorrectDodge => {
                    timing += 10;
                    control += 8;
                    efficiency += 4;
                }
                InteractionOutcome::LateDodge => {
                    read_quality -= 3;
                    timing -= 8;
                    efficiency -= 4;
                }
                InteractionOutcome::PunishWindowHit => {
                    timing += 12;
                    efficiency += 10;
                    style += 6;
                }
                InteractionOutcome::BadTrade => {
                    control -= 10;
                    efficiency -= 8;
                }
                InteractionOutcome::PanicEscape => {
                    control -= 6;
                    style -= 4;
                }
                InteractionOutcome::PatternAdapted => {
                    adaptation += 14;
                    read_quality += 8;
                }
                InteractionOutcome::PatternExploited => {
                    read_quality += 6;
                    efficiency += 8;
                    style += 8;
                }
                InteractionOutcome::MissedTelegraph => {
                    read_quality -= 12;
                    timing -= 6;
                }
            }
        }

        let read_quality = clamp_score(read_quality);
        let timing = clamp_score(timing);
        let control = clamp_score(control);
        let adaptation = clamp_score(adaptation);
        let efficiency = clamp_score(efficiency);
        let style = clamp_score(style);
        let average = (read_quality + timing + control + adaptation + efficiency + style) / 6;

        let overall_band = if average >= 80 {
            SkillBand::Mastery
        } else if average >= 65 {
            SkillBand::Advanced
        } else if average >= 45 {
            SkillBand::Competent
        } else {
            SkillBand::Novice
        };

        let confidence = match events.len() {
            0..=2 => GradeConfidence::Low,
            3..=5 => GradeConfidence::Medium,
            _ => GradeConfidence::High,
        };

        AIInteractionGradeCard {
            read_quality,
            timing,
            control,
            adaptation,
            efficiency,
            style,
            overall_band,
            confidence,
            event_count: events.len() as u32,
            poor_read_streak: poor_read_streak(events),
        }
    }
}

fn clamp_score(value: i32) -> u32 {
    value.clamp(0, 100) as u32
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

pub fn sample_grade_card() -> Result<AIInteractionGradeCard, String> {
    let events = sample_events()?;
    Ok(AIInteractionGrader::grade(&events))
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
