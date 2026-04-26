use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.search_index";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchItem {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub kind: String,
    pub tags: Vec<String>,
    pub dependencies: Vec<String>,
    pub compatible_features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchFixture {
    pub items: Vec<SearchItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub kind: Option<String>,
    pub tag: Option<String>,
}

impl SearchRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct SearchIndex {
    items: Vec<SearchItem>,
}

impl SearchIndex {
    pub fn new(items: Vec<SearchItem>) -> Self {
        Self { items }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let fixture: SearchFixture =
            serde_json::from_str(raw).map_err(|error| error.to_string())?;
        Ok(Self::new(fixture.items))
    }

    pub fn items(&self) -> &[SearchItem] {
        &self.items
    }

    pub fn search(&self, request: &SearchRequest) -> Vec<&SearchItem> {
        let query = normalized(request.query.as_deref().unwrap_or_default());
        let kind = normalized_option(request.kind.as_deref());
        let tag = normalized_option(request.tag.as_deref());

        self.items
            .iter()
            .filter(|item| matches_query(item, &query))
            .filter(|item| matches_kind(item, kind.as_deref()))
            .filter(|item| matches_tag(item, tag.as_deref()))
            .collect()
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_items.json")
}

pub fn parse_fixture(raw: &str) -> Result<SearchFixture, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_index() -> Result<SearchIndex, String> {
    SearchIndex::from_fixture_str(sample_fixture())
}

fn normalized(value: &str) -> String {
    value.trim().to_lowercase()
}

fn normalized_option(value: Option<&str>) -> Option<String> {
    value
        .map(normalized)
        .and_then(|value| (!value.is_empty()).then_some(value))
}

fn matches_query(item: &SearchItem, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    item.title.to_lowercase().contains(query)
        || item.summary.to_lowercase().contains(query)
        || item
            .tags
            .iter()
            .any(|tag| tag.to_lowercase().contains(query))
}

fn matches_kind(item: &SearchItem, kind: Option<&str>) -> bool {
    kind.is_none_or(|kind| item.kind.eq_ignore_ascii_case(kind))
}

fn matches_tag(item: &SearchItem, tag: Option<&str>) -> bool {
    tag.is_none_or(|tag| {
        item.tags
            .iter()
            .any(|item_tag| item_tag.eq_ignore_ascii_case(tag))
    })
}
