use feature_core::parse_feature_manifest;
use runbook_panel::{
    FEATURE_ID, RunbookState, RunbookStep, RunbookStepStatus, sample_fixture, sample_state,
};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "runbook_panel");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec!["title", "steps", "selected_index", "status_action"]
    );
    assert_eq!(
        manifest.outputs.items,
        vec!["selected_step", "steps", "counts"]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["title"].as_str(), Some("Feature Wave Runbook"));
    assert_eq!(fixture["steps"].as_array().map(Vec::len), Some(6));
}

#[test]
fn sample_state_loads_repo_relevant_operator_flow() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.title, "Feature Wave Runbook");
    assert_eq!(state.selected_index, 1);
    assert_eq!(
        state.selected_step().map(|step| step.id.as_str()),
        Some("implement")
    );

    let counts = state.counts();
    assert_eq!(counts.pending, 5);
    assert_eq!(counts.active, 1);
    assert_eq!(counts.completed, 0);
    assert_eq!(counts.blocked, 0);
}

#[test]
fn selection_clamps_safely() {
    let mut state = RunbookState::new(
        "Clamp test",
        vec![
            RunbookStep {
                id: "one".into(),
                title: "One".into(),
                detail: "First".into(),
                status: RunbookStepStatus::Pending,
            },
            RunbookStep {
                id: "two".into(),
                title: "Two".into(),
                detail: "Second".into(),
                status: RunbookStepStatus::Pending,
            },
        ],
    );

    state.set_selected_index(99);
    assert_eq!(state.selected_index, 1);
    assert_eq!(
        state.selected_step().map(|step| step.id.as_str()),
        Some("two")
    );

    state.move_down();
    assert_eq!(state.selected_index, 1);

    state.move_up();
    state.move_up();
    assert_eq!(state.selected_index, 0);
    assert_eq!(
        state.selected_step().map(|step| step.id.as_str()),
        Some("one")
    );

    let mut empty = RunbookState::new("Empty", vec![]);
    empty.set_selected_index(7);
    empty.move_down();
    empty.move_up();
    assert_eq!(empty.selected_index, 0);
    assert!(empty.selected_step().is_none());
}

#[test]
fn only_one_step_can_be_active_at_a_time() {
    let mut state = sample_state().expect("sample state should load");
    state.set_selected_index(4);
    state.mark_selected_active();

    let active_steps = state
        .steps
        .iter()
        .filter(|step| step.status == RunbookStepStatus::Active)
        .map(|step| step.id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(active_steps, vec!["review"]);
    assert_eq!(state.steps[1].status, RunbookStepStatus::Pending);
}

#[test]
fn completed_and_blocked_persist_until_reset() {
    let mut state = sample_state().expect("sample state should load");
    state.set_selected_index(0);
    state.mark_selected_completed();
    state.set_selected_index(5);
    state.mark_selected_blocked();
    state.set_selected_index(2);
    state.mark_selected_active();

    assert_eq!(state.steps[0].status, RunbookStepStatus::Completed);
    assert_eq!(state.steps[5].status, RunbookStepStatus::Blocked);
    assert_eq!(state.steps[2].status, RunbookStepStatus::Active);

    state.reset_statuses();
    assert!(
        state
            .steps
            .iter()
            .all(|step| step.status == RunbookStepStatus::Pending)
    );
}

#[test]
fn counts_stay_correct_across_transitions() {
    let mut state = sample_state().expect("sample state should load");
    let initial = state.counts();
    assert_eq!(initial.pending, 5);
    assert_eq!(initial.active, 1);

    state.set_selected_index(0);
    state.mark_selected_completed();
    let after_completed = state.counts();
    assert_eq!(after_completed.pending, 4);
    assert_eq!(after_completed.active, 1);
    assert_eq!(after_completed.completed, 1);
    assert_eq!(after_completed.blocked, 0);

    state.set_selected_index(5);
    state.mark_selected_blocked();
    let after_blocked = state.counts();
    assert_eq!(after_blocked.pending, 3);
    assert_eq!(after_blocked.active, 1);
    assert_eq!(after_blocked.completed, 1);
    assert_eq!(after_blocked.blocked, 1);

    state.set_selected_index(4);
    state.mark_selected_active();
    let after_active_move = state.counts();
    assert_eq!(after_active_move.pending, 3);
    assert_eq!(after_active_move.active, 1);
    assert_eq!(after_active_move.completed, 1);
    assert_eq!(after_active_move.blocked, 1);
    assert_eq!(state.steps[1].status, RunbookStepStatus::Pending);
}
