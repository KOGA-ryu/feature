use ai_interaction_event::InteractionOutcome;
use duel_arena::{FEATURE_ID, sample_fixture, sample_state};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "duel_arena");
    assert_eq!(manifest.kind.to_string(), "simulation_pattern");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["arena_max_x"].as_i64(), Some(12));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert!(state.in_contact_range());
}

#[test]
fn push_and_tick_clamp_inside_arena() {
    let mut state = sample_state().expect("sample state should load");
    assert!(state.apply_push(1, -10));
    state.tick(1);
    assert_eq!(state.player.position_x, 0);
}

#[test]
fn exchange_applies_damage_and_knockback() {
    let mut state = sample_state().expect("sample state should load");
    let outcome = state.resolve_exchange(1, 1, 2, InteractionOutcome::PunishWindowHit);
    assert!(outcome.is_none());
    assert_eq!(state.target.actor.health, 3);
    assert_eq!(state.target.velocity_x, 2);
}

#[test]
fn defeating_exchange_reports_outcome() {
    let mut state = sample_state().expect("sample state should load");
    let outcome = state
        .resolve_exchange(1, 5, 2, InteractionOutcome::PunishWindowHit)
        .expect("outcome should resolve");

    assert_eq!(outcome.winner_id, Some(1));
    assert_eq!(state.last_outcome.as_ref(), Some(&outcome));
}
