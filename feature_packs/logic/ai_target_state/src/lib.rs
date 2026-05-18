use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.ai_target_state";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetPosture {
    Guarding,
    Pressing,
    Overcommitted,
    Recovering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetAwareness {
    Relaxed,
    Tracking,
    Alerted,
    LockedOn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetPressure {
    Stable,
    Strained,
    Panicked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AITargetState {
    pub label: String,
    pub posture: TargetPosture,
    pub awareness: TargetAwareness,
    pub pressure: TargetPressure,
    pub commitment_ticks_remaining: u32,
    pub vulnerability_ticks_remaining: u32,
}

impl Default for TargetPosture {
    fn default() -> Self {
        Self::Guarding
    }
}

impl Default for TargetAwareness {
    fn default() -> Self {
        Self::Relaxed
    }
}

impl Default for TargetPressure {
    fn default() -> Self {
        Self::Stable
    }
}

impl Default for AITargetState {
    fn default() -> Self {
        Self::new("street bruiser")
    }
}

impl AITargetState {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            posture: TargetPosture::default(),
            awareness: TargetAwareness::default(),
            pressure: TargetPressure::default(),
            commitment_ticks_remaining: 0,
            vulnerability_ticks_remaining: 0,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let mut state: Self = serde_json::from_str(raw).map_err(|error| error.to_string())?;
        state.normalize();
        Ok(state)
    }

    pub fn set_posture(&mut self, posture: TargetPosture) {
        self.posture = posture;
    }

    pub fn set_awareness(&mut self, awareness: TargetAwareness) {
        self.awareness = awareness;
    }

    pub fn set_pressure(&mut self, pressure: TargetPressure) {
        self.pressure = pressure;
    }

    pub fn start_commitment(&mut self, ticks: u32) {
        self.commitment_ticks_remaining = ticks;
    }

    pub fn expose_vulnerability(&mut self, ticks: u32) {
        self.vulnerability_ticks_remaining = ticks;
    }

    pub fn tick(&mut self, elapsed_ticks: u32) {
        if elapsed_ticks == 0 {
            return;
        }

        self.commitment_ticks_remaining = self
            .commitment_ticks_remaining
            .saturating_sub(elapsed_ticks);
        self.vulnerability_ticks_remaining = self
            .vulnerability_ticks_remaining
            .saturating_sub(elapsed_ticks);

        if self.commitment_ticks_remaining == 0 && self.posture == TargetPosture::Overcommitted {
            self.posture = TargetPosture::Recovering;
        }
    }

    pub fn is_committed(&self) -> bool {
        self.commitment_ticks_remaining > 0
    }

    pub fn is_vulnerable(&self) -> bool {
        self.vulnerability_ticks_remaining > 0
    }

    fn normalize(&mut self) {
        if self.label.trim().is_empty() {
            self.label = "unnamed target".into();
        }
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

pub fn sample_state() -> Result<AITargetState, String> {
    AITargetState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
