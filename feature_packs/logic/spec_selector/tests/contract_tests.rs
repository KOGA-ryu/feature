use feature_core::parse_feature_manifest;
use spec_selector::{
    FEATURE_ID, SpecSelector, sample_incompatible_input, sample_invalid_input,
    sample_selection_result, sample_valid_input, select_spec,
};

#[test]
fn valid_selection_builds_spec_and_preserves_order() {
    let result = sample_selection_result().expect("valid selection should succeed");

    assert_eq!(
        result.selected_feature_ids,
        vec![
            "ui.left_rail",
            "logic.search_index",
            "workflow.review_packet_flow"
        ]
    );
    assert_eq!(result.generated_spec.selected_features.len(), 3);
    assert!(
        result
            .generated_spec
            .selected_logic
            .contains(&"logic.search_index".into())
    );
}

#[test]
fn incompatible_pair_drops_lower_scored_feature_deterministically() {
    let input = sample_incompatible_input().expect("fixture should parse");
    let result = select_spec(input).expect("selection should succeed");

    assert_eq!(result.selected_feature_ids, vec!["ui.theme_editor"]);
    assert!(
        result
            .rejected_feature_ids
            .contains(&"ui.right_inspector".into())
    );
    assert!(
        result
            .findings
            .iter()
            .any(|finding| finding.code == "incompatible_pair_resolved")
    );
}

#[test]
fn below_threshold_feature_is_excluded() {
    let mut input = sample_valid_input().expect("fixture should parse");
    let left_rail = input
        .reuse_inputs
        .iter_mut()
        .find(|item| item.feature_id == "ui.left_rail")
        .expect("left rail score should exist");
    left_rail.status = reuse_score::FeatureLifecycleStatus::Experimental;
    left_rail.usage_count = 0;
    left_rail.has_docs = false;
    left_rail.has_fixtures = false;
    left_rail.reviewed_recently = false;
    left_rail.failure_count = 2;

    let result = select_spec(input).expect("selection should succeed");

    assert!(!result.selected_feature_ids.contains(&"ui.left_rail".into()));
    assert!(
        result
            .decisions
            .iter()
            .find(|decision| decision.feature_id == "ui.left_rail")
            .expect("decision should exist")
            .reasons
            .iter()
            .any(|reason| reason.contains("below minimum"))
    );
}

#[test]
fn missing_reuse_input_fails() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation = select_spec(input).expect_err("missing reuse input should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "missing_reuse_input")
    );
}

#[test]
fn missing_rule_warning_is_retained_on_success() {
    let mut input = sample_valid_input().expect("fixture should parse");
    input.compatibility_rules.clear();

    let result = select_spec(input).expect("selection should still succeed");

    assert!(
        result
            .findings
            .iter()
            .any(|finding| finding.code == "missing_rule")
    );
    assert_eq!(result.compatibility_report.missing_rules.len(), 3);
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "spec_selector");
    assert_eq!(manifest.inputs.items.len(), 4);
    assert_eq!(manifest.outputs.items.len(), 4);
}

#[test]
fn selector_object_selects_specs() {
    let selector = SpecSelector;
    let input = sample_valid_input().expect("fixture should parse");
    let validation = selector.validate(&input);
    let result = selector
        .select(input)
        .expect("selector should produce a selection result");

    assert!(validation.is_valid());
    assert_eq!(
        result.generated_spec.required_surfaces,
        vec!["ui.left_rail"]
    );
}
