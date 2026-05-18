use feature_core::parse_feature_manifest;
use text_editor_actions::{
    FEATURE_ID, TextActionCategory, TextActionId, TextActionInput, TextActionOutput,
    TextEnabledRule, TextHostPlacement, TextUndoBehavior, action_record, all_text_actions,
    execute_text_action, parse_text_action_id, sample_fixture,
};
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
        vec!["text_action_records", "text_action_output", "enabled_state"]
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
            "text.select_all",
            "text.current_line_text",
            "text.line_range_text",
            "text.trim_trailing_whitespace",
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

    assert_eq!(
        execute_text_action(
            &mut editor,
            TextActionId::CopyPlain,
            TextActionInput::empty()
        ),
        TextActionOutput::Text("two".into())
    );
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
