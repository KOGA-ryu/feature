use feature_core::parse_feature_manifest;
use runner_track::{
    FEATURE_ID, RunnerLane, RunnerSpeedState, RunnerTrackState, sample_fixture, sample_state,
};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "runner_track");
    assert_eq!(manifest.kind.to_string(), "simulation_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["obstacles"].as_array().map(Vec::len), Some(3));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.current_lane, RunnerLane::Center);
    assert_eq!(state.speed_state, RunnerSpeedState::Cruising);
}

#[test]
fn lane_shifting_is_bounded() {
    let mut state = RunnerTrackState::default();
    assert!(state.shift_left());
    assert_eq!(state.current_lane, RunnerLane::Left);
    assert!(!state.shift_left());
    assert!(state.shift_right());
    assert_eq!(state.current_lane, RunnerLane::Center);
    assert!(state.shift_right());
    assert_eq!(state.current_lane, RunnerLane::Right);
    assert!(!state.shift_right());
}

#[test]
fn forward_advance_and_collision_detection_are_deterministic() {
    let mut state = sample_state().expect("sample state should load");
    state.advance(5);
    let collisions = state.detect_collisions();
    assert_eq!(collisions.len(), 1);
    assert_eq!(collisions[0].id, 1);
    assert!(state.obstacles[0].resolved);
}

#[test]
fn passed_obstacle_cleanup_removes_behind_entries() {
    let mut state = sample_state().expect("sample state should load");
    state.advance(9);
    let collisions = state.detect_collisions();
    assert_eq!(collisions.len(), 1);
    let removed = state.clear_passed_obstacles();
    assert_eq!(removed, 2);
    assert_eq!(
        state
            .obstacles
            .iter()
            .map(|obstacle| obstacle.id)
            .collect::<Vec<_>>(),
        vec![3]
    );
}
