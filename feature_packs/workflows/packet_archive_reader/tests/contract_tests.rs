use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use feature_core::parse_feature_manifest;
use feature_wave_packet_flow::sample_valid_bundle;
use packet_archive_reader::{
    FEATURE_ID, PacketArchiveReaderInput, PacketArchiveReaderSeverity, read_packet_archive,
};
use packet_archive_writer::{ArchivedPacketFile, PacketArchiveManifest};
use serde::Serialize;

#[test]
fn valid_archive_round_trips_into_wave_bundle() {
    let temp = TestRoot::new("valid_archive_round_trip");
    let wave_label = "sample-wave";
    let bundle = sample_valid_bundle().expect("sample bundle should build");
    let archive_dir = write_valid_archive(temp.root(), wave_label, &bundle);

    let report = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some(wave_label.into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect("valid archive should read");

    assert_eq!(report.wave_label, wave_label);
    assert_eq!(report.archive_dir, archive_dir.display().to_string());
    assert_eq!(report.wave_bundle.wave_feature_ids, bundle.wave_feature_ids);
    assert_eq!(
        report.wave_bundle.worker_bundles.len(),
        bundle.worker_bundles.len()
    );
    assert_eq!(
        report.wave_bundle.worker_bundles[0].packet,
        bundle.worker_bundles[0].packet
    );
    assert_eq!(
        report.wave_bundle.integrator_packet,
        bundle.integrator_packet
    );
    assert_eq!(report.wave_bundle.reviewer_packet, bundle.reviewer_packet);
    assert_eq!(
        report.wave_bundle.upstream_findings,
        bundle.upstream_findings
    );
    assert_eq!(report.worker_documents.len(), bundle.worker_bundles.len());
    assert!(report.readme_text.contains("Packet Wave Archive"));
    assert!(
        report.worker_documents[0]
            .markdown_text
            .contains("Worker Packet")
    );
}

#[test]
fn worker_feature_order_is_preserved_from_manifest() {
    let temp = TestRoot::new("worker_feature_order");
    let wave_label = "sample-wave";
    let bundle = sample_valid_bundle().expect("sample bundle should build");
    write_valid_archive(temp.root(), wave_label, &bundle);

    let report = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some(wave_label.into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect("valid archive should read");

    let document_ids = report
        .worker_documents
        .iter()
        .map(|document| document.feature_id.as_deref().unwrap_or_default())
        .collect::<Vec<_>>();
    assert_eq!(document_ids, bundle.wave_feature_ids);
    assert_eq!(report.wave_bundle.wave_feature_ids, bundle.wave_feature_ids);
}

#[test]
fn invalid_wave_label_fails() {
    let validation = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some("Bad Label".into()),
        archive_root: None,
    })
    .expect_err("invalid wave label should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "invalid_wave_label")
    );
}

