use std::fmt;
use std::fs;
use std::path::Path;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use feature_wave_packet_flow::{
    FeatureWaveLaneAssignment, FeatureWavePacketBundle, FeatureWaveWorkerBundle,
};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "workflow.packet_archive_writer";
pub const REPO_ROOT: &str = "/Users/kogaryu/dev/features";
pub const PACKET_WAVES_ROOT: &str = "/Users/kogaryu/dev/features/state/packet_waves";

const STARTUP_COMMANDS: [&str; 3] = ["pwd", "git branch --show-current", "git status --short"];
const READ_FIRST_FILES: [&str; 5] = [
    "/Users/kogaryu/dev/features/AGENTS.md",
    "/Users/kogaryu/dev/features/README.md",
    "/Users/kogaryu/dev/features/docs/feature_contract.md",
    "/Users/kogaryu/dev/features/docs/testing_ground_rules.md",
    "/Users/kogaryu/dev/features/docs/chatgpt_feature_packets.md",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveWriterInput {
    pub wave_label: Option<String>,
    pub overwrite_existing: bool,
    pub wave_bundle: Option<FeatureWavePacketBundle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedPacketFile {
    pub role: String,
    pub lane_name: Option<String>,
    pub feature_id: Option<String>,
    pub markdown_path: String,
    pub json_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveManifest {
    pub wave_label: String,
    pub archive_dir: String,
    pub project: String,
    pub branch_name: String,
    pub max_workers: usize,
    pub wave_feature_ids: Vec<String>,
    pub deferred_feature_ids: Vec<String>,
    pub rejected_feature_ids: Vec<String>,
    pub worker_files: Vec<ArchivedPacketFile>,
    pub integrator_file: ArchivedPacketFile,
    pub reviewer_file: ArchivedPacketFile,
    pub upstream_findings: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveWriteReport {
    pub wave_label: String,
    pub archive_dir: String,
    pub manifest_path: String,
    pub readme_path: String,
    pub worker_files: Vec<ArchivedPacketFile>,
    pub integrator_file: ArchivedPacketFile,
    pub reviewer_file: ArchivedPacketFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketArchiveSeverity {
    Error,
    Warning,
}

impl fmt::Display for PacketArchiveSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveFinding {
    pub severity: PacketArchiveSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveValidation {
    pub findings: Vec<PacketArchiveFinding>,
}

impl PacketArchiveValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == PacketArchiveSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PacketArchiveWriter;

impl PacketArchiveWriter {
    pub fn validate(&self, input: &PacketArchiveWriterInput) -> PacketArchiveValidation {
        validate_packet_archive_writer_input(input)
    }

    pub fn write(
        &self,
        input: PacketArchiveWriterInput,
    ) -> Result<PacketArchiveWriteReport, PacketArchiveValidation> {
        write_packet_archive(input)
    }
}

pub fn write_packet_archive(
    input: PacketArchiveWriterInput,
) -> Result<PacketArchiveWriteReport, PacketArchiveValidation> {
    write_packet_archive_with_root(&input, Path::new(PACKET_WAVES_ROOT))
}

pub fn validate_packet_archive_writer_input(
    input: &PacketArchiveWriterInput,
) -> PacketArchiveValidation {
    validate_packet_archive_writer_input_with_root(input, Path::new(PACKET_WAVES_ROOT))
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_packet_archive_writer_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_packet_archive_writer_input.json")
}

pub fn parse_packet_archive_writer_input(raw: &str) -> Result<PacketArchiveWriterInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_input() -> Result<PacketArchiveWriterInput, String> {
    parse_packet_archive_writer_input(sample_valid_fixture())
}

pub fn sample_invalid_input() -> Result<PacketArchiveWriterInput, String> {
    parse_packet_archive_writer_input(sample_invalid_fixture())
}

#[allow(dead_code)]
pub(crate) fn write_packet_archive_with_root_for_test(
    input: PacketArchiveWriterInput,
    root: &Path,
) -> Result<PacketArchiveWriteReport, PacketArchiveValidation> {
    write_packet_archive_with_root(&input, root)
}

fn write_packet_archive_with_root(
    input: &PacketArchiveWriterInput,
    root: &Path,
) -> Result<PacketArchiveWriteReport, PacketArchiveValidation> {
    let validation = validate_packet_archive_writer_input_with_root(input, root);
    if !validation.is_valid() {
        return Err(validation);
    }

    let wave_label = require_string(input.wave_label.clone());
    let wave_bundle = input
        .wave_bundle
        .as_ref()
        .expect("validated wave bundle should exist");
    let archive_dir = root.join(&wave_label);

    fs::create_dir_all(root).map_err(|error| io_validation("archive_root", error, root))?;
    if archive_dir.exists() && input.overwrite_existing {
        remove_existing_archive_dir(&archive_dir, root)?;
    }

    let worker_dir = archive_dir.join("worker");
    let integrator_dir = archive_dir.join("integrator");
    let reviewer_dir = archive_dir.join("reviewer");
    fs::create_dir_all(&worker_dir)
        .map_err(|error| io_validation("worker_dir", error, &worker_dir))?;
    fs::create_dir_all(&integrator_dir)
        .map_err(|error| io_validation("integrator_dir", error, &integrator_dir))?;
    fs::create_dir_all(&reviewer_dir)
        .map_err(|error| io_validation("reviewer_dir", error, &reviewer_dir))?;

    let mut worker_files = Vec::new();
    for (worker_bundle, lane_assignment) in wave_bundle
        .worker_bundles
        .iter()
        .zip(wave_bundle.lane_assignments.iter())
    {
        let file_name = format!(
            "{}__{}",
            lane_assignment.lane_name,
            worker_bundle.feature_id.replace('.', "_")
        );
        let json_path = worker_dir.join(format!("{file_name}.json"));
        let markdown_path = worker_dir.join(format!("{file_name}.md"));
        write_json_file(&json_path, &worker_bundle.packet)?;
        write_markdown_file(
            &markdown_path,
            &render_worker_packet_markdown(worker_bundle, lane_assignment),
        )?;
        worker_files.push(ArchivedPacketFile {
            role: "worker".into(),
            lane_name: Some(lane_assignment.lane_name.clone()),
            feature_id: Some(worker_bundle.feature_id.clone()),
            markdown_path: stringify_path(&markdown_path),
            json_path: stringify_path(&json_path),
        });
    }

    let integrator_json_path = integrator_dir.join("integrator.json");
    let integrator_markdown_path = integrator_dir.join("integrator.md");
    write_json_file(&integrator_json_path, &wave_bundle.integrator_packet)?;
    write_markdown_file(
        &integrator_markdown_path,
        &render_shared_packet_markdown(
            "Integrator Packet",
            &wave_bundle.integrator_packet.prompt_text,
            &wave_bundle.integrator_packet.verification_commands,
            &wave_bundle.integrator_packet.allowed_writes,
            &wave_bundle.integrator_packet.forbidden_writes,
        ),
    )?;
    let integrator_file = ArchivedPacketFile {
        role: "integrator".into(),
        lane_name: None,
        feature_id: None,
        markdown_path: stringify_path(&integrator_markdown_path),
        json_path: stringify_path(&integrator_json_path),
    };

    let reviewer_json_path = reviewer_dir.join("reviewer.json");
    let reviewer_markdown_path = reviewer_dir.join("reviewer.md");
    write_json_file(&reviewer_json_path, &wave_bundle.reviewer_packet)?;
    write_markdown_file(
        &reviewer_markdown_path,
        &render_shared_packet_markdown(
            "Reviewer Packet",
            &wave_bundle.reviewer_packet.prompt_text,
            &wave_bundle.reviewer_packet.verification_commands,
            &wave_bundle.reviewer_packet.allowed_writes,
            &wave_bundle.reviewer_packet.forbidden_writes,
        ),
    )?;
    let reviewer_file = ArchivedPacketFile {
        role: "reviewer".into(),
        lane_name: None,
        feature_id: None,
        markdown_path: stringify_path(&reviewer_markdown_path),
        json_path: stringify_path(&reviewer_json_path),
    };

    let readme_path = archive_dir.join("README.md");
    write_markdown_file(
        &readme_path,
        &render_archive_readme(&wave_label, wave_bundle, &worker_files),
    )?;

    let manifest_path = archive_dir.join("wave_manifest.json");
    let archive_manifest = PacketArchiveManifest {
        wave_label: wave_label.clone(),
        archive_dir: stringify_path(&archive_dir),
        project: wave_bundle.project.clone(),
        branch_name: wave_bundle.branch_name.clone(),
        max_workers: wave_bundle.max_workers,
        wave_feature_ids: wave_bundle.wave_feature_ids.clone(),
        deferred_feature_ids: wave_bundle.deferred_feature_ids.clone(),
        rejected_feature_ids: wave_bundle.rejected_feature_ids.clone(),
        worker_files: worker_files.clone(),
        integrator_file: integrator_file.clone(),
        reviewer_file: reviewer_file.clone(),
        upstream_findings: serialize_upstream_findings(&wave_bundle.upstream_findings)?,
    };
    write_json_file(&manifest_path, &archive_manifest)?;

    Ok(PacketArchiveWriteReport {
        wave_label,
        archive_dir: stringify_path(&archive_dir),
        manifest_path: stringify_path(&manifest_path),
        readme_path: stringify_path(&readme_path),
        worker_files,
        integrator_file,
        reviewer_file,
    })
}

fn validate_packet_archive_writer_input_with_root(
    input: &PacketArchiveWriterInput,
    root: &Path,
) -> PacketArchiveValidation {
    let mut findings = Vec::new();

    validate_wave_label(&mut findings, input.wave_label.as_deref());

    let Some(wave_bundle) = input.wave_bundle.as_ref() else {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "missing_wave_bundle".into(),
            path: "wave_bundle".into(),
            message: "Packet archive writer requires a wave_bundle.".into(),
        });
        return PacketArchiveValidation { findings };
    };

    if wave_bundle.worker_bundles.is_empty() {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "empty_worker_wave".into(),
            path: "wave_bundle.worker_bundles".into(),
            message: "Wave bundle must contain at least one worker bundle.".into(),
        });
    }
    if wave_bundle.wave_feature_ids.len() != wave_bundle.worker_bundles.len() {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "wave_feature_count_mismatch".into(),
            path: "wave_bundle.wave_feature_ids".into(),
            message: "wave_feature_ids length must match worker_bundles length.".into(),
        });
    }
    if wave_bundle.lane_assignments.len() != wave_bundle.worker_bundles.len() {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "lane_assignment_count_mismatch".into(),
            path: "wave_bundle.lane_assignments".into(),
            message: "lane_assignments length must match worker_bundles length.".into(),
        });
    }
    for (index, worker_bundle) in wave_bundle.worker_bundles.iter().enumerate() {
        if target_kind_name(&worker_bundle.packet.target_kind) != "feature_worker" {
            findings.push(PacketArchiveFinding {
                severity: PacketArchiveSeverity::Error,
                code: "invalid_worker_packet_kind".into(),
                path: format!("wave_bundle.worker_bundles[{index}].packet.target_kind"),
                message: "Worker bundles must use feature_worker packet kinds.".into(),
            });
        }
        if wave_bundle
            .wave_feature_ids
            .get(index)
            .map(|feature_id| feature_id != &worker_bundle.feature_id)
            .unwrap_or(false)
        {
            findings.push(PacketArchiveFinding {
                severity: PacketArchiveSeverity::Error,
                code: "worker_feature_order_mismatch".into(),
                path: format!("wave_bundle.wave_feature_ids[{index}]"),
                message: "wave_feature_ids must match worker bundle feature order.".into(),
            });
        }
        if wave_bundle
            .lane_assignments
            .get(index)
            .map(|lane| lane.feature_id != worker_bundle.feature_id)
            .unwrap_or(false)
        {
            findings.push(PacketArchiveFinding {
                severity: PacketArchiveSeverity::Error,
                code: "lane_feature_mismatch".into(),
                path: format!("wave_bundle.lane_assignments[{index}].feature_id"),
                message: "Lane assignments must align with worker bundle feature ids.".into(),
            });
        }
    }
    if target_kind_name(&wave_bundle.integrator_packet.target_kind) != "integrator" {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "invalid_integrator_packet_kind".into(),
            path: "wave_bundle.integrator_packet.target_kind".into(),
            message: "Integrator packet must use integrator target kind.".into(),
        });
    }
    if target_kind_name(&wave_bundle.reviewer_packet.target_kind) != "reviewer" {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "invalid_reviewer_packet_kind".into(),
            path: "wave_bundle.reviewer_packet.target_kind".into(),
            message: "Reviewer packet must use reviewer target kind.".into(),
        });
    }

    if let Some(wave_label) = input.wave_label.as_deref() {
        let target_dir = root.join(wave_label);
        if target_dir.exists() && !input.overwrite_existing {
            findings.push(PacketArchiveFinding {
                severity: PacketArchiveSeverity::Error,
                code: "archive_already_exists".into(),
                path: "wave_label".into(),
                message: format!(
                    "Archive directory {} already exists and overwrite_existing is false.",
                    target_dir.display()
                ),
            });
        }
    }

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    PacketArchiveValidation { findings }
}

