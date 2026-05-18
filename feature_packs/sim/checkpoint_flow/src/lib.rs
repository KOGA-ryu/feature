use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "sim.checkpoint_flow";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageMode {
    Brawler,
    Racing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointMarker {
    pub id: u64,
    pub label: String,
    pub mode: StageMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingTransition {
    pub from: StageMode,
    pub to: StageMode,
    pub target_checkpoint_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointFlowState {
    pub checkpoints: Vec<CheckpointMarker>,
    pub current_checkpoint_index: usize,
    pub current_mode: StageMode,
    pub completed_checkpoint_ids: Vec<u64>,
    pub pending_transition: Option<PendingTransition>,
}

impl Default for StageMode {
    fn default() -> Self {
        Self::Brawler
    }
}

impl Default for CheckpointFlowState {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl CheckpointFlowState {
    pub fn new(checkpoints: Vec<CheckpointMarker>) -> Self {
        let current_mode = checkpoints
            .first()
            .map(|checkpoint| checkpoint.mode)
            .unwrap_or_default();
        Self {
            checkpoints,
            current_checkpoint_index: 0,
            current_mode,
            completed_checkpoint_ids: Vec::new(),
            pending_transition: None,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let mut state: Self = serde_json::from_str(raw).map_err(|error| error.to_string())?;
        state.normalize();
        Ok(state)
    }

    pub fn current_checkpoint(&self) -> Option<&CheckpointMarker> {
        self.checkpoints.get(self.clamped_index())
    }

    pub fn complete_current_checkpoint(&mut self) -> bool {
        let Some(current) = self.current_checkpoint().cloned() else {
            return false;
        };
        if self.pending_transition.is_some() {
            return false;
        }
        if !self.completed_checkpoint_ids.contains(&current.id) {
            self.completed_checkpoint_ids.push(current.id);
        }

        let next_index = self.current_checkpoint_index.saturating_add(1);
        let Some(next) = self.checkpoints.get(next_index).cloned() else {
            return true;
        };

        if next.mode != self.current_mode {
            self.pending_transition = Some(PendingTransition {
                from: self.current_mode,
                to: next.mode,
                target_checkpoint_id: next.id,
            });
        } else {
            self.current_checkpoint_index = next_index;
            self.current_mode = next.mode;
        }
        true
    }

    pub fn finish_transition(&mut self) -> bool {
        let Some(pending) = self.pending_transition.take() else {
            return false;
        };
        if self.current_checkpoint_index + 1 < self.checkpoints.len() {
            self.current_checkpoint_index += 1;
        }
        self.current_mode = pending.to;
        true
    }

    pub fn restart_from_checkpoint(&mut self) -> bool {
        let Some(current) = self.current_checkpoint().cloned() else {
            return false;
        };
        self.pending_transition = None;
        self.current_mode = current.mode;
        true
    }

    fn normalize(&mut self) {
        if self.checkpoints.is_empty() {
            self.current_checkpoint_index = 0;
            self.current_mode = StageMode::default();
            self.pending_transition = None;
            return;
        }

        self.current_checkpoint_index = self.clamped_index();
        self.current_mode = self
            .checkpoints
            .get(self.current_checkpoint_index)
            .map(|checkpoint| checkpoint.mode)
            .unwrap_or_default();

        self.completed_checkpoint_ids.retain(|id| {
            self.checkpoints
                .iter()
                .any(|checkpoint| checkpoint.id == *id)
        });
    }

    fn clamped_index(&self) -> usize {
        match self.checkpoints.len() {
            0 => 0,
            len => self.current_checkpoint_index.min(len - 1),
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

pub fn sample_state() -> Result<CheckpointFlowState, String> {
    CheckpointFlowState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
