use feature_core::parse_feature_manifest;
use session_notes::{FEATURE_ID, SessionNotesState, sample_fixture, sample_state};

#[test]
fn sample_fixture_is_valid_json() {
    let _: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");

    assert_eq!(state.entries.len(), 3);
    assert_eq!(state.next_id, 4);
    assert_eq!(state.latest_entry().map(|entry| entry.id), Some(3),);
}

#[test]
fn blank_adds_and_blank_updates_are_ignored() {
    let mut state = sample_state().expect("sample state should load");

    assert!(!state.add_entry("   ", "2026-04-27T10:00:00Z"));
    assert!(!state.update_entry(1, "   "));
    assert_eq!(state.entries.len(), 3);
    assert_eq!(
        state
            .entries
            .iter()
            .find(|entry| entry.id == 1)
            .unwrap()
            .text,
        "Lock the feature contract before wiring the demo shell."
    );
}

#[test]
fn query_filtering_is_deterministic() {
    let mut state = sample_state().expect("sample state should load");
    state.set_query("reviewer receipt");

    let visible = state.visible_entries();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, 3);

    state.set_query("2026-04-27t09:12");
    let visible = state.visible_entries();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, 2);
}

#[test]
fn pin_toggle_does_not_reorder_entries() {
    let mut state = sample_state().expect("sample state should load");

    assert!(state.toggle_pinned(2));
    let ids = state
        .entries
        .iter()
        .map(|entry| entry.id)
        .collect::<Vec<_>>();
    assert_eq!(ids, vec![1, 2, 3]);
    assert!(state.entries[1].pinned);
}

#[test]
fn delete_clamps_selection() {
    let mut state = sample_state().expect("sample state should load");
    state.set_selected_index(99);
    assert_eq!(state.selected_entry().map(|entry| entry.id), Some(3));

    assert!(state.delete_entry(3));
    assert_eq!(state.selected_entry().map(|entry| entry.id), Some(2));

    assert!(state.delete_entry(2));
    assert_eq!(state.selected_entry().map(|entry| entry.id), Some(1));
}

#[test]
fn latest_entry_follows_insertion_order() {
    let mut state = sample_state().expect("sample state should load");

    assert!(state.add_entry("Newest note", "2026-04-27T10:00:00Z"));
    assert_eq!(state.latest_entry().map(|entry| entry.id), Some(4));
    assert_eq!(
        state.latest_entry().map(|entry| entry.text.as_str()),
        Some("Newest note")
    );
}

#[test]
fn counts_reflect_total_pinned_and_visible_entries() {
    let mut state = sample_state().expect("sample state should load");

    let counts = state.counts();
    assert_eq!(counts.total, 3);
    assert_eq!(counts.pinned, 1);
    assert_eq!(counts.visible, 3);

    state.set_query("timer");
    let counts = state.counts();
    assert_eq!(counts.total, 3);
    assert_eq!(counts.pinned, 1);
    assert_eq!(counts.visible, 1);
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "session_notes");
    assert_eq!(manifest.inputs.items.len(), 4);
    assert_eq!(manifest.outputs.items.len(), 4);
}

#[test]
fn empty_state_is_supported() {
    let state = SessionNotesState::new(Vec::new());

    assert!(state.visible_entries().is_empty());
    assert!(state.selected_entry().is_none());
    assert_eq!(state.counts().total, 0);
}
