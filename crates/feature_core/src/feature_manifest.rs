use std::fs;
use std::path::Path;

use crate::{FeatureLabError, FeatureLabResult, FeatureManifest};

pub fn parse_feature_manifest(raw: &str) -> FeatureLabResult<FeatureManifest> {
    let manifest: FeatureManifest = toml::from_str(raw)?;
    manifest.validate()?;
    Ok(manifest)
}

pub fn load_feature_manifest(path: &Path) -> FeatureLabResult<FeatureManifest> {
    let raw = fs::read_to_string(path)
        .map_err(|error| FeatureLabError::io(Some(path.to_path_buf()), error))?;
    let manifest: FeatureManifest = toml::from_str(&raw)
        .map_err(|error| FeatureLabError::toml(Some(path.to_path_buf()), error))?;
    manifest.validate()?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::parse_feature_manifest;

    #[test]
    fn parses_valid_manifest() {
        let raw = r#"
id = "ui.right_inspector"
name = "right_inspector"
kind = "ui_pattern"
status = "experimental"
summary = "Persistent right-side detail panel."
dependencies = ["feature_core"]
compatible_features = ["ui.left_rail"]
tags = ["ui", "layout"]
owner = "feature_lab"
created_at = "2026-04-26T00:00:00Z"
updated_at = "2026-04-26T00:00:00Z"
[inputs]
items = ["selection"]
[outputs]
items = ["detail_view"]
"#;

        let manifest = parse_feature_manifest(raw).expect("manifest should parse");
        assert_eq!(manifest.id, "ui.right_inspector");
        assert_eq!(manifest.inputs.items, vec!["selection"]);
    }
}
