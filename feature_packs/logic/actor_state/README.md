# logic.actor_state

`logic.actor_state` is the reusable gameplay state model for one player or enemy
actor across both the brawler and runner lanes.

## Purpose

Provide one deterministic, headless actor record with:

- health and max health
- remaining lives
- x/depth position
- facing
- action-state tracking
- invulnerability and recovery timers
- defeat detection

## V1 Behavior

- single-player-first actor state
- deterministic timer ticking
- safe damage application with life loss and defeat detection
- shared action states for hitstun, knockdown, and crash recovery hooks

## Non-goals

- co-op slot ownership
- inventory or stat trees
- equipment systems
- animation or physics ownership

## Intended Host Pattern

Use this crate as the reusable actor record beneath combat resolution,
encounter simulation, runner crash handling, and HUD surfaces.
