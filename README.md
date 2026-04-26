# Features

`/Users/kogaryu/dev/features` is a Rust feature lab, not a generic wiki repo.

Core rule:
every reusable feature is isolated, testable, documented, searchable, and
demoable. Codex should build one feature crate at a time instead of spraying
loose files across the repo.

## Workspace

- `crates/feature_core`: shared metadata, manifest parsing, and result types
- `crates/feature_registry`: feature discovery and registry loading
- `crates/feature_runner`: command and test execution helpers
- `crates/feature_cli`: list, show, and test features from the terminal
- `crates/feature_lab_ui`: egui/eframe harness for browsing and testing
- `feature_packs/`: one crate per feature
- `legacy/python_foundation/`: preserved Python-based foundation from the first cut

## First Vertical Slice

1. Rust workspace boots
2. `feature_core` defines metadata types
3. each feature has `feature.toml`
4. `feature_registry` loads known features
5. `feature_cli` can list, show, and test features
6. `feature_lab_ui` displays list, detail, metadata, and test output
7. one demo feature exists: `ui.right_inspector`
8. `cargo test --workspace` passes

## Commands

```bash
cargo fmt --all --check
cargo test --workspace
cargo run -p feature_cli -- list
cargo run -p feature_cli -- show ui.right_inspector
cargo run -p feature_cli -- test ui.right_inspector
cargo run -p feature_lab_ui
```

## ChatGPT Coding Windows

Use these when opening a fresh ChatGPT coding window against this repo:

- [AGENTS.md](/Users/kogaryu/dev/features/AGENTS.md)
- [Bootstrap Packet](/Users/kogaryu/dev/features/docs/chatgpt_coding_window_bootstrap.md)
- [Feature Packets](/Users/kogaryu/dev/features/docs/chatgpt_feature_packets.md)
