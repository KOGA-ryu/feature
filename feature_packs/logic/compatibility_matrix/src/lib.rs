use std::collections::BTreeMap;
use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.compatibility_matrix";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityStatus {
    Compatible,
    Risky,
    Incompatible,
}

impl fmt::Display for CompatibilityStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compatible => formatter.write_str("compatible"),
            Self::Risky => formatter.write_str("risky"),
            Self::Incompatible => formatter.write_str("incompatible"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureCompatibilityRule {
    pub left_feature_id: String,
    pub right_feature_id: String,
    pub status: CompatibilityStatus,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityRequest {
    pub selected_feature_ids: Vec<String>,
    pub rules: Vec<FeatureCompatibilityRule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityPairing {
    pub left_feature_id: String,
    pub right_feature_id: String,
    pub status: CompatibilityStatus,
    pub rationale: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilitySeverity {
    Error,
    Warning,
}

impl fmt::Display for CompatibilitySeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityFinding {
    pub severity: CompatibilitySeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityValidation {
    pub findings: Vec<CompatibilityFinding>,
}

impl CompatibilityValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == CompatibilitySeverity::Error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityReport {
    pub pairings: Vec<CompatibilityPairing>,
    pub missing_rules: Vec<String>,
    pub compatible_pairs: Vec<String>,
    pub risky_pairs: Vec<String>,
    pub incompatible_pairs: Vec<String>,
    pub findings: Vec<CompatibilityFinding>,
}

#[derive(Debug, Clone, Default)]
pub struct CompatibilityMatrix;

impl CompatibilityMatrix {
    pub fn validate(&self, request: &CompatibilityRequest) -> CompatibilityValidation {
        validate_compatibility_request(request)
    }

    pub fn evaluate(
        &self,
        request: CompatibilityRequest,
    ) -> Result<CompatibilityReport, CompatibilityValidation> {
        evaluate_compatibility(request)
    }
}

pub fn evaluate_compatibility(
    request: CompatibilityRequest,
) -> Result<CompatibilityReport, CompatibilityValidation> {
    let validation = validate_compatibility_request(&request);
    if !validation.is_valid() {
        return Err(validation);
    }

    let rules = request
        .rules
        .into_iter()
        .map(|rule| {
            (
                normalized_pair_key(&rule.left_feature_id, &rule.right_feature_id),
                rule,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut pairings = Vec::new();
    let mut missing_rules = Vec::new();
    let mut compatible_pairs = Vec::new();
    let mut risky_pairs = Vec::new();
    let mut incompatible_pairs = Vec::new();
    let mut findings = Vec::new();

    for left_index in 0..request.selected_feature_ids.len() {
        for right_index in left_index + 1..request.selected_feature_ids.len() {
            let left_feature_id = request.selected_feature_ids[left_index].trim().to_string();
            let right_feature_id = request.selected_feature_ids[right_index].trim().to_string();
            let pair_label = format!("{left_feature_id} + {right_feature_id}");
            let key = normalized_pair_key(&left_feature_id, &right_feature_id);

            let pairing = match rules.get(&key) {
                Some(rule) => CompatibilityPairing {
                    left_feature_id: left_feature_id.clone(),
                    right_feature_id: right_feature_id.clone(),
                    status: rule.status,
                    rationale: rule.rationale.clone(),
                },
                None => {
                    missing_rules.push(pair_label.clone());
                    risky_pairs.push(pair_label.clone());
                    findings.push(CompatibilityFinding {
                        severity: CompatibilitySeverity::Warning,
                        code: "missing_rule".into(),
                        path: pair_label.clone(),
                        message: "No explicit compatibility rule exists for this feature pair."
                            .into(),
                    });
                    CompatibilityPairing {
                        left_feature_id: left_feature_id.clone(),
                        right_feature_id: right_feature_id.clone(),
                        status: CompatibilityStatus::Risky,
                        rationale: "No explicit compatibility rule found.".into(),
                    }
                }
            };

            match pairing.status {
                CompatibilityStatus::Compatible => compatible_pairs.push(pair_label),
                CompatibilityStatus::Risky => {
                    if !risky_pairs.contains(&pair_label) {
                        risky_pairs.push(pair_label);
                    }
                }
                CompatibilityStatus::Incompatible => incompatible_pairs.push(pair_label),
            }

            pairings.push(pairing);
        }
    }

    Ok(CompatibilityReport {
        pairings,
        missing_rules,
        compatible_pairs,
        risky_pairs,
        incompatible_pairs,
        findings,
    })
}

pub fn validate_compatibility_request(request: &CompatibilityRequest) -> CompatibilityValidation {
    let mut findings = Vec::new();

    for (index, feature_id) in request.selected_feature_ids.iter().enumerate() {
        if feature_id.trim().is_empty() {
            findings.push(CompatibilityFinding {
                severity: CompatibilitySeverity::Error,
                code: "blank_feature_id".into(),
                path: format!("selected_feature_ids[{index}]"),
                message: "Selected feature ids must not be blank.".into(),
            });
        }
    }

    for (index, rule) in request.rules.iter().enumerate() {
        if rule.left_feature_id.trim().is_empty() {
            findings.push(CompatibilityFinding {
                severity: CompatibilitySeverity::Error,
                code: "blank_rule_feature_id".into(),
                path: format!("rules[{index}].left_feature_id"),
                message: "Compatibility rules require a non-blank left_feature_id.".into(),
            });
        }
        if rule.right_feature_id.trim().is_empty() {
            findings.push(CompatibilityFinding {
                severity: CompatibilitySeverity::Error,
                code: "blank_rule_feature_id".into(),
                path: format!("rules[{index}].right_feature_id"),
                message: "Compatibility rules require a non-blank right_feature_id.".into(),
            });
        }
        if rule.rationale.trim().is_empty() {
            findings.push(CompatibilityFinding {
                severity: CompatibilitySeverity::Error,
                code: "blank_rationale".into(),
                path: format!("rules[{index}].rationale"),
                message: "Compatibility rules require a rationale.".into(),
            });
        }
    }

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    CompatibilityValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_compatibility_request.json")
}

pub fn sample_missing_rule_fixture() -> &'static str {
    include_str!("../fixtures/missing_rule_request.json")
}

pub fn sample_incompatible_fixture() -> &'static str {
    include_str!("../fixtures/incompatible_rule_request.json")
}

pub fn parse_compatibility_request(raw: &str) -> Result<CompatibilityRequest, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_request() -> Result<CompatibilityRequest, String> {
    parse_compatibility_request(sample_valid_fixture())
}

pub fn sample_missing_rule_request() -> Result<CompatibilityRequest, String> {
    parse_compatibility_request(sample_missing_rule_fixture())
}

pub fn sample_incompatible_request() -> Result<CompatibilityRequest, String> {
    parse_compatibility_request(sample_incompatible_fixture())
}

pub fn sample_valid_report() -> Result<CompatibilityReport, String> {
    evaluate_compatibility(sample_valid_request()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn normalized_pair_key(left_feature_id: &str, right_feature_id: &str) -> String {
    let left_feature_id = left_feature_id.trim();
    let right_feature_id = right_feature_id.trim();
    if left_feature_id <= right_feature_id {
        format!("{left_feature_id}::{right_feature_id}")
    } else {
        format!("{right_feature_id}::{left_feature_id}")
    }
}
