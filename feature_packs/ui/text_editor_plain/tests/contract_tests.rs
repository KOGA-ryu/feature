use feature_core::parse_feature_manifest;
use text_editor_plain::{
    EditorCommand, EditorPosition, EditorSelection, FEATURE_ID, SampleDocumentFixture,
    TextEditorPlain, sample_document_text, sample_fixture, trim_trailing_whitespace_text,
};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "text_editor_plain");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec!["document_text", "editor_commands"]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "document_text",
            "selection",
            "dirty_state",
            "history_state",
            "export_text"
        ]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: SampleDocumentFixture =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert!(fixture.document_text.lines().count() >= 4);
}

#[test]
fn from_text_normalizes_line_endings() {
    let editor = TextEditorPlain::from_text("alpha\r\nbeta\rgamma".into());
    assert_eq!(editor.text(), "alpha\nbeta\ngamma");
    assert_eq!(editor.line_count(), 3);
    assert!(!editor.is_dirty());
}

#[test]
fn insert_text_at_caret_and_replace_selection() {
    let mut editor = TextEditorPlain::new();
    editor.apply(EditorCommand::InsertText("hello".into()));
    assert_eq!(editor.text(), "hello");
    assert_eq!(editor.cursor(), EditorPosition { line: 0, column: 5 });
    assert!(editor.is_dirty());

    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 0, column: 1 },
        caret: EditorPosition { line: 0, column: 4 },
    }));
    editor.apply(EditorCommand::InsertText("i".into()));
    assert_eq!(editor.text(), "hio");
    assert_eq!(editor.cursor(), EditorPosition { line: 0, column: 2 });
}

#[test]
fn newline_and_delete_behaviors_work_across_lines() {
    let mut editor = TextEditorPlain::from_text("ab".into());
    editor.apply(EditorCommand::MoveRight { extend: false });
    editor.apply(EditorCommand::InsertNewline);
    assert_eq!(editor.text(), "a\nb");
    assert_eq!(editor.cursor(), EditorPosition { line: 1, column: 0 });

    editor.apply(EditorCommand::Backspace);
    assert_eq!(editor.text(), "ab");
    assert_eq!(editor.cursor(), EditorPosition { line: 0, column: 1 });

    editor.apply(EditorCommand::MoveRight { extend: false });
    editor.apply(EditorCommand::MoveLineStart { extend: false });
    editor.apply(EditorCommand::DeleteForward);
    assert_eq!(editor.text(), "b");

    let mut join_editor = TextEditorPlain::from_text("ab\ncd".into());
    join_editor.apply(EditorCommand::SetSelection(EditorSelection::collapsed(
        EditorPosition { line: 0, column: 2 },
    )));
    join_editor.apply(EditorCommand::DeleteForward);
    assert_eq!(join_editor.text(), "abcd");
}

#[test]
fn navigation_and_preferred_column_are_preserved() {
    let mut editor = TextEditorPlain::from_text("wide\nx\nmedium".into());
    editor.apply(EditorCommand::MoveLineEnd { extend: false });
    editor.apply(EditorCommand::MoveUp { extend: false });
    assert_eq!(editor.cursor(), EditorPosition { line: 0, column: 4 });

    editor.apply(EditorCommand::MoveDown { extend: false });
    assert_eq!(editor.cursor(), EditorPosition { line: 1, column: 1 });

    editor.apply(EditorCommand::MoveDown { extend: false });
    assert_eq!(editor.cursor(), EditorPosition { line: 2, column: 4 });

    editor.apply(EditorCommand::MoveLeft { extend: false });
    editor.apply(EditorCommand::MoveUp { extend: false });
    assert_eq!(editor.cursor(), EditorPosition { line: 1, column: 1 });
}

#[test]
fn select_all_and_set_selection_clamp_to_bounds() {
    let mut editor = TextEditorPlain::from_text("alpha\nbeta".into());
    editor.apply(EditorCommand::SelectAll);
    assert!(editor.has_selection());
    assert_eq!(editor.selected_text().as_deref(), Some("alpha\nbeta"));

    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition {
            line: 99,
            column: 99,
        },
        caret: EditorPosition { line: 0, column: 2 },
    }));
    assert_eq!(
        editor.selection(),
        EditorSelection {
            anchor: EditorPosition { line: 1, column: 4 },
            caret: EditorPosition { line: 0, column: 2 },
        }
    );
}

#[test]
fn dirty_state_and_mark_clean_follow_rules() {
    let mut editor = TextEditorPlain::from_text("alpha".into());
    assert!(!editor.is_dirty());

    editor.apply(EditorCommand::MoveRight { extend: false });
    assert!(!editor.is_dirty());

    editor.apply(EditorCommand::InsertText("!".into()));
    assert!(editor.is_dirty());

    editor.mark_clean();
    assert!(!editor.is_dirty());

    editor.load_text("fresh".into());
    assert_eq!(editor.text(), "fresh");
    assert!(!editor.is_dirty());
    assert_eq!(editor.undo_depth(), 0);
    assert_eq!(editor.redo_depth(), 0);
}

#[test]
fn undo_redo_restore_text_and_selection() {
    let mut editor = TextEditorPlain::from_text("one\ntwo".into());
    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 1, column: 0 },
        caret: EditorPosition { line: 1, column: 3 },
    }));
    editor.apply(EditorCommand::InsertText("TWO".into()));
    assert_eq!(editor.text(), "one\nTWO");
    assert_eq!(editor.cursor(), EditorPosition { line: 1, column: 3 });

    editor.apply(EditorCommand::Undo);
    assert_eq!(editor.text(), "one\ntwo");
    assert_eq!(
        editor.selection(),
        EditorSelection {
            anchor: EditorPosition { line: 1, column: 0 },
            caret: EditorPosition { line: 1, column: 3 },
        }
    );

    editor.apply(EditorCommand::Redo);
    assert_eq!(editor.text(), "one\nTWO");
    assert_eq!(editor.cursor(), EditorPosition { line: 1, column: 3 });
}

