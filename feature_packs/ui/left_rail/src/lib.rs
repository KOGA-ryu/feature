use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.left_rail";
const EMPTY_STATE_MESSAGE: &str = "No navigation items are available.";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RailItem {
    pub id: String,
    pub title: String,
    pub section: String,
    pub icon: String,
    pub badge_count: Option<u32>,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RailFixture {
    pub items: Vec<RailItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RailSection {
    pub name: String,
    pub items: Vec<RailItem>,
}

#[derive(Debug, Clone)]
pub struct LeftRailState {
    items: Vec<RailItem>,
    selected_index: usize,
    collapsed: bool,
}

impl LeftRailState {
    pub fn new(items: Vec<RailItem>) -> Self {
        Self {
            items,
            selected_index: 0,
            collapsed: false,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let fixture: RailFixture = serde_json::from_str(raw).map_err(|error| error.to_string())?;
        Ok(Self::new(fixture.items))
    }

    pub fn set_selected_index(&mut self, selected_index: usize) {
        self.selected_index = selected_index;
        self.clamp_selected_index();
    }

    pub fn selected_index(&self) -> usize {
        self.clamped_selected_index_for(self.items.len())
    }

    pub fn move_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
        self.clamp_selected_index();
    }

    pub fn move_down(&mut self) {
        let item_len = self.items.len();
        if item_len == 0 {
            self.selected_index = 0;
            return;
        }
        self.selected_index = (self.selected_index + 1).min(item_len - 1);
    }

    pub fn set_collapsed(&mut self, collapsed: bool) {
        self.collapsed = collapsed;
    }

    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }

    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    pub fn items(&self) -> &[RailItem] {
        &self.items
    }

    pub fn grouped_sections(&self) -> Vec<RailSection> {
        let mut sections: Vec<RailSection> = Vec::new();
        for item in &self.items {
            if let Some(section) = sections
                .iter_mut()
                .find(|section| section.name == item.section)
            {
                section.items.push(item.clone());
            } else {
                sections.push(RailSection {
                    name: item.section.clone(),
                    items: vec![item.clone()],
                });
            }
        }
        sections
    }

    pub fn selected_item(&self) -> Option<&RailItem> {
        self.items.get(self.selected_index())
    }

    pub fn activate_selected(&self) -> Option<String> {
        self.selected_item()
            .and_then(|item| item.enabled.then(|| item.id.clone()))
    }

    pub fn total_badge_count(&self) -> u32 {
        self.items
            .iter()
            .map(|item| item.badge_count.unwrap_or(0))
            .sum()
    }

    pub fn empty_state_message(&self) -> Option<&'static str> {
        self.items.is_empty().then_some(EMPTY_STATE_MESSAGE)
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_navigation.json")
}

pub fn sample_rail() -> Result<LeftRailState, String> {
    LeftRailState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

impl LeftRailState {
    fn clamp_selected_index(&mut self) {
        self.selected_index = self.clamped_selected_index_for(self.items.len());
    }

    fn clamped_selected_index_for(&self, item_len: usize) -> usize {
        match item_len {
            0 => 0,
            len => self.selected_index.min(len - 1),
        }
    }
}
