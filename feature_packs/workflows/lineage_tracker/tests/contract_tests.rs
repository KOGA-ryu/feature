use feature_core::parse_feature_manifest;
use lineage_tracker::{
    FEATURE_ID, LineageStage, LineageStageStatus, LineageTracker, sample_complete_input,
    sample_invalid_input, sample_lineage_record, sample_partial_input, track_lineage,
};
use prompt_generator::PromptTargetKind;

#[test]
fn complete_lineage_input_produces_all_stage_summaries() {
    let record = sample_lineage_record().expect("complete lineage should succeed");

    assert_eq!(record.stage_summaries.len(), 5);
    assert!(
        record
            .stage_summaries
            .iter()
            .all(|summary| summary.status == LineageStageStatus::Complete)
    );
    assert_eq!(record.primary_feature_id, "ui.theme_editor");
    assert_eq!(record.proposed_library_paths.len(), 3);
}

#[test]
fn partial_lineage_input_succeeds_with_warning_stage_summaries() {
    let input = sample_partial_input().expect("partial fixture should parse");
    let record = track_lineage(input).expect("partial lineage should succeed");

    assert!(
        record
            .stage_summaries
            .iter()
            .any(|summary| summary.stage == LineageStage::Prompt
                && summary.status == LineageStageStatus::Warning)
    );
    assert!(record.files_changed.is_empty());
    assert!(record.proposed_library_paths.is_empty());
    assert!(record.merge_recommendation.is_none());
}

#[test]
fn missing_project_fails() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation = track_lineage(input).expect_err("missing project should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "project")
    );
}

#[test]
fn selected_feature_order_is_preserved_from_spec() {
    let record = sample_lineage_record().expect("complete lineage should succeed");

    assert_eq!(
        record.selected_feature_ids,
        vec![
            "ui.theme_editor",
            "logic.theme_token_generator",
            "workflow.review_packet_flow"
        ]
    );
}

#[test]
fn prompt_target_kinds_preserve_input_order() {
    let record = sample_lineage_record().expect("complete lineage should succeed");

    assert_eq!(
        record.prompt_target_kinds,
        vec![
            PromptTargetKind::FeatureWorker,
            PromptTargetKind::Integrator,
            PromptTargetKind::Reviewer
        ]
    );
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "lineage_tracker");
    assert_eq!(manifest.inputs.items.len(), 8);
    assert_eq!(manifest.outputs.items.len(), 3);
}

#[test]
fn tracker_object_tracks_records() {
    let tracker = LineageTracker;
    let input = sample_complete_input().expect("complete fixture should parse");
    let validation = tracker.validate(&input);
    let record = tracker
        .track(input)
        .expect("tracker should produce lineage record");

    assert!(validation.is_valid());
    assert!(record.lineage_id.contains("idea.theme-editor"));
}
