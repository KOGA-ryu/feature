use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};
use text_editor_clipboard::{
    CleanBasicPolicy, ClipboardPayload, ClipboardTransformResult, SelectionExportPolicy,
    clean_basic, copy_code_fence, copy_exact_payload_with_policy,
    normalize_line_endings_with_report, strip_ansi_escape_codes_with_report,
};
use text_editor_plain::{EditorCommand, TextEditorPlain, trim_trailing_whitespace_text};

pub const FEATURE_ID: &str = "ui.text_editor_actions";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextActionCategory {
    Clipboard,
    Selection,
    Cleanup,
    Lines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextEnabledRule {
    Always,
    DocumentHasText,
    DocumentOrInputHasText,
    HasLineRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextUndoBehavior {
    None,
    SelectionOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextHostPlacement {
    CommandPalette,
    ClipboardMenu,
    ContextMenu,
    Toolbar,
    EditMenu,
    CleanupMenu,
}

impl TextHostPlacement {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CommandPalette => "command_palette",
            Self::ClipboardMenu => "clipboard_menu",
            Self::ContextMenu => "context_menu",
            Self::Toolbar => "toolbar",
            Self::EditMenu => "edit_menu",
            Self::CleanupMenu => "cleanup_menu",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextActionId {
    CopyPlain,
    CopyMarkdownBlock,
    CopyPromptBlock,
    CopyCodeFence,
    SelectAll,
    CurrentLineText,
    LineRangeText,
    TrimTrailingWhitespace,
    CleanBasic,
    NormalizeLineEndings,
    StripAnsiEscapeCodes,
}

impl TextActionId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CopyPlain => "text.copy_plain",
            Self::CopyMarkdownBlock => "text.copy_markdown_block",
            Self::CopyPromptBlock => "text.copy_prompt_block",
            Self::CopyCodeFence => "text.copy_code_fence",
            Self::SelectAll => "text.select_all",
            Self::CurrentLineText => "text.current_line_text",
            Self::LineRangeText => "text.line_range_text",
            Self::TrimTrailingWhitespace => "text.trim_trailing_whitespace",
            Self::CleanBasic => "text.clean_basic",
            Self::NormalizeLineEndings => "text.normalize_line_endings",
            Self::StripAnsiEscapeCodes => "text.strip_ansi_escape_codes",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "text.copy_plain" => Some(Self::CopyPlain),
            "text.copy_markdown_block" => Some(Self::CopyMarkdownBlock),
            "text.copy_prompt_block" => Some(Self::CopyPromptBlock),
            "text.copy_code_fence" => Some(Self::CopyCodeFence),
            "text.select_all" => Some(Self::SelectAll),
            "text.current_line_text" => Some(Self::CurrentLineText),
            "text.line_range_text" => Some(Self::LineRangeText),
            "text.trim_trailing_whitespace" => Some(Self::TrimTrailingWhitespace),
            "text.clean_basic" => Some(Self::CleanBasic),
            "text.normalize_line_endings" => Some(Self::NormalizeLineEndings),
            "text.strip_ansi_escape_codes" => Some(Self::StripAnsiEscapeCodes),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextActionRecord {
    pub id: TextActionId,
    pub action_id: &'static str,
    pub label: &'static str,
    pub short_label: &'static str,
    pub category: TextActionCategory,
    pub icon: &'static str,
    pub tooltip: &'static str,
    pub enabled_rule: TextEnabledRule,
    pub undo_behavior: TextUndoBehavior,
    pub host_placements: &'static [TextHostPlacement],
}

impl TextActionRecord {
    pub fn is_enabled(&self, editor: &TextEditorPlain, input: &TextActionInput) -> bool {
        match self.enabled_rule {
            TextEnabledRule::Always => true,
            TextEnabledRule::DocumentHasText => !editor.text().is_empty(),
            TextEnabledRule::DocumentOrInputHasText => {
                !editor.text().is_empty()
                    || input.text.as_deref().is_some_and(|value| !value.is_empty())
            }
            TextEnabledRule::HasLineRange => input.start_line.is_some() && input.end_line.is_some(),
        }
    }

    pub fn disabled_reason(
        &self,
        editor: &TextEditorPlain,
        input: &TextActionInput,
    ) -> Option<&'static str> {
        if self.is_enabled(editor, input) {
            return None;
        }

        match self.enabled_rule {
            TextEnabledRule::Always => None,
            TextEnabledRule::DocumentHasText => Some("document is empty"),
            TextEnabledRule::DocumentOrInputHasText => Some("document and input text are empty"),
            TextEnabledRule::HasLineRange => Some("line range is missing"),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextActionInput {
    pub language: Option<String>,
    pub source: Option<String>,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    pub text: Option<String>,
    pub selection_export_policy: Option<SelectionExportPolicy>,
    pub strip_ansi_escape_codes: Option<bool>,
}

impl TextActionInput {
    pub fn empty() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TextActionOutput {
    None,
    Text(String),
    ClipboardPayload(ClipboardPayload),
    ClipboardTransform(ClipboardTransformResult),
    Disabled { action_id: String, reason: String },
}

pub fn all_text_actions() -> Vec<TextActionRecord> {
    vec![
        TextActionRecord {
            id: TextActionId::CopyPlain,
            action_id: TextActionId::CopyPlain.as_str(),
            label: "Copy Plain",
            short_label: "Copy",
            category: TextActionCategory::Clipboard,
            icon: "copy",
            tooltip: "Copy selected text, or the full document when nothing is selected.",
            enabled_rule: TextEnabledRule::DocumentHasText,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[
                TextHostPlacement::CommandPalette,
                TextHostPlacement::ClipboardMenu,
                TextHostPlacement::ContextMenu,
            ],
        },
        TextActionRecord {
            id: TextActionId::CopyMarkdownBlock,
            action_id: TextActionId::CopyMarkdownBlock.as_str(),
            label: "Copy Markdown Block",
            short_label: "Markdown",
            category: TextActionCategory::Clipboard,
            icon: "clipboard-copy",
            tooltip: "Copy text as a fenced Markdown block.",
            enabled_rule: TextEnabledRule::DocumentHasText,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[
                TextHostPlacement::CommandPalette,
                TextHostPlacement::ClipboardMenu,
                TextHostPlacement::Toolbar,
            ],
        },
        TextActionRecord {
            id: TextActionId::CopyPromptBlock,
            action_id: TextActionId::CopyPromptBlock.as_str(),
            label: "Copy Prompt Block",
            short_label: "Prompt",
            category: TextActionCategory::Clipboard,
            icon: "clipboard-copy",
            tooltip: "Copy text as a prompt-safe block with a source header.",
            enabled_rule: TextEnabledRule::DocumentHasText,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[
                TextHostPlacement::CommandPalette,
                TextHostPlacement::ClipboardMenu,
                TextHostPlacement::Toolbar,
            ],
        },
        TextActionRecord {
            id: TextActionId::CopyCodeFence,
            action_id: TextActionId::CopyCodeFence.as_str(),
            label: "Copy Code Fence",
            short_label: "Fence",
            category: TextActionCategory::Clipboard,
            icon: "braces",
            tooltip: "Copy text as a fenced code block with a language label.",
            enabled_rule: TextEnabledRule::DocumentHasText,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[
                TextHostPlacement::CommandPalette,
                TextHostPlacement::ClipboardMenu,
                TextHostPlacement::Toolbar,
            ],
        },
        TextActionRecord {
            id: TextActionId::SelectAll,
            action_id: TextActionId::SelectAll.as_str(),
            label: "Select All",
            short_label: "All",
            category: TextActionCategory::Selection,
            icon: "scan-text",
            tooltip: "Select the full document.",
            enabled_rule: TextEnabledRule::DocumentHasText,
            undo_behavior: TextUndoBehavior::SelectionOnly,
            host_placements: &[
                TextHostPlacement::CommandPalette,
                TextHostPlacement::EditMenu,
            ],
        },
        TextActionRecord {
            id: TextActionId::CurrentLineText,
            action_id: TextActionId::CurrentLineText.as_str(),
            label: "Current Line Text",
            short_label: "Line",
            category: TextActionCategory::Lines,
            icon: "text-cursor-input",
            tooltip: "Return the current line text without changing editor state.",
            enabled_rule: TextEnabledRule::Always,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[TextHostPlacement::CommandPalette],
        },
        TextActionRecord {
            id: TextActionId::LineRangeText,
            action_id: TextActionId::LineRangeText.as_str(),
            label: "Line Range Text",
            short_label: "Range",
            category: TextActionCategory::Lines,
            icon: "rows-3",
            tooltip: "Return an explicit inclusive range of lines.",
            enabled_rule: TextEnabledRule::HasLineRange,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[TextHostPlacement::CommandPalette],
        },
        TextActionRecord {
            id: TextActionId::TrimTrailingWhitespace,
            action_id: TextActionId::TrimTrailingWhitespace.as_str(),
            label: "Trim Trailing Whitespace",
            short_label: "Trim",
            category: TextActionCategory::Cleanup,
            icon: "eraser",
            tooltip: "Return text with line-end spaces and tabs removed.",
            enabled_rule: TextEnabledRule::DocumentHasText,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[
                TextHostPlacement::CommandPalette,
                TextHostPlacement::CleanupMenu,
            ],
        },
        TextActionRecord {
            id: TextActionId::CleanBasic,
            action_id: TextActionId::CleanBasic.as_str(),
            label: "Clean Basic",
            short_label: "Clean",
            category: TextActionCategory::Cleanup,
            icon: "wand-sparkles",
            tooltip: "Return cleaned text with a transform receipt.",
            enabled_rule: TextEnabledRule::DocumentOrInputHasText,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[
                TextHostPlacement::CommandPalette,
                TextHostPlacement::CleanupMenu,
            ],
        },
        TextActionRecord {
            id: TextActionId::NormalizeLineEndings,
            action_id: TextActionId::NormalizeLineEndings.as_str(),
            label: "Normalize Line Endings",
            short_label: "LF",
            category: TextActionCategory::Cleanup,
            icon: "pilcrow",
            tooltip: "Return text with CRLF/CR line endings normalized to LF.",
            enabled_rule: TextEnabledRule::DocumentOrInputHasText,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[
                TextHostPlacement::CommandPalette,
                TextHostPlacement::CleanupMenu,
            ],
        },
        TextActionRecord {
            id: TextActionId::StripAnsiEscapeCodes,
            action_id: TextActionId::StripAnsiEscapeCodes.as_str(),
            label: "Strip ANSI Escape Codes",
            short_label: "ANSI",
            category: TextActionCategory::Cleanup,
            icon: "eraser",
            tooltip: "Return text with ANSI escape sequences removed.",
            enabled_rule: TextEnabledRule::DocumentOrInputHasText,
            undo_behavior: TextUndoBehavior::None,
            host_placements: &[
                TextHostPlacement::CommandPalette,
                TextHostPlacement::CleanupMenu,
            ],
        },
    ]
}

pub fn action_record(action_id: TextActionId) -> TextActionRecord {
    all_text_actions()
        .into_iter()
        .find(|record| record.id == action_id)
        .expect("all known actions should have records")
}

pub fn parse_text_action_id(value: &str) -> Option<TextActionId> {
    TextActionId::parse(value)
}

pub fn execute_text_action(
    editor: &mut TextEditorPlain,
    action_id: TextActionId,
    input: TextActionInput,
) -> TextActionOutput {
    let record = action_record(action_id);
    if let Some(reason) = record.disabled_reason(editor, &input) {
        return TextActionOutput::Disabled {
            action_id: record.action_id.to_owned(),
            reason: reason.to_owned(),
        };
    }

    match action_id {
        TextActionId::CopyPlain => TextActionOutput::ClipboardPayload(
            copy_exact_payload_with_policy(editor, selection_export_policy(&input)),
        ),
        TextActionId::CopyMarkdownBlock => {
            TextActionOutput::Text(editor.copy_markdown_block(input.language.as_deref()))
        }
        TextActionId::CopyPromptBlock => {
            TextActionOutput::Text(editor.copy_prompt_block(input.source.as_deref()))
        }
        TextActionId::CopyCodeFence => {
            TextActionOutput::ClipboardTransform(copy_code_fence(editor, input.language.as_deref()))
        }
        TextActionId::SelectAll => {
            editor.apply(EditorCommand::SelectAll);
            TextActionOutput::None
        }
        TextActionId::CurrentLineText => TextActionOutput::Text(editor.current_line_text()),
        TextActionId::LineRangeText => {
            let start = input.start_line.expect("enabled line range has start_line");
            let end = input.end_line.expect("enabled line range has end_line");
            TextActionOutput::Text(editor.line_range_text(start, end))
        }
        TextActionId::TrimTrailingWhitespace => {
            let source = input
                .text
                .as_deref()
                .map(str::to_owned)
                .unwrap_or_else(|| editor.selected_text_or_all());
            TextActionOutput::Text(trim_trailing_whitespace_text(&source))
        }
        TextActionId::CleanBasic => {
            let source = input_text_or_selected_text(editor, &input);
            TextActionOutput::ClipboardTransform(clean_basic(
                &source,
                CleanBasicPolicy {
                    strip_ansi_escape_codes: input.strip_ansi_escape_codes.unwrap_or(false),
                    ..CleanBasicPolicy::default()
                },
            ))
        }
        TextActionId::NormalizeLineEndings => {
            let source = input_text_or_selected_text(editor, &input);
            TextActionOutput::ClipboardTransform(normalize_line_endings_with_report(&source))
        }
        TextActionId::StripAnsiEscapeCodes => {
            let source = input_text_or_selected_text(editor, &input);
            TextActionOutput::ClipboardTransform(strip_ansi_escape_codes_with_report(&source))
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
    include_str!("../fixtures/sample_actions.json")
}

fn selection_export_policy(input: &TextActionInput) -> SelectionExportPolicy {
    input
        .selection_export_policy
        .unwrap_or(SelectionExportPolicy::SelectedOrFullDocument)
}

fn input_text_or_selected_text(editor: &TextEditorPlain, input: &TextActionInput) -> String {
    input
        .text
        .as_deref()
        .map(str::to_owned)
        .unwrap_or_else(|| editor.selected_text_or_all())
}
