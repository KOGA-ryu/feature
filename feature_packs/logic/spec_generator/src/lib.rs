use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.spec_generator";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectedFeatureKind {
    Ui,
    Logic,
    Workflow,
}

impl fmt::Display for SelectedFeatureKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ui => formatter.write_str("ui"),
            Self::Logic => formatter.write_str("logic"),
            Self::Workflow => formatter.write_str("workflow"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedFeatureRef {
    pub feature_id: String,
    pub kind: SelectedFeatureKind,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppIdeaInput {
    pub product_intent: Option<String>,
    pub audience: Option<String>,
    pub selected_features: Vec<SelectedFeatureRef>,
    pub data_models: Vec<String>,
    pub first_vertical_slice: Vec<String>,
    pub out_of_scope: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedSpec {
    pub product_intent: String,
    pub audience: String,
    pub required_surfaces: Vec<String>,
    pub selected_patterns: Vec<String>,
    pub selected_logic: Vec<String>,
    pub selected_workflows: Vec<String>,
    pub data_models: Vec<String>,
    pub folder_structure: Vec<String>,
    pub first_vertical_slice: Vec<String>,
    pub out_of_scope: Vec<String>,
    pub dex_prompt_seed: String,
    pub review_checklist: Vec<String>,
    pub selected_features: Vec<SelectedFeatureRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecGenerationSeverity {
    Error,
    Warning,
}

impl fmt::Display for SpecGenerationSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecGenerationFinding {
    pub severity: SpecGenerationSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecGenerationValidation {
    pub findings: Vec<SpecGenerationFinding>,
}

impl SpecGenerationValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == SpecGenerationSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpecGenerator;

impl SpecGenerator {
    pub fn validate(&self, input: &AppIdeaInput) -> SpecGenerationValidation {
        validate_spec_input(input)
    }

    pub fn generate(&self, input: AppIdeaInput) -> Result<GeneratedSpec, SpecGenerationValidation> {
        generate_spec(input)
    }
}

pub fn generate_spec(input: AppIdeaInput) -> Result<GeneratedSpec, SpecGenerationValidation> {
    let validation = validate_spec_input(&input);
    if !validation.is_valid() {
        return Err(validation);
    }

    let product_intent = required_string(input.product_intent);
    let audience = required_string(input.audience);
    let selected_features = input.selected_features;
    let required_surfaces = selected_features
        .iter()
        .filter(|feature| feature.kind == SelectedFeatureKind::Ui)
        .map(|feature| feature.feature_id.clone())
        .collect::<Vec<_>>();
    let selected_logic = selected_features
        .iter()
        .filter(|feature| feature.kind == SelectedFeatureKind::Logic)
        .map(|feature| feature.feature_id.clone())
        .collect::<Vec<_>>();
    let selected_workflows = selected_features
        .iter()
        .filter(|feature| feature.kind == SelectedFeatureKind::Workflow)
        .map(|feature| feature.feature_id.clone())
        .collect::<Vec<_>>();
    let selected_patterns = selected_features
        .iter()
        .filter(|feature| feature.kind != SelectedFeatureKind::Logic)
        .map(|feature| feature.feature_id.clone())
        .collect::<Vec<_>>();

    let data_models = trim_list(input.data_models);
    let first_vertical_slice = trim_list(input.first_vertical_slice);
    let out_of_scope = trim_list(input.out_of_scope);

    Ok(GeneratedSpec {
        product_intent: product_intent.clone(),
        audience: audience.clone(),
        required_surfaces,
        selected_patterns,
        selected_logic,
        selected_workflows,
        data_models,
        folder_structure: default_folder_structure(),
        first_vertical_slice,
        out_of_scope,
        dex_prompt_seed: format!(
            "Build for {audience}: {product_intent}. Use selected features in the declared order."
        ),
        review_checklist: default_review_checklist(),
        selected_features,
    })
}

pub fn validate_spec_input(input: &AppIdeaInput) -> SpecGenerationValidation {
    let mut findings = Vec::new();

    validate_required_text(
        &mut findings,
        "product_intent",
        input.product_intent.as_deref(),
    );
    validate_required_text(&mut findings, "audience", input.audience.as_deref());

    if input.selected_features.is_empty() {
        findings.push(SpecGenerationFinding {
            severity: SpecGenerationSeverity::Error,
            code: "missing_selected_features".into(),
            path: "selected_features".into(),
            message: "At least one selected feature is required to generate a spec.".into(),
        });
    }

    for (index, feature) in input.selected_features.iter().enumerate() {
        if feature.feature_id.trim().is_empty() {
            findings.push(SpecGenerationFinding {
                severity: SpecGenerationSeverity::Error,
                code: "selected_feature_id_missing".into(),
                path: format!("selected_features[{index}].feature_id"),
                message: "Selected feature id must not be blank.".into(),
            });
        }
        if feature.summary.trim().is_empty() {
            findings.push(SpecGenerationFinding {
                severity: SpecGenerationSeverity::Warning,
                code: "selected_feature_summary_blank".into(),
                path: format!("selected_features[{index}].summary"),
                message: "Selected feature summary is blank; downstream prompts may be weaker."
                    .into(),
            });
        }
    }

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    SpecGenerationValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_app_idea.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_app_idea.json")
}

pub fn parse_app_idea_fixture(raw: &str) -> Result<AppIdeaInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_input() -> Result<AppIdeaInput, String> {
    parse_app_idea_fixture(sample_valid_fixture())
}

pub fn sample_invalid_input() -> Result<AppIdeaInput, String> {
    parse_app_idea_fixture(sample_invalid_fixture())
}

pub fn sample_generated_spec() -> Result<GeneratedSpec, String> {
    generate_spec(sample_valid_input()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn validate_required_text(
    findings: &mut Vec<SpecGenerationFinding>,
    path: &str,
    value: Option<&str>,
) {
    let missing = value.map(|value| value.trim().is_empty()).unwrap_or(true);
    if missing {
        findings.push(SpecGenerationFinding {
            severity: SpecGenerationSeverity::Error,
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

fn trim_list(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

fn default_folder_structure() -> Vec<String> {
    vec![
        "Cargo.toml".into(),
        "README.md".into(),
        "docs/contract.md".into(),
        "docs/roadmap.md".into(),
        "docs/decisions.md".into(),
        "src/app/".into(),
        "src/components/".into(),
        "src/features/".into(),
        "src/lib/".into(),
        "src/data/".into(),
        "src/schemas/".into(),
        "tests/".into(),
    ]
}

fn default_review_checklist() -> Vec<String> {
    vec![
        "Generated spec contains every required section.".into(),
        "Selected feature order is preserved.".into(),
        "Folder structure matches feature-lab conventions.".into(),
        "First vertical slice is bounded and testable.".into(),
        "Out-of-scope items remain explicit.".into(),
    ]
}
