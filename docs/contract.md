# Contract

This repo is a local-first Rust workspace for isolated reusable features.

Non-negotiable rules:

1. Every feature lives in its own crate under `feature_packs/`.
2. Every feature includes `Cargo.toml`, `feature.toml`, `README.md`, `src/`,
   `tests/`, and `fixtures/`.
3. Shared behavior belongs in workspace crates, not in random top-level files.
4. The UI is a harness only: browse features, inspect metadata, run tests, and
   preview demo surfaces.
5. If a feature needs shared changes, those changes must be narrow and justified.
