use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.activity_stream";
const EMPTY_STATE_MESSAGE: &str = "No activity entries match the current query.";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub id: String,
    pub title: String,
    pub detail: String,
    pub kind: String,
    pub status: String,
    pub actor: String,
    pub timestamp: String,
    pub unread: bool,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityFixture {
    pub entries: Vec<ActivityEntry>,
}

#[derive(Debug, Clone)]
pub struct ActivityStream {
    entries: Vec<ActivityEntry>,
    query: String,
    selected_index: usize,
}

impl ActivityStream {
    pub fn new(entries: Vec<ActivityEntry>) -> Self {
        Self {
            entries,
            query: String::new(),
            selected_index: 0,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let fixture: ActivityFixture =
            serde_json::from_str(raw).map_err(|error| error.to_string())?;
        Ok(Self::new(fixture.entries))
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.clamp_selected_index();
    }

    pub fn set_selected_index(&mut self, selected_index: usize) {
        self.selected_index = selected_index;
        self.clamp_selected_index();
    }

    pub fn selected_index(&self) -> usize {
        self.clamped_selected_index_for(self.visible_entries().len())
    }

    pub fn move_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
        self.clamp_selected_index();
    }

    pub fn move_down(&mut self) {
        let visible_len = self.visible_entries().len();
        if visible_len == 0 {
            self.selected_index = 0;
            return;
        }
        self.selected_index = (self.selected_index + 1).min(visible_len - 1);
    }

    pub fn visible_entries(&self) -> Vec<&ActivityEntry> {
        let query = normalized_query(&self.query);
        self.entries
            .iter()
            .filter(|entry| matches_query(entry, &query))
            .collect()
    }

    pub fn selected_entry(&self) -> Option<&ActivityEntry> {
        let visible = self.visible_entries();
        visible.get(self.selected_index()).copied()
    }

    pub fn activate_selected(&self) -> Option<String> {
        self.selected_entry()
            .and_then(|entry| entry.actionable.then(|| entry.id.clone()))
    }

    pub fn visible_unread_count(&self) -> usize {
        self.visible_entries()
            .into_iter()
            .filter(|entry| entry.unread)
            .count()
    }

    pub fn empty_state_message(&self) -> Option<&'static str> {
        self.visible_entries()
            .is_empty()
            .then_some(EMPTY_STATE_MESSAGE)
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_activity.json")
}

pub fn sample_stream() -> Result<ActivityStream, String> {
    ActivityStream::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

fn normalized_query(query: &str) -> String {
    query.trim().to_lowercase()
}

fn matches_query(entry: &ActivityEntry, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    entry.id.to_lowercase().contains(query)
        || entry.title.to_lowercase().contains(query)
        || entry.detail.to_lowercase().contains(query)
        || entry.kind.to_lowercase().contains(query)
        || entry.status.to_lowercase().contains(query)
        || entry.actor.to_lowercase().contains(query)
}

impl ActivityStream {
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
