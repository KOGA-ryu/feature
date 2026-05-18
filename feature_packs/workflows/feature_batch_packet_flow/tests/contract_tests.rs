use feature_batch_packet_flow::{
    FEATURE_ID, FeatureBatchPacketFlow, build_feature_batch_packets, sample_invalid_input,
    sample_valid_bundle, sample_valid_input,
};
use feature_core::parse_feature_manifest;
use prompt_generator::PromptTargetKind;

#[test]
fn valid_fixture_builds_full_ordered_batch_bundle() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(
        bundle.ordered_feature_ids,
        vec!["logic.search_index", "workflow.review_packet_flow"]
    );
    assert_eq!(bundle.step_bundles.len(), 2);
}

#[test]
fn subset_queue_preserves_deferred_feature_ids_in_spec_order() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(
        bundle.deferred_feature_ids,
        vec!["ui.left_rail", "workflow.feature_build_packet_flow"]
    );
}

#[test]
fn step_order_matches_the_explicit_queue_order() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(bundle.step_bundles[0].step_index, 1);
    assert_eq!(bundle.step_bundles[0].feature_id, "logic.search_index");
    assert_eq!(bundle.step_bundles[1].step_index, 2);
    assert_eq!(
        bundle.step_bundles[1].feature_id,
        "workflow.review_packet_flow"
    );
}

#[test]
fn step_packets_are_single_feature_scoped() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");
    let first_step = &bundle.step_bundles[0];

    assert_eq!(
        first_step.worker_packet.target_kind,
        PromptTargetKind::FeatureWorker
    );
    assert_eq!(
        first_step.integrator_packet.target_kind,
        PromptTargetKind::Integrator
    );
    assert_eq!(
        first_step.reviewer_packet.target_kind,
        PromptTargetKind::Reviewer
    );
    assert!(
        first_step
            .integrator_packet
            .prompt_text
            .contains("Features to integrate:\n- logic.search_index")
    );
    assert!(
        first_step
            .reviewer_packet
            .prompt_text
            .contains("Wave features:\n- logic.search_index")
    );
}

#[test]
fn integrator_packets_allow_optional_feature_lab_ui_writes() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");
    let first_step = &bundle.step_bundles[0];

    assert!(
        first_step
            .integrator_packet
            .allowed_writes
            .iter()
            .any(|path| path.contains("feature_lab_ui"))
    );
    assert!(
        !first_step
            .integrator_packet
            .forbidden_writes
            .iter()
            .any(|path| path.contains("feature_lab_ui"))
    );
}

#[test]
fn review_contract_fields_are_deterministic() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");
    let first_step = &bundle.step_bundles[0];

    assert_eq!(
        first_step.review_contract.required_sections,
        vec![
            "files_changed",
            "commands_run",
            "tests_run",
            "risks",
            "known_issues",
            "follow_up_tasks",
            "merge_recommendation"
        ]
    );
    assert!(first_step.review_contract.stop_on_failure);
    assert!(
        first_step
            .review_contract
            .required_commands
            .contains(&"cargo fmt --all --check".to_string())
    );
    assert!(
        first_step
            .review_contract
            .required_commands
            .iter()
            .any(|command| command.contains("feature_packs/logic/search_index/Cargo.toml"))
    );
}

#[test]
fn task_id_format_is_deterministic() {
    let bundle = sample_valid_bundle().expect("valid bundle should build");

    assert_eq!(
        bundle.step_bundles[0].review_contract.task_id,
        "task.batch_01.logic_search_index"
    );
    assert_eq!(
        bundle.step_bundles[1].review_contract.task_id,
        "task.batch_02.workflow_review_packet_flow"
    );
}

#[test]
fn missing_project_branch_and_spec_fail() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation =
        build_feature_batch_packets(input).expect_err("missing required fields should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "project")
    );
    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "branch_name")
    );
    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "spec")
    );
}

#[test]
fn empty_ordered_feature_ids_fail() {
    let mut input = sample_valid_input().expect("valid fixture should parse");
    input.ordered_feature_ids.clear();

    let validation =
        build_feature_batch_packets(input).expect_err("missing queue should fail validation");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "ordered_feature_ids")
    );
}

#[test]
fn duplicate_feature_ids_fail() {
    let mut input = sample_valid_input().expect("valid fixture should parse");
    input
        .ordered_feature_ids
        .push("logic.search_index".to_string());

    let validation =
        build_feature_batch_packets(input).expect_err("duplicate queue ids should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "duplicate_feature_id")
    );
}

#[test]
fn queued_feature_missing_from_spec_fails() {
    let mut input = sample_valid_input().expect("valid fixture should parse");
    input.ordered_feature_ids[0] = "logic.missing_feature".into();

    let validation =
        build_feature_batch_packets(input).expect_err("unknown queued feature should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "queued_feature_missing_from_spec")
    );
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "feature_batch_packet_flow");
    assert_eq!(manifest.inputs.items.len(), 4);
    assert_eq!(manifest.outputs.items.len(), 5);
}

#[test]
fn flow_object_builds_packet_bundles() {
    let flow = FeatureBatchPacketFlow;
    let input = sample_valid_input().expect("valid fixture should parse");
    let validation = flow.validate(&input);
    let bundle = flow.build(input).expect("flow should build bundle");

    assert!(validation.is_valid());
    assert_eq!(bundle.step_bundles.len(), 2);
}
