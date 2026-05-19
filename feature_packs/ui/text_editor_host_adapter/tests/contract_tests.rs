use feature_core::parse_feature_manifest;
use text_editor_actions::{
    TextActionId, TextActionInput, TextActionOutput, TextHostPlacement, execute_text_action,
};
use text_editor_clipboard::{
    ClipboardChange, ClipboardChangeId, ClipboardPayloadKind, ClipboardTransformResult,
    ClipboardWarning, ClipboardWarningId, ClipboardWarningSeverity, SelectionExportPolicy,
};
use text_editor_host_adapter::{
    FEATURE_ID, HostActionResultKind, TextHostProfile, execute_host_action,
    host_result_from_output, hotkey_label, render_host_actions, render_host_actions_for_placement,
    sample_fixture,
};
use text_editor_plain::TextEditorPlain;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "text_editor_host_adapter");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec![
            "editor_state",
            "text_action_records",
            "text_action_input",
            "host_profile"
        ]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "host_action_items",
            "host_action_result",
            "clipboard_payload_metadata",
            "clipboard_receipt_summary"
        ]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(value["profiles"][0], "linux_desktop");
    assert_eq!(value["outputs"][0], "host_action_items");
}

#[test]
fn render_host_actions_preserves_action_metadata_and_enabled_state() {
    let editor = TextEditorPlain::from_text("hello".into());
    let items = render_host_actions(
        &editor,
        &TextActionInput::empty(),
        TextHostProfile::LinuxDesktop,
    );
    let copy = items
        .iter()
        .find(|item| item.action_id == "text.copy_plain")
        .expect("copy action should render");

    assert_eq!(copy.label, "Copy Plain");
    assert_eq!(copy.icon, "copy");
    assert_eq!(copy.hotkey_label.as_deref(), Some("Ctrl+C"));
    assert!(copy.enabled);
    assert_eq!(copy.disabled_reason, None);
    assert!(copy.placements.contains(&TextHostPlacement::ClipboardMenu));
}

#[test]
fn disabled_actions_keep_visible_reason_for_hosts() {
    let editor = TextEditorPlain::new();
    let items = render_host_actions(
        &editor,
        &TextActionInput::empty(),
        TextHostProfile::LinuxDesktop,
    );
    let clean = items
        .iter()
        .find(|item| item.action_id == "text.clean_basic")
        .expect("clean action should render");

    assert!(!clean.enabled);
    assert_eq!(
        clean.disabled_reason.as_deref(),
        Some("document and input text are empty")
    );
}

#[test]
fn explicit_input_text_enables_cleanup_actions_without_document_text() {
    let editor = TextEditorPlain::new();
    let items = render_host_actions(
        &editor,
        &TextActionInput {
            text: Some("pasted text".into()),
            ..TextActionInput::empty()
        },
        TextHostProfile::LinuxDesktop,
    );
    let clean = items
        .iter()
        .find(|item| item.action_id == "text.clean_basic")
        .expect("clean action should render");

    assert!(clean.enabled);
    assert_eq!(clean.disabled_reason, None);
}

#[test]
fn placement_filter_uses_action_registry_placements() {
    let editor = TextEditorPlain::from_text("hello".into());
    let toolbar = render_host_actions_for_placement(
        &editor,
        &TextActionInput::empty(),
        TextHostProfile::LinuxDesktop,
        TextHostPlacement::Toolbar,
    );
    let action_ids: Vec<&str> = toolbar.iter().map(|item| item.action_id.as_str()).collect();

    assert_eq!(
        action_ids,
        vec![
            "text.copy_markdown_block",
            "text.copy_prompt_block",
            "text.copy_code_fence"
        ]
    );
}

#[test]
fn hotkey_profiles_stay_conservative() {
    assert_eq!(
        hotkey_label(TextActionId::CopyPlain, TextHostProfile::LinuxDesktop),
        Some("Ctrl+C")
    );
    assert_eq!(
        hotkey_label(TextActionId::CopyPlain, TextHostProfile::Terminal),
        Some("Ctrl+Shift+C")
    );
    assert_eq!(
        hotkey_label(TextActionId::SelectAll, TextHostProfile::Macos),
        Some("Cmd+A")
    );
    assert_eq!(
        hotkey_label(TextActionId::CleanBasic, TextHostProfile::LinuxDesktop),
        None
    );
}

