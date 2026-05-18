# Gameplay Systems

Canonical reusable gameplay-systems index for `/Users/kogaryu/dev/features`.

This lane is for headless-first game mechanics and thin game-facing adapters,
not one-off game shells. The family split is:

- `logic.*`: combat, actor, and rule resolution state
- `sim.*`: movement rails, encounter space, checkpoints, and deterministic simulation
- `ui.*`: HUD and player-facing panels over those systems
- `workflow.*`: future build and review flows for game slices if needed later

## Vertical: `hybrid_brawler_racer`

Battletoads-style hybrid foundation:
- side-view beat-em-up encounter space
- over-the-shoulder runner/racing segments
- checkpoint transition law between modes
- simulation zoom driven by AI interaction grading

Companion integration contract:
- [docs/hybrid_brawler_racer_sprite_integration_contract.md](/Users/kogaryu/dev/features/docs/hybrid_brawler_racer_sprite_integration_contract.md)

## logic.actor_state

- feature id: `logic.actor_state`
- crate path: `feature_packs/logic/actor_state`
- summary: shared actor record with health, lives, facing, positions, and recovery timers
- current status: `tested`
- inputs: `health`, `lives`, `position`, `action_state`, `timer_ticks`
- outputs: `actor_state`, `defeat_state`, `invulnerability_state`, `recovery_state`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/logic/actor_state/Cargo.toml`
  - `cargo run -p feature_cli -- show logic.actor_state`
  - `cargo run -p feature_cli -- test logic.actor_state`
- intended host pattern: reusable actor record shared across combat, runner, checkpoint, and HUD hosts

## logic.hit_resolution

- feature id: `logic.hit_resolution`
- crate path: `feature_packs/logic/hit_resolution`
- summary: deterministic single-target hit application over actor state
- current status: `tested`
- inputs: `target_actor`, `hit_request`
- outputs: `target_actor`, `hit_result`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/logic/hit_resolution/Cargo.toml`
  - `cargo run -p feature_cli -- show logic.hit_resolution`
  - `cargo run -p feature_cli -- test logic.hit_resolution`
- intended host pattern: combat resolution layer after a host has already selected the hit target

## sim.beat_em_up_plane

- feature id: `sim.beat_em_up_plane`
- crate path: `feature_packs/sim/beat_em_up_plane`
- summary: bounded brawler encounter plane with deterministic x/depth movement and engagement queries
- current status: `tested`
- inputs: `bounds`, `actors`, `movement_commands`, `depth_threshold`
- outputs: `actors`, `bounded_positions`, `engagement_candidates`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/sim/beat_em_up_plane/Cargo.toml`
  - `cargo run -p feature_cli -- show sim.beat_em_up_plane`
  - `cargo run -p feature_cli -- test sim.beat_em_up_plane`
- intended host pattern: reusable encounter-space simulation for scrolling brawler segments

## sim.runner_track

- feature id: `sim.runner_track`
- crate path: `feature_packs/sim/runner_track`
- summary: lane-based runner state with forward distance, obstacle stream, and collision detection
- current status: `tested`
- inputs: `lane_commands`, `speed_state`, `forward_distance`, `obstacles`
- outputs: `current_lane`, `distance_travelled`, `collisions`, `remaining_obstacles`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/sim/runner_track/Cargo.toml`
  - `cargo run -p feature_cli -- show sim.runner_track`
  - `cargo run -p feature_cli -- test sim.runner_track`
- intended host pattern: reusable over-the-shoulder runner or escape segment simulation

## sim.crash_recovery

- feature id: `sim.crash_recovery`
- crate path: `feature_packs/sim/crash_recovery`
- summary: deterministic crash lockout and recovery timing for the runner lane
- current status: `tested`
- inputs: `crash_events`, `recovery_ticks`
- outputs: `crash_state`, `steer_eligibility`, `collision_eligibility`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/sim/crash_recovery/Cargo.toml`
  - `cargo run -p feature_cli -- show sim.crash_recovery`
  - `cargo run -p feature_cli -- test sim.crash_recovery`
- intended host pattern: reusable crash-cycle state beside runner-track simulation and future damage/HUD adapters

## sim.checkpoint_flow

- feature id: `sim.checkpoint_flow`
- crate path: `feature_packs/sim/checkpoint_flow`
- summary: linear checkpoint progression and restart law across alternating brawler and runner modes
- current status: `tested`
- inputs: `checkpoints`, `stage_events`, `restart_commands`
- outputs: `current_checkpoint`, `current_mode`, `pending_transition`, `completed_checkpoints`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/sim/checkpoint_flow/Cargo.toml`
  - `cargo run -p feature_cli -- show sim.checkpoint_flow`
  - `cargo run -p feature_cli -- test sim.checkpoint_flow`
- intended host pattern: reusable stage backbone for hybrid mode switching and restart law

## ui.health_hud

- feature id: `ui.health_hud`
- crate path: `feature_packs/ui/health_hud`
- summary: headless HUD adapter for segmented health, lives, and brief damage flash timing
- current status: `tested`
- inputs: `actor_state`, `damage_events`, `tick_events`
- outputs: `health_segments`, `lives`, `damage_flash_state`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/health_hud/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.health_hud`
  - `cargo run -p feature_cli -- test ui.health_hud`
- intended host pattern: thin health/lives HUD over actor state in combat, runner, or hybrid stage shells

## Wave: `simulation_zoom_ai_grading`

This wave adds readable AI targets, deterministic grading, and simulation zoom
eligibility on top of the first hybrid foundation.

## logic.ai_target_state

- feature id: `logic.ai_target_state`
- crate path: `feature_packs/logic/ai_target_state`
- summary: readable AI target state with explicit posture, awareness, pressure, and commitment or vulnerability windows
- current status: `tested`
- inputs: `telegraph_inputs`, `awareness_updates`, `pressure_updates`, `tick_events`
- outputs: `target_state`, `commitment_window`, `vulnerability_window`, `readability_hints`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/logic/ai_target_state/Cargo.toml`
  - `cargo run -p feature_cli -- show logic.ai_target_state`
  - `cargo run -p feature_cli -- test logic.ai_target_state`
