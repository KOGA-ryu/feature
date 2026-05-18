use session_notes_panel::SessionNotesPanelState;
use text_editor_plain::EditorCommand;

pub(super) fn sync_session_notes_panel_composer(panel: &mut SessionNotesPanelState, value: &str) {
    if panel.composer().text() == value {
        return;
    }

    panel.apply_composer_command(EditorCommand::SelectAll);
    if value.is_empty() {
        panel.apply_composer_command(EditorCommand::Backspace);
    } else {
        panel.apply_composer_command(EditorCommand::InsertText(value.to_owned()));
    }
}

pub(super) fn sync_session_notes_panel_selected_editor(
    panel: &mut SessionNotesPanelState,
    value: &str,
) {
    if panel.selected_editor().map(|editor| editor.text()) == Some(value) {
        return;
    }

    let _ = panel.apply_selected_editor_command(EditorCommand::SelectAll);
    if value.is_empty() {
        let _ = panel.apply_selected_editor_command(EditorCommand::Backspace);
    } else {
        let _ = panel.apply_selected_editor_command(EditorCommand::InsertText(value.to_owned()));
    }
}
