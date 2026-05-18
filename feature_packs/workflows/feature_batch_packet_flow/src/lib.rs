use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use prompt_generator::{
    GeneratedPromptPacket, PromptGenerationFinding, PromptGenerationSeverity, PromptRequest,
    PromptTargetKind, generate_prompt,
};
use serde::{Deserialize, Serialize};
use spec_generator::{GeneratedSpec, SelectedFeatureRef};

pub const FEATURE_ID: &str = "workflow.feature_batch_packet_flow";
const REQUIRED_REVIEW_SECTIONS: [&str; 7] = [
    "files_changed",
    "commands_run",
    "tests_run",
    "risks",
    "known_issues",
    "follow_up_tasks",
    "merge_recommendation",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBatchPacketInput {
    pub project: Option<String>,
    pub branch_name: Option<String>,
    pub spec: Option<GeneratedSpec>,
    pub ordered_feature_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBatchStepReviewContract {
    pub task_id: String,
    pub feature_id: String,
    pub required_sections: Vec<String>,
    pub required_commands: Vec<String>,
    pub stop_on_failure: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBatchStepBundle {
    pub step_index: usize,
    pub feature_id: String,
    pub feature_summary: String,
    pub worker_packet: GeneratedPromptPacket,
    pub integrator_packet: GeneratedPromptPacket,
    pub reviewer_packet: GeneratedPromptPacket,
    pub review_contract: FeatureBatchStepReviewContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBatchPacketBundle {
    pub project: String,
    pub branch_name: String,
    pub ordered_feature_ids: Vec<String>,
    pub step_bundles: Vec<FeatureBatchStepBundle>,
    pub deferred_feature_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureBatchPacketSeverity {
    Error,
    Warning,
}

impl fmt::Display for FeatureBatchPacketSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBatchPacketFinding {
    pub severity: FeatureBatchPacketSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBatchPacketValidation {
    pub findings: Vec<FeatureBatchPacketFinding>,
}

impl FeatureBatchPacketValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == FeatureBatchPacketSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureBatchPacketFlow;

impl FeatureBatchPacketFlow {
    pub fn validate(&self, input: &FeatureBatchPacketInput) -> FeatureBatchPacketValidation {
        validate_feature_batch_packet_input(input)
    }

    pub fn build(
        &self,
        input: FeatureBatchPacketInput,
    ) -> Result<FeatureBatchPacketBundle, FeatureBatchPacketValidation> {
        build_feature_batch_packets(input)
    }
}

pub fn build_feature_batch_packets(
    input: FeatureBatchPacketInput,
) -> Result<FeatureBatchPacketBundle, FeatureBatchPacketValidation> {
    let validation = validate_feature_batch_packet_input(&input);
    if !validation.is_valid() {
        return Err(validation);
    }

    let project = required_string(input.project);
    let branch_name = required_string(input.branch_name);
    let spec = input.spec.expect("validated spec should exist");
    let ordered_feature_ids = input
        .ordered_feature_ids
        .into_iter()
        .map(|feature_id| feature_id.trim().to_string())
        .collect::<Vec<_>>();
    let feature_lookup = selected_feature_lookup(&spec);
    let step_total = ordered_feature_ids.len();
    let step_width = step_total.to_string().len().max(2);

    let step_bundles = ordered_feature_ids
        .iter()
        .enumerate()
        .map(|(index, feature_id)| {
            let step_index = index + 1;
            let feature_summary = feature_lookup
                .get(feature_id.as_str())
                .map(|feature| feature.summary.clone())
                .unwrap_or_default();

            let worker_packet = wrap_step_packet(
                generate_prompt(PromptRequest {
                    target_kind: PromptTargetKind::FeatureWorker,
                    spec: Some(spec.clone()),
                    target_feature_id: Some(feature_id.clone()),
                    wave_feature_ids: Vec::new(),
                    allow_feature_lab_ui_writes: false,
                })
                .map_err(convert_prompt_validation)?,
                step_index,
                step_total,
                feature_id,
            );
            let integrator_packet = wrap_step_packet(
                generate_prompt(PromptRequest {
                    target_kind: PromptTargetKind::Integrator,
                    spec: Some(spec.clone()),
                    target_feature_id: None,
                    wave_feature_ids: vec![feature_id.clone()],
                    allow_feature_lab_ui_writes: true,
                })
                .map_err(convert_prompt_validation)?,
                step_index,
                step_total,
                feature_id,
            );
            let reviewer_packet = wrap_step_packet(
                generate_prompt(PromptRequest {
                    target_kind: PromptTargetKind::Reviewer,
                    spec: Some(spec.clone()),
                    target_feature_id: None,
                    wave_feature_ids: vec![feature_id.clone()],
                    allow_feature_lab_ui_writes: false,
                })
                .map_err(convert_prompt_validation)?,
                step_index,
                step_total,
                feature_id,
            );
            let review_contract = build_review_contract(
                step_index,
                step_width,
                feature_id,
                &worker_packet,
                &integrator_packet,
                &reviewer_packet,
            );

            Ok(FeatureBatchStepBundle {
                step_index,
                feature_id: feature_id.clone(),
                feature_summary,
                worker_packet,
                integrator_packet,
                reviewer_packet,
                review_contract,
            })
        })
        .collect::<Result<Vec<_>, FeatureBatchPacketValidation>>()?;

    let ordered_set = ordered_feature_ids.iter().cloned().collect::<BTreeSet<_>>();
    let deferred_feature_ids = spec
        .selected_features
        .iter()
        .filter(|feature| !ordered_set.contains(feature.feature_id.as_str()))
        .map(|feature| feature.feature_id.clone())
        .collect::<Vec<_>>();

    Ok(FeatureBatchPacketBundle {
        project,
        branch_name,
        ordered_feature_ids,
        step_bundles,
        deferred_feature_ids,
    })
}

pub fn validate_feature_batch_packet_input(
    input: &FeatureBatchPacketInput,
) -> FeatureBatchPacketValidation {
    let mut findings = Vec::new();

    validate_required_text(&mut findings, "project", input.project.as_deref());
    validate_required_text(&mut findings, "branch_name", input.branch_name.as_deref());

    let Some(spec) = input.spec.as_ref() else {
        findings.push(FeatureBatchPacketFinding {
            severity: FeatureBatchPacketSeverity::Error,
            code: "missing_spec".into(),
            path: "spec".into(),
            message: "Feature batch packet flow requires a generated spec.".into(),
        });
        return FeatureBatchPacketValidation { findings };
    };

    if input.ordered_feature_ids.is_empty() {
        findings.push(FeatureBatchPacketFinding {
            severity: FeatureBatchPacketSeverity::Error,
            code: "missing_ordered_feature_ids".into(),
            path: "ordered_feature_ids".into(),
            message: "ordered_feature_ids must contain at least one queued feature id.".into(),
        });
    }

    let selected_lookup = selected_feature_lookup(spec);
    let mut seen = BTreeSet::new();
    for (index, feature_id) in input.ordered_feature_ids.iter().enumerate() {
        let trimmed_feature_id = feature_id.trim();
        let path = format!("ordered_feature_ids[{index}]");
        if trimmed_feature_id.is_empty() {
            findings.push(FeatureBatchPacketFinding {
                severity: FeatureBatchPacketSeverity::Error,
                code: "blank_feature_id".into(),
                path: path.clone(),
                message: "Queued feature ids must not be blank.".into(),
            });
            continue;
        }
        if !seen.insert(trimmed_feature_id.to_string()) {
            findings.push(FeatureBatchPacketFinding {
                severity: FeatureBatchPacketSeverity::Error,
                code: "duplicate_feature_id".into(),
                path: path.clone(),
                message: format!("Queued feature id {trimmed_feature_id} appears more than once."),
            });
        }
        if !selected_lookup.contains_key(trimmed_feature_id) {
            findings.push(FeatureBatchPacketFinding {
                severity: FeatureBatchPacketSeverity::Error,
                code: "queued_feature_missing_from_spec".into(),
                path,
                message: format!(
                    "Queued feature id {trimmed_feature_id} must appear in spec.selected_features."
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

    FeatureBatchPacketValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_feature_batch_packet_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_feature_batch_packet_input.json")
}

pub fn parse_feature_batch_packet_input(raw: &str) -> Result<FeatureBatchPacketInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_input() -> Result<FeatureBatchPacketInput, String> {
    parse_feature_batch_packet_input(sample_valid_fixture())
}

pub fn sample_invalid_input() -> Result<FeatureBatchPacketInput, String> {
    parse_feature_batch_packet_input(sample_invalid_fixture())
}

pub fn sample_valid_bundle() -> Result<FeatureBatchPacketBundle, String> {
    build_feature_batch_packets(sample_valid_input()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn build_review_contract(
    step_index: usize,
    step_width: usize,
    feature_id: &str,
    worker_packet: &GeneratedPromptPacket,
    integrator_packet: &GeneratedPromptPacket,
    reviewer_packet: &GeneratedPromptPacket,
) -> FeatureBatchStepReviewContract {
    let required_commands = [
        &worker_packet.verification_commands,
        &integrator_packet.verification_commands,
        &reviewer_packet.verification_commands,
    ]
    .into_iter()
    .flatten()
    .fold(Vec::new(), |mut commands, command| {
        if !commands.iter().any(|existing| existing == command) {
            commands.push(command.clone());
        }
        commands
    });

    FeatureBatchStepReviewContract {
        task_id: format!(
            "task.batch_{:0width$}.{}",
            step_index,
            feature_id.replace('.', "_"),
            width = step_width
        ),
        feature_id: feature_id.to_string(),
        required_sections: REQUIRED_REVIEW_SECTIONS
            .iter()
            .map(|section| section.to_string())
            .collect(),
        required_commands,
        stop_on_failure: true,
    }
}

fn wrap_step_packet(
    mut packet: GeneratedPromptPacket,
    step_index: usize,
    step_total: usize,
    feature_id: &str,
) -> GeneratedPromptPacket {
    let header = format!(
        "Step {step_index}/{step_total}\nFeature:\n{feature_id}\n\nContinue to the next feature only after worker proof, integration proof, reviewer proof, and review-packet receipt succeed.\nStop on blocker.\n\n"
    );
    packet.title = format!("Step {step_index}/{step_total} {}", packet.title);
    packet.prompt_text = format!("{header}{}", packet.prompt_text);
    packet
}

fn selected_feature_lookup(spec: &GeneratedSpec) -> BTreeMap<&str, &SelectedFeatureRef> {
    spec.selected_features
        .iter()
        .map(|feature| (feature.feature_id.as_str(), feature))
        .collect::<BTreeMap<_, _>>()
}

fn convert_prompt_validation(
    validation: prompt_generator::PromptGenerationValidation,
) -> FeatureBatchPacketValidation {
    FeatureBatchPacketValidation {
        findings: validation
            .findings
            .into_iter()
            .map(convert_prompt_finding)
            .collect(),
    }
}

fn convert_prompt_finding(finding: PromptGenerationFinding) -> FeatureBatchPacketFinding {
    FeatureBatchPacketFinding {
        severity: match finding.severity {
            PromptGenerationSeverity::Error => FeatureBatchPacketSeverity::Error,
            PromptGenerationSeverity::Warning => FeatureBatchPacketSeverity::Warning,
        },
        code: finding.code,
        path: finding.path,
        message: finding.message,
    }
}

fn validate_required_text(
    findings: &mut Vec<FeatureBatchPacketFinding>,
    path: &str,
    value: Option<&str>,
) {
    let missing = value.map(|value| value.trim().is_empty()).unwrap_or(true);
    if missing {
        findings.push(FeatureBatchPacketFinding {
            severity: FeatureBatchPacketSeverity::Error,
            code: "missing_required_field".into(),
            path: path.into(),
            message: format!("{path} is required and must not be blank."),
        });
    }
}

fn required_string(value: Option<String>) -> String {
    value
        .expect("validated string should exist")
        .trim()
        .to_string()
}