- intended host pattern: readable authored target record for coarse encounters and zoom-eligible duel hosts

## logic.ai_perception_model

- feature id: `logic.ai_perception_model`
- crate path: `feature_packs/logic/ai_perception_model`
- summary: deterministic snapshot of what a target noticed, missed, or misread
- current status: `tested`
- inputs: `perception_events`, `signal_reads`, `tick_events`
- outputs: `perception_snapshot`, `certainty_state`, `misread_state`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/logic/ai_perception_model/Cargo.toml`
  - `cargo run -p feature_cli -- show logic.ai_perception_model`
  - `cargo run -p feature_cli -- test logic.ai_perception_model`
- intended host pattern: deterministic AI readability layer ahead of intent selection and grading

## logic.ai_intent_model

- feature id: `logic.ai_intent_model`
- crate path: `feature_packs/logic/ai_intent_model`
- summary: authored intent timing with telegraph, committed, and recovery phases
- current status: `tested`
- inputs: `perception_snapshot`, `intent_choice`, `phase_ticks`
- outputs: `intent_state`, `commitment_phase`, `readable_intent`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/logic/ai_intent_model/Cargo.toml`
  - `cargo run -p feature_cli -- show logic.ai_intent_model`
  - `cargo run -p feature_cli -- test logic.ai_intent_model`
- intended host pattern: authored target behavior timing for brawler encounters and duel zoom

## logic.ai_interaction_event

- feature id: `logic.ai_interaction_event`
- crate path: `feature_packs/logic/ai_interaction_event`
- summary: canonical event vocabulary for grading AI interaction quality
- current status: `tested`
- inputs: `interaction_observations`, `intent_context`, `timeline_ticks`
- outputs: `interaction_events`, `interaction_outcomes`, `poor_read_streak`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/logic/ai_interaction_event/Cargo.toml`
  - `cargo run -p feature_cli -- show logic.ai_interaction_event`
  - `cargo run -p feature_cli -- test logic.ai_interaction_event`
- intended host pattern: durable interaction ledger between encounter simulation and skill grading

## logic.ai_interaction_grader

- feature id: `logic.ai_interaction_grader`
- crate path: `feature_packs/logic/ai_interaction_grader`
- summary: deterministic grade card with stable bands, confidence, and per-dimension feedback
- current status: `tested`
- inputs: `interaction_events`
- outputs: `grade_card`, `skill_band`, `grade_confidence`, `poor_read_streak`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/logic/ai_interaction_grader/Cargo.toml`
  - `cargo run -p feature_cli -- show logic.ai_interaction_grader`
  - `cargo run -p feature_cli -- test logic.ai_interaction_grader`
- intended host pattern: replayable AI interaction scoring for escalation and visible feedback

## sim.layer_escalation

- feature id: `sim.layer_escalation`
- crate path: `feature_packs/sim/layer_escalation`
- summary: deterministic decision law for staying coarse, offering zoom, or forcing duel zoom
- current status: `tested`
- inputs: `grade_card`
- outputs: `stay_coarse`, `offer_zoom`, `force_zoom`, `next_layer`, `reward_modifier`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/sim/layer_escalation/Cargo.toml`
  - `cargo run -p feature_cli -- show sim.layer_escalation`
  - `cargo run -p feature_cli -- test sim.layer_escalation`
- intended host pattern: escalation law between coarse encounter hosts and deeper simulation layers

## sim.duel_arena

- feature id: `sim.duel_arena`
- crate path: `feature_packs/sim/duel_arena`
- summary: richer duel zoom state with positional response, exchanges, knockback-like impulses, and deterministic outcomes
- current status: `tested`
- inputs: `arena_actors`, `exchange_events`, `push_impulses`, `tick_events`
- outputs: `duel_state`, `contact_response`, `duel_outcome`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/sim/duel_arena/Cargo.toml`
  - `cargo run -p feature_cli -- show sim.duel_arena`
  - `cargo run -p feature_cli -- test sim.duel_arena`
- intended host pattern: first deep simulation zoom returned to the canonical hybrid progression state

## ui.ai_grade_hud

- feature id: `ui.ai_grade_hud`
- crate path: `feature_packs/ui/ai_grade_hud`
- summary: thin HUD adapter for visible band, confidence, zoom readiness, and short teaching feedback
- current status: `tested`
- inputs: `grade_card`, `escalation_decision`
- outputs: `visible_band`, `visible_confidence`, `zoom_ready`, `feedback_summary`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/ai_grade_hud/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.ai_grade_hud`
  - `cargo run -p feature_cli -- test ui.ai_grade_hud`
- intended host pattern: player-facing skill-feedback layer without turning the encounter into raw score spam

## Companion Contract

Use [docs/hybrid_brawler_racer_sprite_integration_contract.md](/Users/kogaryu/dev/features/docs/hybrid_brawler_racer_sprite_integration_contract.md)
as the canonical v1 source for:

- atlas ids
- clip ids
- playback law
- event markers
- runtime mapping into the current gameplay features
