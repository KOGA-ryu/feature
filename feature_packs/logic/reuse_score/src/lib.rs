use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.reuse_score";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureLifecycleStatus {
    Draft,
    Experimental,
    Tested,
    Stable,
    Deprecated,
}

impl fmt::Display for FeatureLifecycleStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => formatter.write_str("draft"),
            Self::Experimental => formatter.write_str("experimental"),
            Self::Tested => formatter.write_str("tested"),
            Self::Stable => formatter.write_str("stable"),
            Self::Deprecated => formatter.write_str("deprecated"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReuseScoreInput {
    pub feature_id: String,
    pub status: FeatureLifecycleStatus,
    pub usage_count: u32,
    pub has_tests: bool,
    pub has_docs: bool,
    pub has_examples: bool,
    pub has_fixtures: bool,
    pub reviewed_recently: bool,
    pub failure_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReuseScoreBand {
    Low,
    Medium,
    High,
    Preferred,
}

impl fmt::Display for ReuseScoreBand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => formatter.write_str("low"),
            Self::Medium => formatter.write_str("medium"),
            Self::High => formatter.write_str("high"),
            Self::Preferred => formatter.write_str("preferred"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReuseScoreCard {
    pub feature_id: String,
    pub score: u32,
    pub band: ReuseScoreBand,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReuseScoreSeverity {
    Error,
    Warning,
}

impl fmt::Display for ReuseScoreSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReuseScoreFinding {
    pub severity: ReuseScoreSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReuseScoreValidation {
    pub findings: Vec<ReuseScoreFinding>,
}

impl ReuseScoreValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == ReuseScoreSeverity::Error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReuseScoreReport {
    pub cards: Vec<ReuseScoreCard>,
    pub findings: Vec<ReuseScoreFinding>,
}

#[derive(Debug, Clone, Default)]
pub struct ReuseScorer;

impl ReuseScorer {
    pub fn validate(&self, inputs: &[ReuseScoreInput]) -> ReuseScoreValidation {
        validate_reuse_inputs(inputs)
    }

    pub fn score(
        &self,
        inputs: Vec<ReuseScoreInput>,
    ) -> Result<ReuseScoreReport, ReuseScoreValidation> {
        score_reuse(inputs)
    }
}

pub fn score_reuse(inputs: Vec<ReuseScoreInput>) -> Result<ReuseScoreReport, ReuseScoreValidation> {
    let validation = validate_reuse_inputs(&inputs);
    if !validation.is_valid() {
        return Err(validation);
    }

    let mut cards = Vec::new();
    let mut findings = Vec::new();

    for (index, input) in inputs.into_iter().enumerate() {
        let deprecated_positive = input.status == FeatureLifecycleStatus::Deprecated
            && (input.usage_count > 0
                || input.has_tests
                || input.has_docs
                || input.has_examples
                || input.has_fixtures
                || input.reviewed_recently);
        if deprecated_positive {
            findings.push(ReuseScoreFinding {
                severity: ReuseScoreSeverity::Warning,
                code: "deprecated_with_positive_signals".into(),
                path: format!("inputs[{index}]"),
                message:
                    "Deprecated feature still carries positive reuse signals; review before reuse."
                        .into(),
            });
        }

        cards.push(score_one(input));
    }

    cards.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.feature_id.cmp(&right.feature_id))
    });

    Ok(ReuseScoreReport { cards, findings })
}

pub fn validate_reuse_inputs(inputs: &[ReuseScoreInput]) -> ReuseScoreValidation {
    let mut findings = Vec::new();

    for (index, input) in inputs.iter().enumerate() {
        if input.feature_id.trim().is_empty() {
            findings.push(ReuseScoreFinding {
                severity: ReuseScoreSeverity::Error,
                code: "blank_feature_id".into(),
                path: format!("inputs[{index}].feature_id"),
                message: "feature_id is required and must not be blank.".into(),
            });
        }
    }

    ReuseScoreValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_mixed_fixture() -> &'static str {
    include_str!("../fixtures/mixed_reuse_inputs.json")
}

pub fn sample_deprecated_fixture() -> &'static str {
    include_str!("../fixtures/deprecated_feature_inputs.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_reuse_inputs.json")
}

pub fn parse_reuse_inputs(raw: &str) -> Result<Vec<ReuseScoreInput>, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_mixed_inputs() -> Result<Vec<ReuseScoreInput>, String> {
    parse_reuse_inputs(sample_mixed_fixture())
}

pub fn sample_deprecated_inputs() -> Result<Vec<ReuseScoreInput>, String> {
    parse_reuse_inputs(sample_deprecated_fixture())
}

pub fn sample_invalid_inputs() -> Result<Vec<ReuseScoreInput>, String> {
    parse_reuse_inputs(sample_invalid_fixture())
}

pub fn sample_mixed_report() -> Result<ReuseScoreReport, String> {
    score_reuse(sample_mixed_inputs()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn score_one(input: ReuseScoreInput) -> ReuseScoreCard {
    let mut score = 0_i32;
    let mut reasons = Vec::new();

    let status_weight = match input.status {
        FeatureLifecycleStatus::Draft => 0,
        FeatureLifecycleStatus::Experimental => 10,
        FeatureLifecycleStatus::Tested => 25,
        FeatureLifecycleStatus::Stable => 40,
        FeatureLifecycleStatus::Deprecated => -50,
    };
    score += status_weight;
    reasons.push(format!("status {} ({:+})", input.status, status_weight));

    let usage_bonus = 8 * input.usage_count.min(5) as i32;
    if usage_bonus > 0 {
        score += usage_bonus;
        reasons.push(format!("usage count bonus ({:+})", usage_bonus));
    }

    if input.has_tests {
        score += 15;
        reasons.push("tests evidence (+15)".into());
    }
    if input.has_docs {
        score += 10;
        reasons.push("docs evidence (+10)".into());
    }
    if input.has_examples {
        score += 10;
        reasons.push("examples evidence (+10)".into());
    }
    if input.has_fixtures {
        score += 5;
        reasons.push("fixtures evidence (+5)".into());
    }
    if input.reviewed_recently {
        score += 10;
        reasons.push("recent review (+10)".into());
    }

    let failure_penalty = 5 * input.failure_count.min(5) as i32;
    if failure_penalty > 0 {
        score -= failure_penalty;
        reasons.push(format!("failure penalty (-{})", failure_penalty));
    }

    let score = score.clamp(0, 100) as u32;
    let band = score_to_band(score);

    ReuseScoreCard {
        feature_id: input.feature_id.trim().to_string(),
        score,
        band,
        reasons,
    }
}

fn score_to_band(score: u32) -> ReuseScoreBand {
    match score {
        0..=24 => ReuseScoreBand::Low,
        25..=49 => ReuseScoreBand::Medium,
        50..=74 => ReuseScoreBand::High,
        _ => ReuseScoreBand::Preferred,
    }
}
