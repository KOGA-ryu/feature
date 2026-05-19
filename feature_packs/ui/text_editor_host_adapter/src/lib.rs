use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};
use text_editor_actions::{
    TextActionCategory, TextActionId, TextActionInput, TextActionOutput, TextActionRecord,
    TextEnabledRule, TextHostPlacement, TextUndoBehavior, all_text_actions, execute_text_action,
};
use text_editor_clipboard::{
    ClipboardChange, ClipboardPayload, ClipboardPayloadMetadata, ClipboardTransformResult,
    ClipboardWarning, ClipboardWarningSeverity,
};
use text_editor_plain::TextEditorPlain;

pub const FEATURE_ID: &str = "ui.text_editor_host_adapter";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextHostProfile {
    LinuxDesktop,
    Terminal,
    Macos,
    Web,
    AiPromptBox,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostActionItem {
    pub action_id: String,
    pub label: String,
    pub short_label: String,
    pub icon: String,
    pub tooltip: String,
    pub category: TextActionCategory,
    pub placements: Vec<TextHostPlacement>,
    pub hotkey_label: Option<String>,
    pub enabled_rule: TextEnabledRule,
    pub enabled: bool,
    pub disabled_reason: Option<String>,
    pub undo_behavior: TextUndoBehavior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostActionResultKind {
    None,
    Text,
    ClipboardPayload,
    ClipboardTransform,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardReceiptSummary {
    pub change_count: usize,
    pub warning_count: usize,
    pub changes: Vec<String>,
    pub warnings: Vec<String>,
}

impl ClipboardReceiptSummary {
    pub fn from_transform(result: &ClipboardTransformResult) -> Self {
        Self {
            change_count: result.changes.iter().map(|change| change.count).sum(),
            warning_count: result.warnings.len(),
            changes: result.changes.iter().map(format_change).collect(),
            warnings: result.warnings.iter().map(format_warning).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostActionResult {
    pub action_id: String,
    pub kind: HostActionResultKind,
    pub display_text: String,
    pub clipboard_text: Option<String>,
    pub clipboard_payload: Option<ClipboardPayload>,
    pub payload_metadata: Option<ClipboardPayloadMetadata>,
    pub receipt_summary: Option<ClipboardReceiptSummary>,
    pub warnings: Vec<String>,
}

pub fn render_host_actions(
    editor: &TextEditorPlain,
    input: &TextActionInput,
    profile: TextHostProfile,
) -> Vec<HostActionItem> {
    all_text_actions()
        .iter()
        .map(|record| host_action_item(record, editor, input, profile))
        .collect()
}

pub fn render_host_actions_for_placement(
    editor: &TextEditorPlain,
    input: &TextActionInput,
    profile: TextHostProfile,
    placement: TextHostPlacement,
) -> Vec<HostActionItem> {
    render_host_actions(editor, input, profile)
        .into_iter()
        .filter(|item| item.placements.contains(&placement))
        .collect()
}

pub fn host_action_item(
    record: &TextActionRecord,
    editor: &TextEditorPlain,
    input: &TextActionInput,
    profile: TextHostProfile,
) -> HostActionItem {
    HostActionItem {
        action_id: record.action_id.to_owned(),
        label: record.label.to_owned(),
        short_label: record.short_label.to_owned(),
        icon: record.icon.to_owned(),
        tooltip: record.tooltip.to_owned(),
        category: record.category,
        placements: record.host_placements.to_vec(),
        hotkey_label: hotkey_label(record.id, profile).map(str::to_owned),
        enabled_rule: record.enabled_rule,
        enabled: record.is_enabled(editor, input),
        disabled_reason: record.disabled_reason(editor, input).map(str::to_owned),
        undo_behavior: record.undo_behavior,
    }
}

pub fn execute_host_action(
    editor: &mut TextEditorPlain,
    action_id: TextActionId,
    input: TextActionInput,
) -> HostActionResult {
    host_result_from_output(action_id, execute_text_action(editor, action_id, input))
}

pub fn host_result_from_output(
    action_id: TextActionId,
    output: TextActionOutput,
) -> HostActionResult {
    let action_id = action_id.as_str().to_owned();
    match output {
        TextActionOutput::None => HostActionResult {
            action_id,
            kind: HostActionResultKind::None,
            display_text: "Action completed.".to_owned(),
            clipboard_text: None,
            clipboard_payload: None,
            payload_metadata: None,
            receipt_summary: None,
            warnings: Vec::new(),
        },
        TextActionOutput::Text(text) => HostActionResult {
            action_id,
            kind: HostActionResultKind::Text,
            display_text: "Text output ready.".to_owned(),
            clipboard_text: Some(text),
            clipboard_payload: None,
            payload_metadata: None,
            receipt_summary: None,
            warnings: Vec::new(),
        },
        TextActionOutput::ClipboardPayload(payload) => {
            let receipt_summary = ClipboardReceiptSummary::from_transform(&payload.receipt);
            let warnings = receipt_summary.warnings.clone();
            HostActionResult {
                action_id,
                kind: HostActionResultKind::ClipboardPayload,
                display_text: "Clipboard payload ready.".to_owned(),
                clipboard_text: Some(payload.text.clone()),
                payload_metadata: Some(payload.metadata.clone()),
                receipt_summary: Some(receipt_summary),
                clipboard_payload: Some(payload),
                warnings,
            }
        }
        TextActionOutput::ClipboardTransform(result) => {
            let receipt_summary = ClipboardReceiptSummary::from_transform(&result);
            let display_text = transform_display_text(&receipt_summary);
            let warnings = receipt_summary.warnings.clone();
            HostActionResult {
                action_id,
                kind: HostActionResultKind::ClipboardTransform,
                display_text,
                clipboard_text: Some(result.text),
                clipboard_payload: None,
                payload_metadata: None,
                receipt_summary: Some(receipt_summary),
                warnings,
            }
        }
        TextActionOutput::Disabled { action_id, reason } => HostActionResult {
            action_id,
            kind: HostActionResultKind::Disabled,
            display_text: format!("Action disabled: {reason}"),
            clipboard_text: None,
            clipboard_payload: None,
            payload_metadata: None,
            receipt_summary: None,
            warnings: vec![reason],
        },
    }
}

pub fn hotkey_label(action_id: TextActionId, profile: TextHostProfile) -> Option<&'static str> {
    match profile {
        TextHostProfile::LinuxDesktop | TextHostProfile::Web | TextHostProfile::AiPromptBox => {
            match action_id {
                TextActionId::CopyPlain => Some("Ctrl+C"),
                TextActionId::SelectAll => Some("Ctrl+A"),
                _ => None,
            }
        }
        TextHostProfile::Terminal => match action_id {
            TextActionId::CopyPlain => Some("Ctrl+Shift+C"),
            _ => None,
        },
        TextHostProfile::Macos => match action_id {
            TextActionId::CopyPlain => Some("Cmd+C"),
            TextActionId::SelectAll => Some("Cmd+A"),
            _ => None,
        },
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_host_actions.json")
}

fn transform_display_text(summary: &ClipboardReceiptSummary) -> String {
    if summary.change_count == 0 && summary.warning_count == 0 {
        return "Clipboard text ready. No changes.".to_owned();
    }

    format!(
        "Clipboard text ready. {} changes, {} warnings.",
        summary.change_count, summary.warning_count
    )
}

fn format_change(change: &ClipboardChange) -> String {
    format!("{}: {}", change.change_id.as_str(), change.count)
}

fn format_warning(warning: &ClipboardWarning) -> String {
    format!(
        "{}: {}",
        warning.warning_id.as_str(),
        severity_label(warning.severity)
    )
}

fn severity_label(severity: ClipboardWarningSeverity) -> &'static str {
    match severity {
        ClipboardWarningSeverity::Info => "info",
        ClipboardWarningSeverity::Warn => "warn",
    }
}
