# ui.health_hud

`ui.health_hud` is the reusable HUD-state adapter for showing segmented health,
lives, and a short damage flash.

## Purpose

Provide one thin, headless UI state model that syncs from `logic.actor_state`
without owning combat logic.

## V1 Behavior

- segmented health display
- remaining lives display
- damage flash timing when synced actor health or lives drop

## Non-goals

- inventory widgets
- score display
- split-screen HUD
- theme ownership beyond host styling

## Intended Host Pattern

Use this crate as the minimal HUD state layer above actor state in a combat or
runner host shell, including `feature_lab_ui` demos.
