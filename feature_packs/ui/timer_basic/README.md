# `ui.timer_basic`

`ui.timer_basic` is a deterministic countdown timer primitive for operator tooling and thin UI demos.

## Purpose

Provide a small headless timer state model that can be driven entirely through explicit commands and elapsed-time ticks.

## V1 Behavior

- countdown-only timer state
- explicit `start`, `pause`, `resume`, `reset`, and `tick` controls
- no wall clock or background execution
- deterministic remaining-time formatting and progress calculation
- safe completion clamping at zero

## Non-goals

- stopwatch mode
- persistence
- alarms, sounds, or notifications
- async scheduling or OS timers

## Intended Host Pattern

Use this crate as the headless timer engine behind a thin control surface in `feature_lab_ui` or another operator-facing shell.
