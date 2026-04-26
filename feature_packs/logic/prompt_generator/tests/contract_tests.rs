use feature_core::parse_feature_manifest;
use prompt_generator::{
    FEATURE_ID, PromptRequest, PromptTargetKind, generate_prompt, sample_feature_worker_request,
    sample_integrator_request, sample_reviewer_request,
};

#[test]
fn worker_prompt_includes_feature_local_allowed_writes_and_shared_forbiddance() {
    let request = sample_feature_worker_request().expect("worker fixture should parse");
    let packet = generate_prompt(request).expect("worker prompt should generate");

    assert_eq!(packet.target_kind, PromptTargetKind::FeatureWorker);
    assert!(
        packet
            .allowed_writes
            .iter()
            .any(|path| path.ends_with("feature_packs/logic/spec_generator/**"))
    );
    assert!(
        packet
            .forbidden_writes
            .iter()
            .any(|path| path.contains("Cargo.toml"))
    );
    assert!(packet.prompt_text.contains("Required files"));
}

#[test]
fn integrator_prompt_includes_only_shared_choke_points() {
    let request = sample_integrator_request().expect("integrator fixture should parse");
    let packet = generate_prompt(request).expect("integrator prompt should generate");

    assert_eq!(packet.target_kind, PromptTargetKind::Integrator);
    assert_eq!(packet.allowed_writes.len(), 2);
    assert!(
        packet
            .forbidden_writes
            .iter()
            .any(|path| path.contains("feature_lab_ui"))
    );
}

#[test]
fn reviewer_prompt_is_read_only() {
    let request = sample_reviewer_request().expect("reviewer fixture should parse");
    let packet = generate_prompt(request).expect("reviewer prompt should generate");

    assert_eq!(packet.target_kind, PromptTargetKind::Reviewer);
    assert!(packet.allowed_writes.is_empty());
    assert!(packet.prompt_text.contains("do not modify files"));
    assert!(
        packet
            .verification_commands
            .iter()
            .any(|command| command == "cargo run -p feature_lab_ui")
    );
}

#[test]
fn missing_spec_input_fails() {
    let request = PromptRequest {
        target_kind: PromptTargetKind::Integrator,
        spec: None,
        target_feature_id: None,
        wave_feature_ids: vec!["logic.spec_generator".into()],
    };
    let validation = generate_prompt(request).expect_err("missing spec should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "spec")
    );
}

#[test]
fn prompt_output_is_deterministic() {
    let request = sample_feature_worker_request().expect("worker fixture should parse");
    let left = generate_prompt(request.clone()).expect("worker prompt should generate");
    let right = generate_prompt(request).expect("worker prompt should generate");

    assert_eq!(left, right);
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "prompt_generator");
    assert_eq!(manifest.inputs.items.len(), 4);
    assert_eq!(manifest.outputs.items.len(), 3);
}
