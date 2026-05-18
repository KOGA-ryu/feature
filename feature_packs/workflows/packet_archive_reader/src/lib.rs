use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use feature_wave_packet_flow::{
    FeatureWaveLaneAssignment, FeatureWavePacketBundle, FeatureWaveWorkerBundle,
};
use packet_archive_writer::{ArchivedPacketFile, PACKET_WAVES_ROOT, PacketArchiveManifest};
use prompt_generator::{GeneratedPromptPacket, PromptTargetKind};
use serde::{Deserialize, Serialize};
use spec_selector::SpecSelectionFinding;

pub const FEATURE_ID: &str = "workflow.packet_archive_reader";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveReaderInput {
    pub wave_label: Option<String>,
    pub archive_root: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedPacketDocument {
    pub role: String,
    pub lane_name: Option<String>,
    pub feature_id: Option<String>,
    pub markdown_path: String,
    pub markdown_text: String,
    pub json_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveReadReport {
    pub wave_label: String,
    pub archive_dir: String,
    pub manifest_path: String,
    pub readme_path: String,
    pub readme_text: String,
    pub archive_manifest: PacketArchiveManifest,
    pub wave_bundle: FeatureWavePacketBundle,
    pub worker_documents: Vec<ArchivedPacketDocument>,
    pub integrator_document: ArchivedPacketDocument,
    pub reviewer_document: ArchivedPacketDocument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketArchiveReaderSeverity {
    Error,
    Warning,
}

impl fmt::Display for PacketArchiveReaderSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveReaderFinding {
    pub severity: PacketArchiveReaderSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveReaderValidation {
    pub findings: Vec<PacketArchiveReaderFinding>,
}

impl PacketArchiveReaderValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == PacketArchiveReaderSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PacketArchiveReader;

impl PacketArchiveReader {
    pub fn validate(&self, input: &PacketArchiveReaderInput) -> PacketArchiveReaderValidation {
        validate_packet_archive_reader_input(input)
    }

    pub fn read(
        &self,
        input: PacketArchiveReaderInput,
    ) -> Result<PacketArchiveReadReport, PacketArchiveReaderValidation> {
        read_packet_archive(input)
    }
}

pub fn read_packet_archive(
    input: PacketArchiveReaderInput,
) -> Result<PacketArchiveReadReport, PacketArchiveReaderValidation> {
    read_packet_archive_with_root(&input, None)
}

pub fn validate_packet_archive_reader_input(
    input: &PacketArchiveReaderInput,
) -> PacketArchiveReaderValidation {
    let mut findings = Vec::new();

    validate_wave_label(&mut findings, input.wave_label.as_deref());

    if let Some(root) = input.archive_root.as_deref() {
        if root.trim().is_empty() {
            findings.push(PacketArchiveReaderFinding {
                severity: PacketArchiveReaderSeverity::Error,
                code: "blank_archive_root".into(),
                path: "archive_root".into(),
                message: "archive_root must not be blank when provided.".into(),
            });
        } else {
            let path = Path::new(root);
            if path.exists() && !path.is_dir() {
                findings.push(PacketArchiveReaderFinding {
                    severity: PacketArchiveReaderSeverity::Error,
                    code: "archive_root_not_directory".into(),
                    path: "archive_root".into(),
                    message: "archive_root must point to a directory when it exists.".into(),
                });
            }
        }
    }

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    PacketArchiveReaderValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_packet_archive_reader_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_packet_archive_reader_input.json")
}

pub fn parse_packet_archive_reader_input(raw: &str) -> Result<PacketArchiveReaderInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_input() -> Result<PacketArchiveReaderInput, String> {
    parse_packet_archive_reader_input(sample_valid_fixture())
}

pub fn sample_invalid_input() -> Result<PacketArchiveReaderInput, String> {
    parse_packet_archive_reader_input(sample_invalid_fixture())
}

fn read_packet_archive_with_root(
    input: &PacketArchiveReaderInput,
    root_override: Option<&Path>,
) -> Result<PacketArchiveReadReport, PacketArchiveReaderValidation> {
    let validation = validate_packet_archive_reader_input(input);
    if !validation.is_valid() {
        return Err(validation);
    }

    let wave_label = required_string(input.wave_label.clone());
    let archive_root = match root_override {
        Some(path) => path.to_path_buf(),
        None => resolved_archive_root(input),
    };
    let archive_dir = archive_root.join(&wave_label);
    let mut findings = Vec::new();

    if !archive_dir.exists() {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "missing_archive_directory".into(),
            path: stringify_path(&archive_dir),
            message: "Resolved archive directory does not exist.".into(),
        });
        return Err(PacketArchiveReaderValidation { findings });
    }
    if !archive_dir.is_dir() {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "archive_dir_not_directory".into(),
            path: stringify_path(&archive_dir),
            message: "Resolved archive directory must be a directory.".into(),
        });
        return Err(PacketArchiveReaderValidation { findings });
    }

    let canonical_archive_dir = match fs::canonicalize(&archive_dir) {
        Ok(path) => path,
        Err(error) => {
            return Err(PacketArchiveReaderValidation {
                findings: vec![PacketArchiveReaderFinding {
                    severity: PacketArchiveReaderSeverity::Error,
                    code: "unreadable_archive_directory".into(),
                    path: stringify_path(&archive_dir),
                    message: error.to_string(),
                }],
            });
        }
    };

    let manifest_path = archive_dir.join("wave_manifest.json");
    let raw_manifest = match read_required_file(
        &manifest_path,
        "missing_manifest",
        "wave_manifest.json is required in the archive directory.",
        "unreadable_manifest",
    ) {
        Ok(contents) => contents,
        Err(validation) => return Err(validation),
    };
    let archive_manifest = match serde_json::from_str::<PacketArchiveManifest>(&raw_manifest) {
        Ok(manifest) => manifest,
        Err(error) => {
            return Err(PacketArchiveReaderValidation {
                findings: vec![PacketArchiveReaderFinding {
                    severity: PacketArchiveReaderSeverity::Error,
                    code: "invalid_manifest_json".into(),
                    path: stringify_path(&manifest_path),
                    message: error.to_string(),
                }],
            });
        }
    };

    validate_archive_manifest_header(
        &mut findings,
        &archive_manifest,
        &wave_label,
        &archive_dir,
        &canonical_archive_dir,
        &manifest_path,
    );

    let readme_path = archive_dir.join("README.md");
    let readme_text = match read_required_file(
        &readme_path,
        "missing_readme",
        "README.md is required in the archive directory.",
        "unreadable_readme",
    ) {
        Ok(contents) => contents,
        Err(validation) => return Err(validation),
    };

    if archive_manifest.worker_files.is_empty() {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "empty_worker_wave".into(),
            path: format!("{}::worker_files", stringify_path(&manifest_path)),
            message: "Manifest worker_files must not be empty.".into(),
        });
    }
    if archive_manifest.worker_files.len() != archive_manifest.wave_feature_ids.len() {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "wave_feature_count_mismatch".into(),
            path: format!("{}::wave_feature_ids", stringify_path(&manifest_path)),
            message: "worker_files length must match wave_feature_ids length.".into(),
        });
    }
    if archive_manifest.worker_files.len() > archive_manifest.max_workers {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "max_workers_exceeded".into(),
            path: format!("{}::max_workers", stringify_path(&manifest_path)),
            message: "worker_files length must not exceed manifest.max_workers.".into(),
        });
    }

    let mut worker_documents = Vec::new();
    let mut worker_bundles = Vec::new();
    let mut lane_assignments = Vec::new();
    for (index, archived_file) in archive_manifest.worker_files.iter().enumerate() {
        if archived_file.role != "worker" {
            findings.push(PacketArchiveReaderFinding {
                severity: PacketArchiveReaderSeverity::Error,
                code: "invalid_worker_file_role".into(),
                path: archived_file.json_path.clone(),
                message: "Manifest worker file role must be worker.".into(),
            });
        }

        let lane_name = archived_file
            .lane_name
            .clone()
            .filter(|value| !value.trim().is_empty());
        if lane_name.is_none() {
            findings.push(PacketArchiveReaderFinding {
                severity: PacketArchiveReaderSeverity::Error,
                code: "missing_worker_lane_name".into(),
                path: archived_file.markdown_path.clone(),
                message: "Worker files must include a lane_name.".into(),
            });
        }

        let feature_id = archived_file
            .feature_id
            .clone()
            .filter(|value| !value.trim().is_empty());
        if feature_id.is_none() {
            findings.push(PacketArchiveReaderFinding {
                severity: PacketArchiveReaderSeverity::Error,
                code: "missing_worker_feature_id".into(),
                path: archived_file.json_path.clone(),
                message: "Worker files must include a feature_id.".into(),
            });
        }
        if let (Some(feature_id), Some(expected_feature_id)) = (
            feature_id.as_ref(),
            archive_manifest.wave_feature_ids.get(index),
        ) {
            if feature_id != expected_feature_id {
                findings.push(PacketArchiveReaderFinding {
                    severity: PacketArchiveReaderSeverity::Error,
                    code: "worker_feature_order_mismatch".into(),
                    path: archived_file.json_path.clone(),
                    message: format!(
                        "Worker feature {} does not match manifest wave feature {} at index {}.",
                        feature_id, expected_feature_id, index
                    ),
                });
            }
        }

        match read_archived_packet_document(
            archived_file,
            "worker",
            PromptTargetKind::FeatureWorker,
            &canonical_archive_dir,
        ) {
            Ok((document, packet)) => {
                if let (Some(feature_id), Some(lane_name)) = (feature_id, lane_name) {
                    worker_documents.push(document);
                    worker_bundles.push(FeatureWaveWorkerBundle {
                        feature_id: feature_id.clone(),
                        // Packet archives do not currently persist worker selection reasons.
                        selection_reason: String::new(),
                        packet: packet.clone(),
                    });
                    lane_assignments.push(FeatureWaveLaneAssignment {
                        lane_name,
                        feature_id,
                        allowed_writes: packet.allowed_writes.clone(),
                        forbidden_writes: packet.forbidden_writes.clone(),
                    });
                }
            }
            Err(mut document_findings) => findings.append(&mut document_findings),
        }
    }

    if archive_manifest.integrator_file.role != "integrator" {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "invalid_integrator_file_role".into(),
            path: archive_manifest.integrator_file.json_path.clone(),
            message: "Manifest integrator_file role must be integrator.".into(),
        });
    }
    let (integrator_document, integrator_packet) = match read_archived_packet_document(
        &archive_manifest.integrator_file,
        "integrator",
        PromptTargetKind::Integrator,
        &canonical_archive_dir,
    ) {
        Ok(result) => result,
        Err(mut document_findings) => {
            findings.append(&mut document_findings);
            placeholder_shared_document("integrator")
        }
    };

    if archive_manifest.reviewer_file.role != "reviewer" {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "invalid_reviewer_file_role".into(),
            path: archive_manifest.reviewer_file.json_path.clone(),
            message: "Manifest reviewer_file role must be reviewer.".into(),
        });
    }
    let (reviewer_document, reviewer_packet) = match read_archived_packet_document(
        &archive_manifest.reviewer_file,
        "reviewer",
        PromptTargetKind::Reviewer,
        &canonical_archive_dir,
    ) {
        Ok(result) => result,
        Err(mut document_findings) => {
            findings.append(&mut document_findings);
            placeholder_shared_document("reviewer")
        }
    };

    let upstream_findings = match deserialize_upstream_findings(&archive_manifest.upstream_findings)
    {
        Ok(findings) => findings,
        Err(mut upstream_validation) => {
            findings.append(&mut upstream_validation);
            Vec::new()
        }
    };

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    let final_validation = PacketArchiveReaderValidation { findings };
    if !final_validation.is_valid() {
        return Err(final_validation);
    }

    Ok(PacketArchiveReadReport {
        wave_label: archive_manifest.wave_label.clone(),
        archive_dir: stringify_path(&archive_dir),
        manifest_path: stringify_path(&manifest_path),
        readme_path: stringify_path(&readme_path),
        readme_text,
        wave_bundle: FeatureWavePacketBundle {
            project: archive_manifest.project.clone(),
            branch_name: archive_manifest.branch_name.clone(),
            max_workers: archive_manifest.max_workers,
            wave_feature_ids: archive_manifest.wave_feature_ids.clone(),
            worker_bundles,
            integrator_packet,
            reviewer_packet,
            lane_assignments,
            deferred_feature_ids: archive_manifest.deferred_feature_ids.clone(),
            rejected_feature_ids: archive_manifest.rejected_feature_ids.clone(),
            upstream_findings,
        },
        archive_manifest,
        worker_documents,
        integrator_document,
        reviewer_document,
    })
}

