use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.text_editor_plain";
pub const DEFAULT_HISTORY_LIMIT: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct EditorPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EditorSelection {
    pub anchor: EditorPosition,
    pub caret: EditorPosition,
}

impl EditorSelection {
    pub fn collapsed(position: EditorPosition) -> Self {
        Self {
            anchor: position,
            caret: position,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EditorCommand {
    InsertText(String),
    InsertNewline,
    Backspace,
    DeleteForward,
    MoveLeft { extend: bool },
    MoveRight { extend: bool },
    MoveUp { extend: bool },
    MoveDown { extend: bool },
    MoveLineStart { extend: bool },
    MoveLineEnd { extend: bool },
    SelectAll,
    SetSelection(EditorSelection),
    Undo,
    Redo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EditorSnapshot {
    text: String,
    selection: EditorSelection,
    preferred_column: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEditorPlain {
    text: String,
    selection: EditorSelection,
    clean_text: String,
    #[serde(skip)]
    preferred_column: Option<usize>,
    #[serde(skip)]
    undo_stack: Vec<EditorSnapshot>,
    #[serde(skip)]
    redo_stack: Vec<EditorSnapshot>,
    #[serde(skip, default = "default_history_limit")]
    history_limit: usize,
}

impl Default for TextEditorPlain {
    fn default() -> Self {
        Self::new()
    }
}

impl TextEditorPlain {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            selection: EditorSelection::collapsed(EditorPosition::default()),
            clean_text: String::new(),
            preferred_column: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            history_limit: DEFAULT_HISTORY_LIMIT,
        }
    }

    pub fn from_text(text: String) -> Self {
        let normalized = normalize_line_endings(&text);
        Self {
            clean_text: normalized.clone(),
            text: normalized,
            selection: EditorSelection::collapsed(EditorPosition::default()),
            preferred_column: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            history_limit: DEFAULT_HISTORY_LIMIT,
        }
    }

    pub fn load_text(&mut self, text: String) {
        let normalized = normalize_line_endings(&text);
        self.text = normalized.clone();
        self.clean_text = normalized;
        self.selection = EditorSelection::collapsed(EditorPosition::default());
        self.preferred_column = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    pub fn apply(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::InsertText(text) => self.insert_text(text),
            EditorCommand::InsertNewline => self.insert_text("\n".into()),
            EditorCommand::Backspace => self.backspace(),
            EditorCommand::DeleteForward => self.delete_forward(),
            EditorCommand::MoveLeft { extend } => self.move_left(extend),
            EditorCommand::MoveRight { extend } => self.move_right(extend),
            EditorCommand::MoveUp { extend } => self.move_vertical(-1, extend),
            EditorCommand::MoveDown { extend } => self.move_vertical(1, extend),
            EditorCommand::MoveLineStart { extend } => self.move_line_edge(true, extend),
            EditorCommand::MoveLineEnd { extend } => self.move_line_edge(false, extend),
            EditorCommand::SelectAll => self.select_all(),
            EditorCommand::SetSelection(selection) => self.set_selection(selection),
            EditorCommand::Undo => self.undo(),
            EditorCommand::Redo => self.redo(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn selection(&self) -> EditorSelection {
        self.selection
    }

    pub fn cursor(&self) -> EditorPosition {
        self.selection.caret
    }

    pub fn has_selection(&self) -> bool {
        self.selection.anchor != self.selection.caret
    }

    pub fn selected_text(&self) -> Option<String> {
        if !self.has_selection() {
            return None;
        }

        let (start, end) = self.selection_bounds();
        Some(self.slice_text(start, end).to_owned())
    }

    pub fn selected_text_or_all(&self) -> String {
        self.selected_text().unwrap_or_else(|| self.text.clone())
    }

    pub fn copy_plain(&self) -> String {
        self.selected_text_or_all()
    }

    pub fn copy_markdown_block(&self, language: Option<&str>) -> String {
        fenced_block(language.unwrap_or_default(), &self.selected_text_or_all())
    }

    pub fn copy_prompt_block(&self, source: Option<&str>) -> String {
        let source = source
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("unknown");
        format!(
            "Source: {source}\n\n{}",
            fenced_block("text", &self.selected_text_or_all())
        )
    }

    pub fn current_line_text(&self) -> String {
        self.line_text(self.cursor().line).unwrap_or_default()
    }

    pub fn line_text(&self, index: usize) -> Option<String> {
        self.lines().get(index).map(|value| (*value).to_owned())
    }

    pub fn line_range_text(&self, start: usize, end_inclusive: usize) -> String {
        let lines = self.lines();
        if lines.is_empty() || start >= lines.len() {
            return String::new();
        }

        let end = end_inclusive.min(lines.len() - 1);
        if start > end {
            return String::new();
        }

        lines[start..=end].join("\n")
    }

    pub fn line_count(&self) -> usize {
        self.lines().len()
    }

    pub fn character_count(&self) -> usize {
        self.text.chars().count()
    }

    pub fn is_dirty(&self) -> bool {
        self.text != self.clean_text
    }

    pub fn mark_clean(&mut self) {
        self.clean_text = self.text.clone();
    }

    pub fn undo_depth(&self) -> usize {
        self.undo_stack.len()
    }

    pub fn redo_depth(&self) -> usize {
        self.redo_stack.len()
    }

    fn insert_text(&mut self, text: String) {
        if text.is_empty() {
            return;
        }

        let normalized = normalize_line_endings(&text);
        self.replace_selection_with(&normalized);
    }

    fn backspace(&mut self) {
        if self.has_selection() {
            self.replace_selection_with("");
            return;
        }

        let caret_index = self.position_to_char_index(self.cursor());
        if caret_index == 0 {
            return;
        }

        self.replace_char_range(caret_index - 1, caret_index, "");
    }

    fn delete_forward(&mut self) {
        if self.has_selection() {
            self.replace_selection_with("");
            return;
        }

        let caret_index = self.position_to_char_index(self.cursor());
        if caret_index >= self.character_count() {
            return;
        }

        self.replace_char_range(caret_index, caret_index + 1, "");
    }

    fn move_left(&mut self, extend: bool) {
        self.preferred_column = None;
        if !extend && self.has_selection() {
            let (start, _) = self.selection_bounds();
            self.selection = EditorSelection::collapsed(start);
            return;
        }

        let caret = self.cursor();
        let next = if caret.column > 0 {
            EditorPosition {
                line: caret.line,
                column: caret.column - 1,
            }
        } else if caret.line > 0 {
            EditorPosition {
                line: caret.line - 1,
                column: self.line_length(caret.line - 1),
            }
        } else {
            caret
        };
        self.set_cursor_position(next, extend);
    }

    fn move_right(&mut self, extend: bool) {
        self.preferred_column = None;
        if !extend && self.has_selection() {
            let (_, end) = self.selection_bounds();
            self.selection = EditorSelection::collapsed(end);
            return;
        }

        let caret = self.cursor();
        let line_length = self.line_length(caret.line);
        let next = if caret.column < line_length {
            EditorPosition {
                line: caret.line,
                column: caret.column + 1,
            }
        } else if caret.line + 1 < self.line_count() {
            EditorPosition {
                line: caret.line + 1,
                column: 0,
            }
        } else {
            caret
        };
        self.set_cursor_position(next, extend);
    }

    fn move_vertical(&mut self, delta: isize, extend: bool) {
        let caret = self.cursor();
        let target_column = self.preferred_column.unwrap_or(caret.column);
        let target_line = if delta < 0 {
            caret.line.saturating_sub(delta.unsigned_abs())
        } else {
            caret
                .line
                .saturating_add(delta as usize)
                .min(self.line_count().saturating_sub(1))
        };

        let next = EditorPosition {
            line: target_line,
            column: target_column.min(self.line_length(target_line)),
        };

        self.preferred_column = Some(target_column);
        self.set_cursor_position(next, extend);
    }

    fn move_line_edge(&mut self, start: bool, extend: bool) {
        self.preferred_column = None;
        let caret = self.cursor();
        let next = EditorPosition {
            line: caret.line,
            column: if start {
                0
            } else {
                self.line_length(caret.line)
            },
        };
        self.set_cursor_position(next, extend);
    }

    fn select_all(&mut self) {
        let end = self.position_from_char_index(self.character_count());
        self.selection = EditorSelection {
            anchor: EditorPosition::default(),
            caret: end,
        };
        self.preferred_column = None;
    }

    fn set_selection(&mut self, selection: EditorSelection) {
        self.selection = EditorSelection {
            anchor: self.clamp_position(selection.anchor),
            caret: self.clamp_position(selection.caret),
        };
        self.preferred_column = None;
    }

    fn undo(&mut self) {
        let Some(snapshot) = self.undo_stack.pop() else {
            return;
        };

        self.redo_stack.push(self.snapshot());
        self.restore(snapshot);
    }

    fn redo(&mut self) {
        let Some(snapshot) = self.redo_stack.pop() else {
            return;
        };

        self.undo_stack.push(self.snapshot());
        self.restore(snapshot);
    }

    fn replace_selection_with(&mut self, replacement: &str) {
        let (start, end) = self.selection_char_bounds();
        self.replace_char_range(start, end, replacement);
    }

    fn replace_char_range(&mut self, start: usize, end: usize, replacement: &str) {
        if start == end && replacement.is_empty() {
            return;
        }

        self.push_undo_snapshot();

        let start_byte = char_index_to_byte_index(&self.text, start);
        let end_byte = char_index_to_byte_index(&self.text, end);
        let normalized = normalize_line_endings(replacement);

        self.text.replace_range(start_byte..end_byte, &normalized);

        let caret_index = start + normalized.chars().count();
        let caret = self.position_from_char_index(caret_index);
        self.selection = EditorSelection::collapsed(caret);
        self.preferred_column = None;
        self.redo_stack.clear();
    }

    fn push_undo_snapshot(&mut self) {
        self.undo_stack.push(self.snapshot());
        if self.undo_stack.len() > self.history_limit {
            let overflow = self.undo_stack.len() - self.history_limit;
            self.undo_stack.drain(0..overflow);
        }
    }

    fn snapshot(&self) -> EditorSnapshot {
        EditorSnapshot {
            text: self.text.clone(),
            selection: self.selection,
            preferred_column: self.preferred_column,
        }
    }

    fn restore(&mut self, snapshot: EditorSnapshot) {
        self.text = snapshot.text;
        self.selection = snapshot.selection;
        self.preferred_column = snapshot.preferred_column;
    }

    fn set_cursor_position(&mut self, position: EditorPosition, extend: bool) {
        let clamped = self.clamp_position(position);
        if extend {
            self.selection.caret = clamped;
        } else {
            self.selection = EditorSelection::collapsed(clamped);
        }
    }

    fn selection_bounds(&self) -> (EditorPosition, EditorPosition) {
        if self.selection.anchor <= self.selection.caret {
            (self.selection.anchor, self.selection.caret)
        } else {
            (self.selection.caret, self.selection.anchor)
        }
    }

    fn selection_char_bounds(&self) -> (usize, usize) {
        let (start, end) = self.selection_bounds();
        (
            self.position_to_char_index(start),
            self.position_to_char_index(end),
        )
    }

    fn slice_text(&self, start: EditorPosition, end: EditorPosition) -> &str {
        let start_byte = char_index_to_byte_index(&self.text, self.position_to_char_index(start));
        let end_byte = char_index_to_byte_index(&self.text, self.position_to_char_index(end));
        &self.text[start_byte..end_byte]
    }

    fn clamp_position(&self, position: EditorPosition) -> EditorPosition {
        let last_line = self.line_count().saturating_sub(1);
        let line = position.line.min(last_line);
        let column = position.column.min(self.line_length(line));
        EditorPosition { line, column }
    }

    fn position_to_char_index(&self, position: EditorPosition) -> usize {
        let position = self.clamp_position(position);
        let lines = self.lines();

        let mut index = 0usize;
        for line in lines.iter().take(position.line) {
            index += line.chars().count() + 1;
        }
        index + position.column
    }

    fn position_from_char_index(&self, char_index: usize) -> EditorPosition {
        position_from_char_index(&self.text, char_index)
    }

    fn line_length(&self, line: usize) -> usize {
        self.lines()
            .get(line)
            .map_or(0, |value| value.chars().count())
    }

    fn lines(&self) -> Vec<&str> {
        self.text.split('\n').collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SampleDocumentFixture {
    pub document_text: String,
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

pub fn sample_document_text() -> Result<String, String> {
    let fixture: SampleDocumentFixture =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    Ok(normalize_line_endings(&fixture.document_text))
}

pub fn sample_editor() -> Result<TextEditorPlain, String> {
    sample_document_text().map(TextEditorPlain::from_text)
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

fn default_history_limit() -> usize {
    DEFAULT_HISTORY_LIMIT
}

fn normalize_line_endings(raw: &str) -> String {
    raw.replace("\r\n", "\n").replace('\r', "\n")
}

pub fn trim_trailing_whitespace_text(input: &str) -> String {
    normalize_line_endings(input)
        .split('\n')
        .map(|line| line.trim_end_matches([' ', '\t']))
        .collect::<Vec<_>>()
        .join("\n")
}

fn fenced_block(language: &str, content: &str) -> String {
    let language = language.trim();
    let mut output = if language.is_empty() {
        "```".to_string()
    } else {
        format!("```{language}")
    };
    output.push('\n');
    output.push_str(content);
    if !content.is_empty() && !content.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("```");
    output
}

fn char_index_to_byte_index(text: &str, char_index: usize) -> usize {
    if char_index == 0 {
        return 0;
    }
    text.char_indices()
        .nth(char_index)
        .map(|(byte_index, _)| byte_index)
        .unwrap_or(text.len())
}

fn position_from_char_index(text: &str, char_index: usize) -> EditorPosition {
    let target = char_index.min(text.chars().count());
    let mut current = 0usize;
    let mut line = 0usize;
    let mut column = 0usize;

    for character in text.chars() {
        if current == target {
            break;
        }

        if character == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
        current += 1;
    }

    EditorPosition { line, column }
}
