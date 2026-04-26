use activity_stream::{ActivityStream, FEATURE_ID, sample_stream};
use feature_core::parse_feature_manifest;

#[test]
fn empty_query_returns_all_entries_in_order() {
    let stream = sample_stream().expect("sample stream should load");
    let visible = stream.visible_entries();

    assert_eq!(visible.len(), 5);
    assert_eq!(visible[0].id, "activity.review_packet_completed");
    assert_eq!(visible[1].id, "activity.tests_failed");
    assert!(visible.iter().any(|entry| entry.actionable));
    assert!(visible.iter().any(|entry| !entry.actionable));
}

#[test]
fn query_filters_by_title() {
    let mut stream = sample_stream().expect("sample stream should load");
    stream.set_query("workspace tests");

    let visible = stream.visible_entries();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, "activity.tests_failed");
}

#[test]
fn query_filters_by_actor() {
    let mut stream = sample_stream().expect("sample stream should load");
    stream.set_query("render-queue");

    let visible = stream.visible_entries();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, "activity.snapshot_exported");
}

#[test]
fn selected_index_clamps_correctly() {
    let mut stream = sample_stream().expect("sample stream should load");
    stream.set_selected_index(99);
    assert_eq!(stream.selected_index(), 4);

    stream.move_down();
    assert_eq!(stream.selected_index(), 4);

    for _ in 0..10 {
        stream.move_up();
    }
    assert_eq!(stream.selected_index(), 0);

    stream.set_query("spec sheet");
    assert_eq!(stream.selected_index(), 0);
}

#[test]
fn non_actionable_entry_cannot_activate() {
    let mut stream = sample_stream().expect("sample stream should load");
    stream.set_query("snapshot");

    assert_eq!(
        stream.selected_entry().map(|entry| entry.id.as_str()),
        Some("activity.snapshot_exported")
    );
    assert_eq!(stream.activate_selected(), None);
}

#[test]
fn actionable_entry_activates() {
    let mut stream = sample_stream().expect("sample stream should load");
    stream.set_query("review packet");

    assert_eq!(
        stream.activate_selected(),
        Some("activity.review_packet_completed".into())
    );
}

#[test]
fn visible_unread_count_tracks_filtering() {
    let mut stream = sample_stream().expect("sample stream should load");
    assert_eq!(stream.visible_unread_count(), 3);

    stream.set_query("registry");
    assert_eq!(stream.visible_unread_count(), 1);
}

#[test]
fn empty_result_state_works() {
    let mut stream = sample_stream().expect("sample stream should load");
    stream.set_query("no-such-activity");

    assert!(stream.visible_entries().is_empty());
    assert_eq!(
        stream.empty_state_message(),
        Some("No activity entries match the current query.")
    );
    assert_eq!(stream.activate_selected(), None);
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "activity_stream");
    assert_eq!(manifest.inputs.items.len(), 3);
    assert_eq!(manifest.outputs.items.len(), 3);
}

#[test]
fn empty_stream_state_is_supported() {
    let stream = ActivityStream::new(Vec::new());

    assert!(stream.visible_entries().is_empty());
    assert_eq!(
        stream.empty_state_message(),
        Some("No activity entries match the current query.")
    );
}
