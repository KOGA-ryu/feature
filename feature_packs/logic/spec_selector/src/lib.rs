use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use compatibility_matrix::{
    CompatibilityPairing, CompatibilityReport, CompatibilityRequest, CompatibilitySeverity,
    CompatibilityStatus, FeatureCompatibilityRule, evaluate_compatibility,
    validate_compatibility_request,
};
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use reuse_score::{
    ReuseScoreBand, ReuseScoreCard, ReuseScoreFinding, ReuseScoreInput, ReuseScoreReport,
    ReuseScoreSeverity, score_reuse, validate_reuse_inputs,
};
use serde::{Deserialize, Serialize};
use spec_generator::{
    AppIdeaInput, GeneratedSpec, SelectedFeatureRef, SpecGenerationSeverity, generate_spec,
    validate_spec_input,
};

pub const FEATURE_ID: &str = "logic.spec_selector";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecSelectionInput {
    pub app_idea: Option<AppIdeaInput>,
    pub reuse_inputs: Vec<ReuseScoreInput>,
    pub compatibility_rules: Vec<FeatureCompatibilityRule>,
    pub minimum_score: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedFeatureDecision {
    pub feature_id: String,
    pub selected: bool,
    pub score: Option<u32>,
    pub band: Option<ReuseScoreBand>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecSelectionSeverity {
    Error,
    Warning,
}

impl fmt::Display for SpecSelectionSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecSelectionFinding {
    pub severity: SpecSelectionSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecSelectionValidation {
    pub findings: Vec<SpecSelectionFinding>,
}

impl SpecSelectionValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == SpecSelectionSeverity::Error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecSelectionResult {
    pub generated_spec: GeneratedSpec,
    pub compatibility_report: CompatibilityReport,
    pub reuse_report: ReuseScoreReport,
    pub decisions: Vec<SelectedFeatureDecision>,
    pub selected_feature_ids: Vec<String>,
    pub rejected_feature_ids: Vec<String>,
    pub findings: Vec<SpecSelectionFinding>,
}

#[derive(Debug, Clone, Default)]
pub struct SpecSelector;

impl SpecSelector {
    pub fn validate(&self, input: &SpecSelectionInput) -> SpecSelectionValidation {
        validate_spec_selection_input(input)
    }

    pub fn select(
        &self,
        input: SpecSelectionInput,
    ) -> Result<SpecSelectionResult, SpecSelectionValidation> {
        select_spec(input)
    }
}

pub fn select_spec(
    input: SpecSelectionInput,
) -> Result<SpecSelectionResult, SpecSelectionValidation> {
    let validation = validate_spec_selection_input(&input);
    if !validation.is_valid() {
        return Err(validation);
    }

    let validation_warnings = validation
        .findings
        .into_iter()
        .filter(|finding| finding.severity == SpecSelectionSeverity::Warning)
        .collect::<Vec<_>>();
    let app_idea = input.app_idea.expect("validated app idea should exist");
    let reuse_report = score_reuse(input.reuse_inputs.clone()).map_err(convert_reuse_validation)?;
    let score_cards = reuse_report
        .cards
        .iter()
        .map(|card| (card.feature_id.clone(), card.clone()))
        .collect::<BTreeMap<_, _>>();

    let preselected_features = app_idea
        .selected_features
        .iter()
        .filter(|feature| {
            score_cards
                .get(feature.feature_id.as_str())
                .map(|card| card.score >= input.minimum_score)
                .unwrap_or(false)
        })
        .cloned()
        .collect::<Vec<_>>();
    let preselected_feature_ids = preselected_features
        .iter()
        .map(|feature| feature.feature_id.clone())
        .collect::<Vec<_>>();
    let compatibility_request = CompatibilityRequest {
        selected_feature_ids: preselected_feature_ids.clone(),
        rules: input.compatibility_rules.clone(),
    };
    let compatibility_report =
        evaluate_compatibility(compatibility_request).map_err(convert_compatibility_validation)?;
    let excluded_by_incompatibility = resolve_incompatible_exclusions(
        &preselected_feature_ids,
        &compatibility_report.pairings,
        &score_cards,
    );

    let selected_features = preselected_features
        .iter()
        .filter(|feature| !excluded_by_incompatibility.contains_key(feature.feature_id.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    let selected_feature_ids = selected_features
        .iter()
        .map(|feature| feature.feature_id.clone())
        .collect::<Vec<_>>();
    let rejected_feature_ids = app_idea
        .selected_features
        .iter()
        .filter(|feature| !selected_feature_ids.contains(&feature.feature_id))
        .map(|feature| feature.feature_id.clone())
        .collect::<Vec<_>>();

    let compatibility_notes = feature_compatibility_notes(&compatibility_report.pairings);
    let decisions = app_idea
        .selected_features
        .iter()
        .map(|feature| {
            build_decision(
                feature,
                score_cards.get(feature.feature_id.as_str()),
                input.minimum_score,
                excluded_by_incompatibility.get(feature.feature_id.as_str()),
                compatibility_notes.get(feature.feature_id.as_str()),
            )
        })
        .collect::<Vec<_>>();

    let generated_spec = generate_spec(AppIdeaInput {
        product_intent: app_idea.product_intent.clone(),
        audience: app_idea.audience.clone(),
        selected_features,
        data_models: app_idea.data_models.clone(),
        first_vertical_slice: app_idea.first_vertical_slice.clone(),
        out_of_scope: app_idea.out_of_scope.clone(),
    })
    .map_err(convert_spec_validation)?;

    let mut findings = validation_warnings;
    findings.extend(convert_reuse_warnings(&reuse_report.findings));
    findings.extend(convert_compatibility_warnings(
        &compatibility_report.findings,
    ));
    findings.extend(
        excluded_by_incompatibility
            .into_iter()
            .map(|(feature_id, other_feature_id)| SpecSelectionFinding {
                severity: SpecSelectionSeverity::Warning,
                code: "incompatible_pair_resolved".into(),
                path: feature_id,
                message: format!(
                    "Excluded because it is incompatible with higher-ranked feature {other_feature_id}."
                ),
            }),
    );
    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.code.cmp(&right.code))
    });

    Ok(SpecSelectionResult {
        generated_spec,
        compatibility_report,
        reuse_report,
        decisions,
        selected_feature_ids,
        rejected_feature_ids,
        findings,
    })
}

pub fn validate_spec_selection_input(input: &SpecSelectionInput) -> SpecSelectionValidation {
    let mut findings = Vec::new();

    let Some(app_idea) = input.app_idea.as_ref() else {
        findings.push(SpecSelectionFinding {
            severity: SpecSelectionSeverity::Error,
            code: "missing_app_idea".into(),
            path: "app_idea".into(),
            message: "Spec selection requires an app_idea.".into(),
        });
        return SpecSelectionValidation { findings };
    };

    findings.extend(
        validate_spec_input(app_idea)
            .findings
            .into_iter()
            .map(convert_spec_generation_finding),
    );
    findings.extend(
        validate_reuse_inputs(&input.reuse_inputs)
            .findings
            .into_iter()
            .map(convert_reuse_validation_finding),
    );
    findings.extend(
        validate_compatibility_request(&CompatibilityRequest {
            selected_feature_ids: app_idea
                .selected_features
                .iter()
                .map(|feature| feature.feature_id.clone())
                .collect(),
            rules: input.compatibility_rules.clone(),
        })
        .findings
        .into_iter()
        .map(convert_compatibility_validation_finding),
    );

    let mut counts = BTreeMap::<String, usize>::new();
    for reuse_input in &input.reuse_inputs {
        *counts
            .entry(reuse_input.feature_id.trim().to_string())
            .or_default() += 1;
    }

    for (index, feature) in app_idea.selected_features.iter().enumerate() {
        if !counts.contains_key(feature.feature_id.trim()) {
            findings.push(SpecSelectionFinding {
                severity: SpecSelectionSeverity::Error,
                code: "missing_reuse_input".into(),
                path: format!("selected_features[{index}]"),
                message: format!(
                    "No reuse-score input exists for selected feature {}.",
                    feature.feature_id
                ),
            });
        }
    }

    for (feature_id, count) in counts {
        if feature_id.is_empty() || count < 2 {
            continue;
        }
        findings.push(SpecSelectionFinding {
            severity: SpecSelectionSeverity::Error,
            code: "duplicate_reuse_input".into(),
            path: "reuse_inputs".into(),
            message: format!("Feature {feature_id} has duplicate reuse-score inputs."),
        });
    }

    let selected_feature_ids = app_idea
        .selected_features
        .iter()
        .map(|feature| feature.feature_id.as_str())
        .collect::<BTreeSet<_>>();
    for (index, reuse_input) in input.reuse_inputs.iter().enumerate() {
        if !selected_feature_ids.contains(reuse_input.feature_id.trim()) {
            findings.push(SpecSelectionFinding {
                severity: SpecSelectionSeverity::Warning,
                code: "unused_reuse_input".into(),
                path: format!("reuse_inputs[{index}]"),
                message: format!(
                    "Reuse-score input for {} does not match any selected feature.",
                    reuse_input.feature_id
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
    SpecSelectionValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_valid_fixture() -> &'static str {
    include_str!("../fixtures/valid_selection_input.json")
}

pub fn sample_incompatible_fixture() -> &'static str {
    include_str!("../fixtures/incompatible_selection_input.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/invalid_selection_input.json")
}

pub fn parse_selection_input(raw: &str) -> Result<SpecSelectionInput, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_valid_input() -> Result<SpecSelectionInput, String> {
    parse_selection_input(sample_valid_fixture())
}

pub fn sample_incompatible_input() -> Result<SpecSelectionInput, String> {
    parse_selection_input(sample_incompatible_fixture())
}

pub fn sample_invalid_input() -> Result<SpecSelectionInput, String> {
    parse_selection_input(sample_invalid_fixture())
}

pub fn sample_selection_result() -> Result<SpecSelectionResult, String> {
    select_spec(sample_valid_input()?).map_err(|validation| {
        validation
            .findings
            .into_iter()
            .map(|finding| format!("{}: {}", finding.path, finding.message))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

fn resolve_incompatible_exclusions(
    selected_feature_ids: &[String],
    pairings: &[CompatibilityPairing],
    score_cards: &BTreeMap<String, ReuseScoreCard>,
) -> BTreeMap<String, String> {
    let order = selected_feature_ids
        .iter()
        .enumerate()
        .map(|(index, feature_id)| (feature_id.clone(), index))
        .collect::<BTreeMap<_, _>>();
    let mut excluded = BTreeMap::new();

    for pairing in pairings
        .iter()
        .filter(|pairing| pairing.status == CompatibilityStatus::Incompatible)
    {
        if excluded.contains_key(pairing.left_feature_id.as_str())
            || excluded.contains_key(pairing.right_feature_id.as_str())
        {
            continue;
        }

        let left_score = score_cards
            .get(pairing.left_feature_id.as_str())
            .map(|card| card.score)
            .unwrap_or(0);
        let right_score = score_cards
            .get(pairing.right_feature_id.as_str())
            .map(|card| card.score)
            .unwrap_or(0);

        let loser = if left_score < right_score {
            pairing.left_feature_id.clone()
        } else if right_score < left_score {
            pairing.right_feature_id.clone()
        } else {
            let left_index = order
                .get(pairing.left_feature_id.as_str())
                .copied()
                .unwrap_or(usize::MAX);
            let right_index = order
                .get(pairing.right_feature_id.as_str())
                .copied()
                .unwrap_or(usize::MAX);
            if left_index < right_index {
                pairing.right_feature_id.clone()
            } else {
                pairing.left_feature_id.clone()
            }
        };
        let winner = if loser == pairing.left_feature_id {
            pairing.right_feature_id.clone()
        } else {
            pairing.left_feature_id.clone()
        };
        excluded.insert(loser, winner);
    }

    excluded
}

fn feature_compatibility_notes(pairings: &[CompatibilityPairing]) -> BTreeMap<String, Vec<String>> {
    let mut notes = BTreeMap::<String, Vec<String>>::new();
    for pairing in pairings {
        if pairing.status == CompatibilityStatus::Compatible {
            continue;
        }
        let note = match pairing.status {
            CompatibilityStatus::Risky => format!(
                "Risky pairing with {}: {}",
                pairing.right_feature_id, pairing.rationale
            ),
            CompatibilityStatus::Incompatible => format!(
                "Incompatible pairing with {}: {}",
                pairing.right_feature_id, pairing.rationale
            ),
            CompatibilityStatus::Compatible => continue,
        };
        notes
            .entry(pairing.left_feature_id.clone())
            .or_default()
            .push(note);

        let mirrored = match pairing.status {
            CompatibilityStatus::Risky => format!(
                "Risky pairing with {}: {}",
                pairing.left_feature_id, pairing.rationale
            ),
            CompatibilityStatus::Incompatible => format!(
                "Incompatible pairing with {}: {}",
                pairing.left_feature_id, pairing.rationale
            ),
            CompatibilityStatus::Compatible => continue,
        };
        notes
            .entry(pairing.right_feature_id.clone())
            .or_default()
            .push(mirrored);
    }
    notes
}

fn build_decision(
    feature: &SelectedFeatureRef,
    score_card: Option<&ReuseScoreCard>,
    minimum_score: u32,
    incompatibility_winner: Option<&String>,
    compatibility_notes: Option<&Vec<String>>,
) -> SelectedFeatureDecision {
    let mut reasons = Vec::new();
    let mut selected = true;
    let mut score = None;
    let mut band = None;

    if let Some(card) = score_card {
        score = Some(card.score);
        band = Some(card.band);
        reasons.push(format!(
            "Reuse score {} ({}) with reasons: {}",
            card.score,
            card.band,
            card.reasons.join(", ")
        ));
        if card.score < minimum_score {
            selected = false;
            reasons.push(format!(
                "Excluded because score {} is below minimum {}.",
                card.score, minimum_score
            ));
        }
    } else {
        selected = false;
        reasons.push("Excluded because no reuse score card was available.".into());
    }

    if let Some(other_feature_id) = incompatibility_winner {
        selected = false;
        reasons.push(format!(
            "Excluded because it is incompatible with higher-ranked feature {other_feature_id}."
        ));
    }

    if let Some(notes) = compatibility_notes {
        reasons.extend(notes.iter().cloned());
    }

    SelectedFeatureDecision {
        feature_id: feature.feature_id.clone(),
        selected,
        score,
        band,
        reasons,
    }
}

fn convert_spec_validation(
    validation: spec_generator::SpecGenerationValidation,
) -> SpecSelectionValidation {
    SpecSelectionValidation {
        findings: validation
            .findings
            .into_iter()
            .map(convert_spec_generation_finding)
            .collect(),
    }
}

fn convert_reuse_validation(
    validation: reuse_score::ReuseScoreValidation,
) -> SpecSelectionValidation {
    SpecSelectionValidation {
        findings: validation
            .findings
            .into_iter()
            .map(convert_reuse_validation_finding)
            .collect(),
    }
}

fn convert_compatibility_validation(
    validation: compatibility_matrix::CompatibilityValidation,
) -> SpecSelectionValidation {
    SpecSelectionValidation {
        findings: validation
            .findings
            .into_iter()
            .map(convert_compatibility_validation_finding)
            .collect(),
    }
}

fn convert_spec_generation_finding(
    finding: spec_generator::SpecGenerationFinding,
) -> SpecSelectionFinding {
    SpecSelectionFinding {
        severity: match finding.severity {
            SpecGenerationSeverity::Error => SpecSelectionSeverity::Error,
            SpecGenerationSeverity::Warning => SpecSelectionSeverity::Warning,
        },
        code: finding.code,
        path: finding.path,
        message: finding.message,
    }
}

fn convert_reuse_validation_finding(
    finding: reuse_score::ReuseScoreFinding,
) -> SpecSelectionFinding {
    SpecSelectionFinding {
        severity: match finding.severity {
            ReuseScoreSeverity::Error => SpecSelectionSeverity::Error,
            ReuseScoreSeverity::Warning => SpecSelectionSeverity::Warning,
        },
        code: finding.code,
        path: finding.path,
        message: finding.message,
    }
}

fn convert_compatibility_validation_finding(
    finding: compatibility_matrix::CompatibilityFinding,
) -> SpecSelectionFinding {
    SpecSelectionFinding {
        severity: match finding.severity {
            CompatibilitySeverity::Error => SpecSelectionSeverity::Error,
            CompatibilitySeverity::Warning => SpecSelectionSeverity::Warning,
        },
        code: finding.code,
        path: finding.path,
        message: finding.message,
    }
}

fn convert_reuse_warnings(findings: &[ReuseScoreFinding]) -> Vec<SpecSelectionFinding> {
    findings
        .iter()
        .cloned()
        .map(convert_reuse_validation_finding)
        .collect()
}

fn convert_compatibility_warnings(
    findings: &[compatibility_matrix::CompatibilityFinding],
) -> Vec<SpecSelectionFinding> {
    findings
        .iter()
        .cloned()
        .map(convert_compatibility_validation_finding)
        .collect()
}
