use actor_state::ActorState;
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.health_hud";
const DAMAGE_FLASH_TICKS: u32 = 3;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthSegment {
    pub index: usize,
    pub filled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HealthHudState {
    pub segments: Vec<HealthSegment>,
    pub lives: u32,
    pub current_health: u32,
    pub max_health: u32,
    pub damage_flash_ticks: u32,
}

impl HealthHudState {
    pub fn from_actor(actor: &ActorState) -> Self {
        let mut state = Self::default();
        state.sync_from_actor(actor);
        state.damage_flash_ticks = 0;
        state
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw).map_err(|error| error.to_string())
    }

    pub fn sync_from_actor(&mut self, actor: &ActorState) {
        if (self.current_health > 0 || self.lives > 0)
            && (actor.health < self.current_health || actor.lives < self.lives)
        {
            self.damage_flash_ticks = DAMAGE_FLASH_TICKS;
        }

        self.current_health = actor.health;
        self.max_health = actor.max_health;
        self.lives = actor.lives;
        self.segments = (0..actor.max_health as usize)
            .map(|index| HealthSegment {
                index,
                filled: index < actor.health as usize,
            })
            .collect();
    }

    pub fn tick_flash(&mut self, elapsed_ticks: u32) {
        self.damage_flash_ticks = self.damage_flash_ticks.saturating_sub(elapsed_ticks);
    }

    pub fn is_flash_active(&self) -> bool {
        self.damage_flash_ticks > 0
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

pub fn sample_state() -> Result<HealthHudState, String> {
    HealthHudState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
