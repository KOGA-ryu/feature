# workflow.feature_batch_packet_flow

`workflow.feature_batch_packet_flow` turns an explicit ordered queue plus a
`GeneratedSpec` into a one-feature-at-a-time Dex batch contract.

It owns:

- deterministic queue validation and preservation
- one worker packet, one integrator packet, and one reviewer packet per step
- per-step review contract generation
- deferred feature tracking for spec features omitted from the active queue

It does not own:

- feature selection or ranking
- automated execution of the batch
- completed review packet authoring
- parallel lane scheduling

## Why It Exists

The repo already had:

- `workflow.feature_build_packet_flow` for one isolated feature
- `workflow.feature_wave_packet_flow` for parallel multi-worker waves

This workflow fills the missing middle: one agent can run a deterministic
multi-feature batch end-to-end without inventing the handoff contract each
time.

## Current Scope

- explicit ordered feature queues only
- queue can be a subset of `spec.selected_features`
- each step is worker -> integrator -> reviewer -> review packet + receipt
- continue to the next feature unless blocked

## Non-Goals

- no automated batch runner
- no queue derivation from `SpecSelectionResult`
- no multi-agent concurrency
- no packet archive persistence in this cut
