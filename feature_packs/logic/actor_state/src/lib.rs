use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.actor_state";
const DEFAULT_RECOVERY_ON_LIFE_LOSS: u32 = 3;
const DEFAULT_INVULNERABILITY_ON_LIFE_LOSS: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorFacing {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorActionState {
    Idle,
    Moving,
    Attacking,
    Hitstun,
    KnockedDown,
    Crashed,
    Defeated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorState {
    pub max_health: u32,
    pub health: u32,
    pub lives: u32,
    pub position_x: i32,
    pub position_depth: i32,
    pub facing: ActorFacing,
    pub action_state: ActorActionState,
    pub invulnerable_ticks: u32,
    pub recovery_ticks: u32,
}

impl Default for ActorFacing {
    fn default() -> Self {
        Self::Right
    }
}

impl Default for ActorActionState {
    fn default() -> Self {
        Self::Idle
    }
}

impl Default for ActorState {
    fn default() -> Self {
        Self::new(5, 1)
    }
}

impl ActorState {
    pub fn new(max_health: u32, lives: u32) -> Self {
        let max_health = max_health.max(1);
        if lives == 0 {
            return Self {
                max_health,
                health: 0,
                lives: 0,
                position_x: 0,
                position_depth: 0,
                facing: ActorFacing::default(),
                action_state: ActorActionState::Defeated,
                invulnerable_ticks: 0,
                recovery_ticks: 0,
            };
        }

        Self {
            max_health,
            health: max_health,
            lives,
            position_x: 0,
            position_depth: 0,
            facing: ActorFacing::default(),
            action_state: ActorActionState::Idle,
            invulnerable_ticks: 0,
            recovery_ticks: 0,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let mut state: Self = serde_json::from_str(raw).map_err(|error| error.to_string())?;
        state.normalize();
        Ok(state)
    }

    pub fn set_position(&mut self, position_x: i32, position_depth: i32) {
        self.position_x = position_x;
        self.position_depth = position_depth;
    }

    pub fn set_facing(&mut self, facing: ActorFacing) {
        self.facing = facing;
    }

    pub fn enter_action_state(&mut self, action_state: ActorActionState, recovery_ticks: u32) {
        if self.is_defeated() {
            return;
        }
        self.action_state = action_state;
        self.recovery_ticks = recovery_ticks;
    }

    pub fn grant_invulnerability(&mut self, ticks: u32) {
        self.invulnerable_ticks = self.invulnerable_ticks.max(ticks);
    }

    pub fn tick_state(&mut self, elapsed_ticks: u32) {
        if elapsed_ticks == 0 {
            return;
        }

        self.invulnerable_ticks = self.invulnerable_ticks.saturating_sub(elapsed_ticks);
        self.recovery_ticks = self.recovery_ticks.saturating_sub(elapsed_ticks);

        if !self.is_defeated()
            && self.recovery_ticks == 0
            && matches!(
                self.action_state,
                ActorActionState::Attacking
                    | ActorActionState::Hitstun
                    | ActorActionState::KnockedDown
                    | ActorActionState::Crashed
            )
        {
            self.action_state = ActorActionState::Idle;
        }
    }

    pub fn apply_damage(&mut self, amount: u32) -> u32 {
        if amount == 0 || self.is_invulnerable() || self.is_defeated() {
            return 0;
        }

        let damage_applied = amount.min(self.health);
        self.health = self.health.saturating_sub(damage_applied);

        if self.health == 0 {
            if self.lives > 1 {
                self.lives -= 1;
                self.health = self.max_health;
                self.action_state = ActorActionState::KnockedDown;
                self.recovery_ticks = DEFAULT_RECOVERY_ON_LIFE_LOSS;
                self.invulnerable_ticks = self
                    .invulnerable_ticks
                    .max(DEFAULT_INVULNERABILITY_ON_LIFE_LOSS);
            } else {
                self.lives = 0;
                self.action_state = ActorActionState::Defeated;
                self.recovery_ticks = 0;
                self.invulnerable_ticks = 0;
            }
        }

        damage_applied
    }

    pub fn is_invulnerable(&self) -> bool {
        self.invulnerable_ticks > 0
    }

    pub fn is_defeated(&self) -> bool {
        self.lives == 0 || self.action_state == ActorActionState::Defeated
    }

    fn normalize(&mut self) {
        self.max_health = self.max_health.max(1);
        self.health = self.health.min(self.max_health);

        if self.lives == 0 {
            self.health = 0;
            self.action_state = ActorActionState::Defeated;
            self.invulnerable_ticks = 0;
            self.recovery_ticks = 0;
        } else if self.health == 0 {
            self.health = self.max_health;
        }
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

pub fn sample_state() -> Result<ActorState, String> {
    ActorState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
