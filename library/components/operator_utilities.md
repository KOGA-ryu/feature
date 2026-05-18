# Operator Utilities

Canonical utility library index for reusable operator-facing helpers in
`/Users/kogaryu/dev/features`.

These features are the long-lived home for general-purpose utility behavior.
Hosts can adapt them with thin UI shells, but the core logic stays isolated in
feature crates.

## ui.scratchpad

- feature id: `ui.scratchpad`
- crate path: `feature_packs/ui/scratchpad`
- summary: plain-text scratchpad state for quick operator notes with count helpers
- current status: `tested`
- inputs: `text`
- outputs: `text`, `character_count`, `line_count`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/scratchpad/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.scratchpad`
  - `cargo run -p feature_cli -- test ui.scratchpad`
- intended host pattern: headless state crate plus thin editor surface in utility drawers, sidecars, or inspector panels

## ui.calculator_basic

- feature id: `ui.calculator_basic`
- crate path: `feature_packs/ui/calculator_basic`
- summary: basic calculator reducer with left-to-right evaluation, percent, and bounded history
- current status: `tested`
- inputs: `digit`, `decimal`, `percent`, `operator`, `evaluate`, `clear`
- outputs: `display`, `history`, `error_state`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/calculator_basic/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.calculator_basic`
  - `cargo run -p feature_cli -- test ui.calculator_basic`
- intended host pattern: headless reducer plus thin keypad/display shell in app utilities, side panels, or workbench demos

## ui.checklist_single

- feature id: `ui.checklist_single`
- crate path: `feature_packs/ui/checklist_single`
- summary: single durable checklist state with trimmed adds, stable ordering, and simple item actions
- current status: `tested`
- inputs: `title`, `toggle_item_id`, `delete_item_id`, `clear_completed`
- outputs: `items`, `has_completed_items`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/checklist_single/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.checklist_single`
  - `cargo run -p feature_cli -- test ui.checklist_single`
- intended host pattern: headless checklist state plus thin task list shell for operator setup, capture prep, or review workflows

## ui.text_editor_plain

- feature id: `ui.text_editor_plain`
- crate path: `feature_packs/ui/text_editor_plain`
- summary: plain-text editor core with multi-line document state, selection, navigation, undo/redo, and dirty tracking
- current status: `tested`
- inputs: `document_text`, `editor_commands`
- outputs: `document_text`, `selection`, `dirty_state`, `history_state`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/text_editor_plain/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.text_editor_plain`
  - `cargo run -p feature_cli -- test ui.text_editor_plain`
- intended host pattern: headless editor engine plus thin keyboard-driven shells in workbenches, drawers, inspectors, or future document surfaces

## ui.timer_basic

- feature id: `ui.timer_basic`
- crate path: `feature_packs/ui/timer_basic`
- summary: deterministic countdown timer state driven entirely by explicit ticks and timer commands
- current status: `tested`
- inputs: `duration_seconds`, `timer_commands`, `elapsed_seconds`
- outputs: `total_seconds`, `remaining_seconds`, `status`, `progress_ratio`, `formatted_remaining`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/timer_basic/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.timer_basic`
  - `cargo run -p feature_cli -- test ui.timer_basic`
- intended host pattern: headless countdown engine plus thin timer controls in utility drawers, operator panels, or focus-session shells

## ui.runbook_panel

- feature id: `ui.runbook_panel`
- crate path: `feature_packs/ui/runbook_panel`
- summary: reusable runbook state with one selected step, explicit status transitions, and aggregate counts
- current status: `tested`
- inputs: `title`, `steps`, `selected_index`, `status_action`
- outputs: `selected_step`, `steps`, `counts`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/runbook_panel/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.runbook_panel`
  - `cargo run -p feature_cli -- test ui.runbook_panel`
- intended host pattern: step-tracking surface for operator workflows, proofs, and guided task flows with a thin host shell

## logic.session_notes

- feature id: `logic.session_notes`
- crate path: `feature_packs/logic/session_notes`
- summary: headless session-note ledger with filtered visibility, selection, pinning, and deterministic insertion order
- current status: `tested`
- inputs: `entries`, `query`, `selection`, `note_actions`
- outputs: `visible_entries`, `selected_entry`, `session_notes_counts`, `latest_entry`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/logic/session_notes/Cargo.toml`
  - `cargo run -p feature_cli -- show logic.session_notes`
  - `cargo run -p feature_cli -- test logic.session_notes`
- intended host pattern: note-ledger logic adapted into session sidebars, capture inboxes, or operator scratch surfaces without changing the core ledger law

## ui.quick_capture_inbox

- feature id: `ui.quick_capture_inbox`
- crate path: `feature_packs/ui/quick_capture_inbox`
- summary: headless intake queue for raw captures with draft entry, filtered visibility, and stable status transitions
- current status: `tested`
- inputs: `items`, `draft_text`, `query`, `selection`, `capture_actions`
- outputs: `visible_items`, `selected_item`, `inbox_counts`, `draft_text`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/quick_capture_inbox/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.quick_capture_inbox`
  - `cargo run -p feature_cli -- test ui.quick_capture_inbox`
- intended host pattern: raw operator intake panels that collect ideas, follow-ups, and rough notes before they become curated ledger entries or documents

## logic.document_history

- feature id: `logic.document_history`
- crate path: `feature_packs/logic/document_history`
- summary: single-document revision ledger with working text, named snapshots, restore, and clean-baseline dirty tracking
- current status: `tested`
- inputs: `working_text`, `revisions`, `selected_revision_index`, `history_actions`
- outputs: `working_text`, `selected_revision`, `document_history_counts`, `dirty_state`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/logic/document_history/Cargo.toml`
  - `cargo run -p feature_cli -- show logic.document_history`
  - `cargo run -p feature_cli -- test logic.document_history`
- intended host pattern: headless revision history adapted into editor sidebars, review drafts, or session note editing panels without coupling history law to a specific UI shell

## ui.session_notes_panel

- feature id: `ui.session_notes_panel`
- crate path: `feature_packs/ui/session_notes_panel`
- summary: host-state adapter that combines note storage, plain-text editing, and per-note revision history into a richer operator notes panel
- current status: `tested`
- inputs: `entries`, `query`, `selection`, `editor_commands`, `snapshot_actions`
- outputs: `notes`, `mode`, `selected_editor`, `selected_history`
- proof commands:
  - `cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/session_notes_panel/Cargo.toml`
  - `cargo run -p feature_cli -- show ui.session_notes_panel`
  - `cargo run -p feature_cli -- test ui.session_notes_panel`
- intended host pattern: richer note side panels and workbenches that keep `logic.session_notes` as the ledger owner while layering editing and revision history through thin host-state adapters