#[test]
fn history_bound_is_enforced() {
    let mut editor = TextEditorPlain::new();
    for _ in 0..120 {
        editor.apply(EditorCommand::InsertText("a".into()));
    }

    assert_eq!(editor.undo_depth(), 100);
    assert_eq!(editor.redo_depth(), 0);
}

#[test]
fn sample_document_text_loads() {
    let document = sample_document_text().expect("sample document should load");
    assert!(document.contains("Feature Lab text editor sample."));
    assert!(document.contains("Cmd/Ctrl + Z"));
}

#[test]
fn selected_text_or_all_prefers_non_empty_selection() {
    let mut editor = TextEditorPlain::from_text("alpha\nbeta\ngamma".into());
    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 1, column: 0 },
        caret: EditorPosition { line: 1, column: 4 },
    }));

    assert_eq!(editor.selected_text_or_all(), "beta");
    assert_eq!(editor.copy_plain(), "beta");
}

#[test]
fn selected_text_or_all_falls_back_to_full_document() {
    let editor = TextEditorPlain::from_text("alpha\nbeta".into());

    assert_eq!(editor.selected_text(), None);
    assert_eq!(editor.selected_text_or_all(), "alpha\nbeta");
    assert_eq!(editor.copy_plain(), "alpha\nbeta");
}

#[test]
fn empty_document_export_is_deterministic() {
    let editor = TextEditorPlain::new();

    assert_eq!(editor.copy_plain(), "");
    assert_eq!(editor.copy_markdown_block(Some("text")), "```text\n```");
    assert_eq!(
        editor.copy_prompt_block(None),
        "source: unknown\ncontent:\n```text\n```"
    );
}

#[test]
fn markdown_block_preserves_exact_selected_content() {
    let mut editor = TextEditorPlain::from_text("fn main() {\n    println!(\"hi\");\n}\n".into());
    editor.apply(EditorCommand::SelectAll);

    assert_eq!(
        editor.copy_markdown_block(Some("rust")),
        "```rust\nfn main() {\n    println!(\"hi\");\n}\n```"
    );
    assert_eq!(
        editor.copy_markdown_block(None),
        "```\nfn main() {\n    println!(\"hi\");\n}\n```"
    );
}

#[test]
fn prompt_block_includes_source_or_unknown() {
    let mut editor = TextEditorPlain::from_text("one\ntwo".into());
    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 0, column: 0 },
        caret: EditorPosition { line: 0, column: 3 },
    }));

    assert_eq!(
        editor.copy_prompt_block(Some("notes/session.md")),
        "source: notes/session.md\ncontent:\n```text\none\n```"
    );
    assert_eq!(
        editor.copy_prompt_block(Some("   ")),
        "source: unknown\ncontent:\n```text\none\n```"
    );
}

#[test]
fn line_helpers_return_current_single_and_clamped_ranges() {
    let mut editor = TextEditorPlain::from_text("zero\none\ntwo\nthree".into());
    editor.apply(EditorCommand::SetSelection(EditorSelection::collapsed(
        EditorPosition { line: 2, column: 1 },
    )));

    assert_eq!(editor.current_line_text(), "two");
    assert_eq!(editor.line_text(1).as_deref(), Some("one"));
    assert_eq!(editor.line_text(99), None);
    assert_eq!(editor.line_range_text(1, 2), "one\ntwo");
    assert_eq!(editor.line_range_text(2, 99), "two\nthree");
    assert_eq!(editor.line_range_text(99, 100), "");
    assert_eq!(editor.line_range_text(3, 2), "");
}

#[test]
fn trim_trailing_whitespace_preserves_blank_lines_and_normalizes_endings() {
    assert_eq!(
        trim_trailing_whitespace_text("alpha  \r\n\t\r\nbeta\t \n\n"),
        "alpha\n\nbeta\n\n"
    );
}

#[test]
fn export_helpers_do_not_mutate_editor_state() {
    let mut editor = TextEditorPlain::from_text("alpha\nbeta\n".into());
    editor.apply(EditorCommand::MoveDown { extend: false });
    editor.apply(EditorCommand::MoveRight { extend: false });
    editor.apply(EditorCommand::InsertText("!".into()));
    editor.apply(EditorCommand::SetSelection(EditorSelection {
        anchor: EditorPosition { line: 1, column: 0 },
        caret: EditorPosition { line: 1, column: 2 },
    }));

    let text = editor.text().to_owned();
    let selection = editor.selection();
    let dirty = editor.is_dirty();
    let undo_depth = editor.undo_depth();
    let redo_depth = editor.redo_depth();

    let _ = editor.selected_text_or_all();
    let _ = editor.copy_plain();
    let _ = editor.copy_markdown_block(Some("text"));
    let _ = editor.copy_prompt_block(Some("fixture"));
    let _ = editor.current_line_text();
    let _ = editor.line_text(0);
    let _ = editor.line_range_text(0, 2);

    assert_eq!(editor.text(), text);
    assert_eq!(editor.selection(), selection);
    assert_eq!(editor.is_dirty(), dirty);
    assert_eq!(editor.undo_depth(), undo_depth);
    assert_eq!(editor.redo_depth(), redo_depth);
}
