use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "sim.runner_track";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunnerLane {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunnerSpeedState {
    Slow,
    Cruising,
    Fast,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunnerObstacle {
    pub id: u64,
    pub label: String,
    pub lane: RunnerLane,
    pub distance: u32,
    pub resolved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunnerTrackState {
    pub current_lane: RunnerLane,
    pub speed_state: RunnerSpeedState,
    pub distance_travelled: u32,
    pub obstacles: Vec<RunnerObstacle>,
}

impl Default for RunnerLane {
    fn default() -> Self {
        Self::Center
    }
}

impl Default for RunnerSpeedState {
    fn default() -> Self {
        Self::Cruising
    }
}

impl Default for RunnerTrackState {
    fn default() -> Self {
        Self::new(
            RunnerLane::default(),
            RunnerSpeedState::default(),
            Vec::new(),
        )
    }
}

impl RunnerTrackState {
    pub fn new(
        current_lane: RunnerLane,
        speed_state: RunnerSpeedState,
        obstacles: Vec<RunnerObstacle>,
    ) -> Self {
        Self {
            current_lane,
            speed_state,
            distance_travelled: 0,
            obstacles,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw).map_err(|error| error.to_string())
    }

    pub fn shift_left(&mut self) -> bool {
        let next = match self.current_lane {
            RunnerLane::Left => return false,
            RunnerLane::Center => RunnerLane::Left,
            RunnerLane::Right => RunnerLane::Center,
        };
        self.current_lane = next;
        true
    }

    pub fn shift_right(&mut self) -> bool {
        let next = match self.current_lane {
            RunnerLane::Left => RunnerLane::Center,
            RunnerLane::Center => RunnerLane::Right,
            RunnerLane::Right => return false,
        };
        self.current_lane = next;
        true
    }

    pub fn set_speed_state(&mut self, speed_state: RunnerSpeedState) {
        self.speed_state = speed_state;
    }

    pub fn advance(&mut self, distance: u32) {
        self.distance_travelled = self.distance_travelled.saturating_add(distance);
    }

    pub fn detect_collisions(&mut self) -> Vec<RunnerObstacle> {
        let mut collisions = Vec::new();
        for obstacle in &mut self.obstacles {
            if obstacle.resolved {
                continue;
            }
            if obstacle.lane == self.current_lane && obstacle.distance <= self.distance_travelled {
                obstacle.resolved = true;
                collisions.push(obstacle.clone());
            }
        }
        collisions
    }

    pub fn clear_passed_obstacles(&mut self) -> usize {
        let before = self.obstacles.len();
        self.obstacles
            .retain(|obstacle| obstacle.distance > self.distance_travelled);
        before.saturating_sub(self.obstacles.len())
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

pub fn sample_state() -> Result<RunnerTrackState, String> {
    RunnerTrackState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}
