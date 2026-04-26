# Parallel Dex Lane Matrix

This repo can support `5` feature-building Dex workers at once only if shared-file edits are centralized.

## Rule

- Worker Dex own feature crates only.
- Integrator Dex owns shared workspace and harness files only.
- Reviewer Dex owns no files and runs proof.
- If a worker thinks it needs a shared-file change, it stops and emits an escalation packet instead of editing the shared file directly.

## Shared Choke Points

These files should have exactly one owner during a parallel wave:

- `Cargo.toml`
- `crates/feature_lab_ui/Cargo.toml`
- `crates/feature_lab_ui/src/app.rs`
- `crates/feature_lab_ui/src/panels/demo_panel.rs`
- `crates/feature_registry/**` only if discovery rules change

## 5-Worker Wave

| Lane | Owner | Target | Allowed writes | Forbidden writes | Depends on | Exit condition |
| --- | --- | --- | --- | --- | --- | --- |
| `W1` | Worker Dex 1 | `logic.search_index` | `feature_packs/logic/search_index/**` | root `Cargo.toml`, `crates/**`, other `feature_packs/**` | none | feature crate complete, tests local, escalation emitted if blocked |
| `W2` | Worker Dex 2 | `workflow.review_packet_flow` | `feature_packs/workflows/review_packet_flow/**` | root `Cargo.toml`, `crates/**`, other `feature_packs/**` | none | feature crate complete, tests local, escalation emitted if blocked |
| `W3` | Worker Dex 3 | `logic.spec_generator` | `feature_packs/logic/spec_generator/**` | root `Cargo.toml`, `crates/**`, other `feature_packs/**` | none | feature crate complete, tests local, escalation emitted if blocked |
| `W4` | Worker Dex 4 | `logic.prompt_generator` | `feature_packs/logic/prompt_generator/**` | root `Cargo.toml`, `crates/**`, other `feature_packs/**` | none | feature crate complete, tests local, escalation emitted if blocked |
| `W5` | Worker Dex 5 | `workflow.feature_extraction_flow` | `feature_packs/workflows/feature_extraction_flow/**` | root `Cargo.toml`, `crates/**`, other `feature_packs/**` | none | feature crate complete, tests local, escalation emitted if blocked |
| `INT` | Integrator Dex | shared wiring | `Cargo.toml`, `crates/feature_lab_ui/Cargo.toml`, `crates/feature_lab_ui/src/app.rs`, `crates/feature_lab_ui/src/panels/demo_panel.rs` | feature crate internals unless fixing a compile break with rationale | waits for `W1-W5` | workspace compiles, list/show/test paths wired |
| `RVW` | Reviewer Dex | verification only | none | all repo files | waits for `INT` | proof commands run and review packet emitted |

## Worker Contract

Each worker must produce:

- `Cargo.toml`
- `feature.toml`
- `README.md`
- `src/lib.rs`
- `tests/contract_tests.rs`
- `fixtures/*`

Each worker must not:

- edit `Cargo.toml`
- edit `crates/feature_lab_ui/**`
- edit `crates/feature_registry/**`
- edit another feature crate

## Integrator Contract

The integrator handles exactly three jobs:

1. Add new feature crates to the root workspace.
2. Add minimal harness wiring so the feature has a demo surface when needed.
3. Resolve compile integration issues without redesigning worker crates.

The integrator does not redesign feature behavior or broaden scope.

## Reviewer Contract

The reviewer runs:

- `cargo fmt --all --check`
- `cargo test --workspace`
- `cargo run -p feature_cli -- list`
- `cargo run -p feature_cli -- show <feature_id>` for each feature in the wave
- `cargo run -p feature_cli -- test <feature_id>` for each feature in the wave
- `cargo run -p feature_lab_ui`

The reviewer emits findings, risks, and merge readiness. The reviewer does not patch unless explicitly reassigned.

## Merge Order

1. `W1-W5` finish their feature crates independently.
2. Integrator batches shared-file edits once.
3. Reviewer runs proof once the integrated wave compiles.
4. Coordinator either accepts the wave or splits out failed lanes.

## Stop Conditions

Any worker must stop and escalate if:

- the feature cannot compile without changing shared crates
- another lane already owns the needed file
- the feature scope drifts into another crate
- the requested feature implies a new architecture instead of an isolated crate
