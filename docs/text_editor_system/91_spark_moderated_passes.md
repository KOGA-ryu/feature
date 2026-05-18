# Spark Moderated Passes

The first four text-editor system builds are documentation-only. Each pass must
be reviewed before the next pass starts.

## Pass 1: Editor System Map + Build Roadmap

Assigned docs:

- `README.md`
- `00_map.md`
- `90_build_order.md`

Goal:

Define the complete package shape and build order: engine, actions, clipboard,
cleanup, host adapters, and golden tests.

Review bucket:

```text
/Users/kogaryu/dev/features_binder/.dex/spark_reviews/text_editor_docs_pass_1/review.md
```

## Pass 2: Action Registry + Host Contract

Assigned docs:

- `02_action_registry_contract.md`
- `30_host_ui_contract.md`
- `31_egui_host_adapter.md`
- `32_qt_host_adapter.md`

Goal:

Define the action records that hosts render into buttons, menus, hotkeys, and
tooltips.

Review bucket:

```text
/Users/kogaryu/dev/features_binder/.dex/spark_reviews/text_editor_docs_pass_2/review.md
```

## Pass 3: Clipboard Export + Terminal Text Helpers

Assigned docs:

- `14_clipboard_export.md`
- `15_paste_cleanup.md`
- `19_ai_prompt_helpers.md`
- `20_terminal_text_helpers.md`

Goal:

Lock copy-safe text behavior and terminal/chat cleanup behavior for the first
implementation slice.

Review bucket:

```text
/Users/kogaryu/dev/features_binder/.dex/spark_reviews/text_editor_docs_pass_3/review.md
```

## Pass 4: Core Editing Feature Contracts

Assigned docs:

- `10_core_engine.md`
- `11_text_input.md`
- `12_navigation.md`
- `13_selection.md`
- `16_search_replace.md`
- `17_markdown_formatting.md`
- `18_draft_file_state.md`
- `40_golden_tests.md`

Goal:

Define remaining editor action families with reference behavior, access
patterns, hotkeys, icons, tests, and acceptance.

Review bucket:

```text
/Users/kogaryu/dev/features_binder/.dex/spark_reviews/text_editor_docs_pass_4/review.md
```

## Moderation Rule

Do not start code implementation until the first four pass reviews have been
read and criticized.

## Later Capability Docs

These docs are staged for later passes and should not be forced into the first
code slice:

- `21_undo_redo_history.md`
- `22_code_editing.md`
- `23_view_controls.md`
- `24_structure_outline.md`
- `25_validation_cleanup.md`
- `26_collaboration_review.md`
- `27_advanced_editing.md`
- `28_editor_anatomy_reference.md`
- `29_reusable_components_matrix.md`

The first implementation slice remains copy/export helpers in
`ui.text_editor_plain`.

## Cross-Cutting Policy Docs

These docs apply across all passes and should be consulted before code work:

- `05_text_model_unicode_policy.md`
- `06_error_empty_state_policy.md`
- `07_security_privacy_redaction.md`
- `08_accessibility_keyboard_policy.md`
- `09_fixture_example_policy.md`
- `33_terminal_host_adapter.md`
- `34_web_host_adapter.md`
- `41_spark_code_review_rubric.md`
- `42_release_gate_checklist.md`
