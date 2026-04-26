use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use packet_archive_writer::{ArchivedPacketFile, PACKET_WAVES_ROOT, PacketArchiveManifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "workflow.packet_archive_index";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveIndexInput {
    pub archive_root: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchivedWaveStatus {
    Complete,
    Incomplete,
    Invalid,
}

impl fmt::Display for ArchivedWaveStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Complete => formatter.write_str("complete"),
            Self::Incomplete => formatter.write_str("incomplete"),
            Self::Invalid => formatter.write_str("invalid"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketArchiveIndexSeverity {
    Error,
    Warning,
}

impl fmt::Display for PacketArchiveIndexSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveIndexFinding {
    pub severity: PacketArchiveIndexSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedWaveSummary {
    pub wave_label: String,
    pub status: ArchivedWaveStatus,
    pub manifest_path: String,
    pub archive_dir: String,
    pub project: Option<String>,
    pub branch_name: Option<String>,
    pub wave_feature_ids: Vec<String>,
    pub deferred_feature_ids: Vec<String>,
    pub rejected_feature_ids: Vec<String>,
    pub worker_files: Vec<ArchivedPacketFile>,
    pub integrator_file: Option<ArchivedPacketFile>,
    pub reviewer_file: Option<ArchivedPacketFile>,
    pub findings: Vec<PacketArchiveIndexFinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveIndexReport {
    pub archive_root: String,
    pub waves: Vec<ArchivedWaveSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketArchiveIndexValidation {
    pub findings: Vec<PacketArchiveIndexFinding>,
}

impl PacketArchiveIndexValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == PacketArchiveIndexSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PacketArchiveIndexer;

impl PacketArchiveIndexer {
    pub fn validate(&self, input: &PacketArchiveIndexInput) -> PacketArchiveIndexValidation {
        validate_packet_archive_index_input(input)
    }

    pub fn index(
        &self,
        input: PacketArchiveIndexInput,
    ) -> Result<PacketArchiveIndexReport, PacketArchiveIndexValidation> {
        index_packet_archives(input)
    }
}

pub fn index_packet_archives(
    input: PacketArchiveIndexInput,
) -> Result<PacketArchiveIndexReport, PacketArchiveIndexValidation> {
    index_packet_archives_with_root(&input, None)
}

pub fn validate_packet_archive_index_input(
    input: &PacketArchiveIndexInput,
) -> PacketArchiveIndexValidation {
    let mut findings = Vec::new();

    if let Some(root) = input.archive_root.as_deref() {
        if root.trim().is_empty() {
            findings.push(PacketArchiveIndexFinding {
                severity: PacketArchiveIndexSeverity::Error,
                code: "blank_archive_root".into(),
                path: "archive_root".into(),
                message: "archive_root must not be blank when provided.".into(),
            });
        } else {
            let path = Path::new(root);
            if path.exists() && !path.is_dir() {
                findings.push(PacketArchiveIndexFinding {
                    severity: PacketArchiveIndexSeverity::Error,
                    code: "archive_root_not_directory".into(),
                    path: "archive_root".into(),
                    message: "archive_root must point to a directory when it exists.".into(),
                });
            }
        }
    }

    PacketArchiveIndexValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_packet_archive_index_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_packet_archive_index_input.json")
}

pub fn parse_packet_archive_index_input(raw: &str) -> Result<PacketArchiveIndexInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_input() -> Result<PacketArchiveIndexInput, String> {
    parse_packet_archive_index_input(sample_valid_fixture())
}

pub fn sample_invalid_input() -> Result<PacketArchiveIndexInput, String> {
    parse_packet_archive_index_input(sample_invalid_fixture())
}

fn index_packet_archives_with_root(
    input: &PacketArchiveIndexInput,
    root_override: Option<&Path>,
) -> Result<PacketArchiveIndexReport, PacketArchiveIndexValidation> {
    let validation = validate_packet_archive_index_input(input);
    if !validation.is_valid() {
        return Err(validation);
    }

    let archive_root = match root_override {
        Some(path) => path.to_path_buf(),
        None => resolved_archive_root(input),
    };
    if !archive_root.exists() {
        return Ok(PacketArchiveIndexReport {
            archive_root: stringify_path(&archive_root),
            waves: Vec::new(),
        });
    }

    let mut wave_dirs = fs::read_dir(&archive_root)
        .map_err(|error| directory_validation("archive_root", error, &archive_root))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|file_type| file_type.is_dir())
                .map(|_| entry.path())
        })
        .collect::<Vec<_>>();
    wave_dirs.sort_by(|left, right| left.file_name().cmp(&right.file_name()));

    let waves = wave_dirs
        .iter()
        .map(|wave_dir| summarize_wave_dir(wave_dir))
        .collect::<Vec<_>>();

    Ok(PacketArchiveIndexReport {
        archive_root: stringify_path(&archive_root),
        waves,
    })
}

fn resolved_archive_root(input: &PacketArchiveIndexInput) -> PathBuf {
    input
        .archive_root
        .as_ref()
        .map(|root| PathBuf::from(root.trim()))
        .unwrap_or_else(|| PathBuf::from(PACKET_WAVES_ROOT))
}

fn summarize_wave_dir(wave_dir: &Path) -> ArchivedWaveSummary {
    let wave_label = wave_dir
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "(unknown)".into());
    let manifest_path = wave_dir.join("wave_manifest.json");

    if !manifest_path.exists() {
        return ArchivedWaveSummary {
            wave_label,
            status: ArchivedWaveStatus::Invalid,
            manifest_path: stringify_path(&manifest_path),
            archive_dir: stringify_path(wave_dir),
            project: None,
            branch_name: None,
            wave_feature_ids: Vec::new(),
            deferred_feature_ids: Vec::new(),
            rejected_feature_ids: Vec::new(),
            worker_files: Vec::new(),
            integrator_file: None,
            reviewer_file: None,
            findings: vec![PacketArchiveIndexFinding {
                severity: PacketArchiveIndexSeverity::Error,
                code: "missing_manifest".into(),
                path: stringify_path(&manifest_path),
                message: "Archive directory is missing wave_manifest.json.".into(),
            }],
        };
    }

    let raw_manifest = match fs::read_to_string(&manifest_path) {
        Ok(contents) => contents,
        Err(error) => {
            return ArchivedWaveSummary {
                wave_label,
                status: ArchivedWaveStatus::Invalid,
                manifest_path: stringify_path(&manifest_path),
                archive_dir: stringify_path(wave_dir),
                project: None,
                branch_name: None,
                wave_feature_ids: Vec::new(),
                deferred_feature_ids: Vec::new(),
                rejected_feature_ids: Vec::new(),
                worker_files: Vec::new(),
                integrator_file: None,
                reviewer_file: None,
                findings: vec![PacketArchiveIndexFinding {
                    severity: PacketArchiveIndexSeverity::Error,
                    code: "unreadable_manifest".into(),
                    path: stringify_path(&manifest_path),
                    message: error.to_string(),
                }],
            };
        }
    };

    let manifest = match serde_json::from_str::<PacketArchiveManifest>(&raw_manifest) {
        Ok(manifest) => manifest,
        Err(error) => {
            return ArchivedWaveSummary {
                wave_label,
                status: ArchivedWaveStatus::Invalid,
                manifest_path: stringify_path(&manifest_path),
                archive_dir: stringify_path(wave_dir),
                project: None,
                branch_name: None,
                wave_feature_ids: Vec::new(),
                deferred_feature_ids: Vec::new(),
                rejected_feature_ids: Vec::new(),
                worker_files: Vec::new(),
                integrator_file: None,
                reviewer_file: None,
                findings: vec![PacketArchiveIndexFinding {
                    severity: PacketArchiveIndexSeverity::Error,
                    code: "invalid_manifest_json".into(),
                    path: stringify_path(&manifest_path),
                    message: error.to_string(),
                }],
            };
        }
    };

    let mut findings = Vec::new();
    let mut invalid = false;
    let mut incomplete = false;

    if manifest.wave_label != wave_label {
        invalid = true;
        findings.push(PacketArchiveIndexFinding {
            severity: PacketArchiveIndexSeverity::Error,
            code: "wave_label_mismatch".into(),
            path: stringify_path(&manifest_path),
            message: format!(
                "Manifest wave_label {} does not match directory label {}.",
                manifest.wave_label, wave_label
            ),
        });
    }

    let actual_archive_dir = stringify_path(wave_dir);
    if !Path::new(&manifest.archive_dir).is_absolute() {
        invalid = true;
        findings.push(PacketArchiveIndexFinding {
            severity: PacketArchiveIndexSeverity::Error,
            code: "non_absolute_archive_dir".into(),
            path: stringify_path(&manifest_path),
            message: "Manifest archive_dir must be absolute.".into(),
        });
    } else if manifest.archive_dir != actual_archive_dir {
        invalid = true;
        findings.push(PacketArchiveIndexFinding {
            severity: PacketArchiveIndexSeverity::Error,
            code: "archive_dir_mismatch".into(),
            path: stringify_path(&manifest_path),
            message: format!(
                "Manifest archive_dir {} does not match actual directory {}.",
                manifest.archive_dir, actual_archive_dir
            ),
        });
    }

    for worker_file in &manifest.worker_files {
        let result = validate_archived_file(worker_file, "worker", true, true);
        invalid |= result.invalid;
        incomplete |= result.incomplete;
        findings.extend(result.findings);
    }
    if manifest.integrator_file.role != "integrator" {
        invalid = true;
        findings.push(PacketArchiveIndexFinding {
            severity: PacketArchiveIndexSeverity::Error,
            code: "invalid_integrator_role".into(),
            path: manifest.integrator_file.json_path.clone(),
            message: "Integrator file role must be integrator.".into(),
        });
    }
    let integrator_result =
        validate_archived_file(&manifest.integrator_file, "integrator", false, false);
    invalid |= integrator_result.invalid;
    incomplete |= integrator_result.incomplete;
    findings.extend(integrator_result.findings);

    if manifest.reviewer_file.role != "reviewer" {
        invalid = true;
        findings.push(PacketArchiveIndexFinding {
            severity: PacketArchiveIndexSeverity::Error,
            code: "invalid_reviewer_role".into(),
            path: manifest.reviewer_file.json_path.clone(),
            message: "Reviewer file role must be reviewer.".into(),
        });
    }
    let reviewer_result = validate_archived_file(&manifest.reviewer_file, "reviewer", false, false);
    invalid |= reviewer_result.invalid;
    incomplete |= reviewer_result.incomplete;
    findings.extend(reviewer_result.findings);

    let status = if invalid {
        ArchivedWaveStatus::Invalid
    } else if incomplete {
        ArchivedWaveStatus::Incomplete
    } else {
        ArchivedWaveStatus::Complete
    };

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    ArchivedWaveSummary {
        wave_label,
        status,
        manifest_path: stringify_path(&manifest_path),
        archive_dir: actual_archive_dir,
        project: Some(manifest.project),
        branch_name: Some(manifest.branch_name),
        wave_feature_ids: manifest.wave_feature_ids,
        deferred_feature_ids: manifest.deferred_feature_ids,
        rejected_feature_ids: manifest.rejected_feature_ids,
        worker_files: manifest.worker_files,
        integrator_file: Some(manifest.integrator_file),
        reviewer_file: Some(manifest.reviewer_file),
        findings,
    }
}

#[derive(Debug)]
struct FileValidationResult {
    invalid: bool,
    incomplete: bool,
    findings: Vec<PacketArchiveIndexFinding>,
}

fn validate_archived_file(
    file: &ArchivedPacketFile,
    expected_role: &str,
    require_lane: bool,
    require_feature: bool,
) -> FileValidationResult {
    let mut findings = Vec::new();
    let mut invalid = false;
    let mut incomplete = false;

    if file.role != expected_role {
        invalid = true;
        findings.push(PacketArchiveIndexFinding {
            severity: PacketArchiveIndexSeverity::Error,
            code: "invalid_file_role".into(),
            path: file.json_path.clone(),
            message: format!("Expected role {expected_role} but found {}.", file.role),
        });
    }
    if require_lane && file.lane_name.as_deref().unwrap_or("").trim().is_empty() {
        invalid = true;
        findings.push(PacketArchiveIndexFinding {
            severity: PacketArchiveIndexSeverity::Error,
            code: "missing_lane_name".into(),
            path: file.json_path.clone(),
            message: "Worker archive file is missing lane_name.".into(),
        });
    }
    if require_feature && file.feature_id.as_deref().unwrap_or("").trim().is_empty() {
        invalid = true;
        findings.push(PacketArchiveIndexFinding {
            severity: PacketArchiveIndexSeverity::Error,
            code: "missing_feature_id".into(),
            path: file.json_path.clone(),
            message: "Worker archive file is missing feature_id.".into(),
        });
    }

    for (path_label, path_value) in [
        ("markdown_path", file.markdown_path.as_str()),
        ("json_path", file.json_path.as_str()),
    ] {
        let path = Path::new(path_value);
        if !path.is_absolute() {
            invalid = true;
            findings.push(PacketArchiveIndexFinding {
                severity: PacketArchiveIndexSeverity::Error,
                code: "non_absolute_packet_path".into(),
                path: path_value.into(),
                message: format!("{path_label} must be absolute."),
            });
            continue;
        }
        if !path.exists() {
            incomplete = true;
            findings.push(PacketArchiveIndexFinding {
                severity: PacketArchiveIndexSeverity::Warning,
                code: "missing_packet_file".into(),
                path: path_value.into(),
                message: format!("Referenced {path_label} does not exist."),
            });
        }
    }

    FileValidationResult {
        invalid,
        incomplete,
        findings,
    }
}

fn directory_validation(
    code: &str,
    error: std::io::Error,
    path: &Path,
) -> PacketArchiveIndexValidation {
    PacketArchiveIndexValidation {
        findings: vec![PacketArchiveIndexFinding {
            severity: PacketArchiveIndexSeverity::Error,
            code: code.into(),
            path: stringify_path(path),
            message: error.to_string(),
        }],
    }
}

fn stringify_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