#[test]
fn missing_archive_directory_fails() {
    let temp = TestRoot::new("missing_archive_directory");

    let validation = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some("sample-wave".into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect_err("missing archive dir should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "missing_archive_directory")
    );
}

#[test]
fn missing_wave_manifest_fails() {
    let temp = TestRoot::new("missing_manifest");
    let archive_dir = temp.root().join("sample-wave");
    fs::create_dir_all(&archive_dir).expect("archive dir should create");
    fs::write(archive_dir.join("README.md"), "# test\n").expect("readme should write");

    let validation = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some("sample-wave".into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect_err("missing manifest should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "missing_manifest")
    );
}

#[test]
fn missing_worker_json_file_fails() {
    let temp = TestRoot::new("missing_worker_json");
    let wave_label = "sample-wave";
    let bundle = sample_valid_bundle().expect("sample bundle should build");
    let archive_dir = write_valid_archive(temp.root(), wave_label, &bundle);
    let first_worker_file = worker_file_stem(&bundle, 0);
    let json_path = archive_dir
        .join("worker")
        .join(format!("{first_worker_file}.json"));
    fs::remove_file(&json_path).expect("worker json should remove");

    let validation = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some(wave_label.into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect_err("missing worker json should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "missing_json_file")
    );
}

#[test]
fn missing_worker_markdown_file_fails() {
    let temp = TestRoot::new("missing_worker_markdown");
    let wave_label = "sample-wave";
    let bundle = sample_valid_bundle().expect("sample bundle should build");
    let archive_dir = write_valid_archive(temp.root(), wave_label, &bundle);
    let first_worker_file = worker_file_stem(&bundle, 0);
    let markdown_path = archive_dir
        .join("worker")
        .join(format!("{first_worker_file}.md"));
    fs::remove_file(&markdown_path).expect("worker markdown should remove");

    let validation = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some(wave_label.into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect_err("missing worker markdown should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "missing_markdown_file")
    );
}

#[test]
fn wrong_packet_target_kind_fails() {
    let temp = TestRoot::new("wrong_packet_kind");
    let wave_label = "sample-wave";
    let bundle = sample_valid_bundle().expect("sample bundle should build");
    let archive_dir = write_valid_archive(temp.root(), wave_label, &bundle);
    let first_worker_file = worker_file_stem(&bundle, 0);
    let json_path = archive_dir
        .join("worker")
        .join(format!("{first_worker_file}.json"));
    let raw = fs::read_to_string(&json_path).expect("worker json should read");
    let mut packet: serde_json::Value = serde_json::from_str(&raw).expect("json should parse");
    packet["target_kind"] = serde_json::Value::String("reviewer".into());
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&packet).expect("json should serialize"),
    )
    .expect("worker json should rewrite");

    let validation = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some(wave_label.into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect_err("wrong target kind should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "invalid_worker_packet_kind")
    );
}

#[test]
fn manifest_path_escaping_outside_archive_dir_fails() {
    let temp = TestRoot::new("outside_archive_dir");
    let wave_label = "sample-wave";
    let bundle = sample_valid_bundle().expect("sample bundle should build");
    let archive_dir = write_valid_archive(temp.root(), wave_label, &bundle);
    let outside_dir = temp.root().join("outside");
    fs::create_dir_all(&outside_dir).expect("outside dir should create");
    let outside_json = outside_dir.join("worker.json");
    let outside_md = outside_dir.join("worker.md");
    fs::write(
        &outside_json,
        serde_json::to_string_pretty(&bundle.worker_bundles[0].packet)
            .expect("packet should serialize"),
    )
    .expect("outside json should write");
    fs::write(&outside_md, "# outside\n").expect("outside markdown should write");

    let manifest_path = archive_dir.join("wave_manifest.json");
    let raw_manifest = fs::read_to_string(&manifest_path).expect("manifest should read");
    let mut manifest: PacketArchiveManifest =
        serde_json::from_str(&raw_manifest).expect("manifest should parse");
    manifest.worker_files[0].json_path = outside_json.display().to_string();
    manifest.worker_files[0].markdown_path = outside_md.display().to_string();
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).expect("manifest should serialize"),
    )
    .expect("manifest should rewrite");

    let validation = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some(wave_label.into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect_err("outside archive path should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.code == "file_outside_archive_dir")
    );
}

#[test]
fn upstream_findings_deserialize_correctly() {
    let temp = TestRoot::new("upstream_findings");
    let wave_label = "sample-wave";
    let bundle = sample_valid_bundle().expect("sample bundle should build");
    write_valid_archive(temp.root(), wave_label, &bundle);

    let report = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some(wave_label.into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect("valid archive should read");

    assert_eq!(
        report.archive_manifest.upstream_findings.len(),
        bundle.upstream_findings.len()
    );
    assert_eq!(
        report.wave_bundle.upstream_findings,
        bundle.upstream_findings
    );
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "packet_archive_reader");
    assert_eq!(manifest.inputs.items, vec!["wave_label", "archive_root"]);
    assert_eq!(
        manifest.outputs.items,
        vec![
            "packet_archive_read_report",
            "packet_archive_reader_validation",
            "archive_manifest",
            "wave_bundle",
            "worker_documents",
            "integrator_document",
            "reviewer_document",
        ]
    );
}

#[test]
fn reader_report_keeps_readme_and_documents() {
    let temp = TestRoot::new("readme_and_documents");
    let wave_label = "sample-wave";
    let bundle = sample_valid_bundle().expect("sample bundle should build");
    write_valid_archive(temp.root(), wave_label, &bundle);

    let report = read_packet_archive(PacketArchiveReaderInput {
        wave_label: Some(wave_label.into()),
        archive_root: Some(temp.root().display().to_string()),
    })
    .expect("valid archive should read");

    assert!(report.readme_text.contains("Lane to feature map"));
    assert_eq!(report.integrator_document.role, "integrator");
    assert_eq!(report.reviewer_document.role, "reviewer");
    assert!(
        report
            .integrator_document
            .markdown_text
            .contains("Integrator Packet")
    );
    assert!(
        report
            .reviewer_document
            .markdown_text
            .contains("Reviewer Packet")
    );
}

#[test]
fn invalid_fixture_parses_and_fails_validation() {
    let input = packet_archive_reader::sample_invalid_input().expect("fixture should parse");
    let validation = read_packet_archive(input).expect_err("invalid fixture should fail");

    assert!(validation.findings.iter().any(|finding| {
        finding.severity == PacketArchiveReaderSeverity::Error
            && (finding.code == "invalid_wave_label" || finding.code == "blank_archive_root")
    }));
}

struct TestRoot {
    path: PathBuf,
}

impl TestRoot {
    fn new(label: &str) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nonce = COUNTER.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be valid")
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "packet_archive_reader_{label}_{}_{}_{}",
            std::process::id(),
            timestamp,
            nonce
        ));
        fs::create_dir_all(&path).expect("temp root should create");
        Self { path }
    }

    fn root(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_valid_archive(
    root: &Path,
    wave_label: &str,
    bundle: &feature_wave_packet_flow::FeatureWavePacketBundle,
) -> PathBuf {
    let archive_dir = root.join(wave_label);
    let worker_dir = archive_dir.join("worker");
    let integrator_dir = archive_dir.join("integrator");
    let reviewer_dir = archive_dir.join("reviewer");
    fs::create_dir_all(&worker_dir).expect("worker dir should create");
    fs::create_dir_all(&integrator_dir).expect("integrator dir should create");
    fs::create_dir_all(&reviewer_dir).expect("reviewer dir should create");

    let mut worker_files = Vec::new();
    for (worker_bundle, lane_assignment) in bundle
        .worker_bundles
        .iter()
        .zip(bundle.lane_assignments.iter())
    {
        let file_name = format!(
            "{}__{}",
            lane_assignment.lane_name,
            worker_bundle.feature_id.replace('.', "_")
        );
        let json_path = worker_dir.join(format!("{file_name}.json"));
        let markdown_path = worker_dir.join(format!("{file_name}.md"));
        fs::write(
            &json_path,
            serde_json::to_string_pretty(&worker_bundle.packet)
                .expect("worker packet should serialize"),
        )
        .expect("worker json should write");
        fs::write(
            &markdown_path,
            format!("# Worker Packet: {}\n", worker_bundle.feature_id),
        )
        .expect("worker markdown should write");

        worker_files.push(ArchivedPacketFile {
            role: "worker".into(),
            lane_name: Some(lane_assignment.lane_name.clone()),
            feature_id: Some(worker_bundle.feature_id.clone()),
            markdown_path: markdown_path.display().to_string(),
            json_path: json_path.display().to_string(),
        });
    }

    let integrator_json_path = integrator_dir.join("integrator.json");
    let integrator_markdown_path = integrator_dir.join("integrator.md");
    fs::write(
        &integrator_json_path,
        serde_json::to_string_pretty(&bundle.integrator_packet)
            .expect("integrator packet should serialize"),
    )
    .expect("integrator json should write");
    fs::write(&integrator_markdown_path, "# Integrator Packet\n")
        .expect("integrator markdown should write");
    let integrator_file = ArchivedPacketFile {
        role: "integrator".into(),
        lane_name: None,
        feature_id: None,
        markdown_path: integrator_markdown_path.display().to_string(),
        json_path: integrator_json_path.display().to_string(),
    };

    let reviewer_json_path = reviewer_dir.join("reviewer.json");
    let reviewer_markdown_path = reviewer_dir.join("reviewer.md");
    fs::write(
        &reviewer_json_path,
        serde_json::to_string_pretty(&bundle.reviewer_packet)
            .expect("reviewer packet should serialize"),
    )
    .expect("reviewer json should write");
    fs::write(&reviewer_markdown_path, "# Reviewer Packet\n")
        .expect("reviewer markdown should write");
    let reviewer_file = ArchivedPacketFile {
        role: "reviewer".into(),
        lane_name: None,
        feature_id: None,
        markdown_path: reviewer_markdown_path.display().to_string(),
        json_path: reviewer_json_path.display().to_string(),
    };

    let manifest = PacketArchiveManifest {
        wave_label: wave_label.into(),
        archive_dir: archive_dir.display().to_string(),
        project: bundle.project.clone(),
        branch_name: bundle.branch_name.clone(),
        max_workers: bundle.max_workers,
        wave_feature_ids: bundle.wave_feature_ids.clone(),
        deferred_feature_ids: bundle.deferred_feature_ids.clone(),
        rejected_feature_ids: bundle.rejected_feature_ids.clone(),
        worker_files,
        integrator_file,
        reviewer_file,
        upstream_findings: serialize_values(&bundle.upstream_findings),
    };
    fs::write(
        archive_dir.join("wave_manifest.json"),
        serde_json::to_string_pretty(&manifest).expect("manifest should serialize"),
    )
    .expect("manifest should write");
    fs::write(
        archive_dir.join("README.md"),
        "# Packet Wave Archive: sample-wave\n\nLane to feature map\n",
    )
    .expect("readme should write");

    archive_dir
}

fn serialize_values<T: Serialize>(values: &[T]) -> Vec<serde_json::Value> {
    values
        .iter()
        .map(|value| serde_json::to_value(value).expect("value should serialize"))
        .collect()
}

fn worker_file_stem(
    bundle: &feature_wave_packet_flow::FeatureWavePacketBundle,
    index: usize,
) -> String {
    let worker_bundle = &bundle.worker_bundles[index];
    let lane_assignment = &bundle.lane_assignments[index];
    format!(
        "{}__{}",
        lane_assignment.lane_name,
        worker_bundle.feature_id.replace('.', "_")
    )
}
