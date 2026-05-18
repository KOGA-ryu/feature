# sim.crash_recovery

`sim.crash_recovery` is the reusable crash-cycle model for the runner lane.

## Purpose

Provide deterministic lockout and recovery timing for:

- entering crash state
- steering lockout
- collision lockout
- returning to stable play

## V1 Behavior

- explicit crash trigger
- explicit tick-driven recovery
- no animation coupling

## Non-goals

- cinematic camera timing
- ragdoll
- particle or effects ownership

## Intended Host Pattern

Use this crate beside `sim.runner_track` when a host needs deterministic crash
and recovery eligibility without coupling to rendering or animation systems.
