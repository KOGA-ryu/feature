#[path = "../src/lib.rs"]
mod packet_archive_writer_impl;

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use feature_core::parse_feature_manifest;
use packet_archive_writer_impl::{
    FEATURE_ID, PacketArchiveManifest, PacketArchiveWriter, parse_packet_archive_writer_input,
    sample_invalid_input, sample_valid_input,
};

#[test]
fn valid_input_writes_the_full_archive_tree() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let root = temp_root("valid-write");
    let report = packet_archive_writer_impl::write_packet_archive_with_root_for_test(input, &root)
        .expect("valid archive should write");

    assert!(PathBuf::from(&report.archive_dir).exists());
    assert!(PathBuf::from(&report.manifest_path).exists());
    assert!(PathBuf::from(&report.readme_path).exists());
    assert_eq!(report.worker_files.len(), 2);
    assert!(PathBuf::from(&report.integrator_file.markdown_path).exists());
    assert!(PathBuf::from(&report.reviewer_file.markdown_path).exists());

    cleanup_root(&root);
}

#[test]
fn readme_includes_startup_steps_and_lane_mapping() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let root = temp_root("readme");
    let report = packet_archive_writer_impl::write_packet_archive_with_root_for_test(input, &root)
        .expect("valid archive should write");
    let readme = fs::read_to_string(&report.readme_path).expect("readme should be readable");

    assert!(readme.contains("Startup sequence:"));
    assert!(readme.contains("`pwd`"));
    assert!(readme.contains("`git branch --show-current`"));
    assert!(readme.contains("lane_01 -> ui.left_rail"));
    assert!(readme.contains("worker windows use `worker/*.md`"));

    cleanup_root(&root);
}

#[test]
fn worker_markdown_includes_exact_prompt_text_verbatim() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let expected_prompt = input
        .wave_bundle
        .as_ref()
        .expect("wave bundle should exist")
        .worker_bundles[0]
        .packet
        .prompt_text
        .clone();
    let root = temp_root("worker-markdown");
    let report = packet_archive_writer_impl::write_packet_archive_with_root_for_test(input, &root)
        .expect("valid archive should write");
    let worker_markdown = fs::read_to_string(&report.worker_files[0].markdown_path)
        .expect("worker markdown should be readable");

    assert!(worker_markdown.contains("```text"));
    assert!(worker_markdown.contains(&expected_prompt));

    cleanup_root(&root);
}

#[test]
fn manifest_records_all_packet_paths_and_preserves_feature_lists() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let root = temp_root("manifest");
    let report = packet_archive_writer_impl::write_packet_archive_with_root_for_test(input, &root)
        .expect("valid archive should write");
    let manifest = serde_json::from_str::<PacketArchiveManifest>(
        &fs::read_to_string(&report.manifest_path).expect("manifest should be readable"),
    )
    .expect("manifest should parse");

    assert_eq!(manifest.worker_files.len(), 2);
    assert_eq!(
        manifest.deferred_feature_ids,
        vec!["workflow.review_packet_flow"]
    );
    assert_eq!(manifest.rejected_feature_ids, vec!["ui.command_palette"]);
    assert_eq!(manifest.integrator_file.role, "integrator");
    assert!(
        manifest.worker_files[0]
            .markdown_path
            .starts_with(&manifest.archive_dir)
    );
    assert!(
        manifest
            .reviewer_file
            .json_path
            .starts_with(&manifest.archive_dir)
    );

    cleanup_root(&root);
}

#[test]
fn invalid_wave_label_fails() {
    let mut input = sample_valid_input().expect("valid fixture should parse");
    input.wave_label = Some("Bad Label".into());

    let validation = packet_archive_writer_impl::write_packet_archive_with_root_for_test(
        input,
        &temp_root("invalid-label"),
    )
    .expect_err("invalid wave label should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "invalid_wave_label")
    );
}

#[test]
fn missing_wave_bundle_fails() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation = packet_archive_writer_impl::write_packet_archive_with_root_for_test(
        input,
        &temp_root("missing-wave"),
    )
    .expect_err("missing wave bundle should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "wave_bundle")
    );
}

