# ui.runbook_panel

`ui.runbook_panel` is a reusable runbook state crate for the Rust feature lab.

## Purpose

Provide deterministic operator-runbook state with one selected step, explicit
status transitions, and simple aggregate counts for thin host surfaces.

## V1 Behavior

- one selected step at a time
- safe selection clamping
- at most one active step at a time
- completed and blocked states persist until reset
- fixture parsing for repo-relevant operator flows

## Non-goals

- editing or reordering steps
- templates or persistence
- file loading beyond fixture parsing
- timers, receipts, or workflow execution

## Intended Host / Use Cases

- operator runbook side panels
- step-tracking surfaces inside `feature_lab_ui`
- proof-oriented workflow demos that need explicit status state