fn validate_wave_label(findings: &mut Vec<PacketArchiveFinding>, value: Option<&str>) {
    let Some(value) = value else {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "missing_wave_label".into(),
            path: "wave_label".into(),
            message: "wave_label is required.".into(),
        });
        return;
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "missing_wave_label".into(),
            path: "wave_label".into(),
            message: "wave_label is required.".into(),
        });
        return;
    }
    let mut chars = trimmed.chars();
    let Some(first) = chars.next() else {
        return;
    };
    if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "invalid_wave_label".into(),
            path: "wave_label".into(),
            message: "wave_label must match ^[a-z0-9][a-z0-9_-]*$.".into(),
        });
        return;
    }
    if chars.any(|ch| !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && ch != '_' && ch != '-') {
        findings.push(PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: "invalid_wave_label".into(),
            path: "wave_label".into(),
            message: "wave_label must match ^[a-z0-9][a-z0-9_-]*$.".into(),
        });
    }
}

fn render_archive_readme(
    wave_label: &str,
    wave_bundle: &FeatureWavePacketBundle,
    worker_files: &[ArchivedPacketFile],
) -> String {
    let mut output = String::new();
    output.push_str(&format!("# Packet Wave Archive: {wave_label}\n\n"));
    output.push_str("Repo path:\n");
    output.push_str(&format!("- `{REPO_ROOT}`\n\n"));
    output.push_str("Startup sequence:\n");
    for command in STARTUP_COMMANDS {
        output.push_str(&format!("- `{command}`\n"));
    }
    output.push('\n');
    output.push_str("Read first:\n");
    for file in READ_FIRST_FILES {
        output.push_str(&format!("- `{file}`\n"));
    }
    output.push('\n');
    output.push_str("Lane to feature map:\n");
    for archived in worker_files {
        output.push_str(&format!(
            "- {} -> {}\n",
            archived.lane_name.as_deref().unwrap_or("(unknown lane)"),
            archived
                .feature_id
                .as_deref()
                .unwrap_or("(unknown feature)")
        ));
    }
    output.push('\n');
    output.push_str("Deferred feature ids:\n");
    write_list_or_none(&mut output, &wave_bundle.deferred_feature_ids);
    output.push('\n');
    output.push_str("Rejected feature ids:\n");
    write_list_or_none(&mut output, &wave_bundle.rejected_feature_ids);
    output.push('\n');
    output.push_str("Use these files:\n");
    output.push_str("- worker windows use `worker/*.md`\n");
    output.push_str("- integrator uses `integrator/integrator.md`\n");
    output.push_str("- reviewer uses `reviewer/reviewer.md`\n");
    output
}

