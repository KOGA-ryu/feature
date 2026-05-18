# scroll_reader_capsule_v1

This folder is the first design/spec slice for `scroll_reader_capsule_v1`.

Chosen path: `/Users/kogaryu/dev/features/docs/scroll_reader_capsule/`.
The repo already has a root `docs/` tree, so the capsule packet lives there
instead of creating a nested `features/docs` directory inside the feature lab.

The scroll reader capsule is a temporary selected-text reading aid:

```text
text snapshot -> hotkey -> floating capsule reader -> pace control -> Esc closes
```

It is not a full app, document library, editor, dashboard, or persistence
surface. The user should feel like they summoned a small physical reading aid,
not opened a workspace.

Runtime rule: the reading surface shows only the pill. Any shaping, color, or
function-placement controls belong in a separate options window that opens only
when the user asks to tune the tool.

Theme rule: V1 has only two built-in theme choices, plus a custom color-wheel
seed path for accent, background, foreground, and contrast. The user does not
need a long preset gallery or an 11-color role editor.

## Packet Map

- `00_product_thesis.md`: scope, hard boundaries, and product intent
- `01_visual_contract.md`: geometry, colors, layers, and visual law
- `02_interaction_model.md`: launch, display behavior, controls, beauty rule, and states
- `03_state_machine.md`: status model and transition rules
- `04_text_normalization_and_tokenization.md`: text input rules and V1 tokens
- `05_scroll_and_speed_controls.md`: wheel, trackpad, and pace behavior
- `06_qt_ui_contract.md`: Qt host responsibilities and constants
- `07_golden_tests.md`: required test cases for a later engine/UI slice
- `08_design_implementation_matrix.md`: small visual decisions mapped to code owners and tests
- `09_speed_reading_mechanics_notes.md`: research-backed mechanics that guide the V1 model
- `parked_ideas.md`: deliberately excluded ideas

## Acceptance Checklist

- large and compact capsule geometry is explicitly documented
- two-background color law is documented
- two built-in theme options and custom color-wheel path are documented
- interaction model is clear
- state machine is clear
- normalization and tokenization behavior is clear
- scroll and wheel behavior is clear
- runtime pill-only rule is documented
- separate options window boundary is documented
