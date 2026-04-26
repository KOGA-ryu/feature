# logic.prompt_generator

`logic.prompt_generator` turns a `GeneratedSpec` into bounded Dex task packets.

It owns:

- prompt request validation
- worker, integrator, and reviewer packet generation
- deterministic verification-command output

It does not own:

- spec assembly
- code execution
- UI rendering

## Current Scope

- `feature_worker` prompts for isolated feature crates
- `integrator` prompts for shared workspace ownership
- `reviewer` prompts for read-only proof

The goal is not creativity. The goal is a reproducible, bounded prompt packet
that Dex can execute without guessing scope.