fn resolved_archive_root(input: &PacketArchiveReaderInput) -> PathBuf {
    input
        .archive_root
        .as_ref()
        .map(|root| PathBuf::from(root.trim()))
        .unwrap_or_else(|| PathBuf::from(PACKET_WAVES_ROOT))
}

fn validate_archive_manifest_header(
    findings: &mut Vec<PacketArchiveReaderFinding>,
    archive_manifest: &PacketArchiveManifest,
    wave_label: &str,
    archive_dir: &Path,
    canonical_archive_dir: &Path,
    manifest_path: &Path,
) {
    if archive_manifest.wave_label != wave_label {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "wave_label_mismatch".into(),
            path: stringify_path(manifest_path),
            message: format!(
                "Manifest wave_label {} does not match requested wave label {}.",
                archive_manifest.wave_label, wave_label
            ),
        });
    }

    let manifest_archive_dir = Path::new(&archive_manifest.archive_dir);
    if !manifest_archive_dir.is_absolute() {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "non_absolute_archive_dir".into(),
            path: stringify_path(manifest_path),
            message: "Manifest archive_dir must be absolute.".into(),
        });
        return;
    }

    match fs::canonicalize(manifest_archive_dir) {
        Ok(path) => {
            if path != canonical_archive_dir {
                findings.push(PacketArchiveReaderFinding {
                    severity: PacketArchiveReaderSeverity::Error,
                    code: "archive_dir_mismatch".into(),
                    path: stringify_path(manifest_path),
                    message: format!(
                        "Manifest archive_dir {} does not match resolved archive directory {}.",
                        archive_manifest.archive_dir,
                        stringify_path(archive_dir)
                    ),
                });
            }
        }
        Err(error) => findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "unreadable_manifest_archive_dir".into(),
            path: stringify_path(manifest_path),
            message: error.to_string(),
        }),
    }
}