fn render_worker_packet_markdown(
    worker_bundle: &FeatureWaveWorkerBundle,
    lane_assignment: &FeatureWaveLaneAssignment,
) -> String {
    render_packet_markdown(
        &format!(
            "Worker Packet: {} / {}",
            lane_assignment.lane_name, worker_bundle.feature_id
        ),
        "Use this in a fresh ChatGPT coding window assigned to this worker lane.",
        &worker_bundle.packet.prompt_text,
        &worker_bundle.packet.verification_commands,
        &worker_bundle.packet.allowed_writes,
        &worker_bundle.packet.forbidden_writes,
    )
}

fn render_shared_packet_markdown(
    title: &str,
    prompt_text: &str,
    verification_commands: &[String],
    allowed_writes: &[String],
    forbidden_writes: &[String],
) -> String {
    render_packet_markdown(
        title,
        "Use this in a fresh ChatGPT coding window assigned to this shared role.",
        prompt_text,
        verification_commands,
        allowed_writes,
        forbidden_writes,
    )
}

fn render_packet_markdown(
    title: &str,
    note: &str,
    prompt_text: &str,
    verification_commands: &[String],
    allowed_writes: &[String],
    forbidden_writes: &[String],
) -> String {
    let mut output = String::new();
    output.push_str(&format!("# {title}\n\n"));
    output.push_str(note);
    output.push_str("\n\nStartup sequence:\n");
    for command in STARTUP_COMMANDS {
        output.push_str(&format!("- `{command}`\n"));
    }
    output.push('\n');
    output.push_str("Read first:\n");
    for file in READ_FIRST_FILES {
        output.push_str(&format!("- `{file}`\n"));
    }
    output.push_str("\nPrompt:\n```text\n");
    output.push_str(prompt_text);
    if !prompt_text.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("```\n\nVerification:\n");
    write_string_list(&mut output, verification_commands);
    output.push('\n');
    output.push_str("Allowed writes:\n");
    write_list_or_none(&mut output, allowed_writes);
    output.push('\n');
    output.push_str("Forbidden writes:\n");
    write_list_or_none(&mut output, forbidden_writes);
    output
}

