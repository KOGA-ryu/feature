use document_history::DocumentHistoryState;
use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};
use session_notes::{SessionNotesFixture, SessionNotesState};
use text_editor_plain::{EditorCommand, TextEditorPlain};

pub const FEATURE_ID: &str = "ui.session_notes_panel";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SessionNotesPanelMode {
    #[default]
    Browse,
    Compose,
    EditSelected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionNotesPanelFixture {
    pub notes: SessionNotesFixture,
    pub selected_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionNotesPanelState {
    pub notes: SessionNotesState,
    pub mode: SessionNotesPanelMode,
    pub composer: TextEditorPlain,
    pub selected_editor: Option<TextEditorPlain>,
    pub selected_history: DocumentHistoryState,
    #[serde(skip)]
    editing_note_id: Option<u64>,
}

impl Default for SessionNotesPanelState {
    fn default() -> Self {
        Self::new(SessionNotesState::default())
    }
}

impl SessionNotesPanelState {
    pub fn new(notes: SessionNotesState) -> Self {
        Self {
            notes,
            mode: SessionNotesPanelMode::Browse,
            composer: TextEditorPlain::new(),
            selected_editor: None,
            selected_history: DocumentHistoryState::new(String::new()),
            editing_note_id: None,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let fixture: SessionNotesPanelFixture =
            serde_json::from_str(raw).map_err(|error| error.to_string())?;
        let mut state = Self::new(SessionNotesState::new(fixture.notes.entries));
        state.notes.set_selected_index(fixture.selected_index);
        Ok(state)
    }

    pub fn set_filter_query(&mut self, query: impl Into<String>) {
        self.notes.set_query(query);
        self.reconcile_edit_session();
    }

    pub fn set_selected_index(&mut self, index: usize) {
        self.notes.set_selected_index(index);
        self.reconcile_edit_session();
    }

    pub fn start_compose(&mut self) {
        self.cancel_edit();
        self.mode = SessionNotesPanelMode::Compose;
        self.composer.load_text(String::new());
    }

    pub fn start_edit_selected(&mut self) -> bool {
        let Some(entry) = self.notes.selected_entry() else {
            return false;
        };

        self.mode = SessionNotesPanelMode::EditSelected;
        self.editing_note_id = Some(entry.id);
        self.selected_editor = Some(TextEditorPlain::from_text(entry.text.clone()));
        self.selected_history = DocumentHistoryState::new(entry.text.clone());
        true
    }

    pub fn cancel_edit(&mut self) {
        self.mode = SessionNotesPanelMode::Browse;
        self.selected_editor = None;
        self.selected_history = DocumentHistoryState::new(String::new());
        self.editing_note_id = None;
    }

    pub fn apply_composer_command(&mut self, command: EditorCommand) {
        self.composer.apply(command);
    }

    pub fn apply_selected_editor_command(&mut self, command: EditorCommand) -> bool {
        let Some(editor) = self.selected_editor.as_mut() else {
            return false;
        };

        editor.apply(command);
        self.selected_history
            .set_working_text(editor.text().to_owned());
        true
    }

    pub fn commit_compose(&mut self, created_at: impl Into<String>) -> bool {
        if self
            .notes
            .add_entry(self.composer.text().to_owned(), created_at)
        {
            self.composer.load_text(String::new());
            self.mode = SessionNotesPanelMode::Browse;
            true
        } else {
            false
        }
    }

    pub fn save_selected_edit(&mut self) -> bool {
        let Some(note_id) = self.editing_note_id else {
            return false;
        };
        let Some(editor) = self.selected_editor.as_ref() else {
            return false;
        };

        let saved = self.notes.update_entry(note_id, editor.text().to_owned());
        if saved {
            self.cancel_edit();
        }
        saved
    }

    pub fn toggle_pinned_selected(&mut self) -> bool {
        let toggled = self
            .notes
            .selected_entry()
            .map(|entry| entry.id)
            .and_then(|id| self.notes.toggle_pinned(id).then_some(id))
            .is_some();
        if toggled {
            self.reconcile_edit_session();
        }
        toggled
    }

    pub fn delete_selected(&mut self) -> bool {
        let deleted = self
            .notes
            .selected_entry()
            .map(|entry| entry.id)
            .and_then(|id| self.notes.delete_entry(id).then_some(id))
            .is_some();
        if deleted {
            self.reconcile_edit_session();
        }
        deleted
    }

    pub fn record_selected_snapshot(
        &mut self,
        label: impl Into<String>,
        recorded_at: impl Into<String>,
    ) -> bool {
        if self.selected_editor.is_none() {
            return false;
        }
        self.selected_history.record_snapshot(label, recorded_at)
    }

    pub fn restore_selected_revision(&mut self) -> bool {
        let restored = self.selected_history.restore_selected_revision();
        if !restored {
            return false;
        }

        let restored_text = self.selected_history.working_text().to_owned();
        let Some(editor) = self.selected_editor.as_mut() else {
            return false;
        };
        replace_editor_text(editor, &restored_text);
        true
    }

    pub fn notes(&self) -> &SessionNotesState {
        &self.notes
    }

    pub fn composer(&self) -> &TextEditorPlain {
        &self.composer
    }

    pub fn selected_editor(&self) -> Option<&TextEditorPlain> {
        self.selected_editor.as_ref()
    }

    pub fn selected_history(&self) -> &DocumentHistoryState {
        &self.selected_history
    }

    fn reconcile_edit_session(&mut self) {
        let Some(active_note_id) = self.editing_note_id else {
            return;
        };
        let still_selected = self
            .notes
            .selected_entry()
            .map(|entry| entry.id)
            .is_some_and(|selected_id| selected_id == active_note_id);

        if !still_selected {
            self.cancel_edit();
        }
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_input.json")
}

pub fn sample_state() -> Result<SessionNotesPanelState, String> {
    SessionNotesPanelState::from_fixture_str(sample_fixture())
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

fn replace_editor_text(editor: &mut TextEditorPlain, replacement: &str) {
    editor.apply(EditorCommand::SelectAll);
    if replacement.is_empty() {
        editor.apply(EditorCommand::Backspace);
    } else {
        editor.apply(EditorCommand::InsertText(replacement.to_owned()));
    }
}
