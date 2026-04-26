# workflow.feature_wave_packet_flow

`workflow.feature_wave_packet_flow` turns a `SpecSelectionResult` into an
ordered build wave for ChatGPT or Dex.

It owns:

- worker packet generation for multiple selected features
- deterministic lane ownership derived from worker packets
- one shared integrator packet for the active wave
- one shared reviewer packet for the active wave
- deferred feature tracking when `max_workers` is lower than the selected count

It does not own:

- selection, scoring, or compatibility evaluation
- prompt text authorship beyond calling `logic.prompt_generator`
- UI rendering
- multi-wave scheduling across several batches

## Why It Exists

The repo can already choose reusable features and generate one bounded task.
This workflow makes the next step operational: hand a coordinated feature wave
to several coding windows without manually assembling prompts.

## Current Scope

- preserve selected feature order from `SpecSelectionResult`
- cap active worker packets by `max_workers`
- keep deferred selected features visible instead of dropping them
- keep integrator and reviewer packets aligned with the active wave

## Non-Goals

- no packet persistence yet
- no lane dependency solver yet
- no command-centre UI integration yet