#[test]
fn empty_worker_wave_fails() {
    let mut input = sample_valid_input().expect("valid fixture should parse");
    let wave_bundle = input
        .wave_bundle
        .as_mut()
        .expect("wave bundle should exist");
    wave_bundle.worker_bundles.clear();
    wave_bundle.wave_feature_ids.clear();
    wave_bundle.lane_assignments.clear();

    let validation = packet_archive_writer_impl::write_packet_archive_with_root_for_test(
        input,
        &temp_root("empty-workers"),
    )
    .expect_err("empty worker wave should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "empty_worker_wave")
    );
}

#[test]
fn existing_target_directory_fails_without_overwrite() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let root = temp_root("collision-fail");
    let first_report =
        packet_archive_writer_impl::write_packet_archive_with_root_for_test(input.clone(), &root)
            .expect("first write should succeed");
    assert!(PathBuf::from(&first_report.archive_dir).exists());

    let validation =
        packet_archive_writer_impl::write_packet_archive_with_root_for_test(input, &root)
            .expect_err("second write should fail without overwrite");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "archive_already_exists")
    );

    cleanup_root(&root);
}

#[test]
fn existing_target_directory_is_replaced_when_overwrite_is_true() {
    let mut input = sample_valid_input().expect("valid fixture should parse");
    let root = temp_root("collision-overwrite");
    let report =
        packet_archive_writer_impl::write_packet_archive_with_root_for_test(input.clone(), &root)
            .expect("first write should succeed");
    let readme_path = report.readme_path.clone();
    fs::write(&readme_path, "stale contents").expect("should be able to mutate readme");

    input.overwrite_existing = true;
    let overwritten =
        packet_archive_writer_impl::write_packet_archive_with_root_for_test(input, &root)
            .expect("overwrite write should succeed");
    let readme = fs::read_to_string(&overwritten.readme_path).expect("readme should be readable");

    assert!(!readme.contains("stale contents"));

    cleanup_root(&root);
}

#[test]
fn lane_file_naming_is_deterministic() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let root = temp_root("lane-names");
    let report = packet_archive_writer_impl::write_packet_archive_with_root_for_test(input, &root)
        .expect("valid archive should write");

    assert!(
        report.worker_files[0]
            .markdown_path
            .ends_with("worker/lane_01__ui_left_rail.md")
    );
    assert!(
        report.worker_files[1]
            .json_path
            .ends_with("worker/lane_02__logic_search_index.json")
    );

    cleanup_root(&root);
}

#[test]
fn invalid_packet_kinds_fail_validation() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let mut raw = serde_json::to_value(input).expect("input should serialize");
    raw["wave_bundle"]["worker_bundles"][0]["packet"]["target_kind"] =
        serde_json::Value::String("reviewer".into());
    raw["wave_bundle"]["integrator_packet"]["target_kind"] =
        serde_json::Value::String("feature_worker".into());
    raw["wave_bundle"]["reviewer_packet"]["target_kind"] =
        serde_json::Value::String("integrator".into());
    let input = parse_packet_archive_writer_input(
        &serde_json::to_string(&raw).expect("raw value should serialize"),
    )
    .expect("mutated input should parse");

    let validation = packet_archive_writer_impl::write_packet_archive_with_root_for_test(
        input,
        &temp_root("wrong-kinds"),
    )
    .expect_err("wrong packet kinds should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "invalid_worker_packet_kind")
    );
    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "invalid_integrator_packet_kind")
    );
    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "invalid_reviewer_packet_kind")
    );
}

#[test]
fn flow_object_validates_inputs() {
    let flow = PacketArchiveWriter;
    let input = sample_valid_input().expect("valid fixture should parse");
    let validation = flow.validate(&input);

    assert!(validation.is_valid());
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "packet_archive_writer");
    assert_eq!(manifest.inputs.items.len(), 3);
    assert_eq!(manifest.outputs.items.len(), 6);
}

fn temp_root(label: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "packet-archive-writer-{label}-{}-{stamp}",
        std::process::id()
    ))
}

fn cleanup_root(root: &PathBuf) {
    let _ = fs::remove_dir_all(root);
}
