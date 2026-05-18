use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.runbook_panel";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RunbookStepStatus {
    #[default]
    Pending,
    Active,
    Completed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStep {
    pub id: String,
    pub title: String,
    pub detail: String,
    #[serde(default)]
    pub status: RunbookStepStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RunbookCounts {
    pub pending: usize,
    pub active: usize,
    pub completed: usize,
    pub blocked: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RunbookState {
    pub title: String,
    pub steps: Vec<RunbookStep>,
    #[serde(default)]
    pub selected_index: usize,
}

impl RunbookState {
    pub fn new(title: impl Into<String>, steps: Vec<RunbookStep>) -> Self {
        let mut state = Self {
            title: title.into(),
            steps,
            selected_index: 0,
        };
        state.normalize();
        state
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let mut state: Self = serde_json::from_str(raw).map_err(|error| error.to_string())?;
        state.normalize();
        Ok(state)
    }

    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_index = self.clamped_index(index);
    }

    pub fn move_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        if self.steps.is_empty() {
            self.selected_index = 0;
            return;
        }
        self.selected_index = (self.selected_index + 1).min(self.steps.len() - 1);
    }

    pub fn selected_step(&self) -> Option<&RunbookStep> {
        self.steps.get(self.selected_index)
    }

    pub fn mark_selected_active(&mut self) {
        let Some(selected_index) = self.selected_step_index() else {
            return;
        };

        for (index, step) in self.steps.iter_mut().enumerate() {
            if index != selected_index && step.status == RunbookStepStatus::Active {
                step.status = RunbookStepStatus::Pending;
            }
        }

        self.steps[selected_index].status = RunbookStepStatus::Active;
    }

    pub fn mark_selected_completed(&mut self) {
        if let Some(selected_index) = self.selected_step_index() {
            self.steps[selected_index].status = RunbookStepStatus::Completed;
        }
    }

    pub fn mark_selected_blocked(&mut self) {
        if let Some(selected_index) = self.selected_step_index() {
            self.steps[selected_index].status = RunbookStepStatus::Blocked;
        }
    }

    pub fn reset_statuses(&mut self) {
        for step in &mut self.steps {
            step.status = RunbookStepStatus::Pending;
        }
    }

    pub fn counts(&self) -> RunbookCounts {
        let mut counts = RunbookCounts::default();
        for step in &self.steps {
            match step.status {
                RunbookStepStatus::Pending => counts.pending += 1,
                RunbookStepStatus::Active => counts.active += 1,
                RunbookStepStatus::Completed => counts.completed += 1,
                RunbookStepStatus::Blocked => counts.blocked += 1,
            }
        }
        counts
    }

    fn normalize(&mut self) {
        self.selected_index = self.clamped_index(self.selected_index);

        let mut active_seen = false;
        for step in &mut self.steps {
            if step.status == RunbookStepStatus::Active {
                if active_seen {
                    step.status = RunbookStepStatus::Pending;
                } else {
                    active_seen = true;
                }
            }
        }
    }

    fn clamped_index(&self, index: usize) -> usize {
        if self.steps.is_empty() {
            0
        } else {
            index.min(self.steps.len() - 1)
        }
    }

    fn selected_step_index(&self) -> Option<usize> {
        if self.steps.is_empty() {
            None
        } else {
            Some(self.clamped_index(self.selected_index))
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

pub fn sample_state() -> Result<RunbookState, String> {
    RunbookState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
