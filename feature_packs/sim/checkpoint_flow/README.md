# sim.checkpoint_flow

`sim.checkpoint_flow` is the reusable linear stage progression and restart law
for alternating brawler and runner segments.

## Purpose

Provide deterministic state for:

- ordered checkpoint progression
- pending mode transitions
- finishing transitions
- restarting from the current checkpoint

## V1 Behavior

- one linear route
- alternating brawler and racing modes
- explicit transition finish step

## Non-goals

- branching paths
- save system
- campaign map
- meta progression

## Intended Host Pattern

Use this crate as the mode-and-checkpoint backbone for a hybrid stage that
switches between encounter space and runner segments.
