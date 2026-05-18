use feature_core::parse_feature_manifest;
use quick_capture_inbox::{
    FEATURE_ID, InboxItem, InboxItemStatus, QuickCaptureInboxState, sample_fixture, sample_state,
};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "quick_capture_inbox");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec![
            "items",
            "draft_text",
            "query",
            "selection",
            "capture_actions"
        ]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "visible_items",
            "selected_item",
            "inbox_counts",
            "draft_text"
        ]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["items"].as_array().map(Vec::len), Some(3));
    assert_eq!(fixture["next_id"].as_u64(), Some(4));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.items.len(), 3);
    assert_eq!(state.selected_index, 1);
    assert_eq!(state.next_id, 4);
    assert_eq!(state.selected_item().map(|item| item.id), Some(2));
}

#[test]
fn blank_captures_are_ignored_and_draft_capture_clears_on_success() {
    let mut state = QuickCaptureInboxState::new(Vec::new());
    state.set_draft_text("   ");
    assert!(!state.capture_draft("2026-04-27T10:00:00Z"));
    assert_eq!(state.draft_text, "   ");
    assert_eq!(state.items.len(), 0);

    state.set_draft_text("  Capture this  ");
    assert!(state.capture_draft("2026-04-27T10:00:00Z"));
    assert_eq!(state.draft_text, "");
    assert_eq!(state.items.len(), 1);
    assert_eq!(state.items[0].text, "Capture this");
    assert_eq!(state.items[0].status, InboxItemStatus::Pending);
}

#[test]
fn new_captures_append_without_reordering_status_changes() {
    let mut state = sample_state().expect("sample state should load");
    let original_ids = state.items.iter().map(|item| item.id).collect::<Vec<_>>();
    assert_eq!(original_ids, vec![1, 2, 3]);

    assert!(state.capture_text("Newest inbox item", "2026-04-27T10:30:00Z"));
    let ids = state.items.iter().map(|item| item.id).collect::<Vec<_>>();
    assert_eq!(ids, vec![1, 2, 3, 4]);

    state.set_selected_index(0);
    assert!(state.mark_selected_processed());
    let ids_after_status = state.items.iter().map(|item| item.id).collect::<Vec<_>>();
    assert_eq!(ids_after_status, vec![1, 2, 3, 4]);
    assert_eq!(state.items[0].status, InboxItemStatus::Processed);
}

#[test]
fn selection_clamps_and_navigation_is_safe() {
    let mut state = sample_state().expect("sample state should load");
    state.set_selected_index(99);
    assert_eq!(state.selected_item().map(|item| item.id), Some(3));

    state.move_down();
    assert_eq!(state.selected_item().map(|item| item.id), Some(3));

    state.move_up();
    state.move_up();
    state.move_up();
    assert_eq!(state.selected_item().map(|item| item.id), Some(1));

    let mut empty = QuickCaptureInboxState::new(Vec::new());
    empty.move_down();
    empty.move_up();
    assert_eq!(empty.selected_index, 0);
    assert!(empty.selected_item().is_none());
}

#[test]
fn query_matches_text_captured_at_and_status() {
    let mut state = sample_state().expect("sample state should load");

    state.set_query("reviewer receipts");
    let visible = state.visible_items();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, 2);

    state.set_query("2026-04-27t09:15");
    let visible = state.visible_items();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, 3);

    state.set_query("archived");
    let visible = state.visible_items();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, 3);
}

#[test]
fn processed_and_archived_persist_until_delete() {
    let mut state = sample_state().expect("sample state should load");
    state.set_selected_index(0);
    assert!(state.mark_selected_processed());
    assert_eq!(state.items[0].status, InboxItemStatus::Processed);

    state.set_query("archive");
    assert!(state.mark_selected_archived());
    let archived = state.items.iter().find(|item| item.id == 3).unwrap();
    assert_eq!(archived.status, InboxItemStatus::Archived);
}

#[test]
fn delete_selected_clamps_selection() {
    let mut state = sample_state().expect("sample state should load");
    state.set_selected_index(99);
    assert_eq!(state.selected_item().map(|item| item.id), Some(3));

    assert!(state.delete_selected());
    assert_eq!(state.selected_item().map(|item| item.id), Some(2));
    assert_eq!(
        state.items.iter().map(|item| item.id).collect::<Vec<_>>(),
        vec![1, 2]
    );
}

#[test]
fn counts_reflect_total_pending_and_visible() {
    let mut state = sample_state().expect("sample state should load");

    let counts = state.counts();
    assert_eq!(counts.total, 3);
    assert_eq!(counts.pending, 1);
    assert_eq!(counts.visible, 3);

    state.set_query("processed");
    let counts = state.counts();
    assert_eq!(counts.total, 3);
    assert_eq!(counts.pending, 1);
    assert_eq!(counts.visible, 1);
}

#[test]
fn empty_state_is_supported() {
    let state = QuickCaptureInboxState::new(Vec::<InboxItem>::new());
    assert!(state.visible_items().is_empty());
    assert!(state.selected_item().is_none());
    let counts = state.counts();
    assert_eq!(counts.total, 0);
    assert_eq!(counts.pending, 0);
    assert_eq!(counts.visible, 0);
}
