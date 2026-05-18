use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.quick_capture_inbox";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InboxItemStatus {
    #[default]
    Pending,
    Processed,
    Archived,
}

impl InboxItemStatus {
    pub fn as_query_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processed => "processed",
            Self::Archived => "archived",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboxItem {
    pub id: u64,
    pub text: String,
    pub captured_at: String,
    #[serde(default)]
    pub status: InboxItemStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InboxCounts {
    pub total: usize,
    pub pending: usize,
    pub visible: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickCaptureInboxState {
    pub items: Vec<InboxItem>,
    #[serde(default)]
    pub draft_text: String,
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub selected_index: usize,
    #[serde(default)]
    pub next_id: u64,
}

impl Default for QuickCaptureInboxState {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl QuickCaptureInboxState {
    pub fn new(items: Vec<InboxItem>) -> Self {
        let mut state = Self {
            next_id: next_id_for(&items),
            items,
            draft_text: String::new(),
            query: String::new(),
            selected_index: 0,
        };
        state.clamp_selected_index();
        state
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let mut state: Self = serde_json::from_str(raw).map_err(|error| error.to_string())?;
        state.next_id = state.next_id.max(next_id_for(&state.items));
        state.clamp_selected_index();
        Ok(state)
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.clamp_selected_index();
    }

    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_index = index;
        self.clamp_selected_index();
    }

    pub fn move_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        let visible_len = self.visible_entries_len();
        if visible_len == 0 {
            self.selected_index = 0;
            return;
        }
        self.selected_index = (self.selected_index + 1).min(visible_len - 1);
    }

    pub fn set_draft_text(&mut self, text: impl Into<String>) {
        self.draft_text = text.into();
    }

    pub fn capture_draft(&mut self, captured_at: impl Into<String>) -> bool {
        let draft = self.draft_text.clone();
        let captured = self.capture_text(draft, captured_at);
        if captured {
            self.draft_text.clear();
        }
        captured
    }

    pub fn capture_text(
        &mut self,
        text: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> bool {
        let trimmed_text = text.into().trim().to_owned();
        if trimmed_text.is_empty() {
            return false;
        }

        self.items.push(InboxItem {
            id: self.next_id,
            text: trimmed_text,
            captured_at: captured_at.into().trim().to_owned(),
            status: InboxItemStatus::Pending,
        });
        self.next_id += 1;
        self.clamp_selected_index();
        true
    }

    pub fn selected_item(&self) -> Option<&InboxItem> {
        let visible = self.visible_items();
        visible
            .get(self.clamped_selected_index_for(visible.len()))
            .copied()
    }

    pub fn visible_items(&self) -> Vec<&InboxItem> {
        let query = normalized_query(&self.query);
        self.items
            .iter()
            .filter(|item| matches_query(item, &query))
            .collect()
    }

    pub fn mark_selected_processed(&mut self) -> bool {
        self.set_selected_status(InboxItemStatus::Processed)
    }

    pub fn mark_selected_archived(&mut self) -> bool {
        self.set_selected_status(InboxItemStatus::Archived)
    }

    pub fn delete_selected(&mut self) -> bool {
        let Some(id) = self.selected_item_id() else {
            return false;
        };

        let original_len = self.items.len();
        self.items.retain(|item| item.id != id);
        let deleted = self.items.len() != original_len;
        if deleted {
            self.clamp_selected_index();
        }
        deleted
    }

    pub fn counts(&self) -> InboxCounts {
        InboxCounts {
            total: self.items.len(),
            pending: self
                .items
                .iter()
                .filter(|item| item.status == InboxItemStatus::Pending)
                .count(),
            visible: self.visible_entries_len(),
        }
    }

    fn set_selected_status(&mut self, status: InboxItemStatus) -> bool {
        let Some(id) = self.selected_item_id() else {
            return false;
        };

        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.status = status;
            true
        } else {
            false
        }
    }

    fn selected_item_id(&self) -> Option<u64> {
        self.selected_item().map(|item| item.id)
    }

    fn clamp_selected_index(&mut self) {
        self.selected_index = self.clamped_selected_index_for(self.visible_entries_len());
    }

    fn clamped_selected_index_for(&self, visible_len: usize) -> usize {
        match visible_len {
            0 => 0,
            len => self.selected_index.min(len - 1),
        }
    }

    fn visible_entries_len(&self) -> usize {
        let query = normalized_query(&self.query);
        self.items
            .iter()
            .filter(|item| matches_query(item, &query))
            .count()
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

pub fn sample_state() -> Result<QuickCaptureInboxState, String> {
    QuickCaptureInboxState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

fn normalized_query(query: &str) -> String {
    query.trim().to_lowercase()
}

fn matches_query(item: &InboxItem, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    item.text.to_lowercase().contains(query)
        || item.captured_at.to_lowercase().contains(query)
        || item.status.as_query_str().contains(query)
}

fn next_id_for(items: &[InboxItem]) -> u64 {
    items.iter().map(|item| item.id).max().unwrap_or(0) + 1
}
