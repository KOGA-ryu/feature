use prompt_generator::{PromptRequest, PromptTargetKind, generate_prompt};
use spec_generator::sample_generated_spec;

#[test]
fn worker_prompt_accepts_sim_feature_ids() {
    let packet = generate_prompt(PromptRequest {
        target_kind: PromptTargetKind::FeatureWorker,
        spec: Some(sample_generated_spec().expect("spec should generate")),
        target_feature_id: Some("sim.runner_track".into()),
        wave_feature_ids: Vec::new(),
        allow_feature_lab_ui_writes: false,
    })
    .expect("sim worker prompt should generate");

    assert!(
        packet
            .allowed_writes
            .iter()
            .any(|path| path.ends_with("feature_packs/sim/runner_track/**"))
    );
}