#[test]
fn text_outputs_become_host_clipboard_payloads_without_side_effects() {
    let mut editor = TextEditorPlain::from_text("hello".into());
    let before = editor.text().to_owned();
    let result = execute_host_action(
        &mut editor,
        TextActionId::CopyPlain,
        TextActionInput::empty(),
    );

    assert_eq!(result.kind, HostActionResultKind::ClipboardPayload);
    assert_eq!(result.clipboard_text.as_deref(), Some("hello"));
    let metadata = result
        .payload_metadata
        .expect("payload metadata should exist");
    assert_eq!(metadata.payload_kind, ClipboardPayloadKind::ExactText);
    assert_eq!(
        metadata.export_policy,
        SelectionExportPolicy::SelectedOrFullDocument
    );
    assert!(metadata.fallback_to_full_document);
    assert_eq!(
        result
            .receipt_summary
            .expect("receipt should exist")
            .change_count,
        0
    );
    assert_eq!(editor.text(), before);
}

#[test]
fn host_result_exposes_selection_export_policy_metadata_without_owning_clipboard() {
    let mut editor = TextEditorPlain::from_text("alpha\nbeta".into());
    let result = execute_host_action(
        &mut editor,
        TextActionId::CopyPlain,
        TextActionInput {
            selection_export_policy: Some(SelectionExportPolicy::FullDocument),
            ..TextActionInput::empty()
        },
    );

    assert_eq!(result.kind, HostActionResultKind::ClipboardPayload);
    assert_eq!(result.clipboard_text.as_deref(), Some("alpha\nbeta"));
    let metadata = result
        .payload_metadata
        .expect("payload metadata should exist");
    assert_eq!(metadata.export_policy, SelectionExportPolicy::FullDocument);
    assert!(!metadata.used_selection);
    assert!(!metadata.fallback_to_full_document);
    assert_eq!(
        result.clipboard_payload.expect("payload should exist").text,
        "alpha\nbeta"
    );
}

#[test]
fn clipboard_transform_outputs_get_receipt_summaries() {
    let result = host_result_from_output(
        TextActionId::CleanBasic,
        TextActionOutput::ClipboardTransform(ClipboardTransformResult {
            text: "clean".into(),
            changes: vec![
                ClipboardChange {
                    change_id: ClipboardChangeId::NormalizedLineEndings,
                    description: "Converted CRLF/CR line endings to LF.".into(),
                    count: 2,
                },
                ClipboardChange {
                    change_id: ClipboardChangeId::TrimmedTrailingWhitespace,
                    description: "Removed trailing spaces and tabs at line ends.".into(),
                    count: 1,
                },
            ],
            warnings: vec![ClipboardWarning {
                warning_id: ClipboardWarningId::AnsiEscapeSequenceIncomplete,
                message: "Incomplete ANSI escape sequence left unchanged.".into(),
                severity: ClipboardWarningSeverity::Warn,
            }],
        }),
    );

    assert_eq!(result.kind, HostActionResultKind::ClipboardTransform);
    assert_eq!(result.clipboard_text.as_deref(), Some("clean"));
    assert_eq!(
        result.display_text,
        "Clipboard text ready. 3 changes, 1 warnings."
    );
    let summary = result.receipt_summary.expect("summary should exist");
    assert_eq!(summary.change_count, 3);
    assert_eq!(summary.warning_count, 1);
    assert_eq!(
        summary.changes,
        vec![
            "normalized_line_endings: 2",
            "trimmed_trailing_whitespace: 1"
        ]
    );
    assert_eq!(
        summary.warnings,
        vec!["ansi_escape_sequence_incomplete: warn"]
    );
}

#[test]
fn disabled_outputs_become_host_disabled_results() {
    let mut editor = TextEditorPlain::new();
    let output = execute_text_action(
        &mut editor,
        TextActionId::CleanBasic,
        TextActionInput::empty(),
    );
    let result = host_result_from_output(TextActionId::CleanBasic, output);

    assert_eq!(result.kind, HostActionResultKind::Disabled);
    assert_eq!(result.clipboard_text, None);
    assert_eq!(
        result.display_text,
        "Action disabled: document and input text are empty"
    );
    assert_eq!(result.warnings, vec!["document and input text are empty"]);
}
