use feature_core::parse_feature_manifest;
use feature_wave_packet_flow::{
    FEATURE_ID, FeatureWavePacketFlow, build_feature_wave_packets, sample_invalid_input,
    sample_valid_bundle, sample_valid_input,
};
use prompt_generator::PromptTargetKind;

#[test]
fn valid_input_preserves_selected_order_and_caps_by_max_workers() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(
        bundle.wave_feature_ids,
        vec!["ui.left_rail", "logic.search_index"]
    );
    assert_eq!(bundle.worker_bundles.len(), 2);
    assert_eq!(
        bundle.deferred_feature_ids,
        vec!["workflow.review_packet_flow"]
    );
}

#[test]
fn worker_and_shared_packets_have_correct_target_kinds() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert!(
        bundle
            .worker_bundles
            .iter()
            .all(|bundle| bundle.packet.target_kind == PromptTargetKind::FeatureWorker)
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
fn lane_assignments_match_worker_packet_write_bounds() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(bundle.lane_assignments.len(), 2);
    assert_eq!(bundle.lane_assignments[0].lane_name, "lane_01");
    assert_eq!(bundle.lane_assignments[1].lane_name, "lane_02");
    assert!(
        bundle.lane_assignments[0]
            .allowed_writes
            .iter()
            .any(|path| path.ends_with("feature_packs/ui/left_rail/**"))
    );
    assert!(
        bundle.lane_assignments[1]
            .allowed_writes
            .iter()
            .any(|path| path.ends_with("feature_packs/logic/search_index/**"))
    );
}

#[test]
fn integrator_and_reviewer_use_the_active_wave_in_order() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert!(
        bundle
            .integrator_packet
            .prompt_text
            .contains("Features to integrate:\n- ui.left_rail\n- logic.search_index")
    );
    assert_eq!(
        bundle
            .reviewer_packet
            .verification_commands
            .iter()
            .filter(|command| command.starts_with("cargo run -p feature_cli -- show "))
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            "cargo run -p feature_cli -- show ui.left_rail".to_string(),
            "cargo run -p feature_cli -- show logic.search_index".to_string()
        ]
    );
    assert_eq!(
        bundle
            .reviewer_packet
            .verification_commands
            .iter()
            .filter(|command| command.starts_with("cargo run -p feature_cli -- test "))
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            "cargo run -p feature_cli -- test ui.left_rail".to_string(),
            "cargo run -p feature_cli -- test logic.search_index".to_string()
        ]
    );
}

#[test]
fn bundle_preserves_rejected_ids_and_upstream_findings() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(bundle.rejected_feature_ids, vec!["ui.command_palette"]);
    assert_eq!(bundle.upstream_findings.len(), 1);
    assert_eq!(bundle.upstream_findings[0].code, "missing_rule");
}

#[test]
fn missing_selection_result_fails() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation =
        build_feature_wave_packets(input).expect_err("missing selection result should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "selection_result")
    );
}

#[test]
fn zero_max_workers_fails() {
    let mut input = sample_valid_input().expect("valid fixture should parse");
    input.max_workers = 0;

    let validation = build_feature_wave_packets(input).expect_err("zero workers should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "max_workers")
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

    let validation =
        build_feature_wave_packets(input).expect_err("no selected features should fail");

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
    assert_eq!(manifest.name, "feature_wave_packet_flow");
    assert_eq!(manifest.inputs.items.len(), 4);
    assert_eq!(manifest.outputs.items.len(), 9);
}

#[test]
fn flow_object_builds_packet_bundles() {
    let flow = FeatureWavePacketFlow;
    let input = sample_valid_input().expect("valid fixture should parse");
    let validation = flow.validate(&input);
    let bundle = flow.build(input).expect("flow should build bundle");

    assert!(validation.is_valid());
    assert_eq!(
        bundle.wave_feature_ids,
        vec!["ui.left_rail", "logic.search_index"]
    );
}
