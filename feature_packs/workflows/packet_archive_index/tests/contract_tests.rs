use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use feature_core::parse_feature_manifest;
use packet_archive_index::{
    ArchivedWaveStatus, FEATURE_ID, PacketArchiveIndexInput, PacketArchiveIndexer,
    index_packet_archives, sample_invalid_input, sample_valid_input,
};
use packet_archive_writer::{ArchivedPacketFile, PacketArchiveManifest};
use serde_json::json;

#[test]
fn empty_root_returns_no_waves() {
    let root = temp_root("empty-root");
    let report = index_packet_archives(PacketArchiveIndexInput {
        archive_root: Some(root.to_string_lossy().into_owned()),
    })
    .expect("empty root should index");

    assert!(report.waves.is_empty());
}

#[test]
fn valid_archive_reports_complete_status_and_absolute_paths() {
    let root = temp_root("complete");
    create_valid_archive(&root, "alpha_wave");

    let report = index_packet_archives(PacketArchiveIndexInput {
        archive_root: Some(root.to_string_lossy().into_owned()),
    })
    .expect("valid root should index");

    assert_eq!(report.waves.len(), 1);
    let wave = &report.waves[0];
    assert_eq!(wave.status, ArchivedWaveStatus::Complete);
    assert_eq!(wave.wave_label, "alpha_wave");
    assert_eq!(
        wave.wave_feature_ids,
        vec!["ui.left_rail", "logic.search_index"]
    );
    assert!(wave.worker_files[0].markdown_path.starts_with('/'));
    assert!(
        wave.integrator_file
            .as_ref()
            .unwrap()
            .json_path
            .starts_with('/')
    );

    cleanup_root(&root);
}

#[test]
fn missing_referenced_file_reports_incomplete_status() {
    let root = temp_root("incomplete");
    let archive_dir = create_valid_archive(&root, "beta_wave");
    fs::remove_file(archive_dir.join("worker/lane_02__logic_search_index.json"))
        .expect("worker json should be removable");

    let report = index_packet_archives(PacketArchiveIndexInput {
        archive_root: Some(root.to_string_lossy().into_owned()),
    })
    .expect("incomplete root should still index");

    assert_eq!(report.waves[0].status, ArchivedWaveStatus::Incomplete);
    assert!(
        report.waves[0]
            .findings
            .iter()
            .any(|finding| finding.code == "missing_packet_file")
    );

    cleanup_root(&root);
}

#[test]
fn malformed_manifest_reports_invalid_status() {
    let root = temp_root("invalid-manifest");
    let archive_dir = root.join("broken_wave");
    fs::create_dir_all(&archive_dir).expect("archive dir should be creatable");
    fs::write(archive_dir.join("wave_manifest.json"), "{not-json").expect("manifest should write");

    let report = index_packet_archives(PacketArchiveIndexInput {
        archive_root: Some(root.to_string_lossy().into_owned()),
    })
    .expect("invalid root should still index");

    assert_eq!(report.waves[0].status, ArchivedWaveStatus::Invalid);
    assert!(
        report.waves[0]
            .findings
            .iter()
            .any(|finding| finding.code == "invalid_manifest_json")
    );

    cleanup_root(&root);
}

#[test]
fn missing_manifest_reports_invalid_status() {
    let root = temp_root("missing-manifest");
    fs::create_dir_all(root.join("orphan_wave")).expect("wave dir should be creatable");

    let report = index_packet_archives(PacketArchiveIndexInput {
        archive_root: Some(root.to_string_lossy().into_owned()),
    })
    .expect("missing manifest root should still index");

    assert_eq!(report.waves[0].status, ArchivedWaveStatus::Invalid);
    assert!(
        report.waves[0]
            .findings
            .iter()
            .any(|finding| finding.code == "missing_manifest")
    );

    cleanup_root(&root);
}

#[test]
fn waves_are_sorted_deterministically_by_label() {
    let root = temp_root("sorted");
    create_valid_archive(&root, "zeta_wave");
    create_valid_archive(&root, "alpha_wave");

    let report = index_packet_archives(PacketArchiveIndexInput {
        archive_root: Some(root.to_string_lossy().into_owned()),
    })
    .expect("valid root should index");

    assert_eq!(report.waves.len(), 2);
    assert_eq!(report.waves[0].wave_label, "alpha_wave");
    assert_eq!(report.waves[1].wave_label, "zeta_wave");

    cleanup_root(&root);
}

#[test]
fn blank_archive_root_fails_validation() {
    let input = sample_invalid_input().expect("invalid fixture should parse");
    let validation = packet_archive_index::index_packet_archives(input)
        .expect_err("blank archive root should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "blank_archive_root")
    );
}

