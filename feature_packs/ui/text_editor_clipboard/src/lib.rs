use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};
use text_editor_plain::{TextEditorPlain, trim_trailing_whitespace_text};

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
