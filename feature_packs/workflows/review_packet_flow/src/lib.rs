use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "workflow.review_packet_flow";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeRecommendation {
    Merge,
    MergeWithFollowups,
    DoNotMerge,
}

impl fmt::Display for MergeRecommendation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Merge => formatter.write_str("merge"),
            Self::MergeWithFollowups => formatter.write_str("merge_with_followups"),
            Self::DoNotMerge => formatter.write_str("do_not_merge"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRun {
    pub command: String,
    pub status: String,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPacketDraft {
    pub task_id: Option<String>,
    pub project: Option<String>,
    pub feature_id: Option<String>,
    pub generated_at: Option<String>,
    pub goal: Option<String>,
    pub files_changed: Option<Vec<String>>,
    pub commands_run: Option<Vec<String>>,
    pub tests_run: Option<Vec<TestRun>>,
    pub risks: Option<Vec<String>>,
    pub known_issues: Option<Vec<String>>,
    pub follow_up_tasks: Option<Vec<String>>,
    pub merge_recommendation: Option<MergeRecommendation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPacket {
    pub task_id: String,
    pub project: String,
    pub feature_id: String,
    pub generated_at: String,
    pub goal: String,
    pub files_changed: Vec<String>,
    pub commands_run: Vec<String>,
    pub tests_run: Vec<TestRun>,
    pub risks: Vec<String>,
    pub known_issues: Vec<String>,
    pub follow_up_tasks: Vec<String>,
    pub merge_recommendation: MergeRecommendation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPacketSeverity {
    Error,
    Warning,
}

impl fmt::Display for ReviewPacketSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPacketFinding {
    pub severity: ReviewPacketSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPacketValidation {
    pub findings: Vec<ReviewPacketFinding>,
}

impl ReviewPacketValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == ReviewPacketSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReviewPacketFlow;

impl ReviewPacketFlow {
    pub fn validate(&self, draft: &ReviewPacketDraft) -> ReviewPacketValidation {
        validate_review_packet_draft(draft)
    }

    pub fn build(&self, draft: ReviewPacketDraft) -> Result<ReviewPacket, ReviewPacketValidation> {
        build_review_packet(draft)
    }
}

pub fn validate_review_packet_draft(draft: &ReviewPacketDraft) -> ReviewPacketValidation {
    let mut findings = Vec::new();

    validate_required_text(&mut findings, "task_id", draft.task_id.as_deref());
    validate_required_text(&mut findings, "project", draft.project.as_deref());
    validate_required_text(&mut findings, "feature_id", draft.feature_id.as_deref());
    validate_timestamp(&mut findings, "generated_at", draft.generated_at.as_deref());
    validate_required_text(&mut findings, "goal", draft.goal.as_deref());
    validate_required_list(&mut findings, "files_changed", draft.files_changed.as_ref());
    validate_required_list(&mut findings, "commands_run", draft.commands_run.as_ref());
    validate_tests_run(&mut findings, draft.tests_run.as_ref());
    validate_required_list(&mut findings, "risks", draft.risks.as_ref());
    validate_required_list(&mut findings, "known_issues", draft.known_issues.as_ref());
    validate_required_list(
        &mut findings,
        "follow_up_tasks",
        draft.follow_up_tasks.as_ref(),
    );

    if draft.merge_recommendation.is_none() {
        findings.push(ReviewPacketFinding {
            severity: ReviewPacketSeverity::Error,
            code: "missing_required_section".into(),
            path: "merge_recommendation".into(),
            message: "Review packet draft is missing the merge_recommendation section.".into(),
        });
    }

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    ReviewPacketValidation { findings }
}

pub fn build_review_packet(
    draft: ReviewPacketDraft,
) -> Result<ReviewPacket, ReviewPacketValidation> {
    let validation = validate_review_packet_draft(&draft);
    if !validation.is_valid() {
        return Err(validation);
    }

    Ok(ReviewPacket {
        task_id: require_string(draft.task_id),
        project: require_string(draft.project),
        feature_id: require_string(draft.feature_id),
        generated_at: require_string(draft.generated_at),
        goal: require_string(draft.goal),
        files_changed: require_list(draft.files_changed),
        commands_run: require_list(draft.commands_run),
        tests_run: draft.tests_run.unwrap_or_default(),
        risks: require_list(draft.risks),
        known_issues: require_list(draft.known_issues),
        follow_up_tasks: require_list(draft.follow_up_tasks),
        merge_recommendation: draft
            .merge_recommendation
            .expect("validated merge recommendation should exist"),
    })
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_review_packet_draft.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_review_packet_draft.json")
}

pub fn parse_review_packet_draft(raw: &str) -> Result<ReviewPacketDraft, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_draft() -> Result<ReviewPacketDraft, String> {
    parse_review_packet_draft(sample_valid_fixture())
}

pub fn sample_invalid_draft() -> Result<ReviewPacketDraft, String> {
    parse_review_packet_draft(sample_invalid_fixture())
}

pub fn sample_valid_packet() -> Result<ReviewPacket, String> {
    build_review_packet(sample_valid_draft()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn validate_required_text(
    findings: &mut Vec<ReviewPacketFinding>,
    path: &str,
    value: Option<&str>,
) {
    let missing = value.map(|value| value.trim().is_empty()).unwrap_or(true);
    if missing {
        findings.push(ReviewPacketFinding {
            severity: ReviewPacketSeverity::Error,
            code: "missing_required_section".into(),
            path: path.into(),
            message: format!("Review packet draft is missing the {path} section."),
        });
    }
}

fn validate_required_list<T>(
    findings: &mut Vec<ReviewPacketFinding>,
    path: &str,
    value: Option<&Vec<T>>,
) {
    let missing = value.is_none();
    if missing {
        findings.push(ReviewPacketFinding {
            severity: ReviewPacketSeverity::Error,
            code: "missing_required_section".into(),
            path: path.into(),
            message: format!("Review packet draft is missing the {path} section."),
        });
    }
}

fn validate_tests_run(findings: &mut Vec<ReviewPacketFinding>, value: Option<&Vec<TestRun>>) {
    match value {
        None => findings.push(ReviewPacketFinding {
            severity: ReviewPacketSeverity::Error,
            code: "missing_required_section".into(),
            path: "tests_run".into(),
            message: "Review packet draft is missing the tests_run section.".into(),
        }),
        Some(test_runs) => {
            for (index, test_run) in test_runs.iter().enumerate() {
                if test_run.command.trim().is_empty() {
                    findings.push(ReviewPacketFinding {
                        severity: ReviewPacketSeverity::Error,
                        code: "test_command_missing".into(),
                        path: format!("tests_run[{index}].command"),
                        message: "Test run command must not be blank.".into(),
                    });
                }
            }
        }
    }
}

fn validate_timestamp(findings: &mut Vec<ReviewPacketFinding>, path: &str, value: Option<&str>) {
    match value {
        None => findings.push(ReviewPacketFinding {
            severity: ReviewPacketSeverity::Error,
            code: "missing_required_section".into(),
            path: path.into(),
            message: format!("Review packet draft is missing the {path} section."),
        }),
        Some(value) if !looks_like_utc_timestamp(value) => findings.push(ReviewPacketFinding {
            severity: ReviewPacketSeverity::Error,
            code: "timestamp_invalid".into(),
            path: path.into(),
            message: format!("{path} must use YYYY-MM-DDTHH:MM:SSZ UTC format."),
        }),
        Some(_) => {}
    }
}

fn looks_like_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 20
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'Z'
        && bytes.iter().enumerate().all(|(index, byte)| match index {
            4 | 7 => *byte == b'-',
            10 => *byte == b'T',
            13 | 16 => *byte == b':',
            19 => *byte == b'Z',
            _ => byte.is_ascii_digit(),
        })
}

fn require_string(value: Option<String>) -> String {
    value
        .expect("validated string should exist")
        .trim()
        .to_string()
}

fn require_list<T>(value: Option<Vec<T>>) -> Vec<T> {
    value.expect("validated list should exist")
}
