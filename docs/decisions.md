# Decisions

## 2026-04-26

- Decision: use a Rust workspace instead of a Python-first feature repo.
- Reason: each feature needs to compile, test, and demo in one local toolchain.
- Alternatives rejected: keep the Python foundation at the top level; start with a web UI.
- Reversal condition: revisit only if the Rust workspace blocks isolated feature work.

## 2026-04-26

- Decision: use `egui`/`eframe` for the first feature harness UI.
- Reason: it keeps the testing ground local-first and entirely in Rust.
- Alternatives rejected: React, Tauri, and dynamic plugin loading.
- Reversal condition: revisit only after the feature lab foundation is stable.

## 2026-04-26

- Decision: parallel feature work uses five isolated worker Dex lanes plus one integrator lane and one reviewer lane.
- Reason: feature crates are safely parallel, but root workspace and harness files are shared choke points.
- Alternatives rejected: let every worker edit shared files; serialize all feature work through one Dex.
- Reversal condition: revisit only after harness/demo registration no longer requires shared-file edits for each feature.
