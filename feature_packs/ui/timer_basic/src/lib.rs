use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.timer_basic";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimerStatus {
    Idle,
    Running,
    Paused,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimerState {
    pub total_seconds: u64,
    pub remaining_seconds: u64,
    pub status: TimerStatus,
}

impl Default for TimerState {
    fn default() -> Self {
        Self::new(0)
    }
}

impl TimerState {
    pub fn new(total_seconds: u64) -> Self {
        Self {
            total_seconds,
            remaining_seconds: total_seconds,
            status: TimerStatus::Idle,
        }
    }

    pub fn set_duration(&mut self, total_seconds: u64) {
        self.total_seconds = total_seconds;
        self.remaining_seconds = total_seconds;
        self.status = TimerStatus::Idle;
    }

    pub fn start(&mut self) {
        match self.status {
            TimerStatus::Completed => {
                self.remaining_seconds = self.total_seconds;
            }
            TimerStatus::Idle | TimerStatus::Paused | TimerStatus::Running => {}
        }

        if self.total_seconds == 0 {
            self.remaining_seconds = 0;
            self.status = TimerStatus::Completed;
            return;
        }

        self.status = TimerStatus::Running;
    }

    pub fn pause(&mut self) {
        if self.status == TimerStatus::Running {
            self.status = TimerStatus::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.status == TimerStatus::Paused {
            self.status = TimerStatus::Running;
        }
    }

    pub fn reset(&mut self) {
        self.remaining_seconds = self.total_seconds;
        self.status = TimerStatus::Idle;
    }

    pub fn tick(&mut self, elapsed_seconds: u64) {
        if self.status != TimerStatus::Running {
            return;
        }

        if elapsed_seconds >= self.remaining_seconds {
            self.remaining_seconds = 0;
            self.status = TimerStatus::Completed;
            return;
        }

        self.remaining_seconds -= elapsed_seconds;
    }

    pub fn progress_ratio(&self) -> f32 {
        if self.total_seconds == 0 {
            return if self.status == TimerStatus::Completed {
                1.0
            } else {
                0.0
            };
        }

        let elapsed = self
            .total_seconds
            .saturating_sub(self.remaining_seconds.min(self.total_seconds));
        let ratio = elapsed as f32 / self.total_seconds as f32;
        ratio.clamp(0.0, 1.0)
    }

    pub fn formatted_remaining(&self) -> String {
        format_duration(self.remaining_seconds)
    }

    pub fn is_complete(&self) -> bool {
        self.status == TimerStatus::Completed
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

pub fn sample_state() -> Result<TimerState, String> {
    serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}
