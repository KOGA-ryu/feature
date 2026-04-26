use feature_core::parse_feature_manifest;
use spec_generator::{
    FEATURE_ID, SelectedFeatureKind, generate_spec, sample_invalid_input, sample_valid_input,
};

#[test]
fn valid_idea_builds_spec_with_required_sections() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let spec = generate_spec(input).expect("valid input should generate a spec");

    assert!(!spec.product_intent.is_empty());
    assert!(!spec.audience.is_empty());
    assert!(!spec.required_surfaces.is_empty());
    assert!(!spec.selected_patterns.is_empty());
    assert!(!spec.selected_logic.is_empty());
    assert!(!spec.data_models.is_empty());
    assert!(!spec.folder_structure.is_empty());
    assert!(!spec.first_vertical_slice.is_empty());
    assert!(!spec.dex_prompt_seed.is_empty());
    assert!(!spec.review_checklist.is_empty());
}

#[test]
fn missing_product_intent_fails() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation = generate_spec(input).expect_err("invalid input should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "product_intent")
    );
}

#[test]
fn missing_selected_features_fails() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation = generate_spec(input).expect_err("invalid input should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "selected_features")
    );
}

#[test]
fn out_of_scope_items_are_preserved() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let spec = generate_spec(input).expect("valid input should generate a spec");

    assert_eq!(spec.out_of_scope.len(), 2);
    assert_eq!(spec.out_of_scope[0], "No remote sync.");
}

#[test]
fn selected_feature_grouping_is_stable() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let spec = generate_spec(input).expect("valid input should generate a spec");

    assert_eq!(
        spec.required_surfaces,
        vec!["ui.left_rail", "ui.theme_editor"]
    );
    assert_eq!(spec.selected_logic, vec!["logic.search_index"]);
    assert_eq!(spec.selected_workflows, vec!["workflow.review_packet_flow"]);
    assert_eq!(spec.selected_features[0].kind, SelectedFeatureKind::Ui);
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "spec_generator");
    assert_eq!(manifest.inputs.items.len(), 6);
    assert_eq!(manifest.outputs.items.len(), 3);
}