fn deserialize_upstream_findings(
    raw_findings: &[serde_json::Value],
) -> Result<Vec<SpecSelectionFinding>, Vec<PacketArchiveReaderFinding>> {
    let mut findings = Vec::new();
    let mut output = Vec::new();

    for (index, raw) in raw_findings.iter().enumerate() {
        match serde_json::from_value::<SpecSelectionFinding>(raw.clone()) {
            Ok(finding) => output.push(finding),
            Err(error) => findings.push(PacketArchiveReaderFinding {
                severity: PacketArchiveReaderSeverity::Error,
                code: "invalid_upstream_finding".into(),
                path: format!("archive_manifest.upstream_findings[{index}]"),
                message: error.to_string(),
            }),
        }
    }

    if findings.is_empty() {
        Ok(output)
    } else {
        Err(findings)
    }
}

fn read_archived_packet_document(
    archived_file: &ArchivedPacketFile,
    expected_role: &str,
    expected_target_kind: PromptTargetKind,
    canonical_archive_dir: &Path,
) -> Result<(ArchivedPacketDocument, GeneratedPromptPacket), Vec<PacketArchiveReaderFinding>> {
    let mut findings = Vec::new();

    let markdown_path = validate_archive_member_path(
        &archived_file.markdown_path,
        canonical_archive_dir,
        "markdown_path",
        &mut findings,
    );
    let json_path = validate_archive_member_path(
        &archived_file.json_path,
        canonical_archive_dir,
        "json_path",
        &mut findings,
    );

    let markdown_text = match markdown_path.as_deref() {
        Some(path) => match read_required_file(
            path,
            "missing_markdown_file",
            "Referenced markdown file does not exist.",
            "unreadable_markdown_file",
        ) {
            Ok(contents) => Some(contents),
            Err(validation) => {
                findings.extend(validation.findings);
                None
            }
        },
        None => None,
    };

    let packet = match json_path.as_deref() {
        Some(path) => match read_required_file(
            path,
            "missing_json_file",
            "Referenced packet json file does not exist.",
            "unreadable_json_file",
        ) {
            Ok(contents) => match serde_json::from_str::<GeneratedPromptPacket>(&contents) {
                Ok(packet) => Some(packet),
                Err(error) => {
                    findings.push(PacketArchiveReaderFinding {
                        severity: PacketArchiveReaderSeverity::Error,
                        code: "invalid_packet_json".into(),
                        path: stringify_path(path),
                        message: error.to_string(),
                    });
                    None
                }
            },
            Err(validation) => {
                findings.extend(validation.findings);
                None
            }
        },
        None => None,
    };

    if let Some(packet) = packet.as_ref() {
        if target_kind_name(&packet.target_kind) != target_kind_name(&expected_target_kind) {
            findings.push(PacketArchiveReaderFinding {
                severity: PacketArchiveReaderSeverity::Error,
                code: format!("invalid_{expected_role}_packet_kind"),
                path: archived_file.json_path.clone(),
                message: format!(
                    "{expected_role} packet must use {} target kind.",
                    target_kind_name(&expected_target_kind)
                ),
            });
        }
    }

    if !findings.is_empty() {
        return Err(findings);
    }

    Ok((
        ArchivedPacketDocument {
            role: archived_file.role.clone(),
            lane_name: archived_file.lane_name.clone(),
            feature_id: archived_file.feature_id.clone(),
            markdown_path: archived_file.markdown_path.clone(),
            markdown_text: markdown_text.unwrap_or_default(),
            json_path: archived_file.json_path.clone(),
        },
        packet.expect("packet should exist when validations pass"),
    ))
}

