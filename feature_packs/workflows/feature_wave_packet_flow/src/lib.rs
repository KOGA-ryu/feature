use std::collections::BTreeMap;
use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use prompt_generator::{
    GeneratedPromptPacket, PromptGenerationFinding, PromptGenerationSeverity, PromptRequest,
    PromptTargetKind, generate_prompt,
};
use serde::{Deserialize, Serialize};
use spec_selector::{SelectedFeatureDecision, SpecSelectionFinding, SpecSelectionResult};

pub const FEATURE_ID: &str = "workflow.feature_wave_packet_flow";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureWavePacketInput {
    pub project: Option<String>,
    pub branch_name: Option<String>,
    pub max_workers: usize,
    pub selection_result: Option<SpecSelectionResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureWaveWorkerBundle {
    pub feature_id: String,
    pub selection_reason: String,
    pub packet: GeneratedPromptPacket,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureWaveLaneAssignment {
    pub lane_name: String,
    pub feature_id: String,
    pub allowed_writes: Vec<String>,
    pub forbidden_writes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureWavePacketBundle {
    pub project: String,
    pub branch_name: String,
    pub max_workers: usize,
    pub wave_feature_ids: Vec<String>,
    pub worker_bundles: Vec<FeatureWaveWorkerBundle>,
    pub integrator_packet: GeneratedPromptPacket,
    pub reviewer_packet: GeneratedPromptPacket,
    pub lane_assignments: Vec<FeatureWaveLaneAssignment>,
    pub deferred_feature_ids: Vec<String>,
    pub rejected_feature_ids: Vec<String>,
    pub upstream_findings: Vec<SpecSelectionFinding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureWavePacketSeverity {
    Error,
    Warning,
}

impl fmt::Display for FeatureWavePacketSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureWavePacketFinding {
    pub severity: FeatureWavePacketSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureWavePacketValidation {
    pub findings: Vec<FeatureWavePacketFinding>,
}

impl FeatureWavePacketValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == FeatureWavePacketSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureWavePacketFlow;

impl FeatureWavePacketFlow {
    pub fn validate(&self, input: &FeatureWavePacketInput) -> FeatureWavePacketValidation {
        validate_feature_wave_packet_input(input)
    }

    pub fn build(
        &self,
        input: FeatureWavePacketInput,
    ) -> Result<FeatureWavePacketBundle, FeatureWavePacketValidation> {
        build_feature_wave_packets(input)
    }
}

pub fn build_feature_wave_packets(
    input: FeatureWavePacketInput,
) -> Result<FeatureWavePacketBundle, FeatureWavePacketValidation> {
    let validation = validate_feature_wave_packet_input(&input);
    if !validation.is_valid() {
        return Err(validation);
    }

    let project = required_string(input.project);
    let branch_name = required_string(input.branch_name);
    let max_workers = input.max_workers;
    let selection_result = input
        .selection_result
        .expect("validated selection_result should exist");
    let generated_spec = selection_result.generated_spec.clone();
    let ordered_selected = ordered_selected_decisions(&selection_result);

    let worker_bundles = ordered_selected
        .iter()
        .take(max_workers)
        .map(|decision| {
            let feature_id = decision.feature_id.clone();
            let packet = generate_prompt(PromptRequest {
                target_kind: PromptTargetKind::FeatureWorker,
                spec: Some(generated_spec.clone()),
                target_feature_id: Some(feature_id.clone()),
                wave_feature_ids: Vec::new(),
                allow_feature_lab_ui_writes: false,
            })
            .map_err(convert_prompt_validation)?;
            Ok(FeatureWaveWorkerBundle {
                feature_id,
                selection_reason: decision.reasons.join("\n"),
                packet,
            })
        })
        .collect::<Result<Vec<_>, FeatureWavePacketValidation>>()?;
    let wave_feature_ids = worker_bundles
        .iter()
        .map(|bundle| bundle.feature_id.clone())
        .collect::<Vec<_>>();
    let deferred_feature_ids = ordered_selected
        .iter()
        .skip(max_workers)
        .map(|decision| decision.feature_id.clone())
        .collect::<Vec<_>>();
    let lane_assignments = worker_bundles
        .iter()
        .enumerate()
        .map(|(index, bundle)| FeatureWaveLaneAssignment {
            lane_name: format!("lane_{:02}", index + 1),
            feature_id: bundle.feature_id.clone(),
            allowed_writes: bundle.packet.allowed_writes.clone(),
            forbidden_writes: bundle.packet.forbidden_writes.clone(),
        })
        .collect::<Vec<_>>();

    let integrator_packet = generate_prompt(PromptRequest {
        target_kind: PromptTargetKind::Integrator,
        spec: Some(generated_spec.clone()),
        target_feature_id: None,
        wave_feature_ids: wave_feature_ids.clone(),
        allow_feature_lab_ui_writes: false,
    })
    .map_err(convert_prompt_validation)?;
    let reviewer_packet = generate_prompt(PromptRequest {
        target_kind: PromptTargetKind::Reviewer,
        spec: Some(generated_spec),
        target_feature_id: None,
        wave_feature_ids: wave_feature_ids.clone(),
        allow_feature_lab_ui_writes: false,
    })
    .map_err(convert_prompt_validation)?;

    Ok(FeatureWavePacketBundle {
        project,
        branch_name,
        max_workers,
        wave_feature_ids,
        worker_bundles,
        integrator_packet,
        reviewer_packet,
        lane_assignments,
        deferred_feature_ids,
        rejected_feature_ids: selection_result.rejected_feature_ids,
        upstream_findings: selection_result.findings,
    })
}

pub fn validate_feature_wave_packet_input(
    input: &FeatureWavePacketInput,
) -> FeatureWavePacketValidation {
    let mut findings = Vec::new();

    validate_required_text(&mut findings, "project", input.project.as_deref());
    validate_required_text(&mut findings, "branch_name", input.branch_name.as_deref());
    if input.max_workers == 0 {
        findings.push(FeatureWavePacketFinding {
            severity: FeatureWavePacketSeverity::Error,
            code: "invalid_max_workers".into(),
            path: "max_workers".into(),
            message: "max_workers must be greater than zero.".into(),
        });
    }

    let Some(selection_result) = input.selection_result.as_ref() else {
        findings.push(FeatureWavePacketFinding {
            severity: FeatureWavePacketSeverity::Error,
            code: "missing_selection_result".into(),
            path: "selection_result".into(),
            message: "Feature wave packet flow requires a selection_result.".into(),
        });
        return FeatureWavePacketValidation { findings };
    };

    let selected_ids = selection_result
        .selected_feature_ids
        .iter()
        .enumerate()
        .map(|(index, feature_id)| (feature_id.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    let selected_decisions = selection_result
        .decisions
        .iter()
        .enumerate()
        .filter(|(_, decision)| decision.selected)
        .collect::<Vec<_>>();

    if selected_decisions.is_empty() {
        findings.push(FeatureWavePacketFinding {
            severity: FeatureWavePacketSeverity::Error,
            code: "no_selected_features".into(),
            path: "selection_result.decisions".into(),
            message: "Selection result does not contain any selected feature decisions.".into(),
        });
        return FeatureWavePacketValidation { findings };
    }

    for (index, decision) in &selected_decisions {
        if decision.score.is_none() {
            findings.push(FeatureWavePacketFinding {
                severity: FeatureWavePacketSeverity::Error,
                code: "selected_feature_missing_score".into(),
                path: format!("selection_result.decisions[{index}].score"),
                message: format!(
                    "Selected feature {} must include a numeric score.",
                    decision.feature_id
                ),
            });
        }
        if !selected_ids.contains_key(decision.feature_id.as_str()) {
            findings.push(FeatureWavePacketFinding {
                severity: FeatureWavePacketSeverity::Error,
                code: "selected_feature_not_in_selected_feature_ids".into(),
                path: format!("selection_result.decisions[{index}].feature_id"),
                message: format!(
                    "Selected feature {} must appear in selection_result.selected_feature_ids.",
                    decision.feature_id
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

    FeatureWavePacketValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_feature_wave_packet_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_feature_wave_packet_input.json")
}

pub fn parse_feature_wave_packet_input(raw: &str) -> Result<FeatureWavePacketInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_input() -> Result<FeatureWavePacketInput, String> {
    parse_feature_wave_packet_input(sample_valid_fixture())
}

pub fn sample_invalid_input() -> Result<FeatureWavePacketInput, String> {
    parse_feature_wave_packet_input(sample_invalid_fixture())
}

pub fn sample_valid_bundle() -> Result<FeatureWavePacketBundle, String> {
    build_feature_wave_packets(sample_valid_input()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn ordered_selected_decisions(
    selection_result: &SpecSelectionResult,
) -> Vec<&SelectedFeatureDecision> {
    let order = selection_result
        .selected_feature_ids
        .iter()
        .enumerate()
        .map(|(index, feature_id)| (feature_id.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    let mut decisions = selection_result
        .decisions
        .iter()
        .filter(|decision| decision.selected)
        .filter(|decision| decision.score.is_some())
        .filter(|decision| order.contains_key(decision.feature_id.as_str()))
        .collect::<Vec<_>>();
    decisions.sort_by(|left, right| {
        order
            .get(left.feature_id.as_str())
            .copied()
            .unwrap_or(usize::MAX)
            .cmp(
                &order
                    .get(right.feature_id.as_str())
                    .copied()
                    .unwrap_or(usize::MAX),
            )
            .then_with(|| left.feature_id.cmp(&right.feature_id))
    });
    decisions
}

fn convert_prompt_validation(
    validation: prompt_generator::PromptGenerationValidation,
) -> FeatureWavePacketValidation {
    FeatureWavePacketValidation {
        findings: validation
            .findings
            .into_iter()
            .map(convert_prompt_finding)
            .collect(),
    }
}

fn convert_prompt_finding(finding: PromptGenerationFinding) -> FeatureWavePacketFinding {
    FeatureWavePacketFinding {
        severity: match finding.severity {
            PromptGenerationSeverity::Error => FeatureWavePacketSeverity::Error,
            PromptGenerationSeverity::Warning => FeatureWavePacketSeverity::Warning,
        },
        code: finding.code,
        path: finding.path,
        message: finding.message,
    }
}

fn validate_required_text(
    findings: &mut Vec<FeatureWavePacketFinding>,
    path: &str,
    value: Option<&str>,
) {
    let missing = value.map(|value| value.trim().is_empty()).unwrap_or(true);
    if missing {
        findings.push(FeatureWavePacketFinding {
            severity: FeatureWavePacketSeverity::Error,
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
