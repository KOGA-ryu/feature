use feature_core::parse_feature_manifest;
use text_editor_actions::{
    FEATURE_ID, TextActionCategory, TextActionId, TextActionInput, TextActionOutput,
    TextEnabledRule, TextHostPlacement, TextUndoBehavior, action_record, all_text_actions,
    execute_text_action, parse_text_action_id, sample_fixture,
};
use text_editor_clipboard::{ClipboardChangeId, ClipboardPayloadKind, SelectionExportPolicy};
use text_editor_plain::{EditorCommand, EditorPosition, EditorSelection, TextEditorPlain};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "text_editor_actions");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec!["editor_state", "text_action_id", "text_action_input"]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "text_action_records",
            "text_action_output",
            "enabled_state",
            "clipboard_payload",
            "clipboard_transform_result"
        ]
    );
}

#[test]
fn sample_fixture_lists_known_actions() {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    let actions = value
        .get("actions")
        .and_then(serde_json::Value::as_array)
        .expect("actions should be an array");

    assert_eq!(actions.len(), all_text_actions().len());
    assert_eq!(actions[0].as_str(), Some("text.copy_plain"));
}

#[test]
fn action_records_are_deterministic_and_complete() {
    let action_ids: Vec<&str> = all_text_actions()
        .iter()
        .map(|record| record.action_id)
        .collect();

    assert_eq!(
        action_ids,
        vec![
            "text.copy_plain",
            "text.copy_markdown_block",
            "text.copy_prompt_block",
            "text.copy_code_fence",
            "text.select_all",
            "text.current_line_text",
            "text.line_range_text",
            "text.trim_trailing_whitespace",
            "text.clean_basic",
            "text.normalize_line_endings",
            "text.strip_ansi_escape_codes",
        ]
    );
}

#[test]
fn action_record_metadata_is_host_renderable() {
    let prompt = action_record(TextActionId::CopyPromptBlock);

    assert_eq!(prompt.label, "Copy Prompt Block");
    assert_eq!(prompt.short_label, "Prompt");
    assert_eq!(prompt.category, TextActionCategory::Clipboard);
    assert_eq!(prompt.icon, "clipboard-copy");
    assert!(prompt.tooltip.contains("prompt-safe"));
    assert_eq!(prompt.enabled_rule, TextEnabledRule::DocumentHasText);
    assert_eq!(prompt.undo_behavior, TextUndoBehavior::None);
    assert!(prompt.host_placements.contains(&TextHostPlacement::Toolbar));

    let clean = action_record(TextActionId::CleanBasic);
    assert_eq!(clean.category, TextActionCategory::Cleanup);
    assert_eq!(clean.enabled_rule, TextEnabledRule::DocumentOrInputHasText);
    assert!(
        clean
            .host_placements
            .contains(&TextHostPlacement::CleanupMenu)
    );
}

#[test]
fn host_placements_are_typed_and_stable() {
    let copy = action_record(TextActionId::CopyPlain);
    let placements: Vec<&str> = copy
        .host_placements
        .iter()
        .map(|placement| placement.as_str())
        .collect();

    assert_eq!(
        placements,
        vec!["command_palette", "clipboard_menu", "context_menu"]
    );
}

#[test]
fn text_action_id_parse_round_trips_stable_strings() {
    for record in all_text_actions() {
        assert!(record.action_id.starts_with("text."));
        assert_eq!(parse_text_action_id(record.action_id), Some(record.id));
        assert_eq!(record.id.as_str(), record.action_id);
    }

    assert_eq!(parse_text_action_id("text.unknown"), None);
}

