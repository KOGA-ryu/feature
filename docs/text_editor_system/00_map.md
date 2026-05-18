# Text Editor System Map (Pass 1)

This map is flat and implementation-oriented. It shows all current layers and all
capability families with explicit Track placement.

## System layers

| Layer | Responsibility | Owner |
| --- | --- | --- |
| Policy & Contract | text model policy, error/empty behavior, accessibility, security, release gates | docs + contract review |
| State & Helpers | headless text operations and selection-aware helpers | `text_editor_plain` |
| Actions | action metadata and command registration | `text_editor_actions` |
| Export & Cleanup | copy formats, paste cleanup, terminal cleanup transforms | `text_editor_clipboard` |
| Host Adapters | map action metadata into host models, then toolkit controls | `text_editor_host_adapter` now, `text_editor_host_qt` next |
| Testing & Review | contract tests and spark/release gate artifacts | per crate tests + `41_...` + `42_...` |

## Capability families

| Family | Owner | Track |
| --- | --- | --- |
| Text model policy | docs policy + `text_editor_plain` consumers | early |
| Selection state shape (anchor/active) | `text_editor_plain` | early |
| Line-oriented read helpers | `text_editor_plain` | V1 |
| Selected-or-full text helper | `text_editor_plain` | V1 |
| Copy plain text primitive | `text_editor_plain` | V1 |
| Copy markdown block primitive | `text_editor_plain` | V1 |
| Copy prompt block primitive | `text_editor_plain` | V1 |
| Current-line text helper | `text_editor_plain` | early |
| Line-range text helper | `text_editor_plain` | early |
| Terminal paste cleanup | `text_editor_clipboard` | early |
| Terminal export cleanup | `text_editor_clipboard` | early |
| Rich clipboard/export formats | `text_editor_clipboard` | early |
| Action registry records | `text_editor_actions` | V1 |
| Action execution contract | `text_editor_actions` | V1 |
| Action id/version policy | `text_editor_actions` | early |
| Neutral host action item mapping | `text_editor_host_adapter` | V1 |
| Host result and receipt summary mapping | `text_editor_host_adapter` | V1 |
| Host hotkey profile mapping | `text_editor_host_adapter` + docs | early |
| Host menu/toolbar/context wiring | `text_editor_host_qt` | early |
| Undo/redo command model | `text_editor_plain` | later |
| Navigation command helpers | `text_editor_plain` | later |
| Search/replace (bounded in-doc) | `text_editor_plain` | later |
| Global search engine | future crate decision | future |
| Draft/file state and dirty tracking | `text_editor_plain` | later |
| Markdown formatting helpers | `text_editor_plain`/new focused crates | later |
| AI helper extraction | shared policy + future helper crate(s) | future |
| Multi-cursor editing | future | future |
| Syntax highlighting | future | future |
| Renderer/viewport | host runtime + future engine crate | future |
| Tree-sitter integration | future | future |
| LSP integration | future | future |

## Engine-heavy families (explicitly postponed)

- rope
- piece table
- rendering engine/viewport stack
- syntax highlighting
- tree-sitter
- LSP
- multi-cursor
- global search engine

These are in Track A/future scope and must not block Track B slices.

## Ownership summary by phase

- V1 phase families are owned by:
  - `text_editor_plain`
  - `text_editor_actions`
- Early post-V1 families remain in the same three plus:
  - `text_editor_clipboard`
  - `text_editor_host_adapter`
  - `text_editor_host_qt`
- Later and future families are held behind explicit gates in `90_build_order.md`.

## Ownership Rule

`text_editor_plain` owns primitive pure text helpers used by the first tests.
`text_editor_clipboard` owns richer clipboard/export policy and host-facing
clipboard transforms after those primitives are stable.
`text_editor_host_adapter` owns framework-neutral render/result models. It does
not own widgets, OS clipboard calls, or text cleanup behavior.

Action IDs use the `text.*` namespace.

## Reuse Matrix

Use `29_reusable_components_matrix.md` before implementation to identify shared
types, names, result shapes, fixtures, and host metadata that should not be
reinvented per crate.

## Needs verification markers

Any host-specific behavior (e.g., exact Qt menu placement or shortcut registration
details) not yet proven by running host proof is marked as `needs verification`.
