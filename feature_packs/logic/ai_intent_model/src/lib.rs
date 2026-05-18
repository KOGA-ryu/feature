use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.ai_intent_model";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentKind {
    QuickStrike,
    OvercommitLunge,
    GuardBreakWindup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentCommitment {
    Telegraphing,
    Committed,
    Recovering,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AIIntentState {
    pub current_intent: IntentKind,
    pub commitment: IntentCommitment,
    pub telegraph_ticks_remaining: u32,
    pub commit_ticks_remaining: u32,
    pub recovery_ticks_remaining: u32,
}

impl Default for IntentKind {
    fn default() -> Self {
        Self::QuickStrike
    }
}

impl Default for IntentCommitment {
    fn default() -> Self {
        Self::Telegraphing
    }
}

impl Default for AIIntentState {
    fn default() -> Self {
        Self::new(IntentKind::default(), 2, 2, 1)
    }
}

impl AIIntentState {
    pub fn new(
        current_intent: IntentKind,
        telegraph_ticks_remaining: u32,
        commit_ticks_remaining: u32,
        recovery_ticks_remaining: u32,
    ) -> Self {
        Self {
            current_intent,
            commitment: IntentCommitment::Telegraphing,
            telegraph_ticks_remaining,
            commit_ticks_remaining,
            recovery_ticks_remaining,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw).map_err(|error| error.to_string())
    }

    pub fn choose_intent(
        &mut self,
        current_intent: IntentKind,
        telegraph_ticks_remaining: u32,
        commit_ticks_remaining: u32,
        recovery_ticks_remaining: u32,
    ) {
        *self = Self::new(
            current_intent,
            telegraph_ticks_remaining,
            commit_ticks_remaining,
            recovery_ticks_remaining,
        );
    }

    pub fn cancel(&mut self) {
        self.commitment = IntentCommitment::Cancelled;
        self.telegraph_ticks_remaining = 0;
        self.commit_ticks_remaining = 0;
        self.recovery_ticks_remaining = 0;
    }

    pub fn tick(&mut self, elapsed_ticks: u32) {
        for _ in 0..elapsed_ticks {
            match self.commitment {
                IntentCommitment::Telegraphing => {
                    self.telegraph_ticks_remaining =
                        self.telegraph_ticks_remaining.saturating_sub(1);
                    if self.telegraph_ticks_remaining == 0 {
                        self.commitment = IntentCommitment::Committed;
                    }
                }
                IntentCommitment::Committed => {
                    self.commit_ticks_remaining = self.commit_ticks_remaining.saturating_sub(1);
                    if self.commit_ticks_remaining == 0 {
                        self.commitment = IntentCommitment::Recovering;
                    }
                }
                IntentCommitment::Recovering => {
                    self.recovery_ticks_remaining = self.recovery_ticks_remaining.saturating_sub(1);
                    if self.recovery_ticks_remaining == 0 {
                        self.commitment = IntentCommitment::Cancelled;
                    }
                }
                IntentCommitment::Cancelled => break,
            }
        }
    }

    pub fn is_committed(&self) -> bool {
        self.commitment == IntentCommitment::Committed
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

pub fn sample_state() -> Result<AIIntentState, String> {
    AIIntentState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