fn write_string_list(output: &mut String, values: &[String]) {
    for value in values {
        output.push_str(&format!("- `{value}`\n"));
    }
}

fn write_list_or_none(output: &mut String, values: &[String]) {
    if values.is_empty() {
        output.push_str("- `(none)`\n");
        return;
    }
    write_string_list(output, values);
}

fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), PacketArchiveValidation> {
    let contents =
        serde_json::to_string_pretty(value).map_err(|error| PacketArchiveValidation {
            findings: vec![PacketArchiveFinding {
                severity: PacketArchiveSeverity::Error,
                code: "serialization_failed".into(),
                path: stringify_path(path),
                message: error.to_string(),
            }],
        })?;
    fs::write(path, contents).map_err(|error| io_validation("write_json", error, path))
}

fn serialize_upstream_findings<T: Serialize>(
    findings: &[T],
) -> Result<Vec<serde_json::Value>, PacketArchiveValidation> {
    findings
        .iter()
        .map(|finding| {
            serde_json::to_value(finding).map_err(|error| PacketArchiveValidation {
                findings: vec![PacketArchiveFinding {
                    severity: PacketArchiveSeverity::Error,
                    code: "serialization_failed".into(),
                    path: "upstream_findings".into(),
                    message: error.to_string(),
                }],
            })
        })
        .collect()
}