fn validate_archive_member_path(
    raw_path: &str,
    canonical_archive_dir: &Path,
    field_name: &str,
    findings: &mut Vec<PacketArchiveReaderFinding>,
) -> Option<PathBuf> {
    let path = PathBuf::from(raw_path);
    if !path.is_absolute() {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: format!("non_absolute_{field_name}"),
            path: raw_path.into(),
            message: format!("{field_name} must be absolute."),
        });
        return None;
    }
    match fs::canonicalize(&path) {
        Ok(canonical) => {
            if !canonical.starts_with(canonical_archive_dir) {
                findings.push(PacketArchiveReaderFinding {
                    severity: PacketArchiveReaderSeverity::Error,
                    code: "file_outside_archive_dir".into(),
                    path: raw_path.into(),
                    message: "Manifest file paths must resolve inside the archive directory."
                        .into(),
                });
                return None;
            }
        }
        Err(_) => {
            // Missing-file errors are emitted separately by read_required_file.
        }
    }
    Some(path)
}

fn read_required_file(
    path: &Path,
    missing_code: &str,
    missing_message: &str,
    unreadable_code: &str,
) -> Result<String, PacketArchiveReaderValidation> {
    if !path.exists() {
        return Err(PacketArchiveReaderValidation {
            findings: vec![PacketArchiveReaderFinding {
                severity: PacketArchiveReaderSeverity::Error,
                code: missing_code.into(),
                path: stringify_path(path),
                message: missing_message.into(),
            }],
        });
    }
    fs::read_to_string(path).map_err(|error| PacketArchiveReaderValidation {
        findings: vec![PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: unreadable_code.into(),
            path: stringify_path(path),
            message: error.to_string(),
        }],
    })
}

