# Build Order: Text Editor System, Pass 1

Track B-first is mandatory. Track A is documented but not started.

## Phase 0 — Contract freeze (docs and ownership)

### Scope

- Expand `README.md`, `00_map.md`, and `90_build_order.md`.
- Confirm family ownership by crate/host and track labels (`V1/early/later/future`).
- Reconcile action semantics against:
  - `02_action_registry_contract.md`
  - `28_editor_anatomy_reference.md`
  - `41_spark_code_review_rubric.md`
  - `42_release_gate_checklist.md`

### Inputs / Dependencies

- No code assumptions.
- Current docs must be in `docs/text_editor_system/` and this pass must not edit code.
- Reusable component ownership should be checked against
  `29_reusable_components_matrix.md`.

### Acceptance for phase 0

- All required docs explicitly show Track B first and Track A deferred.
- All families in the map are marked with placement (`V1/early/later/future`)
  and owner.
- `text_editor_plain → text_editor_actions → text_editor_clipboard → text_editor_host_qt`
  sequence is explicit.
- Postponed Track A families are listed by name.

### Stop gate G0

- Stop and rework docs before any implementation if the ownership map is incomplete
  or if host-adapter assumptions are undocumented.

---

## Phase 1 — First code slice: `text_editor_plain copy/export helpers` (required first code slice)

### Scope

- Implement only headless copy/export helpers in `text_editor_plain`.
- No host UI and no system clipboard API calls.
- No new core editor engine behavior.

Suggested helper set:

- `selected_text_or_all`
- `copy_plain`
- `copy_markdown_block`
- `copy_prompt_block`
- `current_line_text`
- `line_range_text`

Ownership note:

- these are primitive pure helpers in `text_editor_plain`
- richer clipboard/export policy moves to `text_editor_clipboard` in Phase 3
- no helper calls the system clipboard

### Inputs / Dependencies

- Completed docs freeze (Phase 0).
- Action registry contract exists as reference for naming and outputs.

### Exact acceptance

- Exact output behavior is tested for:
  - selected text,
  - full-doc fallback when no selection,
  - empty selection,
  - empty document,
  - newline and whitespace preservation.
- Helpers are pure headless functions.
- No `text_editor_actions`, `text_editor_clipboard`, or host adapter code touched.
- No changes to crates, host code, Cargo files, or feature-pack inventory for this
  docs pass.
- Action IDs referenced by tests or docs use the `text.*` namespace.

### Stop gate G1

- Do not proceed if helper behavior mutates editor state, drops newlines, adds UI
  coupling, or uses system clipboard calls.

---

## Phase 2 — Action registry slice

### Scope

- Implement `text_editor_actions` as the single source for command metadata.
- Add action definitions for copy/export and clipboard-focused behavior.

### Inputs / Dependencies

- Phase 1 helpers are implemented and stable.
- Action contract shape is frozen from `02_action_registry_contract.md`.
- Shared action/result/hotkey concepts are checked against
  `29_reusable_components_matrix.md`.

### Exact acceptance

- Action records include contract fields needed by hosts:
  - id
  - label
  - short label
  - category
  - icon
  - tooltip
  - default hotkeys
  - enabled rule
  - input shape
  - output shape
  - undo behavior
  - tests
- Action IDs use `text.*`, for example `text.copy_prompt_block`.
- Default hotkey profile is Linux desktop editor unless a host profile overrides it.
- host rendering uses registry metadata directly (no silent remap).
- Contract tests assert registry records and host-facing output shapes.

### Stop gate G2

- Do not proceed if hosts begin inventing action names or behavior not present in
  the registry contract.

---

## Phase 3 — Clipboard/export module

### Scope

- Implement `text_editor_clipboard` for string transforms:
  - rich clipboard/export policy
  - prompt-safe output
  - markdown code block output
  - terminal cleanup transforms
  - code-fence and plain text variants required by contracts.

### Inputs / Dependencies

- Phase 1 helper primitives and Phase 2 action metadata are available.

### Exact acceptance

- Output variants are deterministic and documented.
- Fixtures are explicit where behavior is non-trivial.
- No persistence side effects.

### Stop gate G3

- Do not proceed if transforms mix rendering logic or mutable state without tests.

---

## Phase 4 — Qt host adapter slice

### Scope

- Implement `text_editor_host_qt` as a thin adapter.
- Map action metadata to host controls (menus, toolbars, hotkeys, context actions).
- Qt is the first graphical proof host.

### Inputs / Dependencies

- Phases 1–3 completed.

### Exact acceptance

- Host behavior is driven from action metadata, not custom command logic.
- Unsupported actions are hidden/disabled from explicit metadata.
- Host behavior not yet proven in a running app is labeled `needs verification`.
- Terminal and web hosts remain later proof targets.

### Stop gate G4

- Do not proceed to extra hosts before this adapter demonstrates stable and bounded
  action routing.

---

## Phase 5 — Expand early Track B families

### Scope

- Add bounded features not done above: selection utilities, navigation/read helpers,
  bounded in-doc search/replace, richer terminal cleanup variants.

### Inputs / Dependencies

- Phase 4 accepted and reviewed.

### Exact acceptance

- Each new helper has exact tests for edge cases.
- All changes remain aligned to action contract ownership.

### Stop gate G5

- Do not absorb Track A engine behaviors into this phase.

---

## Phase 6 — Track A engineering gates (deferred)

### Scope (defer list)

- rope
- piece table
- rendering engine
- viewport renderer
- syntax highlighting
- tree-sitter
- LSP
- multi-cursor
- global search engine

### Inputs / Dependencies

- A host app explicitly requires full editor-engine behavior.
- Track B foundation has passed review gates.

### Exact acceptance

- No Track A feature starts before Track B is stable and reviewed.
- Each Track A feature is introduced as a separate bounded crate with explicit action
  impact statement.

## Dependency summary

- `Phase 1 -> Phase 2 -> Phase 3 -> Phase 4`
- `Phase 5` only after `Phase 4` review.
- `Phase 6` only when `Phase 5` is approved and there is explicit app demand.

## Review loop

Each phase passes through:

1) review file in binder bucket
2) contract/rubric check
3) release gate check

If any hard gate fails, stop and fix before next phase.

## Explicit deferrals for this pass

- piece table
- rope
- rendering engine
- syntax highlighting
- tree-sitter
- LSP
- multi-cursor
- global search engine

These are deferred by design and should only move forward in a later Track A plan.
