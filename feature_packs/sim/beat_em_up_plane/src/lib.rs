use actor_state::ActorState;
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "sim.beat_em_up_plane";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeatEmUpBounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_depth: i32,
    pub max_depth: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaneActor {
    pub id: u64,
    pub label: String,
    pub state: ActorState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeatEmUpPlaneState {
    pub bounds: BeatEmUpBounds,
    pub actors: Vec<PlaneActor>,
}

impl Default for BeatEmUpBounds {
    fn default() -> Self {
        Self {
            min_x: 0,
            max_x: 100,
            min_depth: -10,
            max_depth: 10,
        }
    }
}

impl Default for BeatEmUpPlaneState {
    fn default() -> Self {
        Self::new(BeatEmUpBounds::default(), Vec::new())
    }
}

impl BeatEmUpPlaneState {
    pub fn new(bounds: BeatEmUpBounds, actors: Vec<PlaneActor>) -> Self {
        let mut state = Self { bounds, actors };
        state.normalize();
        state
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let state: Self = serde_json::from_str(raw).map_err(|error| error.to_string())?;
        Ok(Self::new(state.bounds, state.actors))
    }

    pub fn actor(&self, actor_id: u64) -> Option<&PlaneActor> {
        self.actors.iter().find(|actor| actor.id == actor_id)
    }

    pub fn move_actor(&mut self, actor_id: u64, delta_x: i32, delta_depth: i32) -> bool {
        let Some(actor) = self.actors.iter_mut().find(|actor| actor.id == actor_id) else {
            return false;
        };
        actor.state.position_x = clamp_value(
            actor.state.position_x.saturating_add(delta_x),
            self.bounds.min_x,
            self.bounds.max_x,
        );
        actor.state.position_depth = clamp_value(
            actor.state.position_depth.saturating_add(delta_depth),
            self.bounds.min_depth,
            self.bounds.max_depth,
        );
        true
    }

    pub fn engagement_candidates(&self, actor_id: u64, max_depth_delta: i32) -> Vec<u64> {
        let Some(source) = self.actor(actor_id) else {
            return Vec::new();
        };

        self.actors
            .iter()
            .filter(|actor| actor.id != actor_id)
            .filter(|actor| {
                (actor.state.position_depth - source.state.position_depth).abs() <= max_depth_delta
            })
            .map(|actor| actor.id)
            .collect()
    }

    fn normalize(&mut self) {
        if self.bounds.min_x > self.bounds.max_x {
            std::mem::swap(&mut self.bounds.min_x, &mut self.bounds.max_x);
        }
        if self.bounds.min_depth > self.bounds.max_depth {
            std::mem::swap(&mut self.bounds.min_depth, &mut self.bounds.max_depth);
        }

        for actor in &mut self.actors {
            actor.state.position_x =
                clamp_value(actor.state.position_x, self.bounds.min_x, self.bounds.max_x);
            actor.state.position_depth = clamp_value(
                actor.state.position_depth,
                self.bounds.min_depth,
                self.bounds.max_depth,
            );
        }
    }
}

fn clamp_value(value: i32, minimum: i32, maximum: i32) -> i32 {
    value.clamp(minimum, maximum)
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

pub fn sample_state() -> Result<BeatEmUpPlaneState, String> {
    BeatEmUpPlaneState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
