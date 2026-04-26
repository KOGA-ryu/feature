use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};

pub const FEATURE_ID: &str = "ui.right_inspector";

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_input.json")
}

pub fn demo_actions() -> &'static [&'static str] {
    &[
        "focus detail panel",
        "open linked artifact",
        "queue contextual action",
    ]
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
