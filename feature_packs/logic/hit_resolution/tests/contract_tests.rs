use actor_state::{ActorActionState, ActorState};
use feature_core::parse_feature_manifest;
use hit_resolution::{
    FEATURE_ID, HitClass, HitRequest, HitResolution, sample_fixture, sample_scenario,
};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "hit_resolution");
    assert_eq!(manifest.kind.to_string(), "logic_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(manifest.inputs.items, vec!["target_actor", "hit_request"]);
    assert_eq!(manifest.outputs.items, vec!["target_actor", "hit_result"]);
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["request"]["class"].as_str(), Some("heavy"));
}

#[test]
fn sample_scenario_loads() {
    let scenario = sample_scenario().expect("sample scenario should load");
    assert_eq!(scenario.target.health, 3);
    assert_eq!(scenario.request.class, HitClass::Heavy);
}

#[test]
fn normal_hit_applies_damage_and_hitstun() {
    let mut target = ActorState::new(5, 1);
    let request = HitRequest {
        damage: 2,
        class: HitClass::Light,
        invulnerability_ticks: 2,
        recovery_ticks: 3,
    };

    let result = HitResolution::apply(&mut target, &request);
    assert_eq!(result.damage_applied, 2);
    assert_eq!(target.health, 3);
    assert_eq!(target.action_state, ActorActionState::Hitstun);
    assert_eq!(target.recovery_ticks, 3);
    assert_eq!(target.invulnerable_ticks, 2);
    assert!(!result.defeated);
    assert!(!result.ignored);
}

#[test]
fn invulnerable_target_ignores_hit() {
    let mut target = ActorState::new(4, 1);
    target.grant_invulnerability(2);

    let result = HitResolution::apply(&mut target, &HitRequest::default());
    assert_eq!(result.damage_applied, 0);
    assert_eq!(target.health, 4);
    assert!(result.ignored);
}

#[test]
fn knockdown_and_crash_enter_expected_states() {
    let mut knockdown_target = ActorState::new(4, 1);
    let knockdown = HitResolution::apply(
        &mut knockdown_target,
        &HitRequest {
            class: HitClass::Knockdown,
            ..HitRequest::default()
        },
    );
    assert_eq!(knockdown.entered_state, ActorActionState::KnockedDown);

    let mut crash_target = ActorState::new(4, 1);
    let crash = HitResolution::apply(
        &mut crash_target,
        &HitRequest {
            class: HitClass::Crash,
            ..HitRequest::default()
        },
    );
    assert_eq!(crash.entered_state, ActorActionState::Crashed);
}

#[test]
fn fatal_hit_reports_defeat() {
    let mut target = ActorState::new(2, 1);
    let request = HitRequest {
        damage: 3,
        class: HitClass::Heavy,
        invulnerability_ticks: 0,
        recovery_ticks: 2,
    };

    let result = HitResolution::apply(&mut target, &request);
    assert_eq!(result.damage_applied, 2);
    assert!(result.defeated);
    assert_eq!(result.entered_state, ActorActionState::Defeated);
    assert_eq!(target.lives, 0);
}
