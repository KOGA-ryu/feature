# sim.runner_track

`sim.runner_track` is the reusable over-the-shoulder racing and escape segment
state for a lane-based runner section.

## Purpose

Provide deterministic state for:

- lane shifting
- forward distance
- speed-state display
- obstacle collision detection
- passed-obstacle cleanup

## V1 Behavior

- lane-based steering only
- fixed obstacle stream state
- deterministic collision resolution

## Non-goals

- drifting
- jumping
- projectile combat
- freeform vehicle handling

## Intended Host Pattern

Use this crate as the reusable runner-lane simulation beneath crash recovery,
checkpoint flow, and future HUD surfaces.
