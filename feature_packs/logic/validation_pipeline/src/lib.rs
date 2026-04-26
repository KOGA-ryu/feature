use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.validation_pipeline";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ValidationInput {
    pub required_fields: Vec<RequiredField>,
    pub collections: Vec<CollectionField>,
    pub timestamps: Vec<TimestampField>,
    pub dependencies: Vec<String>,
    pub compatible_features: Vec<String>,
    pub available_dependencies: Vec<String>,
    pub available_features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequiredField {
    pub path: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionField {
    pub path: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimestampField {
    pub path: String,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationRule {
    RequiredFieldPresent,
    CollectionNotEmpty,
    UtcTimestampFormat,
    KnownDependency,
    KnownCompatibleFeature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
}

impl fmt::Display for ValidationSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationOutcome {
    Passed,
    PassedWithWarnings,
    Failed,
}

impl fmt::Display for ValidationOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Passed => formatter.write_str("passed"),
            Self::PassedWithWarnings => formatter.write_str("passed_with_warnings"),
            Self::Failed => formatter.write_str("failed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: ValidationSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub outcome: ValidationOutcome,
    pub error_count: usize,
    pub warning_count: usize,
    pub total_findings: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationResult {
    pub summary: ValidationSummary,
    pub findings: Vec<ValidationFinding>,
}

#[derive(Debug, Clone)]
pub struct ValidationPipeline {
    rules: Vec<ValidationRule>,
}

impl Default for ValidationPipeline {
    fn default() -> Self {
        Self {
            rules: vec![
                ValidationRule::RequiredFieldPresent,
                ValidationRule::CollectionNotEmpty,
                ValidationRule::UtcTimestampFormat,
                ValidationRule::KnownDependency,
                ValidationRule::KnownCompatibleFeature,
            ],
        }
    }
}

impl ValidationPipeline {
    pub fn rules(&self) -> &[ValidationRule] {
        &self.rules
    }

    pub fn validate(&self, input: &ValidationInput) -> ValidationResult {
        let mut findings = Vec::new();

        for rule in &self.rules {
            match rule {
                ValidationRule::RequiredFieldPresent => {
                    findings.extend(validate_required_fields(input));
                }
                ValidationRule::CollectionNotEmpty => {
                    findings.extend(validate_collections(input));
                }
                ValidationRule::UtcTimestampFormat => {
                    findings.extend(validate_timestamps(input));
                }
                ValidationRule::KnownDependency => {
                    findings.extend(validate_dependencies(input));
                }
                ValidationRule::KnownCompatibleFeature => {
                    findings.extend(validate_compatible_features(input));
                }
            }
        }

        findings.sort_by(|left, right| {
            left.severity
                .cmp(&right.severity)
                .then_with(|| left.path.cmp(&right.path))
                .then_with(|| left.code.cmp(&right.code))
        });

        let error_count = findings
            .iter()
            .filter(|finding| finding.severity == ValidationSeverity::Error)
            .count();
        let warning_count = findings
            .iter()
            .filter(|finding| finding.severity == ValidationSeverity::Warning)
            .count();
        let outcome = if error_count > 0 {
            ValidationOutcome::Failed
        } else if warning_count > 0 {
            ValidationOutcome::PassedWithWarnings
        } else {
            ValidationOutcome::Passed
        };

        ValidationResult {
            summary: ValidationSummary {
                outcome,
                error_count,
                warning_count,
                total_findings: findings.len(),
            },
            findings,
        }
    }
}

pub fn validate_input(input: &ValidationInput) -> ValidationResult {
    ValidationPipeline::default().validate(input)
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_input.json")
}

pub fn sample_valid_input() -> Result<ValidationInput, String> {
    parse_input_fixture(sample_valid_fixture())
}

pub fn sample_invalid_input() -> Result<ValidationInput, String> {
    parse_input_fixture(sample_invalid_fixture())
}

pub fn parse_input_fixture(raw: &str) -> Result<ValidationInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

fn validate_required_fields(input: &ValidationInput) -> Vec<ValidationFinding> {
    input
        .required_fields
        .iter()
        .filter(|field| {
            field
                .value
                .as_deref()
                .map(|value| value.trim().is_empty())
                .unwrap_or(true)
        })
        .map(|field| ValidationFinding {
            severity: ValidationSeverity::Error,
            code: "required_field_missing".into(),
            path: field.path.clone(),
            message: format!("Required field at {} is missing or blank.", field.path),
        })
        .collect()
}

fn validate_collections(input: &ValidationInput) -> Vec<ValidationFinding> {
    input
        .collections
        .iter()
        .filter(|field| field.items.is_empty())
        .map(|field| ValidationFinding {
            severity: ValidationSeverity::Error,
            code: "collection_empty".into(),
            path: field.path.clone(),
            message: format!("Collection at {} must not be empty.", field.path),
        })
        .collect()
}

fn validate_timestamps(input: &ValidationInput) -> Vec<ValidationFinding> {
    input
        .timestamps
        .iter()
        .filter(|field| !looks_like_utc_timestamp(&field.value))
        .map(|field| ValidationFinding {
            severity: ValidationSeverity::Error,
            code: "timestamp_invalid".into(),
            path: field.path.clone(),
            message: format!(
                "Timestamp at {} must use YYYY-MM-DDTHH:MM:SSZ UTC format.",
                field.path
            ),
        })
        .collect()
}

fn validate_dependencies(input: &ValidationInput) -> Vec<ValidationFinding> {
    input
        .dependencies
        .iter()
        .filter(|dependency| !input.available_dependencies.contains(*dependency))
        .map(|dependency| ValidationFinding {
            severity: ValidationSeverity::Error,
            code: "dependency_missing".into(),
            path: "dependencies".into(),
            message: format!(
                "Dependency {} is not available in the current registry.",
                dependency
            ),
        })
        .collect()
}

fn validate_compatible_features(input: &ValidationInput) -> Vec<ValidationFinding> {
    input
        .compatible_features
        .iter()
        .filter(|feature| !input.available_features.contains(*feature))
        .map(|feature| ValidationFinding {
            severity: ValidationSeverity::Warning,
            code: "compatible_feature_unknown".into(),
            path: "compatible_features".into(),
            message: format!(
                "Compatible feature {} is not present in the current feature set.",
                feature
            ),
        })
        .collect()
}

fn looks_like_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 20 {
        return false;
    }
    matches_timestamp_shape(bytes)
}

fn matches_timestamp_shape(bytes: &[u8]) -> bool {
    bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'Z'
        && digit_slice(bytes, 0, 4)
        && digit_slice(bytes, 5, 7)
        && digit_slice(bytes, 8, 10)
        && digit_slice(bytes, 11, 13)
        && digit_slice(bytes, 14, 16)
        && digit_slice(bytes, 17, 19)
}

fn digit_slice(bytes: &[u8], start: usize, end: usize) -> bool {
    bytes[start..end].iter().all(u8::is_ascii_digit)
}
