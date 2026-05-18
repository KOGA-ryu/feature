use std::collections::BTreeMap;
use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use prompt_generator::{
    GeneratedPromptPacket, PromptGenerationFinding, PromptGenerationSeverity, PromptRequest,
    PromptTargetKind, generate_prompt,
};
use serde::{Deserialize, Serialize};
use spec_selector::{SpecSelectionFinding, SpecSelectionResult};

pub const FEATURE_ID: &str = "workflow.feature_build_packet_flow";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBuildPacketInput {
    pub project: Option<String>,
    pub branch_name: Option<String>,
    pub selection_result: Option<SpecSelectionResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBuildPacketBundle {
    pub project: String,
    pub branch_name: String,
    pub selected_feature_id: String,
    pub selection_reason: String,
    pub worker_packet: GeneratedPromptPacket,
    pub integrator_packet: GeneratedPromptPacket,
    pub reviewer_packet: GeneratedPromptPacket,
    pub rejected_feature_ids: Vec<String>,
    pub upstream_findings: Vec<SpecSelectionFinding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureBuildPacketSeverity {
    Error,
    Warning,
}

impl fmt::Display for FeatureBuildPacketSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBuildPacketFinding {
    pub severity: FeatureBuildPacketSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureBuildPacketValidation {
    pub findings: Vec<FeatureBuildPacketFinding>,
}

impl FeatureBuildPacketValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == FeatureBuildPacketSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureBuildPacketFlow;

impl FeatureBuildPacketFlow {
    pub fn validate(&self, input: &FeatureBuildPacketInput) -> FeatureBuildPacketValidation {
        validate_feature_build_packet_input(input)
    }

    pub fn build(
        &self,
        input: FeatureBuildPacketInput,
    ) -> Result<FeatureBuildPacketBundle, FeatureBuildPacketValidation> {
        build_feature_packets(input)
    }
}

pub fn build_feature_packets(
    input: FeatureBuildPacketInput,
) -> Result<FeatureBuildPacketBundle, FeatureBuildPacketValidation> {
    let validation = validate_feature_build_packet_input(&input);
    if !validation.is_valid() {
        return Err(validation);
    }

    let project = required_string(input.project);
    let branch_name = required_string(input.branch_name);
    let selection_result = input
        .selection_result
        .expect("validated selection_result should exist");
    let selected_feature_id = choose_selected_feature_id(&selection_result)
        .expect("validated selected feature should exist");
    let selected_decision = selection_result
        .decisions
        .iter()
        .find(|decision| decision.feature_id == selected_feature_id)
        .expect("validated decision should exist");
    let selection_reason = selected_decision.reasons.join("\n");
    let generated_spec = selection_result.generated_spec.clone();

    let worker_packet = generate_prompt(PromptRequest {
        target_kind: PromptTargetKind::FeatureWorker,
        spec: Some(generated_spec.clone()),
        target_feature_id: Some(selected_feature_id.clone()),
        wave_feature_ids: Vec::new(),
        allow_feature_lab_ui_writes: false,
    })
    .map_err(convert_prompt_validation)?;
    let integrator_packet = generate_prompt(PromptRequest {
        target_kind: PromptTargetKind::Integrator,
        spec: Some(generated_spec.clone()),
        target_feature_id: None,
        wave_feature_ids: vec![selected_feature_id.clone()],
        allow_feature_lab_ui_writes: false,
    })
    .map_err(convert_prompt_validation)?;
    let reviewer_packet = generate_prompt(PromptRequest {
        target_kind: PromptTargetKind::Reviewer,
        spec: Some(generated_spec),
        target_feature_id: None,
        wave_feature_ids: vec![selected_feature_id.clone()],
        allow_feature_lab_ui_writes: false,
    })
    .map_err(convert_prompt_validation)?;

    Ok(FeatureBuildPacketBundle {
        project,
        branch_name,
        selected_feature_id,
        selection_reason,
        worker_packet,
        integrator_packet,
        reviewer_packet,
        rejected_feature_ids: selection_result.rejected_feature_ids,
        upstream_findings: selection_result.findings,
    })
}

pub fn validate_feature_build_packet_input(
    input: &FeatureBuildPacketInput,
) -> FeatureBuildPacketValidation {
    let mut findings = Vec::new();

    validate_required_text(&mut findings, "project", input.project.as_deref());
    validate_required_text(&mut findings, "branch_name", input.branch_name.as_deref());

    let Some(selection_result) = input.selection_result.as_ref() else {
        findings.push(FeatureBuildPacketFinding {
            severity: FeatureBuildPacketSeverity::Error,
            code: "missing_selection_result".into(),
            path: "selection_result".into(),
            message: "Feature build packet flow requires a selection_result.".into(),
        });
        return FeatureBuildPacketValidation { findings };
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
        findings.push(FeatureBuildPacketFinding {
            severity: FeatureBuildPacketSeverity::Error,
            code: "no_selected_features".into(),
            path: "selection_result.decisions".into(),
            message: "Selection result does not contain any selected feature decisions.".into(),
        });
        return FeatureBuildPacketValidation { findings };
    }

    for (index, decision) in &selected_decisions {
        if decision.score.is_none() {
            findings.push(FeatureBuildPacketFinding {
                severity: FeatureBuildPacketSeverity::Error,
                code: "selected_feature_missing_score".into(),
                path: format!("selection_result.decisions[{index}].score"),
                message: format!(
                    "Selected feature {} must include a numeric score.",
                    decision.feature_id
                ),
            });
        }
        if !selected_ids.contains_key(decision.feature_id.as_str()) {
            findings.push(FeatureBuildPacketFinding {
                severity: FeatureBuildPacketSeverity::Error,
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

    FeatureBuildPacketValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_feature_build_packet_input.json")
}

pub fn sample_tie_break_fixture() -> &'static str {
    include_str!("../fixtures/tie_break_feature_build_packet_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_feature_build_packet_input.json")
}

pub fn parse_feature_build_packet_input(raw: &str) -> Result<FeatureBuildPacketInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_input() -> Result<FeatureBuildPacketInput, String> {
    parse_feature_build_packet_input(sample_valid_fixture())
}

pub fn sample_tie_break_input() -> Result<FeatureBuildPacketInput, String> {
    parse_feature_build_packet_input(sample_tie_break_fixture())
}

pub fn sample_invalid_input() -> Result<FeatureBuildPacketInput, String> {
    parse_feature_build_packet_input(sample_invalid_fixture())
}

pub fn sample_valid_bundle() -> Result<FeatureBuildPacketBundle, String> {
    build_feature_packets(sample_valid_input()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn choose_selected_feature_id(selection_result: &SpecSelectionResult) -> Option<String> {
    let order = selection_result
        .selected_feature_ids
        .iter()
        .enumerate()
        .map(|(index, feature_id)| (feature_id.as_str(), index))
        .collect::<BTreeMap<_, _>>();

    selection_result
        .decisions
        .iter()
        .filter(|decision| decision.selected)
        .filter_map(|decision| decision.score.map(|score| (decision, score)))
        .max_by(
            |(left_decision, left_score), (right_decision, right_score)| {
                left_score.cmp(right_score).then_with(|| {
                    let left_index = order
                        .get(left_decision.feature_id.as_str())
                        .copied()
                        .unwrap_or(usize::MAX);
                    let right_index = order
                        .get(right_decision.feature_id.as_str())
                        .copied()
                        .unwrap_or(usize::MAX);
                    right_index.cmp(&left_index)
                })
            },
        )
        .map(|(decision, _)| decision.feature_id.clone())
}

fn convert_prompt_validation(
    validation: prompt_generator::PromptGenerationValidation,
) -> FeatureBuildPacketValidation {
    FeatureBuildPacketValidation {
        findings: validation
            .findings
            .into_iter()
            .map(convert_prompt_finding)
            .collect(),
    }
}

fn convert_prompt_finding(finding: PromptGenerationFinding) -> FeatureBuildPacketFinding {
    FeatureBuildPacketFinding {
        severity: match finding.severity {
            PromptGenerationSeverity::Error => FeatureBuildPacketSeverity::Error,
            PromptGenerationSeverity::Warning => FeatureBuildPacketSeverity::Warning,
        },
        code: finding.code,
        path: finding.path,
        message: finding.message,
    }
}

fn validate_required_text(
    findings: &mut Vec<FeatureBuildPacketFinding>,
    path: &str,
    value: Option<&str>,
) {
    let missing = value.map(|value| value.trim().is_empty()).unwrap_or(true);
    if missing {
        findings.push(FeatureBuildPacketFinding {
            severity: FeatureBuildPacketSeverity::Error,
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
