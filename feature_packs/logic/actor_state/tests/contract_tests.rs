use actor_state::{
    ActorActionState, ActorFacing, ActorState, FEATURE_ID, sample_fixture, sample_state,
};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "actor_state");
    assert_eq!(manifest.kind.to_string(), "logic_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec!["health", "lives", "position", "action_state", "timer_ticks"]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "actor_state",
            "defeat_state",
            "invulnerability_state",
            "recovery_state"
        ]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["max_health"].as_u64(), Some(5));
    assert_eq!(fixture["facing"].as_str(), Some("right"));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.max_health, 5);
    assert_eq!(state.health, 4);
    assert_eq!(state.lives, 2);
    assert_eq!(state.facing, ActorFacing::Right);
}

#[test]
fn damage_and_life_loss_follow_rules() {
    let mut state = ActorState::new(4, 2);
    assert_eq!(state.apply_damage(2), 2);
    assert_eq!(state.health, 2);
    assert_eq!(state.lives, 2);
    assert!(!state.is_defeated());

    assert_eq!(state.apply_damage(2), 2);
    assert_eq!(state.health, 4);
    assert_eq!(state.lives, 1);
    assert_eq!(state.action_state, ActorActionState::KnockedDown);
    assert!(!state.is_defeated());

    assert_eq!(state.apply_damage(4), 0);
    state.tick_state(3);
    assert_eq!(state.apply_damage(4), 4);
    assert_eq!(state.lives, 0);
    assert_eq!(state.health, 0);
    assert!(state.is_defeated());
}

#[test]
fn invulnerability_blocks_damage_and_ticks_down() {
    let mut state = ActorState::new(5, 1);
    state.grant_invulnerability(3);
    assert!(state.is_invulnerable());
    assert_eq!(state.apply_damage(2), 0);
    assert_eq!(state.health, 5);

    state.tick_state(2);
    assert!(state.is_invulnerable());
    state.tick_state(1);
    assert!(!state.is_invulnerable());
    assert_eq!(state.apply_damage(2), 2);
    assert_eq!(state.health, 3);
}

#[test]
fn recovery_ticks_return_transient_action_states_to_idle() {
    let mut state = ActorState::new(5, 1);
    state.enter_action_state(ActorActionState::Hitstun, 2);
    state.tick_state(1);
    assert_eq!(state.action_state, ActorActionState::Hitstun);
    assert_eq!(state.recovery_ticks, 1);

    state.tick_state(1);
    assert_eq!(state.action_state, ActorActionState::Idle);
    assert_eq!(state.recovery_ticks, 0);
}
