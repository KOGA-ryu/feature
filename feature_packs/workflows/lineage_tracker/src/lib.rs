use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use feature_extraction_flow::ExtractionPacket;
use prompt_generator::{GeneratedPromptPacket, PromptTargetKind};
use review_packet_flow::{MergeRecommendation, ReviewPacket};
use serde::{Deserialize, Serialize};
use spec_generator::{AppIdeaInput, GeneratedSpec};

pub const FEATURE_ID: &str = "workflow.lineage_tracker";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageStage {
    Idea,
    Spec,
    Prompt,
    Review,
    Extraction,
}

impl fmt::Display for LineageStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idea => formatter.write_str("idea"),
            Self::Spec => formatter.write_str("spec"),
            Self::Prompt => formatter.write_str("prompt"),
            Self::Review => formatter.write_str("review"),
            Self::Extraction => formatter.write_str("extraction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageStageStatus {
    Complete,
    Warning,
}

impl fmt::Display for LineageStageStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Complete => formatter.write_str("complete"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageStageSummary {
    pub stage: LineageStage,
    pub status: LineageStageStatus,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageInput {
    pub project: Option<String>,
    pub idea_id: Option<String>,
    pub branch_name: Option<String>,
    pub app_idea: Option<AppIdeaInput>,
    pub generated_spec: Option<GeneratedSpec>,
    pub prompt_packets: Vec<GeneratedPromptPacket>,
    pub review_packet: Option<ReviewPacket>,
    pub extraction_packet: Option<ExtractionPacket>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageRecord {
    pub lineage_id: String,
    pub project: String,
    pub idea_id: String,
    pub branch_name: String,
    pub primary_feature_id: String,
    pub selected_feature_ids: Vec<String>,
    pub stage_summaries: Vec<LineageStageSummary>,
    pub prompt_target_kinds: Vec<PromptTargetKind>,
    pub files_changed: Vec<String>,
    pub proposed_library_paths: Vec<String>,
    pub merge_recommendation: Option<MergeRecommendation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageSeverity {
    Error,
    Warning,
}

impl fmt::Display for LineageSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageFinding {
    pub severity: LineageSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageValidation {
    pub findings: Vec<LineageFinding>,
}

impl LineageValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == LineageSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineageTracker;

impl LineageTracker {
    pub fn validate(&self, input: &LineageInput) -> LineageValidation {
        validate_lineage_input(input)
    }

    pub fn track(&self, input: LineageInput) -> Result<LineageRecord, LineageValidation> {
        track_lineage(input)
    }
}

pub fn track_lineage(input: LineageInput) -> Result<LineageRecord, LineageValidation> {
    let validation = validate_lineage_input(&input);
    if !validation.is_valid() {
        return Err(validation);
    }

    let stage_summaries = build_stage_summaries(&input);
    let project = required_string(input.project);
    let idea_id = required_string(input.idea_id);
    let branch_name = required_string(input.branch_name);
    let app_idea = input.app_idea.expect("validated app idea should exist");
    let selected_feature_ids = input
        .generated_spec
        .as_ref()
        .map(|spec| {
            spec.selected_features
                .iter()
                .map(|feature| feature.feature_id.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let prompt_target_kinds = input
        .prompt_packets
        .iter()
        .map(|packet| packet.target_kind)
        .collect::<Vec<_>>();
    let files_changed = input
        .review_packet
        .as_ref()
        .map(|packet| packet.files_changed.clone())
        .unwrap_or_default();
    let proposed_library_paths = input
        .extraction_packet
        .as_ref()
        .map(|packet| packet.proposed_library_paths.clone())
        .unwrap_or_default();
    let merge_recommendation = input
        .review_packet
        .as_ref()
        .map(|packet| packet.merge_recommendation);
    let primary_feature_id = derive_primary_feature_id(
        &app_idea,
        input.generated_spec.as_ref(),
        input.review_packet.as_ref(),
        input.extraction_packet.as_ref(),
    );

    Ok(LineageRecord {
        lineage_id: format!("{project}:{idea_id}:{branch_name}"),
        project,
        idea_id,
        branch_name,
        primary_feature_id,
        selected_feature_ids,
        stage_summaries,
        prompt_target_kinds,
        files_changed,
        proposed_library_paths,
        merge_recommendation,
    })
}

pub fn validate_lineage_input(input: &LineageInput) -> LineageValidation {
    let mut findings = Vec::new();

    validate_required_text(&mut findings, "project", input.project.as_deref());
    validate_required_text(&mut findings, "idea_id", input.idea_id.as_deref());
    validate_required_text(&mut findings, "branch_name", input.branch_name.as_deref());

    if input.app_idea.is_none() {
        findings.push(LineageFinding {
            severity: LineageSeverity::Error,
            code: "missing_app_idea".into(),
            path: "app_idea".into(),
            message: "Lineage tracking requires the root app_idea artifact.".into(),
        });
    }

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    LineageValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_complete_fixture() -> &'static str {
    include_str!("../fixtures/complete_lineage_input.json")
}

pub fn sample_partial_fixture() -> &'static str {
    include_str!("../fixtures/partial_lineage_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_lineage_input.json")
}

pub fn parse_lineage_input(raw: &str) -> Result<LineageInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_complete_input() -> Result<LineageInput, String> {
    parse_lineage_input(sample_complete_fixture())
}

pub fn sample_partial_input() -> Result<LineageInput, String> {
    parse_lineage_input(sample_partial_fixture())
}

pub fn sample_invalid_input() -> Result<LineageInput, String> {
    parse_lineage_input(sample_invalid_fixture())
}

pub fn sample_lineage_record() -> Result<LineageRecord, String> {
    track_lineage(sample_complete_input()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn build_stage_summaries(input: &LineageInput) -> Vec<LineageStageSummary> {
    vec![
        LineageStageSummary {
            stage: LineageStage::Idea,
            status: LineageStageStatus::Complete,
            message: "App idea is present.".into(),
        },
        stage_summary(
            LineageStage::Spec,
            input.generated_spec.is_some(),
            "Generated spec is present.",
            "Generated spec is missing; downstream lineage is partial.",
        ),
        stage_summary(
            LineageStage::Prompt,
            !input.prompt_packets.is_empty(),
            "Prompt packets are present.",
            "Prompt packets are missing; downstream handoff is not recorded.",
        ),
        stage_summary(
            LineageStage::Review,
            input.review_packet.is_some(),
            "Review packet is present.",
            "Review packet is missing; files changed and merge recommendation are unavailable.",
        ),
        stage_summary(
            LineageStage::Extraction,
            input.extraction_packet.is_some(),
            "Extraction packet is present.",
            "Extraction packet is missing; proposed library paths are unavailable.",
        ),
    ]
}

fn stage_summary(
    stage: LineageStage,
    is_complete: bool,
    complete_message: &str,
    warning_message: &str,
) -> LineageStageSummary {
    LineageStageSummary {
        stage,
        status: if is_complete {
            LineageStageStatus::Complete
        } else {
            LineageStageStatus::Warning
        },
        message: if is_complete {
            complete_message.into()
        } else {
            warning_message.into()
        },
    }
}

fn derive_primary_feature_id(
    app_idea: &AppIdeaInput,
    generated_spec: Option<&GeneratedSpec>,
    review_packet: Option<&ReviewPacket>,
    extraction_packet: Option<&ExtractionPacket>,
) -> String {
    if let Some(packet) = review_packet {
        return packet.feature_id.clone();
    }
    if let Some(packet) = extraction_packet {
        if let Some(feature_id) = feature_id_from_extraction_packet(packet) {
            return feature_id;
        }
    }
    if let Some(spec) = generated_spec {
        if let Some(feature) = spec.selected_features.first() {
            return feature.feature_id.clone();
        }
    }
    app_idea
        .selected_features
        .first()
        .map(|feature| feature.feature_id.clone())
        .unwrap_or_default()
}

fn feature_id_from_extraction_packet(packet: &ExtractionPacket) -> Option<String> {
    packet
        .proposed_library_items
        .first()
        .and_then(|item| item.title.strip_suffix(" blueprint"))
        .map(str::to_string)
        .or_else(|| {
            let prefix = " for ";
            let suffix = " from project ";
            let start = packet.feature_summary.rfind(prefix)?;
            let feature_slice = &packet.feature_summary[start + prefix.len()..];
            let end = feature_slice.find(suffix)?;
            Some(feature_slice[..end].to_string())
        })
}

fn validate_required_text(findings: &mut Vec<LineageFinding>, path: &str, value: Option<&str>) {
    let missing = value.map(|value| value.trim().is_empty()).unwrap_or(true);
    if missing {
        findings.push(LineageFinding {
            severity: LineageSeverity::Error,
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
