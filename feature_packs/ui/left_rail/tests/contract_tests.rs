use feature_core::parse_feature_manifest;
use left_rail::{FEATURE_ID, LeftRailState, sample_rail};

#[test]
fn fixture_loads_and_preserves_items() {
    let rail = sample_rail().expect("sample rail should load");

    assert_eq!(rail.items().len(), 5);
    assert_eq!(rail.items()[0].id, "workspace_home");
    assert!(rail.items().iter().any(|item| !item.enabled));
}

#[test]
fn grouped_sections_preserve_order() {
    let rail = sample_rail().expect("sample rail should load");
    let sections = rail.grouped_sections();

    assert_eq!(sections.len(), 3);
    assert_eq!(sections[0].name, "Workspace");
    assert_eq!(sections[0].items[0].id, "workspace_home");
    assert_eq!(sections[0].items[1].id, "feature_library");
    assert_eq!(sections[1].name, "Build");
    assert_eq!(sections[2].name, "History");
}

#[test]
fn selected_index_clamps_correctly() {
    let mut rail = sample_rail().expect("sample rail should load");
    rail.set_selected_index(99);
    assert_eq!(rail.selected_index(), 4);

    rail.move_down();
    assert_eq!(rail.selected_index(), 4);

    for _ in 0..10 {
        rail.move_up();
    }
    assert_eq!(rail.selected_index(), 0);
}

#[test]
fn disabled_item_cannot_activate() {
    let mut rail = sample_rail().expect("sample rail should load");
    rail.set_selected_index(3);

    assert_eq!(
        rail.selected_item().map(|item| item.id.as_str()),
        Some("artifact_store")
    );
    assert_eq!(rail.activate_selected(), None);
}

#[test]
fn enabled_item_activates() {
    let mut rail = sample_rail().expect("sample rail should load");
    rail.set_selected_index(2);

    assert_eq!(rail.activate_selected(), Some("review_queue".into()));
}

#[test]
fn collapsed_state_toggles() {
    let mut rail = sample_rail().expect("sample rail should load");

    assert!(!rail.is_collapsed());
    rail.toggle_collapsed();
    assert!(rail.is_collapsed());
    rail.set_collapsed(false);
    assert!(!rail.is_collapsed());
}

#[test]
fn empty_state_works() {
    let rail = LeftRailState::new(Vec::new());

    assert_eq!(rail.items().len(), 0);
    assert_eq!(
        rail.empty_state_message(),
        Some("No navigation items are available.")
    );
    assert_eq!(rail.activate_selected(), None);
}

#[test]
fn badge_counts_accumulate() {
    let rail = sample_rail().expect("sample rail should load");

    assert_eq!(rail.total_badge_count(), 13);
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "left_rail");
    assert_eq!(manifest.inputs.items.len(), 3);
    assert_eq!(manifest.outputs.items.len(), 3);
}