fn write_markdown_file(path: &Path, contents: &str) -> Result<(), PacketArchiveValidation> {
    fs::write(path, contents).map_err(|error| io_validation("write_markdown", error, path))
}

fn remove_existing_archive_dir(
    archive_dir: &Path,
    root: &Path,
) -> Result<(), PacketArchiveValidation> {
    let parent_matches = archive_dir.parent() == Some(root);
    let starts_with_root = archive_dir.starts_with(root);
    if !parent_matches || !starts_with_root {
        return Err(PacketArchiveValidation {
            findings: vec![PacketArchiveFinding {
                severity: PacketArchiveSeverity::Error,
                code: "unsafe_archive_delete".into(),
                path: stringify_path(archive_dir),
                message: "Refusing to delete outside the resolved wave directory.".into(),
            }],
        });
    }
    fs::remove_dir_all(archive_dir)
        .map_err(|error| io_validation("remove_archive_dir", error, archive_dir))
}

fn io_validation(action: &str, error: std::io::Error, path: &Path) -> PacketArchiveValidation {
    PacketArchiveValidation {
        findings: vec![PacketArchiveFinding {
            severity: PacketArchiveSeverity::Error,
            code: action.into(),
            path: stringify_path(path),
            message: error.to_string(),
        }],
    }
}

fn stringify_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn require_string(value: Option<String>) -> String {
    value
        .expect("validated string should exist")
        .trim()
        .to_string()
}

fn target_kind_name<T: Serialize>(target_kind: &T) -> String {
    serde_json::to_value(target_kind)
        .ok()
        .and_then(|value| value.as_str().map(ToString::to_string))
        .unwrap_or_default()
}
