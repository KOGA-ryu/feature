use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.document_history";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentRevision {
    pub id: u64,
    pub label: String,
    pub text: String,
    pub recorded_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DocumentHistoryCounts {
    pub revisions: usize,
    pub dirty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentHistoryState {
    pub working_text: String,
    pub revisions: Vec<DocumentRevision>,
    pub selected_revision_index: usize,
    pub next_id: u64,
    pub clean_text: String,
}

impl Default for DocumentHistoryState {
    fn default() -> Self {
        Self::new("")
    }
}

impl DocumentHistoryState {
    pub fn new(initial_text: impl Into<String>) -> Self {
        let text = initial_text.into();
        Self {
            working_text: text.clone(),
            revisions: Vec::new(),
            selected_revision_index: 0,
            next_id: 1,
            clean_text: text,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let mut state: Self = serde_json::from_str(raw).map_err(|error| error.to_string())?;
        state.normalize();
        Ok(state)
    }

    pub fn working_text(&self) -> &str {
        &self.working_text
    }

    pub fn set_working_text(&mut self, text: impl Into<String>) {
        self.working_text = text.into();
    }

    pub fn selected_revision(&self) -> Option<&DocumentRevision> {
        self.revisions.get(self.clamped_selected_index())
    }

    pub fn set_selected_revision_index(&mut self, index: usize) {
        self.selected_revision_index = self.clamp_index(index);
    }

    pub fn record_snapshot(
        &mut self,
        label: impl Into<String>,
        recorded_at: impl Into<String>,
    ) -> bool {
        if self
            .latest_revision()
            .is_some_and(|revision| revision.text == self.working_text)
        {
            return false;
        }

        let revision = DocumentRevision {
            id: self.next_id,
            label: label.into().trim().to_owned(),
            text: self.working_text.clone(),
            recorded_at: recorded_at.into().trim().to_owned(),
        };

        self.revisions.push(revision);
        self.next_id += 1;
        self.selected_revision_index = self.revisions.len().saturating_sub(1);
        true
    }

    pub fn restore_selected_revision(&mut self) -> bool {
        let Some(revision) = self.selected_revision().cloned() else {
            return false;
        };
        self.working_text = revision.text;
        true
    }

    pub fn latest_revision(&self) -> Option<&DocumentRevision> {
        self.revisions.last()
    }

    pub fn revision_count(&self) -> usize {
        self.revisions.len()
    }

    pub fn is_dirty(&self) -> bool {
        self.working_text != self.clean_text
    }

    pub fn mark_clean(&mut self) {
        self.clean_text = self.working_text.clone();
    }

    pub fn counts(&self) -> DocumentHistoryCounts {
        DocumentHistoryCounts {
            revisions: self.revision_count(),
            dirty: self.is_dirty(),
        }
    }

    fn normalize(&mut self) {
        self.selected_revision_index = self.clamp_index(self.selected_revision_index);
        let minimum_next_id = self
            .revisions
            .iter()
            .map(|revision| revision.id)
            .max()
            .unwrap_or(0)
            + 1;
        if self.next_id < minimum_next_id {
            self.next_id = minimum_next_id;
        }
    }

    fn clamp_index(&self, index: usize) -> usize {
        match self.revisions.len() {
            0 => 0,
            len => index.min(len - 1),
        }
    }

    fn clamped_selected_index(&self) -> usize {
        self.clamp_index(self.selected_revision_index)
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

pub fn sample_state() -> Result<DocumentHistoryState, String> {
    DocumentHistoryState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
