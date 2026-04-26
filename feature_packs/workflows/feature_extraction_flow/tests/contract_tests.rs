use feature_core::parse_feature_manifest;
use feature_extraction_flow::{
    FEATURE_ID, FeatureExtractionFlow, extract_feature, sample_extraction_packet,
    sample_invalid_input, sample_valid_input,
};

#[test]
fn valid_input_produces_complete_extraction_packet() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let packet = extract_feature(input).expect("valid extraction should succeed");

    assert!(!packet.feature_summary.is_empty());
    assert!(!packet.proposed_blueprint.is_empty());
    assert!(!packet.proposed_contract_notes.is_empty());
    assert!(!packet.proposed_test_notes.is_empty());
    assert!(!packet.proposed_failure_modes.is_empty());
    assert!(!packet.proposed_library_paths.is_empty());
}

#[test]
fn invalid_review_packet_input_fails() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation = extract_feature(input).expect_err("invalid extraction input should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path.starts_with("review_packet."))
    );
}

#[test]
fn proposed_library_paths_are_emitted() {
    let packet = sample_extraction_packet().expect("valid extraction should succeed");

    assert!(
        packet
            .proposed_library_paths
            .iter()
            .any(|path| path.starts_with("library/specs/"))
    );
    assert_eq!(packet.proposed_library_paths.len(), 3);
}

#[test]
fn serialization_round_trips() {
    let packet = sample_extraction_packet().expect("valid extraction should succeed");
    let raw = serde_json::to_string_pretty(&packet).expect("packet should serialize");
    let parsed: feature_extraction_flow::ExtractionPacket =
        serde_json::from_str(&raw).expect("packet should deserialize");

    assert_eq!(packet, parsed);
}

#[test]
fn workflow_object_builds_packets() {
    let flow = FeatureExtractionFlow;
    let input = sample_valid_input().expect("valid fixture should parse");
    let validation = flow.validate(&input);
    let packet = flow
        .extract(input)
        .expect("flow should extract valid packet");

    assert!(validation.is_valid());
    assert!(packet.feature_summary.contains("ui.theme_editor"));
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "feature_extraction_flow");
    assert_eq!(manifest.inputs.items.len(), 6);
    assert_eq!(manifest.outputs.items.len(), 3);
}
