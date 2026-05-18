use document_history::{DocumentHistoryState, FEATURE_ID, sample_fixture, sample_state};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "document_history");
    assert_eq!(manifest.kind.to_string(), "logic_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(
        manifest.inputs.items,
        vec![
            "working_text",
            "revisions",
            "selected_revision_index",
            "history_actions"
        ]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "working_text",
            "selected_revision",
            "document_history_counts",
            "dirty_state"
        ]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["revisions"].as_array().map(Vec::len), Some(2));
}

#[test]
fn sample_state_loads_and_exposes_selected_revision() {
    let state = sample_state().expect("sample state should load");

    assert_eq!(state.revision_count(), 2);
    assert_eq!(state.next_id, 3);
    assert_eq!(
        state.selected_revision().map(|revision| revision.id),
        Some(2)
    );
    assert_eq!(state.latest_revision().map(|revision| revision.id), Some(2));
    assert!(state.is_dirty());
    assert_eq!(state.counts().revisions, 2);
    assert!(state.counts().dirty);
}

#[test]
fn blank_working_text_is_allowed_and_dirty_tracks_clean_baseline() {
    let mut state = DocumentHistoryState::new("");
    assert_eq!(state.working_text(), "");
    assert!(!state.is_dirty());

    state.set_working_text("   ");
    assert!(state.is_dirty());

    state.mark_clean();
    assert!(!state.is_dirty());
    assert_eq!(state.working_text(), "   ");
}

#[test]
fn selection_clamps_safely() {
    let mut state = sample_state().expect("sample state should load");

    state.set_selected_revision_index(99);
    assert_eq!(state.selected_revision_index, 1);
    assert_eq!(
        state.selected_revision().map(|revision| revision.id),
        Some(2)
    );

    let mut empty = DocumentHistoryState::new("hello");
    empty.set_selected_revision_index(10);
    assert_eq!(empty.selected_revision_index, 0);
    assert!(empty.selected_revision().is_none());
}

#[test]
fn unchanged_snapshot_is_no_op_and_snapshots_preserve_insertion_order() {
    let mut state = DocumentHistoryState::new("draft one");
    assert!(state.record_snapshot("Initial", "2026-04-27T10:00:00Z"));
    assert!(!state.record_snapshot("Duplicate", "2026-04-27T10:01:00Z"));

    state.set_working_text("draft two");
    assert!(state.record_snapshot("Second", "2026-04-27T10:02:00Z"));

    let ids = state
        .revisions
        .iter()
        .map(|revision| revision.id)
        .collect::<Vec<_>>();
    let labels = state
        .revisions
        .iter()
        .map(|revision| revision.label.as_str())
        .collect::<Vec<_>>();

    assert_eq!(ids, vec![1, 2]);
    assert_eq!(labels, vec!["Initial", "Second"]);
    assert_eq!(state.next_id, 3);
}

#[test]
fn restore_updates_working_text_but_keeps_history() {
    let mut state = sample_state().expect("sample state should load");
    let original_count = state.revision_count();

    state.set_selected_revision_index(0);
    assert!(state.restore_selected_revision());
    assert_eq!(
        state.working_text(),
        "Lock the contract.\nImplement the feature."
    );
    assert_eq!(state.revision_count(), original_count);
    assert_eq!(state.latest_revision().map(|revision| revision.id), Some(2));
}

#[test]
fn mark_clean_updates_dirty_baseline_after_restore_or_edit() {
    let mut state = sample_state().expect("sample state should load");
    assert!(state.is_dirty());

    state.mark_clean();
    assert!(!state.is_dirty());

    state.set_working_text("Lock the contract.\nImplement the feature.\nShip the crate.");
    assert!(state.is_dirty());

    state.mark_clean();
    assert!(!state.is_dirty());
}
