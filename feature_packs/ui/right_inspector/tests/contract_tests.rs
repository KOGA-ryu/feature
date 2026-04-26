use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");
    assert_eq!(manifest.id, "ui.right_inspector");
    assert_eq!(manifest.name, "right_inspector");
    assert_eq!(manifest.inputs.items.len(), 3);
    assert_eq!(manifest.outputs.items.len(), 3);
}

#[test]
fn fixture_is_valid_json() {
    let value: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/sample_input.json"))
            .expect("fixture should be valid JSON");
    assert_eq!(value["selected_object"]["name"], "Neuron Mesh");
}

#[test]
fn demo_surface_exposes_actions() {
    assert!(right_inspector::demo_actions().len() >= 3);
}
