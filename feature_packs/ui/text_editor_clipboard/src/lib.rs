use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};
use text_editor_plain::{EditorSelection, TextEditorPlain, trim_trailing_whitespace_text};

pub const FEATURE_ID: &str = "ui.text_editor_clipboard";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardChangeId {
    NormalizedLineEndings,
    TrimmedTrailingWhitespace,
    StrippedAnsiEscapeCodes,
}

impl ClipboardChangeId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NormalizedLineEndings => "normalized_line_endings",
            Self::TrimmedTrailingWhitespace => "trimmed_trailing_whitespace",
            Self::StrippedAnsiEscapeCodes => "stripped_ansi_escape_codes",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardWarningId {
    AnsiEscapeSequenceIncomplete,
}

impl ClipboardWarningId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AnsiEscapeSequenceIncomplete => "ansi_escape_sequence_incomplete",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardWarningSeverity {
    Info,
    Warn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionExportPolicy {
    SelectedOrFullDocument,
    SelectionOnly,
    FullDocument,
}

impl SelectionExportPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelectedOrFullDocument => "selected_or_full_document",
            Self::SelectionOnly => "selection_only",
            Self::FullDocument => "full_document",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardPayloadKind {
    ExactText,
    CleanedText,
}

impl ClipboardPayloadKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactText => "exact_text",
            Self::CleanedText => "cleaned_text",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardLineRange {
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardPayloadMetadata {
    pub payload_kind: ClipboardPayloadKind,
    pub export_policy: SelectionExportPolicy,
    pub used_selection: bool,
    pub fallback_to_full_document: bool,
    pub selection: Option<EditorSelection>,
    pub line_range: Option<ClipboardLineRange>,
    pub source_path: Option<String>,
    pub language: Option<String>,
    pub character_count: usize,
    pub line_count: usize,
    pub first_line_indent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardPayload {
    pub text: String,
    pub metadata: ClipboardPayloadMetadata,
    pub receipt: ClipboardTransformResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardChange {
    pub change_id: ClipboardChangeId,
    pub description: String,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardWarning {
    pub warning_id: ClipboardWarningId,
    pub message: String,
    pub severity: ClipboardWarningSeverity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardTransformResult {
    pub text: String,
    pub changes: Vec<ClipboardChange>,
    pub warnings: Vec<ClipboardWarning>,
}

impl ClipboardTransformResult {
    pub fn exact(text: String) -> Self {
        Self {
            text,
            changes: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn changed(text: String, changes: Vec<ClipboardChange>) -> Self {
        Self {
            text,
            changes,
            warnings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanBasicPolicy {
    pub normalize_line_endings: bool,
    pub trim_trailing_whitespace: bool,
    pub strip_ansi_escape_codes: bool,
}

impl Default for CleanBasicPolicy {
    fn default() -> Self {
        Self {
            normalize_line_endings: true,
            trim_trailing_whitespace: true,
            strip_ansi_escape_codes: false,
        }
    }
}

pub fn copy_exact(editor: &TextEditorPlain) -> ClipboardTransformResult {
    ClipboardTransformResult::exact(editor.selected_text_or_all())
}

pub fn copy_markdown_block(
    editor: &TextEditorPlain,
    language: Option<&str>,
) -> ClipboardTransformResult {
    ClipboardTransformResult::exact(editor.copy_markdown_block(language))
}

pub fn copy_prompt_block(
    editor: &TextEditorPlain,
    source: Option<&str>,
) -> ClipboardTransformResult {
    ClipboardTransformResult::exact(editor.copy_prompt_block(source))
}

pub fn copy_code_fence(
    editor: &TextEditorPlain,
    language: Option<&str>,
) -> ClipboardTransformResult {
    let language = language
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("text");
    ClipboardTransformResult::exact(fenced_block(language, &editor.selected_text_or_all()))
}

pub fn copy_exact_payload(editor: &TextEditorPlain) -> ClipboardPayload {
    copy_exact_payload_with_policy(editor, SelectionExportPolicy::SelectedOrFullDocument)
}

pub fn copy_exact_payload_with_policy(
    editor: &TextEditorPlain,
    export_policy: SelectionExportPolicy,
) -> ClipboardPayload {
    let export = export_text_for_policy(editor, export_policy);
    let receipt = ClipboardTransformResult::exact(export.text.clone());
    payload_from_receipt(export, ClipboardPayloadKind::ExactText, receipt, None, None)
}

pub fn copy_clean_payload_with_policy(
    editor: &TextEditorPlain,
    export_policy: SelectionExportPolicy,
    cleanup_policy: CleanBasicPolicy,
) -> ClipboardPayload {
    let export = export_text_for_policy(editor, export_policy);
    let receipt = clean_basic(&export.text, cleanup_policy);
    payload_from_receipt(
        export,
        ClipboardPayloadKind::CleanedText,
        receipt,
        None,
        None,
    )
}

pub fn clean_basic(input: &str, policy: CleanBasicPolicy) -> ClipboardTransformResult {
    let mut current = input.to_owned();
    let mut changes = Vec::new();
    let mut warnings = Vec::new();

    if policy.normalize_line_endings {
        let result = normalize_line_endings_with_report(&current);
        current = result.text;
        changes.extend(result.changes);
        warnings.extend(result.warnings);
    }

    if policy.strip_ansi_escape_codes {
        let result = strip_ansi_escape_codes_with_report(&current);
        current = result.text;
        changes.extend(result.changes);
        warnings.extend(result.warnings);
    }

    if policy.trim_trailing_whitespace {
        let result = trim_trailing_whitespace_with_report(&current);
        current = result.text;
        changes.extend(result.changes);
        warnings.extend(result.warnings);
    }

    ClipboardTransformResult {
        text: current,
        changes,
        warnings,
    }
}

pub fn normalize_line_endings_with_report(input: &str) -> ClipboardTransformResult {
    let crlf_count = input.matches("\r\n").count();
    let without_crlf = input.replace("\r\n", "\n");
    let cr_count = without_crlf.matches('\r').count();
    let output = without_crlf.replace('\r', "\n");
    let count = crlf_count + cr_count;

    ClipboardTransformResult::changed(
        output,
        change_if_count(
            ClipboardChangeId::NormalizedLineEndings,
            "Converted CRLF/CR line endings to LF.",
            count,
        ),
    )
}

pub fn trim_trailing_whitespace_with_report(input: &str) -> ClipboardTransformResult {
    let normalized = normalize_line_endings(input);
    let count = normalized
        .split('\n')
        .filter(|line| line.ends_with(' ') || line.ends_with('\t'))
        .count();
    let output = trim_trailing_whitespace_text(input);

    ClipboardTransformResult::changed(
        output,
        change_if_count(
            ClipboardChangeId::TrimmedTrailingWhitespace,
            "Removed trailing spaces and tabs at line ends.",
            count,
        ),
    )
}

pub fn strip_ansi_escape_codes_with_report(input: &str) -> ClipboardTransformResult {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut count = 0usize;
    let mut warnings = Vec::new();

    while let Some(character) = chars.next() {
        if character != '\u{1b}' {
            output.push(character);
            continue;
        }

        if chars.peek() != Some(&'[') {
            output.push(character);
            continue;
        }

        let _ = chars.next();
        let mut terminated = false;
        for next in chars.by_ref() {
            if ('@'..='~').contains(&next) {
                terminated = true;
                break;
            }
        }

        if terminated {
            count += 1;
        } else {
            warnings.push(ClipboardWarning {
                warning_id: ClipboardWarningId::AnsiEscapeSequenceIncomplete,
                message: "Encountered an incomplete ANSI escape sequence.".into(),
                severity: ClipboardWarningSeverity::Warn,
            });
        }
    }

    ClipboardTransformResult {
        text: output,
        changes: change_if_count(
            ClipboardChangeId::StrippedAnsiEscapeCodes,
            "Removed ANSI escape sequences.",
            count,
        ),
        warnings,
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_clipboard.json")
}

fn normalize_line_endings(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}

fn change_if_count(
    change_id: ClipboardChangeId,
    description: &str,
    count: usize,
) -> Vec<ClipboardChange> {
    if count == 0 {
        return Vec::new();
    }

    vec![ClipboardChange {
        change_id,
        description: description.into(),
        count,
    }]
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PolicyExport {
    text: String,
    export_policy: SelectionExportPolicy,
    used_selection: bool,
    fallback_to_full_document: bool,
    selection: Option<EditorSelection>,
    line_range: Option<ClipboardLineRange>,
}

fn export_text_for_policy(
    editor: &TextEditorPlain,
    export_policy: SelectionExportPolicy,
) -> PolicyExport {
    match export_policy {
        SelectionExportPolicy::SelectedOrFullDocument if editor.has_selection() => {
            selection_policy_export(editor, export_policy)
        }
        SelectionExportPolicy::SelectedOrFullDocument => {
            full_document_policy_export(editor, export_policy, true)
        }
        SelectionExportPolicy::SelectionOnly if editor.has_selection() => {
            selection_policy_export(editor, export_policy)
        }
        SelectionExportPolicy::SelectionOnly => PolicyExport {
            text: String::new(),
            export_policy,
            used_selection: false,
            fallback_to_full_document: false,
            selection: None,
            line_range: None,
        },
        SelectionExportPolicy::FullDocument => {
            full_document_policy_export(editor, export_policy, false)
        }
    }
}

fn selection_policy_export(
    editor: &TextEditorPlain,
    export_policy: SelectionExportPolicy,
) -> PolicyExport {
    let selection = editor.selection();
    PolicyExport {
        text: editor.selected_text().unwrap_or_default(),
        export_policy,
        used_selection: true,
        fallback_to_full_document: false,
        selection: Some(selection),
        line_range: Some(line_range_for_selection(selection)),
    }
}

fn full_document_policy_export(
    editor: &TextEditorPlain,
    export_policy: SelectionExportPolicy,
    fallback_to_full_document: bool,
) -> PolicyExport {
    PolicyExport {
        text: editor.text().to_owned(),
        export_policy,
        used_selection: false,
        fallback_to_full_document,
        selection: None,
        line_range: Some(ClipboardLineRange {
            start_line: 0,
            end_line: editor.line_count().saturating_sub(1),
        }),
    }
}

fn payload_from_receipt(
    export: PolicyExport,
    payload_kind: ClipboardPayloadKind,
    receipt: ClipboardTransformResult,
    source_path: Option<&str>,
    language: Option<&str>,
) -> ClipboardPayload {
    let text = receipt.text.clone();
    ClipboardPayload {
        metadata: ClipboardPayloadMetadata {
            payload_kind,
            export_policy: export.export_policy,
            used_selection: export.used_selection,
            fallback_to_full_document: export.fallback_to_full_document,
            selection: export.selection,
            line_range: export.line_range,
            source_path: trimmed_non_empty(source_path),
            language: trimmed_non_empty(language),
            character_count: text.chars().count(),
            line_count: text_line_count(&text),
            first_line_indent: first_line_indent(&text),
        },
        text,
        receipt,
    }
}

fn line_range_for_selection(selection: EditorSelection) -> ClipboardLineRange {
    let start = selection.anchor.min(selection.caret);
    let end = selection.anchor.max(selection.caret);
    ClipboardLineRange {
        start_line: start.line,
        end_line: end.line,
    }
}

fn text_line_count(text: &str) -> usize {
    text.split('\n').count()
}

fn first_line_indent(text: &str) -> Option<String> {
    let indent: String = text
        .lines()
        .next()
        .unwrap_or_default()
        .chars()
        .take_while(|character| *character == ' ' || *character == '\t')
        .collect();

    if indent.is_empty() {
        None
    } else {
        Some(indent)
    }
}

fn trimmed_non_empty(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
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
