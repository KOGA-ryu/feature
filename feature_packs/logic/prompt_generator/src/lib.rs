use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};
use spec_generator::GeneratedSpec;

pub const FEATURE_ID: &str = "logic.prompt_generator";
const REPO_ROOT: &str = "/Users/kogaryu/dev/features";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptTargetKind {
    FeatureWorker,
    Integrator,
    Reviewer,
}

impl fmt::Display for PromptTargetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FeatureWorker => formatter.write_str("feature_worker"),
            Self::Integrator => formatter.write_str("integrator"),
            Self::Reviewer => formatter.write_str("reviewer"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptRequest {
    pub target_kind: PromptTargetKind,
    pub spec: Option<GeneratedSpec>,
    pub target_feature_id: Option<String>,
    pub wave_feature_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedPromptPacket {
    pub target_kind: PromptTargetKind,
    pub title: String,
    pub prompt_text: String,
    pub verification_commands: Vec<String>,
    pub allowed_writes: Vec<String>,
    pub forbidden_writes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptGenerationSeverity {
    Error,
    Warning,
}

impl fmt::Display for PromptGenerationSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptGenerationFinding {
    pub severity: PromptGenerationSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptGenerationValidation {
    pub findings: Vec<PromptGenerationFinding>,
}

impl PromptGenerationValidation {
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == PromptGenerationSeverity::Error)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PromptGenerator;

impl PromptGenerator {
    pub fn validate(&self, request: &PromptRequest) -> PromptGenerationValidation {
        validate_prompt_request(request)
    }

    pub fn generate(
        &self,
        request: PromptRequest,
    ) -> Result<GeneratedPromptPacket, PromptGenerationValidation> {
        generate_prompt(request)
    }
}

pub fn generate_prompt(
    request: PromptRequest,
) -> Result<GeneratedPromptPacket, PromptGenerationValidation> {
    let validation = validate_prompt_request(&request);
    if !validation.is_valid() {
        return Err(validation);
    }

    match request.target_kind {
        PromptTargetKind::FeatureWorker => {
            let spec = request.spec.expect("validated spec should exist");
            let feature_id = request
                .target_feature_id
                .expect("validated target feature id should exist");
            let (feature_path, _package_name) = feature_path_and_package(&feature_id)
                .expect("validated feature path should resolve");
            let allowed_writes = vec![format!("{feature_path}/**")];
            let forbidden_writes = vec![
                format!("{REPO_ROOT}/Cargo.toml"),
                format!("{REPO_ROOT}/Cargo.lock"),
                format!("{REPO_ROOT}/crates/**"),
                format!(
                    "any other feature_packs path outside {}",
                    feature_path
                        .strip_prefix(&format!("{REPO_ROOT}/"))
                        .unwrap_or(&feature_path)
                ),
            ];
            let verification_commands = vec![
                format!("cargo test --manifest-path {feature_path}/Cargo.toml"),
                format!("cargo run -p feature_cli -- show {feature_id}"),
                format!("cargo run -p feature_cli -- test {feature_id}"),
            ];
            Ok(GeneratedPromptPacket {
                target_kind: PromptTargetKind::FeatureWorker,
                title: format!("Feature Worker Packet: {feature_id}"),
                prompt_text: format!(
                    "Implement one isolated feature in {REPO_ROOT}.\n\nFeature:\n{feature_id}\n\nLocation:\n{feature_path}\n\nGoal:\n{}\n\nAudience:\n{}\n\nAllowed writes:\n- {feature_path}/**\n\nForbidden writes:\n- {REPO_ROOT}/Cargo.toml\n- {REPO_ROOT}/Cargo.lock\n- {REPO_ROOT}/crates/**\n- any other feature_packs path outside {}\n\nRequired files:\n- Cargo.toml\n- feature.toml\n- README.md\n- src/lib.rs\n- tests/contract_tests.rs\n- fixtures/*\n\nVerification:\n- {}\n- {}\n- {}\n\nDo not edit shared files. Stop after this one feature.",
                    spec.product_intent,
                    spec.audience,
                    feature_path
                        .strip_prefix(&format!("{REPO_ROOT}/"))
                        .unwrap_or(&feature_path),
                    verification_commands[0],
                    verification_commands[1],
                    verification_commands[2],
                ),
                verification_commands,
                allowed_writes,
                forbidden_writes,
            })
        }
        PromptTargetKind::Integrator => {
            let spec = request.spec.expect("validated spec should exist");
            let wave_feature_ids = request.wave_feature_ids;
            let verification_commands = vec![
                "cargo fmt --all --check".into(),
                "cargo test --workspace".into(),
                "cargo run -p feature_cli -- list".into(),
            ];
            Ok(GeneratedPromptPacket {
                target_kind: PromptTargetKind::Integrator,
                title: "Integrator Packet: Assembly Slice".into(),
                prompt_text: format!(
                    "Integrate the completed assembly slice in {REPO_ROOT}.\n\nGoal:\n{}\n\nFeatures to integrate:\n- {}\n\nAllowed writes:\n- {REPO_ROOT}/Cargo.toml\n- {REPO_ROOT}/Cargo.lock\n\nForbidden writes:\n- {REPO_ROOT}/crates/feature_lab_ui/**\n- {REPO_ROOT}/crates/feature_registry/**\n- worker feature crate internals unless a compile fix is absolutely required\n\nRules:\n- do not redesign worker crates\n- batch workspace member additions once\n- preserve isolated feature contracts\n\nVerification:\n- {}\n- {}\n- {}\n",
                    spec.product_intent,
                    wave_feature_ids.join("\n- "),
                    verification_commands[0],
                    verification_commands[1],
                    verification_commands[2],
                ),
                verification_commands,
                allowed_writes: vec![
                    format!("{REPO_ROOT}/Cargo.toml"),
                    format!("{REPO_ROOT}/Cargo.lock"),
                ],
                forbidden_writes: vec![
                    format!("{REPO_ROOT}/crates/feature_lab_ui/**"),
                    format!("{REPO_ROOT}/crates/feature_registry/**"),
                    "worker feature crate internals unless a compile fix is absolutely required"
                        .into(),
                ],
            })
        }
        PromptTargetKind::Reviewer => {
            let spec = request.spec.expect("validated spec should exist");
            let wave_feature_ids = request.wave_feature_ids;
            let mut verification_commands = vec![
                "cargo fmt --all --check".into(),
                "cargo test --workspace".into(),
            ];
            for feature_id in &wave_feature_ids {
                verification_commands
                    .push(format!("cargo run -p feature_cli -- show {feature_id}"));
            }
            for feature_id in &wave_feature_ids {
                verification_commands
                    .push(format!("cargo run -p feature_cli -- test {feature_id}"));
            }
            verification_commands.push("cargo run -p feature_lab_ui".into());

            Ok(GeneratedPromptPacket {
                target_kind: PromptTargetKind::Reviewer,
                title: "Reviewer Packet: Assembly Slice".into(),
                prompt_text: format!(
                    "Review the integrated assembly slice in {REPO_ROOT}.\n\nGoal:\n{}\n\nRules:\n- do not modify files\n- reviewer is read-only\n- report findings by severity, commands run, tests passed or failed, known risks, and merge recommendation\n\nWave features:\n- {}\n",
                    spec.product_intent,
                    wave_feature_ids.join("\n- "),
                ),
                verification_commands,
                allowed_writes: Vec::new(),
                forbidden_writes: vec!["all repo files".into()],
            })
        }
    }
}

pub fn validate_prompt_request(request: &PromptRequest) -> PromptGenerationValidation {
    let mut findings = Vec::new();

    if request.spec.is_none() {
        findings.push(PromptGenerationFinding {
            severity: PromptGenerationSeverity::Error,
            code: "missing_spec".into(),
            path: "spec".into(),
            message: "Prompt generation requires a generated spec.".into(),
        });
    }

    match request.target_kind {
        PromptTargetKind::FeatureWorker => {
            let Some(feature_id) = request.target_feature_id.as_deref() else {
                findings.push(PromptGenerationFinding {
                    severity: PromptGenerationSeverity::Error,
                    code: "missing_target_feature".into(),
                    path: "target_feature_id".into(),
                    message: "Feature worker prompts require target_feature_id.".into(),
                });
                return PromptGenerationValidation { findings };
            };
            if feature_id.trim().is_empty() {
                findings.push(PromptGenerationFinding {
                    severity: PromptGenerationSeverity::Error,
                    code: "blank_target_feature".into(),
                    path: "target_feature_id".into(),
                    message: "Feature worker target_feature_id must not be blank.".into(),
                });
            }
            if feature_path_and_package(feature_id).is_none() {
                findings.push(PromptGenerationFinding {
                    severity: PromptGenerationSeverity::Error,
                    code: "invalid_feature_id".into(),
                    path: "target_feature_id".into(),
                    message: "Feature id must match ui.*, logic.*, or workflow.*".into(),
                });
            }
        }
        PromptTargetKind::Integrator | PromptTargetKind::Reviewer => {
            if request.wave_feature_ids.is_empty() {
                findings.push(PromptGenerationFinding {
                    severity: PromptGenerationSeverity::Error,
                    code: "missing_wave_features".into(),
                    path: "wave_feature_ids".into(),
                    message: "Integrator and reviewer prompts require wave_feature_ids.".into(),
                });
            }
        }
    }

    PromptGenerationValidation { findings }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_feature_worker_fixture() -> &'static str {
    include_str!("../fixtures/feature_worker_request.json")
}

pub fn sample_integrator_fixture() -> &'static str {
    include_str!("../fixtures/integrator_request.json")
}

pub fn sample_reviewer_fixture() -> &'static str {
    include_str!("../fixtures/reviewer_request.json")
}

pub fn parse_prompt_request(raw: &str) -> Result<PromptRequest, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_feature_worker_request() -> Result<PromptRequest, String> {
    parse_prompt_request(sample_feature_worker_fixture())
}

pub fn sample_integrator_request() -> Result<PromptRequest, String> {
    parse_prompt_request(sample_integrator_fixture())
}

pub fn sample_reviewer_request() -> Result<PromptRequest, String> {
    parse_prompt_request(sample_reviewer_fixture())
}

fn feature_path_and_package(feature_id: &str) -> Option<(String, String)> {
    let (category, name) = feature_id.split_once('.')?;
    let directory = match category {
        "ui" => "ui",
        "logic" => "logic",
        "workflow" => "workflows",
        _ => return None,
    };
    Some((
        format!("{REPO_ROOT}/feature_packs/{directory}/{name}"),
        name.to_string(),
    ))
}
