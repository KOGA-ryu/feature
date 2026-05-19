# Text Editor System

## Plain-English map

This package is designed as a **reusable editor system** for apps, not a monolithic
standalone editor. The first goal is to make useful text actions available to
multiple hosts in the same way.

The system is split so:

- `text_editor_plain` owns headless text shape and selected-text helper behavior.
- `text_editor_actions` owns the action registry metadata that every host uses.
- `text_editor_clipboard` owns copy/export and paste-cleanup transforms.
- `text_editor_host_qt` (and later adapters) own host wiring only.

This keeps editor behavior testable and avoids duplicating editor semantics across
UI surfaces.

## Track policy: Track B first, Track A later

Track B is the first implementation track and is mandatory for this pass.

- Track B: reusable editor actions for apps.
  - copy/export helpers, action contracts, host adapters.
- Track A: full editor engine work.
  - cursor engine, rope/piece table, renderer/view pipeline,
    syntax/tree parsing, LSP, multi-cursor, and global search engines.

Track A is still documented for direction, but not started now.

## Why reusable actions, not a standalone editor

A standalone editor implementation locks behavior to one product’s UI and command
model. Reusable actions give us:

- one source of command truth (action IDs, labels, hotkeys, enabled rules),
- one set of behavior tests for all hosts,
- one easy way to move behavior from terminal-like contexts to desktop-like
  contexts,
- no hidden coupling between core text behavior and any specific toolkit.

For this repository, that means action behavior is defined once and rendered by
multiple hosts later.

## Crate sequence for this pass and next

Use this exact order:

1. `text_editor_plain`
2. `text_editor_actions`
3. `text_editor_clipboard`
4. `text_editor_host_qt`
5. host adapters later as needed (`text_editor_host_egui`, terminal, web, etc.)

The first code slice is fixed to:

`text_editor_plain copy/export helpers`

Ownership rule for the first slice:

- `text_editor_plain` owns primitive pure helpers such as selected-or-full text,
  line text, and simple formatted strings needed for exact contract tests.
- `text_editor_clipboard` later owns richer clipboard/export formats,
  terminal-safe output, paste cleanup, and host-facing clipboard transform
  policies.
- No slice may call the system clipboard until a host adapter explicitly owns
  that behavior.

## How the docs, actions, tests, and gates fit together

1. **Docs define intent first**
   - each capability family is tagged `V1/early/later/future`,
   - ownership is explicit (`text_editor_plain`, `text_editor_actions`,
     `text_editor_clipboard`, `text_editor_host_qt`, or host adapters).
2. **Action contracts define shape**
   - action IDs, labels, tooltips, icons, hotkeys, and enablement are contract
     fields.
   - UI/host must call action IDs; hosts do not invent command semantics.
3. **Tests bind behavior**
   - copy/export helpers are tested as pure behavior first.
   - exact string outputs are required.
   - empty-selection and empty-document behavior is explicit.
4. **Release gates validate transitions**
   - review rubric checks scope, evidence shape, and next-slice clarity.
   - release gate checks include no cross-layer coupling and no unrelated file
     changes.

## Current status by track

- Active now: Track B planning, docs expansion, and the first implementation slice
  definition.
- Defer: Track A engine work until a host explicitly requires it.

Engine-heavy items are explicitly marked as postponed in `00_map.md` and
`90_build_order.md`.

Any app behavior tied to Qt or another host that is not yet exercised in running
host proof is marked `needs verification`.

## Locked Defaults

- Action IDs use the `text.*` namespace, for example
  `text.copy_prompt_block`.
- The default hotkey profile is Linux desktop editor unless a host profile
  overrides it.
- The first graphical proof host is Qt, with `QPlainTextEdit` as the first
  practical editor surface.
- Qt consumes editor actions; Qt does not own editor behavior.
- Terminal and web host proof come later.

## Current Host Decision

Decision:

```text
Core behavior lives in Rust/headless feature crates.
Qt is the first desktop host.
QPlainTextEdit is the first Qt text surface.
Custom editor rendering is deferred.
```

Why:

- `QPlainTextEdit` already solves native text input, selection, scrolling,
  keyboard focus, and platform integration well enough for early proof.
- Headless Rust crates keep copy/export/action behavior reusable by Dex Home,
  feature-lab UI, future egui hosts, terminal helpers, and web surfaces.
- A custom renderer would force us to own cursor math, IME, accessibility,
  scrolling, hit testing, font metrics, and selection before the action system
  has proven value.

This means the first UI host should feel practical, but the reusable feature
library must remain host-agnostic.

## Reference anchors

- `02_action_registry_contract.md` for action metadata boundaries.
- `28_editor_anatomy_reference.md` for complete system anatomy and deferred engine
  work.
- `29_reusable_components_matrix.md` for shared components that should not be
  reinvented per crate or host.
- `44_feature_symbol_language.md` for the future local-librarian and compact
  feature-request language.
- `41_spark_code_review_rubric.md` for review expectations.
- `42_release_gate_checklist.md` for final gating.
- `43_text_editor_detail_pass_todo.md` for the current detail-pass checklist.
- `45_ui_style_and_pathing_contract.md` for CSS/QSS, selector, and UI path rules.
- `46_personal_text_editor_reference_dissection.md` for the reference screenshot
  anatomy.
- `47_text_editor_surface_grammar_sheet.md` for per-surface styling grammar.
- `48_zed_event_runtime_architecture.md` for the Zed-derived event/runtime
  reference model and what we should borrow.
- `49_zed_ui_mounted_learning_notes.md` for teaching Zed runtime ideas through
  visible Text Editor UI surfaces.
