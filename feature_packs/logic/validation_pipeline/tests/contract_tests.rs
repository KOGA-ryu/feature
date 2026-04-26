use feature_core::parse_feature_manifest;
use validation_pipeline::{
    FEATURE_ID, ValidationInput, ValidationOutcome, ValidationSeverity, sample_invalid_input,
    sample_valid_input, validate_input,
};

#[test]
fn valid_fixture_passes_without_findings() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let result = validate_input(&input);

    assert_eq!(result.summary.outcome, ValidationOutcome::Passed);
    assert_eq!(result.summary.error_count, 0);
    assert_eq!(result.summary.warning_count, 0);
    assert!(result.findings.is_empty());
}

#[test]
fn invalid_fixture_fails_and_aggregates_counts() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let result = validate_input(&input);

    assert_eq!(result.summary.outcome, ValidationOutcome::Failed);
    assert_eq!(result.summary.error_count, 6);
    assert_eq!(result.summary.warning_count, 1);
    assert_eq!(result.summary.total_findings, 7);
}

#[test]
fn severity_ordering_places_errors_before_warnings() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let result = validate_input(&input);

    let last_error_index = result
        .findings
        .iter()
        .rposition(|finding| finding.severity == ValidationSeverity::Error)
        .expect("should contain errors");
    let first_warning_index = result
        .findings
        .iter()
        .position(|finding| finding.severity == ValidationSeverity::Warning)
        .expect("should contain warnings");

    assert!(last_error_index < first_warning_index);
}

#[test]
fn empty_input_returns_passed() {
    let result = validate_input(&ValidationInput::default());

    assert_eq!(result.summary.outcome, ValidationOutcome::Passed);
    assert!(result.findings.is_empty());
}

#[test]
fn multiple_findings_are_emitted_for_invalid_fixture() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let result = validate_input(&input);

    let codes = result
        .findings
        .iter()
        .map(|finding| finding.code.as_str())
        .collect::<Vec<_>>();

    assert!(codes.contains(&"required_field_missing"));
    assert!(codes.contains(&"collection_empty"));
    assert!(codes.contains(&"timestamp_invalid"));
    assert!(codes.contains(&"dependency_missing"));
    assert!(codes.contains(&"compatible_feature_unknown"));
}

#[test]
fn warning_only_results_pass_with_warnings() {
    let input = ValidationInput {
        compatible_features: vec!["ui.missing_feature".into()],
        available_features: vec!["ui.command_palette".into()],
        ..ValidationInput::default()
    };
    let result = validate_input(&input);

    assert_eq!(
        result.summary.outcome,
        ValidationOutcome::PassedWithWarnings
    );
    assert_eq!(result.summary.error_count, 0);
    assert_eq!(result.summary.warning_count, 1);
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "validation_pipeline");
    assert_eq!(manifest.inputs.items.len(), 5);
    assert_eq!(manifest.outputs.items.len(), 3);
}
