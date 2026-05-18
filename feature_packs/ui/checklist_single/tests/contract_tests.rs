use checklist_single::{FEATURE_ID, sample_fixture, sample_state};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "checklist_single");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(manifest.inputs.items.len(), 4);
    assert_eq!(manifest.outputs.items, vec!["items", "has_completed_items"]);
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["items"].as_array().map(Vec::len), Some(3));
}

#[test]
fn add_toggle_delete_and_clear_completed_follow_rules() {
    let mut state = sample_state().expect("sample state should load");
    assert!(state.add_item("  verify OBS scene  "));
    assert!(!state.add_item("   "));

    let original_titles: Vec<String> = state.items.iter().map(|item| item.title.clone()).collect();
    assert_eq!(original_titles[0], "Confirm capture preview");
    assert_eq!(state.items.last().unwrap().title, "verify OBS scene");

    let last_id = state.items.last().unwrap().id;
    state.toggle_item(last_id);
    assert!(state.has_completed_items());

    let preserved_order: Vec<usize> = state
        .items
        .iter()
        .map(|item| item.created_at_order)
        .collect();
    state.toggle_item(last_id);
    assert_eq!(
        preserved_order,
        state
            .items
            .iter()
            .map(|item| item.created_at_order)
            .collect::<Vec<_>>()
    );

    state.toggle_item(last_id);
    state.clear_completed();
    assert!(state.items.iter().all(|item| item.id != last_id));

    let delete_id = state.items[0].id;
    state.delete_item(delete_id);
    assert!(state.items.iter().all(|item| item.id != delete_id));
}

#[test]
fn sample_state_loads() {
    let state = sample_state().expect("sample state should load");
    assert_eq!(state.items.len(), 3);
    assert!(state.has_completed_items());
}
