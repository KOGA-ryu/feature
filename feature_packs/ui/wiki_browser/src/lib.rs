use feature_core::{
    FeatureKind, FeatureLabResult, FeatureManifest, FeatureStatus, parse_feature_manifest,
};
use feature_registry::{FeatureRegistry, RegisteredFeature};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.wiki_browser";
const EMPTY_STATE_MESSAGE: &str = "No features match the current browser state.";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WikiBrowserEntry {
    pub manifest: FeatureManifest,
    pub package_name: String,
    pub feature_dir: String,
    pub readme_path: String,
    pub fixtures_dir: String,
    pub readme_text: String,
    pub fixture_preview: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WikiBrowserFixture {
    pub entries: Vec<WikiBrowserEntry>,
}

#[derive(Debug, Clone)]
pub struct WikiBrowserState {
    entries: Vec<WikiBrowserEntry>,
    query: String,
    kind_filter: Option<FeatureKind>,
    status_filter: Option<FeatureStatus>,
    tag_filter: Option<String>,
    selected_index: usize,
}

impl WikiBrowserEntry {
    pub fn from_registered_feature(feature: &RegisteredFeature) -> FeatureLabResult<Self> {
        Ok(Self {
            manifest: feature.manifest.clone(),
            package_name: feature.package_name.clone(),
            feature_dir: feature.feature_dir.display().to_string(),
            readme_path: feature.readme_path.display().to_string(),
            fixtures_dir: feature.fixtures_dir.display().to_string(),
            readme_text: feature.readme_text()?,
            fixture_preview: feature.first_fixture_preview()?,
        })
    }
}

impl WikiBrowserState {
    pub fn new(entries: Vec<WikiBrowserEntry>) -> Self {
        Self {
            entries,
            query: String::new(),
            kind_filter: None,
            status_filter: None,
            tag_filter: None,
            selected_index: 0,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let fixture: WikiBrowserFixture =
            serde_json::from_str(raw).map_err(|error| error.to_string())?;
        Ok(Self::new(fixture.entries))
    }

    pub fn from_registry(registry: &FeatureRegistry) -> FeatureLabResult<Self> {
        let entries = registry
            .features()
            .iter()
            .map(WikiBrowserEntry::from_registered_feature)
            .collect::<FeatureLabResult<Vec<_>>>()?;
        Ok(Self::new(entries))
    }

    pub fn entries(&self) -> &[WikiBrowserEntry] {
        &self.entries
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn kind_filter(&self) -> Option<&FeatureKind> {
        self.kind_filter.as_ref()
    }

    pub fn status_filter(&self) -> Option<&FeatureStatus> {
        self.status_filter.as_ref()
    }

    pub fn tag_filter(&self) -> Option<&str> {
        self.tag_filter.as_deref()
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        let previous_feature_id = self.selected_feature_id().map(str::to_owned);
        self.query = query.into();
        self.reconcile_selection(previous_feature_id.as_deref());
    }

    pub fn set_kind_filter(&mut self, kind_filter: Option<FeatureKind>) {
        let previous_feature_id = self.selected_feature_id().map(str::to_owned);
        self.kind_filter = kind_filter;
        self.reconcile_selection(previous_feature_id.as_deref());
    }

    pub fn set_status_filter(&mut self, status_filter: Option<FeatureStatus>) {
        let previous_feature_id = self.selected_feature_id().map(str::to_owned);
        self.status_filter = status_filter;
        self.reconcile_selection(previous_feature_id.as_deref());
    }

    pub fn set_tag_filter(&mut self, tag_filter: Option<String>) {
        let previous_feature_id = self.selected_feature_id().map(str::to_owned);
        self.tag_filter = tag_filter
            .map(|value| value.trim().to_lowercase())
            .filter(|value| !value.is_empty());
        self.reconcile_selection(previous_feature_id.as_deref());
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

    pub fn visible_entries(&self) -> Vec<&WikiBrowserEntry> {
        let query = normalized_query(&self.query);
        self.entries
            .iter()
            .filter(|entry| {
                matches_entry(
                    entry,
                    &query,
                    self.kind_filter.as_ref(),
                    self.status_filter.as_ref(),
                    self.tag_filter.as_deref(),
                )
            })
            .collect()
    }

    pub fn selected_entry(&self) -> Option<&WikiBrowserEntry> {
        let visible = self.visible_entries();
        visible.get(self.selected_index()).copied()
    }

    pub fn selected_manifest(&self) -> Option<&FeatureManifest> {
        self.selected_entry().map(|entry| &entry.manifest)
    }

    pub fn selected_feature_id(&self) -> Option<&str> {
        self.selected_entry()
            .map(|entry| entry.manifest.id.as_str())
    }

    pub fn selected_readme_text(&self) -> Option<&str> {
        self.selected_entry()
            .map(|entry| entry.readme_text.as_str())
    }

    pub fn selected_fixture_preview(&self) -> Option<&str> {
        self.selected_entry()
            .and_then(|entry| entry.fixture_preview.as_deref())
    }

    pub fn selected_feature_dir(&self) -> Option<&str> {
        self.selected_entry()
            .map(|entry| entry.feature_dir.as_str())
    }

    pub fn selected_readme_path(&self) -> Option<&str> {
        self.selected_entry()
            .map(|entry| entry.readme_path.as_str())
    }

    pub fn selected_fixtures_dir(&self) -> Option<&str> {
        self.selected_entry()
            .map(|entry| entry.fixtures_dir.as_str())
    }

    pub fn empty_state_message(&self) -> Option<&'static str> {
        self.visible_entries()
            .is_empty()
            .then_some(EMPTY_STATE_MESSAGE)
    }

    pub fn select_feature(&mut self, feature_id: &str) -> bool {
        let Some(index) = self
            .visible_entries()
            .iter()
            .position(|entry| entry.manifest.id == feature_id)
        else {
            return false;
        };
        self.selected_index = index;
        true
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_browser_entries.json")
}

pub fn sample_browser() -> Result<WikiBrowserState, String> {
    WikiBrowserState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

fn normalized_query(query: &str) -> String {
    query.trim().to_lowercase()
}

fn matches_entry(
    entry: &WikiBrowserEntry,
    query: &str,
    kind_filter: Option<&FeatureKind>,
    status_filter: Option<&FeatureStatus>,
    tag_filter: Option<&str>,
) -> bool {
    if let Some(kind) = kind_filter {
        if &entry.manifest.kind != kind {
            return false;
        }
    }

    if let Some(status) = status_filter {
        if &entry.manifest.status != status {
            return false;
        }
    }

    if let Some(tag) = tag_filter {
        if !entry
            .manifest
            .tags
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(tag))
        {
            return false;
        }
    }

    if query.is_empty() {
        return true;
    }

    entry.manifest.id.to_lowercase().contains(query)
        || entry.manifest.name.to_lowercase().contains(query)
        || entry.manifest.summary.to_lowercase().contains(query)
        || entry
            .manifest
            .tags
            .iter()
            .any(|tag| tag.to_lowercase().contains(query))
}

impl WikiBrowserState {
    fn clamp_selected_index(&mut self) {
        self.selected_index = self.clamped_selected_index_for(self.visible_entries().len());
    }

    fn reconcile_selection(&mut self, previous_feature_id: Option<&str>) {
        let visible = self.visible_entries();
        if visible.is_empty() {
            self.selected_index = 0;
            return;
        }

        if let Some(feature_id) = previous_feature_id {
            if let Some(index) = visible
                .iter()
                .position(|entry| entry.manifest.id == feature_id)
            {
                self.selected_index = index;
                return;
            }
        }

        self.selected_index = self.clamped_selected_index_for(visible.len());
    }

    fn clamped_selected_index_for(&self, visible_len: usize) -> usize {
        match visible_len {
            0 => 0,
            len => self.selected_index.min(len - 1),
        }
    }
}
