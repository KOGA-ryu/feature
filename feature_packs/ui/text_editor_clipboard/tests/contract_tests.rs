use feature_core::parse_feature_manifest;
use text_editor_clipboard::{
    CleanBasicPolicy, ClipboardChangeId, ClipboardLineRange, ClipboardPayloadKind,
    ClipboardWarningId, ClipboardWarningSeverity, FEATURE_ID, SelectionExportPolicy, clean_basic,
    copy_clean_payload_with_policy, copy_code_fence, copy_exact, copy_exact_payload,
    copy_exact_payload_with_policy, copy_markdown_block, copy_prompt_block,
    normalize_line_endings_with_report, sample_fixture, strip_ansi_escape_codes_with_report,
    trim_trailing_whitespace_with_report,
};
use text_editor_plain::{EditorCommand, EditorPosition, EditorSelection, TextEditorPlain};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "text_editor_clipboard");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec![
            "document_text",
            "selection",
            "clipboard_policy",
            "selection_export_policy"
        ]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "clipboard_text",
            "clipboard_payload",
            "payload_metadata",
            "transform_changes",
            "transform_warnings"
        ]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(value["source"].as_str(), Some("notes/session.md"));
    assert_eq!(
        value["default_export_policy"].as_str(),
        Some("selected_or_full_document")
    );
}

#[test]
fn exact_copy_returns_selected_or_all_text_with_empty_receipt() {
    let mut editor = TextEditorPlain::from_text("alpha\nbeta".into());
    assert_eq!(copy_exact(&editor).text, "alpha\nbeta");
    assert!(copy_exact(&editor).changes.is_empty());
    assert!(copy_exact(&editor).warnings.is_empty());

    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 1, column: 0 },
        caret: EditorPosition { line: 1, column: 4 },
    }));
    let result = copy_exact(&editor);

    assert_eq!(result.text, "beta");
    assert!(result.changes.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn exact_payload_defaults_to_selected_or_full_document_policy() {
    let mut editor = TextEditorPlain::from_text("alpha\n    beta\ngamma".into());

    let full_payload = copy_exact_payload(&editor);
    assert_eq!(full_payload.text, "alpha\n    beta\ngamma");
    assert_eq!(full_payload.receipt.text, full_payload.text);
    assert!(full_payload.receipt.changes.is_empty());
    assert_eq!(
        full_payload.metadata.payload_kind,
        ClipboardPayloadKind::ExactText
    );
    assert_eq!(
        full_payload.metadata.export_policy,
        SelectionExportPolicy::SelectedOrFullDocument
    );
    assert!(!full_payload.metadata.used_selection);
    assert!(full_payload.metadata.fallback_to_full_document);
    assert_eq!(
        full_payload.metadata.line_range,
        Some(ClipboardLineRange {
            start_line: 0,
            end_line: 2,
        })
    );

    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 1, column: 4 },
        caret: EditorPosition { line: 1, column: 8 },
    }));
    let selected_payload = copy_exact_payload(&editor);

    assert_eq!(selected_payload.text, "beta");
    assert!(selected_payload.metadata.used_selection);
    assert!(!selected_payload.metadata.fallback_to_full_document);
    assert_eq!(
        selected_payload.metadata.selection,
        Some(editor.selection())
    );
    assert_eq!(
        selected_payload.metadata.line_range,
        Some(ClipboardLineRange {
            start_line: 1,
            end_line: 1,
        })
    );
    assert_eq!(editor.text(), "alpha\n    beta\ngamma");
}

#[test]
fn selection_export_policy_can_require_selection_or_full_document() {
    let mut editor = TextEditorPlain::from_text("alpha\nbeta".into());

    let selection_only =
        copy_exact_payload_with_policy(&editor, SelectionExportPolicy::SelectionOnly);
    assert_eq!(selection_only.text, "");
    assert!(!selection_only.metadata.used_selection);
    assert!(!selection_only.metadata.fallback_to_full_document);
    assert_eq!(selection_only.metadata.line_range, None);

    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 0, column: 1 },
        caret: EditorPosition { line: 0, column: 4 },
    }));
    let full_document =
        copy_exact_payload_with_policy(&editor, SelectionExportPolicy::FullDocument);

    assert_eq!(full_document.text, "alpha\nbeta");
    assert!(!full_document.metadata.used_selection);
    assert_eq!(full_document.metadata.selection, None);
    assert_eq!(
        full_document.metadata.line_range,
        Some(ClipboardLineRange {
            start_line: 0,
            end_line: 1,
        })
    );
    assert_eq!(editor.selected_text().as_deref(), Some("lph"));
}

#[test]
fn cleaned_payload_preserves_transform_receipts_and_metadata() {
    let mut editor = TextEditorPlain::from_text("    alpha  \n\u{1b}[31mbeta\u{1b}[0m\t".into());
    editor.apply(EditorCommand::SelectAll);
    let before = editor.text().to_owned();

    let payload = copy_clean_payload_with_policy(
        &editor,
        SelectionExportPolicy::SelectedOrFullDocument,
        CleanBasicPolicy {
            normalize_line_endings: true,
            trim_trailing_whitespace: true,
            strip_ansi_escape_codes: true,
        },
    );
    let change_ids: Vec<ClipboardChangeId> = payload
        .receipt
        .changes
        .iter()
        .map(|change| change.change_id)
        .collect();

    assert_eq!(payload.text, "    alpha\nbeta");
    assert_eq!(payload.receipt.text, payload.text);
    assert_eq!(
        payload.metadata.payload_kind,
        ClipboardPayloadKind::CleanedText
    );
    assert!(payload.metadata.used_selection);
    assert_eq!(payload.metadata.character_count, 14);
    assert_eq!(payload.metadata.line_count, 2);
    assert_eq!(payload.metadata.first_line_indent.as_deref(), Some("    "));
    assert_eq!(
        change_ids,
        vec![
            ClipboardChangeId::StrippedAnsiEscapeCodes,
            ClipboardChangeId::TrimmedTrailingWhitespace,
        ]
    );
    assert_eq!(editor.text(), before);
}