fn placeholder_shared_document(role: &str) -> (ArchivedPacketDocument, GeneratedPromptPacket) {
    (
        ArchivedPacketDocument {
            role: role.into(),
            lane_name: None,
            feature_id: None,
            markdown_path: String::new(),
            markdown_text: String::new(),
            json_path: String::new(),
        },
        GeneratedPromptPacket {
            target_kind: if role == "integrator" {
                PromptTargetKind::Integrator
            } else {
                PromptTargetKind::Reviewer
            },
            title: String::new(),
            prompt_text: String::new(),
            verification_commands: Vec::new(),
            allowed_writes: Vec::new(),
            forbidden_writes: Vec::new(),
        },
    )
}

fn validate_wave_label(findings: &mut Vec<PacketArchiveReaderFinding>, value: Option<&str>) {
    let Some(value) = value else {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "missing_wave_label".into(),
            path: "wave_label".into(),
            message: "wave_label is required.".into(),
        });
        return;
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
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
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "invalid_wave_label".into(),
            path: "wave_label".into(),
            message: "wave_label must match ^[a-z0-9][a-z0-9_-]*$.".into(),
        });
        return;
    }
    if chars.any(|ch| !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && ch != '_' && ch != '-') {
        findings.push(PacketArchiveReaderFinding {
            severity: PacketArchiveReaderSeverity::Error,
            code: "invalid_wave_label".into(),
            path: "wave_label".into(),
            message: "wave_label must match ^[a-z0-9][a-z0-9_-]*$.".into(),
        });
    }
}

fn stringify_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn required_string(value: Option<String>) -> String {
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
