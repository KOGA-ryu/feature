# Features

`/Users/kogaryu/dev/features` is a Rust feature lab, not a generic wiki repo.

Core rule:
every reusable feature is isolated, testable, documented, searchable, and
demoable. Codex should build one feature crate at a time instead of spraying
loose files across the repo.

## Proof Policy

Terminal-first proof is the default in this repo.

- canonical proof runs in the terminal through `cargo` and `feature_cli`
- `feature_lab_ui` is a browse and optional smoke harness, not the primary
  acceptance path
- only require live Feature Lab interaction when the task explicitly depends on
  harness behavior or when a manual smoke pass is requested

Canonical policy doc:

- [docs/terminal_first_proof.md](/Users/kogaryu/dev/features/docs/terminal_first_proof.md)

## Workspace

- `crates/feature_core`: shared metadata, manifest parsing, and result types
- `crates/feature_registry`: feature discovery and registry loading
- `crates/feature_runner`: command and test execution helpers
- `crates/feature_cli`: list, show, and test features from the terminal
- `crates/feature_lab_ui`: egui/eframe harness for browsing and testing
- `feature_packs/`: one crate per feature
- `legacy/python_foundation/`: preserved Python-based foundation from the first cut

## Canonical Utility Library

Reusable operator utilities live here, not in app-specific sidecars.

- canonical index: [library/components/operator_utilities.md](/Users/kogaryu/dev/features/library/components/operator_utilities.md)
- first utility trio:
  - `ui.scratchpad`
  - `ui.calculator_basic`
  - `ui.checklist_single`
  - `ui.text_editor_plain`
  - `ui.timer_basic`
  - `ui.runbook_panel`
  - `logic.session_notes`
  - `ui.quick_capture_inbox`
  - `logic.document_history`
  - `ui.session_notes_panel`
  - `ui.text_editor_actions`
  - `ui.text_editor_clipboard`
  - `ui.text_editor_host_adapter`

Each utility remains headless-first, testable, documented, searchable, and demoable through `feature_lab_ui`.

## Gameplay Systems Lane

Reusable gameplay systems also live here, not in a one-off game shell.

- canonical index: [library/components/gameplay_systems.md](/Users/kogaryu/dev/features/library/components/gameplay_systems.md)
- companion sprite contract: [docs/hybrid_brawler_racer_sprite_integration_contract.md](/Users/kogaryu/dev/features/docs/hybrid_brawler_racer_sprite_integration_contract.md)
- first hybrid brawler-racer wave:
  - `logic.actor_state`
  - `logic.hit_resolution`
  - `sim.beat_em_up_plane`
  - `sim.runner_track`
  - `sim.crash_recovery`
  - `sim.checkpoint_flow`
  - `ui.health_hud`
