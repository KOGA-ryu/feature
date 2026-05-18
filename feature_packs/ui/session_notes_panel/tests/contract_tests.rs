use feature_core::parse_feature_manifest;
use session_notes_panel::{FEATURE_ID, SessionNotesPanelMode, sample_fixture, sample_state};
use text_editor_plain::EditorCommand;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "session_notes_panel");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec![
            "entries",
            "query",
            "selection",
            "editor_commands",
            "snapshot_actions",
        ]
    );
    assert_eq!(
        manifest.outputs.items,
        vec!["notes", "mode", "selected_editor", "selected_history"]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(
        fixture["notes"]["entries"].as_array().map(Vec::len),
        Some(2)
    );
    assert_eq!(fixture["selected_index"].as_u64(), Some(1));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.mode, SessionNotesPanelMode::Browse);
    assert_eq!(
        state.notes().selected_entry().map(|entry| entry.id),
        Some(2)
    );
    assert!(state.selected_editor().is_none());
}

#[test]
fn compose_path_writes_new_note() {
    let mut state = sample_state().expect("sample state should load");
    state.start_compose();
    state.apply_composer_command(EditorCommand::InsertText("Operator follow-up".into()));

    assert!(state.commit_compose("2026-04-27T10:00:00Z"));
    assert_eq!(state.mode, SessionNotesPanelMode::Browse);
    assert_eq!(state.notes().latest_entry().map(|entry| entry.id), Some(3));
    assert_eq!(
        state
            .notes()
            .latest_entry()
            .map(|entry| entry.text.as_str()),
        Some("Operator follow-up")
    );
}

#[test]
fn edit_path_loads_selected_note_and_save_writes_back() {
    let mut state = sample_state().expect("sample state should load");
    assert!(state.start_edit_selected());

    let original = state
        .selected_editor()
        .expect("selected editor should exist")
        .text()
        .to_owned();
    assert_eq!(original, "Review receipt after workspace proof.");

    assert!(state.apply_selected_editor_command(EditorCommand::MoveLineEnd { extend: false }));
    assert!(state.apply_selected_editor_command(EditorCommand::InsertText(" Updated".into())));
    assert!(state.save_selected_edit());

    assert_eq!(state.mode, SessionNotesPanelMode::Browse);
    assert_eq!(
        state
            .notes()
            .selected_entry()
            .map(|entry| entry.text.as_str()),
        Some("Review receipt after workspace proof. Updated")
    );
}

#[test]
fn selection_change_or_delete_resets_edit_mode() {
    let mut state = sample_state().expect("sample state should load");
    assert!(state.start_edit_selected());
    state.set_selected_index(0);
    assert_eq!(state.mode, SessionNotesPanelMode::Browse);
    assert!(state.selected_editor().is_none());

    assert!(state.start_edit_selected());
    assert!(state.delete_selected());
    assert_eq!(state.mode, SessionNotesPanelMode::Browse);
    assert!(state.selected_editor().is_none());
}

#[test]
fn pin_and_delete_delegate_to_note_ledger() {
    let mut state = sample_state().expect("sample state should load");
    assert!(state.toggle_pinned_selected());
    assert_eq!(
        state.notes().selected_entry().map(|entry| entry.pinned),
        Some(true)
    );

    assert!(state.delete_selected());
    assert_eq!(state.notes().counts().total, 1);
    assert_eq!(
        state.notes().selected_entry().map(|entry| entry.id),
        Some(1)
    );
}

#[test]
fn selected_edit_snapshots_and_restore_use_document_history() {
    let mut state = sample_state().expect("sample state should load");
    assert!(state.start_edit_selected());
    assert!(state.apply_selected_editor_command(EditorCommand::MoveLineEnd { extend: false }));
    assert!(state.apply_selected_editor_command(EditorCommand::InsertText(" Draft".into())));
    assert!(state.record_selected_snapshot("draft", "2026-04-27T10:00:00Z"));
    assert_eq!(state.selected_history().revision_count(), 1);

    assert!(state.apply_selected_editor_command(EditorCommand::InsertText(" changed".into())));
    state.selected_history.set_selected_revision_index(0);
    assert!(state.restore_selected_revision());

    assert_eq!(
        state
            .selected_editor()
            .map(|editor| editor.text().to_owned()),
        Some("Review receipt after workspace proof. Draft".into())
    );
}
