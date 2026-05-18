use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.checklist_single";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: u64,
    pub title: String,
    pub is_completed: bool,
    pub created_at_order: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChecklistState {
    pub items: Vec<ChecklistItem>,
}

impl ChecklistState {
    pub fn add_item(&mut self, title: impl Into<String>) -> bool {
        let trimmed = title.into().trim().to_owned();
        if trimmed.is_empty() {
            return false;
        }

        let next_id = self.items.iter().map(|item| item.id).max().unwrap_or(0) + 1;
        let next_order = self
            .items
            .iter()
            .map(|item| item.created_at_order)
            .max()
            .unwrap_or(0)
            + 1;

        self.items.push(ChecklistItem {
            id: next_id,
            title: trimmed,
            is_completed: false,
            created_at_order: next_order,
        });
        true
    }

    pub fn toggle_item(&mut self, id: u64) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.is_completed = !item.is_completed;
        }
    }

    pub fn delete_item(&mut self, id: u64) {
        self.items.retain(|item| item.id != id);
    }

    pub fn clear_completed(&mut self) {
        self.items.retain(|item| !item.is_completed);
    }

    pub fn has_completed_items(&self) -> bool {
        self.items.iter().any(|item| item.is_completed)
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

pub fn sample_state() -> Result<ChecklistState, String> {
    serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
