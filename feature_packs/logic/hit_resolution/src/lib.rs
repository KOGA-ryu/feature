use actor_state::{ActorActionState, ActorState};
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.hit_resolution";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HitClass {
    Light,
    Heavy,
    Knockdown,
    Crash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HitRequest {
    pub damage: u32,
    pub class: HitClass,
    pub invulnerability_ticks: u32,
    pub recovery_ticks: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HitResult {
    pub damage_applied: u32,
    pub target_health: u32,
    pub target_lives: u32,
    pub entered_state: ActorActionState,
    pub defeated: bool,
    pub life_lost: bool,
    pub ignored: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SampleHitScenario {
    pub target: ActorState,
    pub request: HitRequest,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HitResolution;

impl Default for HitRequest {
    fn default() -> Self {
        Self {
            damage: 1,
            class: HitClass::Light,
            invulnerability_ticks: 2,
            recovery_ticks: 2,
        }
    }
}

impl Default for HitClass {
    fn default() -> Self {
        Self::Light
    }
}

impl HitResolution {
    pub fn apply(target: &mut ActorState, request: &HitRequest) -> HitResult {
        if target.is_invulnerable() || target.is_defeated() {
            return HitResult {
                damage_applied: 0,
                target_health: target.health,
                target_lives: target.lives,
                entered_state: target.action_state,
                defeated: target.is_defeated(),
                life_lost: false,
                ignored: true,
            };
        }

        let previous_lives = target.lives;
        let damage_applied = target.apply_damage(request.damage);
        let life_lost = target.lives < previous_lives;

        if !target.is_defeated() {
            let next_state = match request.class {
                HitClass::Light | HitClass::Heavy => ActorActionState::Hitstun,
                HitClass::Knockdown => ActorActionState::KnockedDown,
                HitClass::Crash => ActorActionState::Crashed,
            };
            let recovery_ticks = if life_lost {
                request.recovery_ticks.max(3)
            } else {
                request.recovery_ticks
            };
            target.enter_action_state(next_state, recovery_ticks);
            target.grant_invulnerability(request.invulnerability_ticks);
        }

        HitResult {
            damage_applied,
            target_health: target.health,
            target_lives: target.lives,
            entered_state: target.action_state,
            defeated: target.is_defeated(),
            life_lost,
            ignored: false,
        }
    }
}

impl SampleHitScenario {
    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw).map_err(|error| error.to_string())
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_input.json")
}

pub fn sample_scenario() -> Result<SampleHitScenario, String> {
    SampleHitScenario::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