- next AI grading wave:
  - `logic.ai_target_state`
  - `logic.ai_perception_model`
  - `logic.ai_intent_model`
  - `logic.ai_interaction_event`
  - `logic.ai_interaction_grader`
  - `sim.layer_escalation`
  - `sim.duel_arena`
  - `ui.ai_grade_hud`

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
cargo run -p feature_cli -- show ui.scratchpad
cargo run -p feature_cli -- show ui.calculator_basic
cargo run -p feature_cli -- show ui.checklist_single
cargo run -p feature_cli -- show ui.text_editor_plain
cargo run -p feature_cli -- show ui.text_editor_actions
cargo run -p feature_cli -- show ui.text_editor_clipboard
cargo run -p feature_cli -- show ui.text_editor_host_adapter
cargo run -p feature_cli -- show ui.timer_basic
cargo run -p feature_cli -- show ui.runbook_panel
cargo run -p feature_cli -- show logic.session_notes
cargo run -p feature_cli -- show ui.quick_capture_inbox
cargo run -p feature_cli -- show logic.document_history
cargo run -p feature_cli -- show ui.session_notes_panel
cargo run -p feature_cli -- show logic.actor_state
cargo run -p feature_cli -- show logic.ai_target_state
cargo run -p feature_cli -- show logic.ai_perception_model
cargo run -p feature_cli -- show logic.ai_intent_model
cargo run -p feature_cli -- show logic.ai_interaction_event
cargo run -p feature_cli -- show logic.ai_interaction_grader
cargo run -p feature_cli -- show logic.hit_resolution
cargo run -p feature_cli -- show sim.beat_em_up_plane
cargo run -p feature_cli -- show sim.duel_arena
cargo run -p feature_cli -- show sim.layer_escalation
cargo run -p feature_cli -- show sim.runner_track
cargo run -p feature_cli -- show sim.crash_recovery
cargo run -p feature_cli -- show sim.checkpoint_flow
cargo run -p feature_cli -- show ui.ai_grade_hud
cargo run -p feature_cli -- show ui.health_hud
cargo run -p feature_cli -- show workflow.feature_batch_packet_flow
cargo run -p feature_cli -- test ui.right_inspector
cargo run -p feature_cli -- test ui.scratchpad
cargo run -p feature_cli -- test ui.calculator_basic
cargo run -p feature_cli -- test ui.checklist_single
cargo run -p feature_cli -- test ui.text_editor_plain
cargo run -p feature_cli -- test ui.text_editor_actions
cargo run -p feature_cli -- test ui.text_editor_clipboard
cargo run -p feature_cli -- test ui.text_editor_host_adapter
cargo run -p feature_cli -- test ui.timer_basic
cargo run -p feature_cli -- test ui.runbook_panel
cargo run -p feature_cli -- test logic.session_notes
cargo run -p feature_cli -- test ui.quick_capture_inbox
cargo run -p feature_cli -- test logic.document_history
cargo run -p feature_cli -- test ui.session_notes_panel
cargo run -p feature_cli -- test logic.actor_state
cargo run -p feature_cli -- test logic.ai_target_state
cargo run -p feature_cli -- test logic.ai_perception_model
cargo run -p feature_cli -- test logic.ai_intent_model
cargo run -p feature_cli -- test logic.ai_interaction_event
cargo run -p feature_cli -- test logic.ai_interaction_grader
cargo run -p feature_cli -- test logic.hit_resolution
cargo run -p feature_cli -- test sim.beat_em_up_plane
cargo run -p feature_cli -- test sim.duel_arena
cargo run -p feature_cli -- test sim.layer_escalation
cargo run -p feature_cli -- test sim.runner_track
cargo run -p feature_cli -- test sim.crash_recovery
cargo run -p feature_cli -- test sim.checkpoint_flow
cargo run -p feature_cli -- test ui.ai_grade_hud
cargo run -p feature_cli -- test ui.health_hud
cargo run -p feature_cli -- test workflow.feature_batch_packet_flow
```

## Automation Launch

For optional local accessibility automation and Computer Use targeting:

```bash
scripts/launch_feature_lab_ui_app.sh
```

This rebuilds a disposable repo-local bundle at:

- `target/macos-app/Feature Lab.app`

Canonical automation identifiers:

- preferred app name: `Feature Lab`
- fallback bundle id: `dev.kogaryu.feature-lab`

## ChatGPT Coding Windows

Use these when opening a fresh ChatGPT coding window against this repo:

- [AGENTS.md](/Users/kogaryu/dev/features/AGENTS.md)
- [Bootstrap Packet](/Users/kogaryu/dev/features/docs/chatgpt_coding_window_bootstrap.md)
- [Terminal-First Proof](/Users/kogaryu/dev/features/docs/terminal_first_proof.md)
- [Feature Packets](/Users/kogaryu/dev/features/docs/chatgpt_feature_packets.md)
- [Sequential Dex Packets](/Users/kogaryu/dev/features/docs/sequential_dex_prompt_packets.md)
- [Parallel Dex Packets](/Users/kogaryu/dev/features/docs/parallel_dex_prompt_packets.md)
