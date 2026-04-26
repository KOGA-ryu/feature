use feature_core::parse_feature_manifest;
use review_packet_flow::{
    FEATURE_ID, MergeRecommendation, ReviewPacketFlow, ReviewPacketSeverity, build_review_packet,
    sample_invalid_draft, sample_valid_draft, sample_valid_packet, validate_review_packet_draft,
};

#[test]
fn valid_draft_builds_packet_cleanly() {
    let draft = sample_valid_draft().expect("valid draft should parse");
    let validation = validate_review_packet_draft(&draft);

    assert!(validation.is_valid());
    assert!(validation.findings.is_empty());

    let packet = build_review_packet(draft).expect("valid draft should build");
    assert_eq!(packet.feature_id, "ui.theme_editor");
    assert_eq!(
        packet.merge_recommendation,
        MergeRecommendation::MergeWithFollowups
    );
}

#[test]
fn missing_required_sections_emit_findings() {
    let draft = sample_invalid_draft().expect("invalid draft should parse");
    let validation = validate_review_packet_draft(&draft);

    assert!(!validation.is_valid());
    let codes = validation
        .findings
        .iter()
        .map(|finding| finding.code.as_str())
        .collect::<Vec<_>>();

    assert!(codes.contains(&"missing_required_section"));
    assert!(codes.contains(&"timestamp_invalid"));
    assert!(codes.contains(&"test_command_missing"));
}

#[test]
fn builder_rejects_invalid_draft() {
    let draft = sample_invalid_draft().expect("invalid draft should parse");
    let validation = build_review_packet(draft).expect_err("invalid draft should not build");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "goal")
    );
    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "merge_recommendation")
    );
}

#[test]
fn packet_serialization_round_trips() {
    let packet = sample_valid_packet().expect("valid packet should build");
    let raw = serde_json::to_string_pretty(&packet).expect("packet should serialize");
    let parsed: review_packet_flow::ReviewPacket =
        serde_json::from_str(&raw).expect("packet should deserialize");

    assert_eq!(packet, parsed);
}

#[test]
fn merge_recommendation_is_preserved() {
    let packet = sample_valid_packet().expect("valid packet should build");
    assert_eq!(
        packet.merge_recommendation.to_string(),
        "merge_with_followups"
    );
}

#[test]
fn workflow_object_builds_and_validates() {
    let flow = ReviewPacketFlow;
    let draft = sample_valid_draft().expect("valid draft should parse");
    let validation = flow.validate(&draft);
    let packet = flow.build(draft).expect("flow should build valid packet");

    assert!(validation.is_valid());
    assert_eq!(packet.project, "features");
}

#[test]
fn severity_ordering_places_errors_first() {
    let draft = sample_invalid_draft().expect("invalid draft should parse");
    let validation = validate_review_packet_draft(&draft);

    assert!(
        validation
            .findings
            .iter()
            .all(|finding| finding.severity == ReviewPacketSeverity::Error)
    );
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "review_packet_flow");
    assert_eq!(manifest.inputs.items.len(), 6);
    assert_eq!(manifest.outputs.items.len(), 3);
}
