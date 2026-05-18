use feature_core::parse_feature_manifest;
use timer_basic::{FEATURE_ID, TimerState, TimerStatus, sample_fixture, sample_state};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "timer_basic");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec!["duration_seconds", "timer_commands", "elapsed_seconds"]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "total_seconds",
            "remaining_seconds",
            "status",
            "progress_ratio",
            "formatted_remaining",
        ]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["total_seconds"].as_u64(), Some(1500));
    assert_eq!(fixture["status"].as_str(), Some("idle"));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.total_seconds, 1500);
    assert_eq!(state.remaining_seconds, 1500);
    assert_eq!(state.status, TimerStatus::Idle);
}

#[test]
fn start_pause_resume_reset_transitions_are_correct() {
    let mut state = TimerState::new(300);
    assert_eq!(state.status, TimerStatus::Idle);

    state.start();
    assert_eq!(state.status, TimerStatus::Running);

    state.tick(60);
    assert_eq!(state.remaining_seconds, 240);

    state.pause();
    assert_eq!(state.status, TimerStatus::Paused);

    state.resume();
    assert_eq!(state.status, TimerStatus::Running);

    state.reset();
    assert_eq!(state.status, TimerStatus::Idle);
    assert_eq!(state.remaining_seconds, 300);
}

#[test]
fn tick_on_idle_or_paused_does_nothing() {
    let mut state = TimerState::new(120);
    state.tick(30);
    assert_eq!(state.remaining_seconds, 120);
    assert_eq!(state.status, TimerStatus::Idle);

    state.start();
    state.pause();
    state.tick(30);
    assert_eq!(state.remaining_seconds, 120);
    assert_eq!(state.status, TimerStatus::Paused);
}

#[test]
fn completion_clamps_at_zero_and_start_from_completed_restarts() {
    let mut state = TimerState::new(90);
    state.start();
    state.tick(120);

    assert_eq!(state.remaining_seconds, 0);
    assert_eq!(state.status, TimerStatus::Completed);
    assert!(state.is_complete());

    state.start();
    assert_eq!(state.status, TimerStatus::Running);
    assert_eq!(state.remaining_seconds, 90);
}

#[test]
fn set_duration_resets_to_idle() {
    let mut state = TimerState::new(200);
    state.start();
    state.tick(50);
    state.pause();

    state.set_duration(600);
    assert_eq!(state.total_seconds, 600);
    assert_eq!(state.remaining_seconds, 600);
    assert_eq!(state.status, TimerStatus::Idle);
}

#[test]
fn formatted_output_is_deterministic() {
    assert_eq!(TimerState::new(125).formatted_remaining(), "02:05");
    assert_eq!(TimerState::new(3661).formatted_remaining(), "01:01:01");
}

#[test]
fn progress_ratio_is_clamped() {
    let mut state = TimerState::new(100);
    assert_eq!(state.progress_ratio(), 0.0);

    state.start();
    state.tick(40);
    assert_eq!(state.progress_ratio(), 0.4);

    state.tick(100);
    assert_eq!(state.progress_ratio(), 1.0);

    let invalid = TimerState {
        total_seconds: 10,
        remaining_seconds: 99,
        status: TimerStatus::Running,
    };
    assert_eq!(invalid.progress_ratio(), 0.0);
}
