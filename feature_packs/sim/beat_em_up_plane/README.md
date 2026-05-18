# sim.beat_em_up_plane

`sim.beat_em_up_plane` is the reusable combat-space simulation for the brawler
lane of a hybrid stage.

## Purpose

Provide one bounded side-view plane with:

- deterministic actor ordering
- x/depth movement
- bounds clamping
- depth-proximity engagement queries

## V1 Behavior

- headless position updates only
- no jumping arc
- no platforming
- no physics impulses

## Non-goals

- pathfinding
- attack range geometry
- camera ownership
- collision response physics

## Intended Host Pattern

Use this crate as the encounter-space layer beneath actor state and hit
resolution when building a scrolling beat-em-up segment.
