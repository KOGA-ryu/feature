use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "sim.crash_recovery";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashState {
    Stable,
    Crashed,
    Recovering,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashRecoveryState {
    pub state: CrashState,
    pub locked_ticks_remaining: u32,
    pub recovery_ticks_remaining: u32,
}

impl Default for CrashState {
    fn default() -> Self {
        Self::Stable
    }
}

impl Default for CrashRecoveryState {
    fn default() -> Self {
        Self::new()
    }
}

impl CrashRecoveryState {
    pub fn new() -> Self {
        Self {
            state: CrashState::Stable,
            locked_ticks_remaining: 0,
            recovery_ticks_remaining: 0,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw).map_err(|error| error.to_string())
    }

    pub fn trigger_crash(&mut self, lockout_ticks: u32, recovery_ticks: u32) -> bool {
        if self.state != CrashState::Stable {
            return false;
        }
        self.state = CrashState::Crashed;
        self.locked_ticks_remaining = lockout_ticks.max(1);
        self.recovery_ticks_remaining = recovery_ticks.max(1);
        true
    }

    pub fn tick(&mut self, elapsed_ticks: u32) {
        if elapsed_ticks == 0 || self.state == CrashState::Stable {
            return;
        }

        let mut remaining = elapsed_ticks;

        if self.state == CrashState::Crashed {
            if remaining >= self.locked_ticks_remaining {
                remaining -= self.locked_ticks_remaining;
                self.locked_ticks_remaining = 0;
                self.state = CrashState::Recovering;
            } else {
                self.locked_ticks_remaining -= remaining;
                return;
            }
        }

        if self.state == CrashState::Recovering {
            if remaining >= self.recovery_ticks_remaining {
                self.recovery_ticks_remaining = 0;
                self.state = CrashState::Stable;
            } else {
                self.recovery_ticks_remaining -= remaining;
            }
        }
    }

    pub fn can_steer(&self) -> bool {
        self.state == CrashState::Stable
    }

    pub fn can_collide(&self) -> bool {
        self.state == CrashState::Stable
    }

    pub fn is_stable(&self) -> bool {
        self.state == CrashState::Stable
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

pub fn sample_state() -> Result<CrashRecoveryState, String> {
    CrashRecoveryState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
