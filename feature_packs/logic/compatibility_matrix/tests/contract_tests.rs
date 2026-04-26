use compatibility_matrix::{
    CompatibilityMatrix, CompatibilitySeverity, CompatibilityStatus, FEATURE_ID,
    evaluate_compatibility, sample_incompatible_request, sample_missing_rule_request,
    sample_valid_report, sample_valid_request,
};
use feature_core::parse_feature_manifest;

#[test]
fn compatible_pair_is_reported_correctly() {
    let report = sample_valid_report().expect("valid report should build");

    assert!(
        report
            .compatible_pairs
            .iter()
            .any(|pair| pair == "ui.command_palette + ui.left_rail")
    );
}

#[test]
fn risky_pair_is_reported_correctly() {
    let report = sample_valid_report().expect("valid report should build");

    assert!(
        report
            .risky_pairs
            .iter()
            .any(|pair| pair == "ui.command_palette + workflow.review_packet_flow")
    );
}

#[test]
fn incompatible_pair_is_reported_correctly() {
    let request = sample_incompatible_request().expect("fixture should parse");
    let report = evaluate_compatibility(request).expect("incompatible rule should still report");

    assert_eq!(report.incompatible_pairs.len(), 1);
    assert_eq!(
        report.incompatible_pairs[0],
        "ui.theme_editor + ui.right_inspector"
    );
}

#[test]
fn symmetric_rule_lookup_works() {
    let report = sample_valid_report().expect("valid report should build");
    let pairing = report
        .pairings
        .iter()
        .find(|pairing| {
            pairing.left_feature_id == "ui.command_palette"
                && pairing.right_feature_id == "ui.left_rail"
        })
        .expect("symmetric pairing should exist");

    assert_eq!(pairing.status, CompatibilityStatus::Compatible);
    assert!(pairing.rationale.contains("discoverability"));
}

#[test]
fn missing_rule_emits_warning() {
    let request = sample_missing_rule_request().expect("fixture should parse");
    let report = evaluate_compatibility(request).expect("missing rule should still report");

    assert_eq!(report.missing_rules.len(), 1);
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.severity == CompatibilitySeverity::Warning
                && finding.code == "missing_rule")
    );
}

#[test]
fn pair_ordering_is_stable() {
    let report = sample_valid_report().expect("valid report should build");

    assert_eq!(report.pairings.len(), 3);
    assert_eq!(report.pairings[0].left_feature_id, "ui.command_palette");
    assert_eq!(report.pairings[0].right_feature_id, "ui.left_rail");
    assert_eq!(report.pairings[1].left_feature_id, "ui.command_palette");
    assert_eq!(
        report.pairings[1].right_feature_id,
        "workflow.review_packet_flow"
    );
    assert_eq!(report.pairings[2].left_feature_id, "ui.left_rail");
    assert_eq!(
        report.pairings[2].right_feature_id,
        "workflow.review_packet_flow"
    );
}

#[test]
fn matrix_object_evaluates_requests() {
    let matrix = CompatibilityMatrix;
    let request = sample_valid_request().expect("fixture should parse");
    let validation = matrix.validate(&request);
    let report = matrix
        .evaluate(request)
        .expect("matrix should evaluate valid request");

    assert!(validation.is_valid());
    assert_eq!(report.pairings.len(), 3);
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "compatibility_matrix");
    assert_eq!(manifest.inputs.items.len(), 2);
    assert_eq!(manifest.outputs.items.len(), 3);
}
