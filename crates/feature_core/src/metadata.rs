use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{FeatureLabError, FeatureLabResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeatureKind {
    UiPattern,
    LogicPattern,
    Workflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeatureStatus {
    Draft,
    Experimental,
    Tested,
    Stable,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct FeatureItems {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureManifest {
    pub id: String,
    pub name: String,
    pub kind: FeatureKind,
    pub status: FeatureStatus,
    pub summary: String,
    pub inputs: FeatureItems,
    pub outputs: FeatureItems,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub compatible_features: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub owner: String,
    pub created_at: String,
    pub updated_at: String,
}

impl FeatureManifest {
    pub fn validate(&self) -> FeatureLabResult<()> {
        if self.id.trim().is_empty() {
            return Err(FeatureLabError::Validation(
                "feature id must not be empty".into(),
            ));
        }
        if !self.id.contains('.') {
            return Err(FeatureLabError::Validation(format!(
                "feature id '{}' should include a category prefix such as ui.right_inspector",
                self.id
            )));
        }
        if self.name.trim().is_empty() {
            return Err(FeatureLabError::Validation(
                "feature name must not be empty".into(),
            ));
        }
        if self.summary.trim().is_empty() {
            return Err(FeatureLabError::Validation(
                "feature summary must not be empty".into(),
            ));
        }
        if self.owner.trim().is_empty() {
            return Err(FeatureLabError::Validation(
                "feature owner must not be empty".into(),
            ));
        }
        for (label, value) in [
            ("created_at", self.created_at.as_str()),
            ("updated_at", self.updated_at.as_str()),
        ] {
            if !value.contains('T') || !value.ends_with('Z') {
                return Err(FeatureLabError::Validation(format!(
                    "{label} must be a timezone-aware UTC timestamp ending in 'Z'"
                )));
            }
        }
        if self.inputs.items.is_empty() {
            return Err(FeatureLabError::Validation(
                "feature inputs must not be empty".into(),
            ));
        }
        if self.outputs.items.is_empty() {
            return Err(FeatureLabError::Validation(
                "feature outputs must not be empty".into(),
            ));
        }
        Ok(())
    }
}

impl Display for FeatureKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::UiPattern => "ui_pattern",
            Self::LogicPattern => "logic_pattern",
            Self::Workflow => "workflow",
        };
        write!(f, "{label}")
    }
}

impl Display for FeatureStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Draft => "draft",
            Self::Experimental => "experimental",
            Self::Tested => "tested",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
        };
        write!(f, "{label}")
    }
}
