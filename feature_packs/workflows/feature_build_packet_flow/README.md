# workflow.feature_build_packet_flow

`workflow.feature_build_packet_flow` turns a `SpecSelectionResult` into a
single-feature worker, integrator, and reviewer packet bundle.

It exists to bridge the gap between:

- selection-ready specs from `logic.spec_selector`
- executable bounded packets from `logic.prompt_generator`

This crate does not rescore features, recompute compatibility, or emit a
multi-feature wave. It picks one selected feature deterministically and builds
the exact packet set needed to hand that feature to ChatGPT or Dex.
