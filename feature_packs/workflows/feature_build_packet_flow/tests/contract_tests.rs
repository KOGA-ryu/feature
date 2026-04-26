use feature_build_packet_flow::{
    FEATURE_ID, FeatureBuildPacketFlow, build_feature_packets, sample_invalid_input,
    sample_tie_break_input, sample_valid_bundle, sample_valid_input,
};
use feature_core::parse_feature_manifest;
use prompt_generator::PromptTargetKind;

#[test]
fn valid_input_selects_the_highest_scoring_selected_feature() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(bundle.selected_feature_id, "logic.search_index");
    assert!(bundle.selection_reason.contains("Reuse score 100"));
}

#[test]
fn tie_on_score_chooses_the_earlier_feature_in_selected_feature_ids() {
    let input = sample_tie_break_input().expect("tie-break fixture should parse");
    let bundle = build_feature_packets(input).expect("tie-break bundle should build");

    assert_eq!(bundle.selected_feature_id, "ui.left_rail");
}

#[test]
fn bundle_copies_rejected_feature_ids() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(bundle.rejected_feature_ids, vec!["ui.command_palette"]);
}

#[test]
fn bundle_preserves_upstream_findings() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(bundle.upstream_findings.len(), 1);
    assert_eq!(bundle.upstream_findings[0].code, "missing_rule");
}

#[test]
fn worker_integrator_and_reviewer_packets_have_correct_target_kinds() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(
        bundle.worker_packet.target_kind,
        PromptTargetKind::FeatureWorker
    );
    assert_eq!(
        bundle.integrator_packet.target_kind,
        PromptTargetKind::Integrator
    );
    assert_eq!(
        bundle.reviewer_packet.target_kind,
        PromptTargetKind::Reviewer
    );
}

#[test]
fn integrator_and_reviewer_use_a_one_item_wave() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert!(
        bundle
            .integrator_packet
            .prompt_text
            .contains("Features to integrate:\n- logic.search_index")
    );
    assert!(
        bundle
            .reviewer_packet
            .verification_commands
            .iter()
            .filter(|command| command.starts_with("cargo run -p feature_cli -- show "))
            .eq(vec!["cargo run -p feature_cli -- show logic.search_index"].iter())
    );
    assert!(
        bundle
            .reviewer_packet
            .verification_commands
            .iter()
            .filter(|command| command.starts_with("cargo run -p feature_cli -- test "))
            .eq(vec!["cargo run -p feature_cli -- test logic.search_index"].iter())
    );
}

#[test]
fn missing_selection_result_fails() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation =
        build_feature_packets(input).expect_err("missing selection result should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "selection_result")
    );
}

#[test]
fn no_selected_features_fails() {
    let mut input = sample_valid_input().expect("valid fixture should parse");
    let selection_result = input
        .selection_result
        .as_mut()
        .expect("selection result should exist");
    selection_result.selected_feature_ids.clear();
    for decision in &mut selection_result.decisions {
        decision.selected = false;
        decision.score = None;
    }

    let validation = build_feature_packets(input).expect_err("no selected features should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "no_selected_features")
    );
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "feature_build_packet_flow");
    assert_eq!(manifest.inputs.items.len(), 3);
    assert_eq!(manifest.outputs.items.len(), 5);
}

#[test]
fn flow_object_builds_packet_bundles() {
    let flow = FeatureBuildPacketFlow;
    let input = sample_valid_input().expect("valid fixture should parse");
    let validation = flow.validate(&input);
    let bundle = flow.build(input).expect("flow should build bundle");

    assert!(validation.is_valid());
    assert_eq!(bundle.selected_feature_id, "logic.search_index");
}
