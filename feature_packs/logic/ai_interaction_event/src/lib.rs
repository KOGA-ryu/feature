use ai_intent_model::IntentKind;
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.ai_interaction_event";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionOutcome {
    BaitedCommit,
    ForcedWhiff,
    CorrectDodge,
    LateDodge,
    PunishWindowHit,
    BadTrade,
    PanicEscape,
    PatternAdapted,
    PatternExploited,
    MissedTelegraph,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AIInteractionEvent {
    pub outcome: InteractionOutcome,
    pub intent: Option<IntentKind>,
    pub tick: u32,
}

impl AIInteractionEvent {
    pub fn new(outcome: InteractionOutcome, intent: Option<IntentKind>, tick: u32) -> Self {
        Self {
            outcome,
            intent,
            tick,
        }
    }
}

pub fn events_from_fixture_str(raw: &str) -> Result<Vec<AIInteractionEvent>, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn poor_read_streak(events: &[AIInteractionEvent]) -> u32 {
    events
        .iter()
        .rev()
        .take_while(|event| is_poor_read(event.outcome))
        .count() as u32
}

pub fn is_poor_read(outcome: InteractionOutcome) -> bool {
    matches!(
        outcome,
        InteractionOutcome::LateDodge
            | InteractionOutcome::BadTrade
            | InteractionOutcome::PanicEscape
            | InteractionOutcome::MissedTelegraph
    )
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

pub fn sample_events() -> Result<Vec<AIInteractionEvent>, String> {
    events_from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
