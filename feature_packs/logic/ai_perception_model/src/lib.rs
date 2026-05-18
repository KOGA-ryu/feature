use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.ai_perception_model";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerceptionEvent {
    NoticedAdvance,
    NoticedRetreat,
    NoticedBait,
    MissedTelegraph,
    MisreadFeint,
    LostTrack,
    RegainedTrack,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AIPerceptionSnapshot {
    pub last_event: Option<PerceptionEvent>,
    pub noticed_count: u32,
    pub missed_count: u32,
    pub misread_count: u32,
    pub certainty: u32,
}

impl Default for AIPerceptionSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

impl AIPerceptionSnapshot {
    pub fn new() -> Self {
        Self {
            last_event: None,
            noticed_count: 0,
            missed_count: 0,
            misread_count: 0,
            certainty: 50,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let mut snapshot: Self = serde_json::from_str(raw).map_err(|error| error.to_string())?;
        snapshot.normalize();
        Ok(snapshot)
    }

    pub fn apply(&mut self, event: PerceptionEvent) {
        self.last_event = Some(event);
        match event {
            PerceptionEvent::NoticedAdvance
            | PerceptionEvent::NoticedRetreat
            | PerceptionEvent::NoticedBait
            | PerceptionEvent::RegainedTrack => {
                self.noticed_count = self.noticed_count.saturating_add(1);
                self.certainty = self.certainty.saturating_add(15).min(100);
            }
            PerceptionEvent::MissedTelegraph => {
                self.missed_count = self.missed_count.saturating_add(1);
                self.certainty = self.certainty.saturating_sub(10);
            }
            PerceptionEvent::MisreadFeint => {
                self.misread_count = self.misread_count.saturating_add(1);
                self.certainty = self.certainty.saturating_sub(20);
            }
            PerceptionEvent::LostTrack => {
                self.certainty = self.certainty.saturating_sub(25);
            }
        }
    }

    pub fn has_recent_misread_pressure(&self) -> bool {
        self.misread_count > 0 || self.missed_count > self.noticed_count
    }

    fn normalize(&mut self) {
        self.certainty = self.certainty.min(100);
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

pub fn sample_state() -> Result<AIPerceptionSnapshot, String> {
    AIPerceptionSnapshot::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