#[test]
fn markdown_prompt_and_code_fence_outputs_are_exact_and_non_mutating() {
    let mut editor = TextEditorPlain::from_text("fn main() {\n    println!(\"hi\");\n}\n".into());
    editor.apply(EditorCommand::SelectAll);
    let selection = editor.selection();
    let text = editor.text().to_owned();

    assert_eq!(
        copy_markdown_block(&editor, Some("rust")).text,
        "```rust\nfn main() {\n    println!(\"hi\");\n}\n```"
    );
    assert_eq!(
        copy_prompt_block(&editor, Some("notes/session.md")).text,
        "Source: notes/session.md\n\n```text\nfn main() {\n    println!(\"hi\");\n}\n```"
    );
    assert_eq!(
        copy_code_fence(&editor, None).text,
        "```text\nfn main() {\n    println!(\"hi\");\n}\n```"
    );

    assert_eq!(editor.text(), text);
    assert_eq!(editor.selection(), selection);
}

#[test]
fn prompt_block_delegates_blank_source_to_plain_editor_contract() {
    let editor = TextEditorPlain::from_text("one".into());

    assert_eq!(
        copy_prompt_block(&editor, Some("   ")).text,
        "Source: unknown\n\n```text\none\n```"
    );
}

#[test]
fn normalize_line_endings_reports_changes() {
    let result = normalize_line_endings_with_report("alpha\r\nbeta\rgamma\n");

    assert_eq!(result.text, "alpha\nbeta\ngamma\n");
    assert_eq!(result.warnings, vec![]);
    assert_eq!(result.changes.len(), 1);
    assert_eq!(
        result.changes[0].change_id,
        ClipboardChangeId::NormalizedLineEndings
    );
    assert_eq!(result.changes[0].count, 2);
}

#[test]
fn trim_trailing_whitespace_reports_changed_lines_without_collapsing_blanks() {
    let result = trim_trailing_whitespace_with_report("alpha  \n\t\nbeta\t \n\n");

    assert_eq!(result.text, "alpha\n\nbeta\n\n");
    assert_eq!(result.warnings, vec![]);
    assert_eq!(result.changes.len(), 1);
    assert_eq!(
        result.changes[0].change_id,
        ClipboardChangeId::TrimmedTrailingWhitespace
    );
    assert_eq!(result.changes[0].count, 3);
}

#[test]
fn strip_ansi_escape_codes_reports_removed_sequences() {
    let result = strip_ansi_escape_codes_with_report("\u{1b}[31mred\u{1b}[0m plain");

    assert_eq!(result.text, "red plain");
    assert_eq!(result.warnings, vec![]);
    assert_eq!(result.changes.len(), 1);
    assert_eq!(
        result.changes[0].change_id,
        ClipboardChangeId::StrippedAnsiEscapeCodes
    );
    assert_eq!(result.changes[0].count, 2);
}

#[test]
fn incomplete_ansi_sequence_emits_warning_without_fake_change() {
    let result = strip_ansi_escape_codes_with_report("alpha \u{1b}[31");

    assert_eq!(result.text, "alpha ");
    assert!(result.changes.is_empty());
    assert_eq!(result.warnings.len(), 1);
    assert_eq!(
        result.warnings[0].warning_id,
        ClipboardWarningId::AnsiEscapeSequenceIncomplete
    );
    assert_eq!(result.warnings[0].severity, ClipboardWarningSeverity::Warn);
}

#[test]
fn clean_basic_applies_only_named_safe_transforms() {
    let result = clean_basic(
        "alpha  \r\n\u{1b}[31mbeta\u{1b}[0m\t\n",
        CleanBasicPolicy {
            normalize_line_endings: true,
            trim_trailing_whitespace: true,
            strip_ansi_escape_codes: true,
        },
    );
    let change_ids: Vec<ClipboardChangeId> = result
        .changes
        .iter()
        .map(|change| change.change_id)
        .collect();

    assert_eq!(result.text, "alpha\nbeta\n");
    assert_eq!(
        change_ids,
        vec![
            ClipboardChangeId::NormalizedLineEndings,
            ClipboardChangeId::StrippedAnsiEscapeCodes,
            ClipboardChangeId::TrimmedTrailingWhitespace,
        ]
    );
    assert_eq!(result.warnings, vec![]);
}

#[test]
fn clean_basic_default_does_not_strip_ansi_or_unwrap_soft_wraps() {
    let result = clean_basic(
        "long\nwrapped \u{1b}[31mred\u{1b}[0m  ",
        CleanBasicPolicy::default(),
    );

    assert_eq!(result.text, "long\nwrapped \u{1b}[31mred\u{1b}[0m");
    assert_eq!(result.changes.len(), 1);
    assert_eq!(
        result.changes[0].change_id,
        ClipboardChangeId::TrimmedTrailingWhitespace
    );
}

#[test]
fn unchanged_cleanup_returns_empty_receipt() {
    let result = clean_basic("alpha\nbeta", CleanBasicPolicy::default());

    assert_eq!(result.text, "alpha\nbeta");
    assert!(result.changes.is_empty());
    assert!(result.warnings.is_empty());
}
