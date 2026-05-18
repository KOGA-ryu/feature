use actor_state::ActorState;
use feature_core::parse_feature_manifest;
use health_hud::{FEATURE_ID, HealthHudState, sample_fixture, sample_state};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "health_hud");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["segments"].as_array().map(Vec::len), Some(5));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.lives, 2);
    assert_eq!(state.current_health, 4);
}

#[test]
fn segment_generation_and_life_display_sync_from_actor() {
    let actor = ActorState::new(5, 2);
    let hud = HealthHudState::from_actor(&actor);
    assert_eq!(hud.segments.len(), 5);
    assert_eq!(
        hud.segments.iter().filter(|segment| segment.filled).count(),
        5
    );
    assert_eq!(hud.lives, 2);
}

#[test]
fn damage_flash_tracks_health_loss_and_ticks_down() {
    let mut actor = ActorState::new(5, 2);
    let mut hud = HealthHudState::from_actor(&actor);
    assert!(!hud.is_flash_active());

    actor.apply_damage(2);
    hud.sync_from_actor(&actor);
    assert!(hud.is_flash_active());
    assert_eq!(hud.current_health, 3);
    assert_eq!(
        hud.segments.iter().filter(|segment| segment.filled).count(),
        3
    );

    hud.tick_flash(2);
    assert!(hud.is_flash_active());
    hud.tick_flash(1);
    assert!(!hud.is_flash_active());
}
