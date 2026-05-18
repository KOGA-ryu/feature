use actor_state::{ActorActionState, ActorState};
use ai_interaction_event::InteractionOutcome;
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "sim.duel_arena";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DuelArenaActor {
    pub id: u64,
    pub label: String,
    pub actor: ActorState,
    pub position_x: i32,
    pub velocity_x: i32,
}

impl Default for DuelArenaActor {
    fn default() -> Self {
        Self {
            id: 1,
            label: "player".into(),
            actor: ActorState::new(5, 1),
            position_x: 4,
            velocity_x: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DuelArenaOutcome {
    pub winner_id: Option<u64>,
    pub decisive_event: Option<InteractionOutcome>,
    pub resolved_at_tick: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DuelArenaState {
    pub arena_min_x: i32,
    pub arena_max_x: i32,
    pub player: DuelArenaActor,
    pub target: DuelArenaActor,
    pub elapsed_ticks: u32,
    pub last_outcome: Option<DuelArenaOutcome>,
}

impl Default for DuelArenaState {
    fn default() -> Self {
        let player = DuelArenaActor::default();
        let target = DuelArenaActor {
            id: 2,
            label: "duelist".into(),
            actor: ActorState::new(4, 1),
            position_x: 6,
            velocity_x: 0,
        };
        Self::new(0, 12, player, target)
    }
}

impl DuelArenaState {
    pub fn new(
        arena_min_x: i32,
        arena_max_x: i32,
        player: DuelArenaActor,
        target: DuelArenaActor,
    ) -> Self {
        Self {
            arena_min_x,
            arena_max_x: arena_max_x.max(arena_min_x + 1),
            player,
            target,
            elapsed_ticks: 0,
            last_outcome: None,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw).map_err(|error| error.to_string())
    }

    pub fn apply_push(&mut self, actor_id: u64, impulse_x: i32) -> bool {
        if let Some(actor) = self.actor_mut(actor_id) {
            actor.velocity_x = actor.velocity_x.saturating_add(impulse_x);
            return true;
        }
        false
    }

    pub fn tick(&mut self, elapsed_ticks: u32) {
        for _ in 0..elapsed_ticks {
            self.elapsed_ticks = self.elapsed_ticks.saturating_add(1);
            step_actor(self.arena_min_x, self.arena_max_x, &mut self.player);
            step_actor(self.arena_min_x, self.arena_max_x, &mut self.target);
        }
    }

    pub fn resolve_exchange(
        &mut self,
        attacker_id: u64,
        damage: u32,
        knockback: i32,
        decisive_event: InteractionOutcome,
    ) -> Option<DuelArenaOutcome> {
        if self.last_outcome.is_some() || !self.in_contact_range() {
            return None;
        }

        let (attacker_position, defender) = if attacker_id == self.player.id {
            (self.player.position_x, &mut self.target)
        } else if attacker_id == self.target.id {
            (self.target.position_x, &mut self.player)
        } else {
            return None;
        };

        let applied_damage = defender.actor.apply_damage(damage);
        let push_direction: i32 = if attacker_position <= defender.position_x {
            1
        } else {
            -1
        };
        defender.velocity_x = defender
            .velocity_x
            .saturating_add(push_direction.saturating_mul(knockback));

        if !defender.actor.is_defeated() && applied_damage > 0 {
            defender
                .actor
                .enter_action_state(ActorActionState::Hitstun, 2);
        }

        if defender.actor.is_defeated() {
            let outcome = DuelArenaOutcome {
                winner_id: Some(attacker_id),
                decisive_event: Some(decisive_event),
                resolved_at_tick: self.elapsed_ticks,
            };
            self.last_outcome = Some(outcome.clone());
            return Some(outcome);
        }

        None
    }

    pub fn in_contact_range(&self) -> bool {
        (self.player.position_x - self.target.position_x).abs() <= 2
    }

    fn actor_mut(&mut self, actor_id: u64) -> Option<&mut DuelArenaActor> {
        if actor_id == self.player.id {
            Some(&mut self.player)
        } else if actor_id == self.target.id {
            Some(&mut self.target)
        } else {
            None
        }
    }
}

fn step_actor(arena_min_x: i32, arena_max_x: i32, actor: &mut DuelArenaActor) {
    actor.position_x = actor
        .position_x
        .saturating_add(actor.velocity_x)
        .clamp(arena_min_x, arena_max_x);
    actor.velocity_x = actor.velocity_x.saturating_mul(4) / 5;
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

pub fn sample_state() -> Result<DuelArenaState, String> {
    DuelArenaState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
