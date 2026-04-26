use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use review_packet_flow::{ReviewPacket, ReviewPacketDraft, build_review_packet};
use serde::{Deserialize, Serialize};
use spec_generator::GeneratedSpec;

pub const FEATURE_ID: &str = "workflow.feature_extraction_flow";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionInput {
    pub feature_id: Option<String>,
    pub generated_spec: Option<GeneratedSpec>,
    pub review_packet: Option<ReviewPacket>,
    pub review_packet_draft: Option<ReviewPacketDraft>,
    pub files_changed: Vec<String>,
    pub reusable_discoveries: Vec<String>,
    pub failure_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposedLibraryItem {
    pub path: String,
    pub title: String,
    pub kind: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionPacket {
    pub feature_summary: String,
    pub proposed_blueprint: String,
    pub proposed_contract_notes: Vec<String>,
    pub proposed_test_notes: Vec<String>,
    pub proposed_failure_modes: Vec<String>,
    pub proposed_library_paths: Vec<String>,
    pub proposed_library_items: Vec<ProposedLibraryItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionSeverity {
    Error,
    Warning,
}

impl fmt::Display for ExtractionSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionFinding {
    pub severity: ExtractionSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionValidation {
    pub findings: Vec<ExtractionFinding>,
}

impl ExtractionValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == ExtractionSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureExtractionFlow;

impl FeatureExtractionFlow {
    pub fn validate(&self, input: &ExtractionInput) -> ExtractionValidation {
        validate_extraction_input(input)
    }

    pub fn extract(
        &self,
        input: ExtractionInput,
    ) -> Result<ExtractionPacket, ExtractionValidation> {
        extract_feature(input)
    }
}

pub fn extract_feature(input: ExtractionInput) -> Result<ExtractionPacket, ExtractionValidation> {
    let validation = validate_extraction_input(&input);
    if !validation.is_valid() {
        return Err(validation);
    }

    let review_packet =
        resolve_review_packet(&input).expect("validated review packet should resolve");
    let feature_id = input
        .feature_id
        .expect("validated feature id should exist")
        .trim()
        .to_string();
    let generated_spec = input
        .generated_spec
        .expect("validated generated spec should exist");
    let feature_name = feature_id
        .split_once('.')
        .map(|(_, name)| name)
        .unwrap_or(feature_id.as_str());

    let proposed_library_paths = vec![
        format!("library/specs/{feature_name}_blueprint.md"),
        format!("library/references/{feature_name}_contract_notes.md"),
        format!("library/references/{feature_name}_failure_modes.md"),
    ];
    let proposed_library_items = vec![
        ProposedLibraryItem {
            path: proposed_library_paths[0].clone(),
            title: format!("{feature_id} blueprint"),
            kind: "blueprint".into(),
            summary: generated_spec.product_intent.clone(),
        },
        ProposedLibraryItem {
            path: proposed_library_paths[1].clone(),
            title: format!("{feature_id} contract notes"),
            kind: "contract_notes".into(),
            summary: format!("Derived from {} changed files.", input.files_changed.len()),
        },
        ProposedLibraryItem {
            path: proposed_library_paths[2].clone(),
            title: format!("{feature_id} failure modes"),
            kind: "failure_modes".into(),
            summary: "Normalized from review risks, known issues, and failure notes.".into(),
        },
    ];

    Ok(ExtractionPacket {
        feature_summary: format!(
            "{} for {} from project {}.",
            generated_spec.product_intent, feature_id, review_packet.project
        ),
        proposed_blueprint: format!(
            "Feature {}\nAudience: {}\nSelected features:\n- {}\nFirst vertical slice:\n- {}",
            feature_id,
            generated_spec.audience,
            generated_spec
                .selected_features
                .iter()
                .map(|feature| feature.feature_id.as_str())
                .collect::<Vec<_>>()
                .join("\n- "),
            generated_spec.first_vertical_slice.join("\n- ")
        ),
        proposed_contract_notes: vec![
            format!("Source review packet task: {}", review_packet.task_id),
            format!("Files changed: {}", input.files_changed.join(", ")),
            format!(
                "Selected logic: {}",
                generated_spec.selected_logic.join(", ")
            ),
        ],
        proposed_test_notes: review_packet
            .tests_run
            .iter()
            .map(|test_run| {
                format!(
                    "{} [{}] {}",
                    test_run.command, test_run.status, test_run.details
                )
            })
            .collect(),
        proposed_failure_modes: normalize_failure_modes(
            &review_packet.risks,
            &review_packet.known_issues,
            &input.failure_notes,
        ),
        proposed_library_paths,
        proposed_library_items,
    })
}

pub fn validate_extraction_input(input: &ExtractionInput) -> ExtractionValidation {
    let mut findings = Vec::new();

    validate_required_text(&mut findings, "feature_id", input.feature_id.as_deref());

    if input.generated_spec.is_none() {
        findings.push(ExtractionFinding {
            severity: ExtractionSeverity::Error,
            code: "missing_generated_spec".into(),
            path: "generated_spec".into(),
            message: "Feature extraction requires a generated spec.".into(),
        });
    }

    if input.files_changed.is_empty() {
        findings.push(ExtractionFinding {
            severity: ExtractionSeverity::Error,
            code: "missing_files_changed".into(),
            path: "files_changed".into(),
            message: "Feature extraction requires at least one changed file.".into(),
        });
    }

    match resolve_review_packet(input) {
        Ok(_) => {}
        Err(validation) => findings.extend(validation.findings),
    }

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    ExtractionValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_extraction_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_extraction_input.json")
}

pub fn parse_extraction_input(raw: &str) -> Result<ExtractionInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_input() -> Result<ExtractionInput, String> {
    parse_extraction_input(sample_valid_fixture())
}

pub fn sample_invalid_input() -> Result<ExtractionInput, String> {
    parse_extraction_input(sample_invalid_fixture())
}

pub fn sample_extraction_packet() -> Result<ExtractionPacket, String> {
    extract_feature(sample_valid_input()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn resolve_review_packet(input: &ExtractionInput) -> Result<ReviewPacket, ExtractionValidation> {
    if let Some(packet) = input.review_packet.clone() {
        return Ok(packet);
    }

    let Some(draft) = input.review_packet_draft.clone() else {
        return Err(ExtractionValidation {
            findings: vec![ExtractionFinding {
                severity: ExtractionSeverity::Error,
                code: "missing_review_packet".into(),
                path: "review_packet".into(),
                message: "Feature extraction requires a source review packet.".into(),
            }],
        });
    };

    build_review_packet(draft).map_err(|validation| ExtractionValidation {
        findings: validation
            .findings
            .into_iter()
            .map(|finding| ExtractionFinding {
                severity: ExtractionSeverity::Error,
                code: "invalid_review_packet".into(),
                path: format!("review_packet.{}", finding.path),
                message: finding.message,
            })
            .collect(),
    })
}

fn validate_required_text(findings: &mut Vec<ExtractionFinding>, path: &str, value: Option<&str>) {
    let missing = value.map(|value| value.trim().is_empty()).unwrap_or(true);
    if missing {
        findings.push(ExtractionFinding {
            severity: ExtractionSeverity::Error,
            code: "missing_required_field".into(),
            path: path.into(),
            message: format!("{path} is required and must not be blank."),
        });
    }
}

fn normalize_failure_modes(
    risks: &[String],
    known_issues: &[String],
    failure_notes: &[String],
) -> Vec<String> {
    let mut combined = risks
        .iter()
        .chain(known_issues.iter())
        .chain(failure_notes.iter())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    combined.sort();
    combined.dedup();
    combined
}
