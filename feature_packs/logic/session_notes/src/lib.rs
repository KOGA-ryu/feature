use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.session_notes";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionNoteEntry {
    pub id: u64,
    pub text: String,
    pub created_at: String,
    pub pinned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionNotesFixture {
    pub entries: Vec<SessionNoteEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SessionNotesCounts {
    pub total: usize,
    pub pinned: usize,
    pub visible: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionNotesState {
    pub entries: Vec<SessionNoteEntry>,
    pub query: String,
    pub selected_index: usize,
    pub next_id: u64,
}

impl Default for SessionNotesState {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl SessionNotesState {
    pub fn new(entries: Vec<SessionNoteEntry>) -> Self {
        let next_id = entries.iter().map(|entry| entry.id).max().unwrap_or(0) + 1;
        Self {
            entries,
            query: String::new(),
            selected_index: 0,
            next_id,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let fixture: SessionNotesFixture =
            serde_json::from_str(raw).map_err(|error| error.to_string())?;
        Ok(Self::new(fixture.entries))
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.clamp_selected_index();
    }

    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_index = index;
        self.clamp_selected_index();
    }

    pub fn selected_entry(&self) -> Option<&SessionNoteEntry> {
        let visible = self.visible_entries();
        visible
            .get(self.clamped_selected_index_for(visible.len()))
            .copied()
    }

    pub fn visible_entries(&self) -> Vec<&SessionNoteEntry> {
        let query = normalized_query(&self.query);
        self.entries
            .iter()
            .filter(|entry| matches_query(entry, &query))
            .collect()
    }

    pub fn add_entry(&mut self, text: impl Into<String>, created_at: impl Into<String>) -> bool {
        let trimmed_text = text.into().trim().to_owned();
        if trimmed_text.is_empty() {
            return false;
        }

        self.entries.push(SessionNoteEntry {
            id: self.next_id,
            text: trimmed_text,
            created_at: created_at.into().trim().to_owned(),
            pinned: false,
        });
        self.next_id += 1;
        self.clamp_selected_index();
        true
    }

    pub fn update_entry(&mut self, id: u64, text: impl Into<String>) -> bool {
        let trimmed_text = text.into().trim().to_owned();
        if trimmed_text.is_empty() {
            return false;
        }

        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) {
            entry.text = trimmed_text;
            true
        } else {
            false
        }
    }

    pub fn toggle_pinned(&mut self, id: u64) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) {
            entry.pinned = !entry.pinned;
            true
        } else {
            false
        }
    }

    pub fn delete_entry(&mut self, id: u64) -> bool {
        let original_len = self.entries.len();
        self.entries.retain(|entry| entry.id != id);
        let deleted = self.entries.len() != original_len;
        if deleted {
            self.clamp_selected_index();
        }
        deleted
    }

    pub fn latest_entry(&self) -> Option<&SessionNoteEntry> {
        self.entries.last()
    }

    pub fn counts(&self) -> SessionNotesCounts {
        SessionNotesCounts {
            total: self.entries.len(),
            pinned: self.entries.iter().filter(|entry| entry.pinned).count(),
            visible: self.visible_entries().len(),
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

pub fn sample_state() -> Result<SessionNotesState, String> {
    SessionNotesState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

fn normalized_query(query: &str) -> String {
    query.trim().to_lowercase()
}

fn matches_query(entry: &SessionNoteEntry, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    entry.text.to_lowercase().contains(query) || entry.created_at.to_lowercase().contains(query)
}

impl SessionNotesState {
    fn clamp_selected_index(&mut self) {
        self.selected_index = self.clamped_selected_index_for(self.visible_entries().len());
    }

    fn clamped_selected_index_for(&self, visible_len: usize) -> usize {
        match visible_len {
            0 => 0,
            len => self.selected_index.min(len - 1),
        }
    }
}