#[test]
fn copy_actions_return_exact_helper_outputs_without_mutation() {
    let mut editor = TextEditorPlain::from_text("one\n    two".into());
    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 1, column: 4 },
        caret: EditorPosition { line: 1, column: 7 },
    }));
    let selection = editor.selection();
    let text = editor.text().to_owned();

    let output = execute_text_action(
        &mut editor,
        TextActionId::CopyPlain,
        TextActionInput::empty(),
    );
    let TextActionOutput::ClipboardPayload(payload) = output else {
        panic!("copy plain should return clipboard payload");
    };
    assert_eq!(payload.text, "two");
    assert_eq!(payload.receipt.text, "two");
    assert_eq!(
        payload.metadata.payload_kind,
        ClipboardPayloadKind::ExactText
    );
    assert_eq!(
        payload.metadata.export_policy,
        SelectionExportPolicy::SelectedOrFullDocument
    );
    assert!(payload.metadata.used_selection);
    assert!(!payload.metadata.fallback_to_full_document);
    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::CopyMarkdownBlock,
            TextActionInput {
                language: Some("text".into()),
                ..TextActionInput::empty()
            },
        ),
        TextActionOutput::Text("```text\ntwo\n```".into())
    );
    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::CopyPromptBlock,
            TextActionInput {
                source: Some("notes.md".into()),
                ..TextActionInput::empty()
            },
        ),
        TextActionOutput::Text("Source: notes.md\n\n```text\ntwo\n```".into())
    );

    assert_eq!(editor.text(), text);
    assert_eq!(editor.selection(), selection);
}

#[test]
fn copy_plain_selection_export_policy_is_explicit() {
    let mut editor = TextEditorPlain::from_text("one\ntwo".into());
    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 0, column: 0 },
        caret: EditorPosition { line: 0, column: 3 },
    }));

    let output = execute_text_action(
        &mut editor,
        TextActionId::CopyPlain,
        TextActionInput {
            selection_export_policy: Some(SelectionExportPolicy::FullDocument),
            ..TextActionInput::empty()
        },
    );
    let TextActionOutput::ClipboardPayload(payload) = output else {
        panic!("copy plain should return clipboard payload");
    };

    assert_eq!(payload.text, "one\ntwo");
    assert_eq!(
        payload.metadata.export_policy,
        SelectionExportPolicy::FullDocument
    );
    assert!(!payload.metadata.used_selection);
    assert_eq!(payload.metadata.selection, None);
}

#[test]
fn copy_code_fence_returns_clipboard_transform_receipt() {
    let editor = TextEditorPlain::from_text("let x = 1;".into());

    assert_eq!(
        execute_text_action(
            &mut editor.clone(),
            TextActionId::CopyCodeFence,
            TextActionInput {
                language: Some("rust".into()),
                ..TextActionInput::empty()
            },
        ),
        TextActionOutput::ClipboardTransform(text_editor_clipboard::ClipboardTransformResult {
            text: "```rust\nlet x = 1;\n```".into(),
            changes: vec![],
            warnings: vec![],
        })
    );
}

#[test]
fn select_all_routes_to_editor_selection_only() {
    let mut editor = TextEditorPlain::from_text("alpha\nbeta".into());
    let text = editor.text().to_owned();
    let undo_depth = editor.undo_depth();

    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::SelectAll,
            TextActionInput::empty()
        ),
        TextActionOutput::None
    );

    assert_eq!(editor.text(), text);
    assert_eq!(editor.undo_depth(), undo_depth);
    assert_eq!(editor.selected_text().as_deref(), Some("alpha\nbeta"));
}

#[test]
fn line_actions_return_current_and_explicit_ranges() {
    let mut editor = TextEditorPlain::from_text("zero\none\ntwo\nthree".into());
    editor.apply(EditorCommand::SetSelection(EditorSelection::collapsed(
        EditorPosition { line: 2, column: 1 },
    )));

    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::CurrentLineText,
            TextActionInput::empty(),
        ),
        TextActionOutput::Text("two".into())
    );
    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::LineRangeText,
            TextActionInput {
                start_line: Some(1),
                end_line: Some(99),
                ..TextActionInput::empty()
            },
        ),
        TextActionOutput::Text("one\ntwo\nthree".into())
    );
}

#[test]
fn line_range_requires_explicit_range() {
    let mut editor = TextEditorPlain::from_text("zero\none".into());

    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::LineRangeText,
            TextActionInput::empty(),
        ),
        TextActionOutput::Disabled {
            action_id: "text.line_range_text".into(),
            reason: "line range is missing".into()
        }
    );
}