#[test]
fn existing_file_as_archive_root_fails_validation() {
    let root = temp_root("file-root");
    fs::create_dir_all(&root).expect("root should be creatable");
    let file_root = root.join("not_a_directory");
    fs::write(&file_root, "x").expect("file root should be writable");

    let validation = packet_archive_index::index_packet_archives(PacketArchiveIndexInput {
        archive_root: Some(file_root.to_string_lossy().into_owned()),
    })
    .expect_err("file archive root should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "archive_root_not_directory")
    );

    cleanup_root(&root);
}

#[test]
fn root_override_from_fixture_is_valid() {
    let input = sample_valid_input().expect("valid fixture should parse");
    let indexer = PacketArchiveIndexer;
    let validation = indexer.validate(&input);

    assert!(validation.is_valid());
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "packet_archive_index");
    assert_eq!(manifest.inputs.items.len(), 1);
    assert_eq!(manifest.outputs.items.len(), 3);
}

fn create_valid_archive(root: &PathBuf, wave_label: &str) -> PathBuf {
    let archive_dir = root.join(wave_label);
    let worker_dir = archive_dir.join("worker");
    let integrator_dir = archive_dir.join("integrator");
    let reviewer_dir = archive_dir.join("reviewer");
    fs::create_dir_all(&worker_dir).expect("worker dir should be creatable");
    fs::create_dir_all(&integrator_dir).expect("integrator dir should be creatable");
    fs::create_dir_all(&reviewer_dir).expect("reviewer dir should be creatable");

    let worker_1_md = worker_dir.join("lane_01__ui_left_rail.md");
    let worker_1_json = worker_dir.join("lane_01__ui_left_rail.json");
    let worker_2_md = worker_dir.join("lane_02__logic_search_index.md");
    let worker_2_json = worker_dir.join("lane_02__logic_search_index.json");
    let integrator_md = integrator_dir.join("integrator.md");
    let integrator_json = integrator_dir.join("integrator.json");
    let reviewer_md = reviewer_dir.join("reviewer.md");
    let reviewer_json = reviewer_dir.join("reviewer.json");

    for path in [
        &worker_1_md,
        &worker_1_json,
        &worker_2_md,
        &worker_2_json,
        &integrator_md,
        &integrator_json,
        &reviewer_md,
        &reviewer_json,
    ] {
        fs::write(path, "{}").expect("archive file should be writable");
    }

    let manifest = PacketArchiveManifest {
        wave_label: wave_label.to_string(),
        archive_dir: archive_dir.to_string_lossy().into_owned(),
        project: "features".into(),
        branch_name: "codex/theme-foundation-cleanup".into(),
        max_workers: 2,
        wave_feature_ids: vec!["ui.left_rail".into(), "logic.search_index".into()],
        deferred_feature_ids: vec!["workflow.review_packet_flow".into()],
        rejected_feature_ids: vec!["ui.command_palette".into()],
        worker_files: vec![
            ArchivedPacketFile {
                role: "worker".into(),
                lane_name: Some("lane_01".into()),
                feature_id: Some("ui.left_rail".into()),
                markdown_path: worker_1_md.to_string_lossy().into_owned(),
                json_path: worker_1_json.to_string_lossy().into_owned(),
            },
            ArchivedPacketFile {
                role: "worker".into(),
                lane_name: Some("lane_02".into()),
                feature_id: Some("logic.search_index".into()),
                markdown_path: worker_2_md.to_string_lossy().into_owned(),
                json_path: worker_2_json.to_string_lossy().into_owned(),
            },
        ],
        integrator_file: ArchivedPacketFile {
            role: "integrator".into(),
            lane_name: None,
            feature_id: None,
            markdown_path: integrator_md.to_string_lossy().into_owned(),
            json_path: integrator_json.to_string_lossy().into_owned(),
        },
        reviewer_file: ArchivedPacketFile {
            role: "reviewer".into(),
            lane_name: None,
            feature_id: None,
            markdown_path: reviewer_md.to_string_lossy().into_owned(),
            json_path: reviewer_json.to_string_lossy().into_owned(),
        },
        upstream_findings: vec![json!({
            "severity": "warning",
            "code": "missing_rule",
            "path": "logic.search_index + workflow.review_packet_flow",
            "message": "No explicit compatibility rule exists for this feature pair."
        })],
    };

    fs::write(
        archive_dir.join("wave_manifest.json"),
        serde_json::to_string_pretty(&manifest).expect("manifest should serialize"),
    )
    .expect("manifest should write");

    archive_dir
}

fn temp_root(label: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "packet-archive-index-{label}-{}-{stamp}",
        std::process::id()
    ))
}

fn cleanup_root(root: &PathBuf) {
    let _ = fs::remove_dir_all(root);
}
