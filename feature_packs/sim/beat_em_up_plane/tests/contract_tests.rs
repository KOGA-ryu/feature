use actor_state::ActorState;
use beat_em_up_plane::{
    BeatEmUpBounds, BeatEmUpPlaneState, FEATURE_ID, PlaneActor, sample_fixture, sample_state,
};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "beat_em_up_plane");
    assert_eq!(manifest.kind.to_string(), "simulation_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["actors"].as_array().map(Vec::len), Some(3));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.actors.len(), 3);
    assert_eq!(state.bounds.min_x, -20);
}

#[test]
fn bounds_clamping_is_enforced() {
    let state = BeatEmUpPlaneState::new(
        BeatEmUpBounds {
            min_x: -5,
            max_x: 5,
            min_depth: -2,
            max_depth: 2,
        },
        vec![PlaneActor {
            id: 1,
            label: "player".into(),
            state: ActorState {
                position_x: 99,
                position_depth: -99,
                ..ActorState::new(5, 1)
            },
        }],
    );

    let actor = state.actor(1).expect("actor should exist");
    assert_eq!(actor.state.position_x, 5);
    assert_eq!(actor.state.position_depth, -2);
}

#[test]
fn deterministic_movement_keeps_actor_order() {
    let mut state = sample_state().expect("sample state should load");
    assert!(state.move_actor(2, 100, -10));
    assert_eq!(
        state
            .actors
            .iter()
            .map(|actor| actor.id)
            .collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    let actor = state.actor(2).expect("actor should exist");
    assert_eq!(actor.state.position_x, 30);
    assert_eq!(actor.state.position_depth, -4);
}

#[test]
fn depth_proximity_query_is_deterministic() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.engagement_candidates(1, 1), vec![2]);
    assert_eq!(state.engagement_candidates(1, 5), vec![2, 3]);
}