#[test]
fn trim_trailing_whitespace_is_output_only() {
    let mut editor = TextEditorPlain::from_text("alpha  \n\nbeta\t".into());
    let text = editor.text().to_owned();
    let selection = editor.selection();

    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::TrimTrailingWhitespace,
            TextActionInput::empty(),
        ),
        TextActionOutput::Text("alpha\n\nbeta".into())
    );
    assert_eq!(editor.text(), text);
    assert_eq!(editor.selection(), selection);

    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::TrimTrailingWhitespace,
            TextActionInput {
                text: Some("x  \ny\t\n".into()),
                ..TextActionInput::empty()
            },
        ),
        TextActionOutput::Text("x\ny\n".into())
    );
}

#[test]
fn clipboard_cleanup_actions_return_receipts_without_mutation() {
    let mut editor = TextEditorPlain::from_text("alpha  \n\u{1b}[31mbeta\u{1b}[0m\t".into());
    let text = editor.text().to_owned();
    let selection = editor.selection();

    let output = execute_text_action(
        &mut editor,
        TextActionId::CleanBasic,
        TextActionInput {
            strip_ansi_escape_codes: Some(true),
            ..TextActionInput::empty()
        },
    );
    let TextActionOutput::ClipboardTransform(result) = output else {
        panic!("clean basic should return clipboard transform result");
    };

    assert_eq!(result.text, "alpha\nbeta");
    assert_eq!(
        result
            .changes
            .iter()
            .map(|change| change.change_id)
            .collect::<Vec<_>>(),
        vec![
            ClipboardChangeId::StrippedAnsiEscapeCodes,
            ClipboardChangeId::TrimmedTrailingWhitespace,
        ]
    );
    assert_eq!(editor.text(), text);
    assert_eq!(editor.selection(), selection);
}

#[test]
fn normalize_and_strip_actions_use_input_text_when_provided() {
    let mut editor = TextEditorPlain::new();

    let output = execute_text_action(
        &mut editor,
        TextActionId::NormalizeLineEndings,
        TextActionInput {
            text: Some("a\r\nb\rc".into()),
            ..TextActionInput::empty()
        },
    );
    let TextActionOutput::ClipboardTransform(result) = output else {
        panic!("normalize should return clipboard transform result");
    };
    assert_eq!(result.text, "a\nb\nc");
    assert_eq!(
        result.changes[0].change_id,
        ClipboardChangeId::NormalizedLineEndings
    );

    let output = execute_text_action(
        &mut editor,
        TextActionId::StripAnsiEscapeCodes,
        TextActionInput {
            text: Some("\u{1b}[32mgreen\u{1b}[0m".into()),
            ..TextActionInput::empty()
        },
    );
    let TextActionOutput::ClipboardTransform(result) = output else {
        panic!("strip ANSI should return clipboard transform result");
    };
    assert_eq!(result.text, "green");
    assert_eq!(
        result.changes[0].change_id,
        ClipboardChangeId::StrippedAnsiEscapeCodes
    );
}

#[test]
fn cleanup_actions_disable_when_editor_and_input_are_empty() {
    let mut editor = TextEditorPlain::new();

    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::CleanBasic,
            TextActionInput::empty()
        ),
        TextActionOutput::Disabled {
            action_id: "text.clean_basic".into(),
            reason: "document and input text are empty".into()
        }
    );
}

#[test]
fn document_text_actions_disable_on_empty_document() {
    let mut editor = TextEditorPlain::new();

    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::CopyPlain,
            TextActionInput::empty()
        ),
        TextActionOutput::Disabled {
            action_id: "text.copy_plain".into(),
            reason: "document is empty".into()
        }
    );
    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::SelectAll,
            TextActionInput::empty()
        ),
        TextActionOutput::Disabled {
            action_id: "text.select_all".into(),
            reason: "document is empty".into()
        }
    );
    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::CurrentLineText,
            TextActionInput::empty(),
        ),
        TextActionOutput::Text(String::new())
    );
}
