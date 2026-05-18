use feature_core::{FeatureKind, FeatureStatus, parse_feature_manifest};
use feature_registry::FeatureRegistry;
use wiki_browser::{FEATURE_ID, WikiBrowserEntry, WikiBrowserState, sample_browser};

#[test]
fn fixture_parses_into_browser_state() {
    let browser = sample_browser().expect("sample browser should load");

    assert_eq!(browser.entries().len(), 5);
    assert_eq!(browser.entries()[0].manifest.id, "ui.command_palette");
    assert_eq!(browser.entries()[0].package_name, "command_palette");
}

#[test]
fn empty_query_returns_all_fixture_entries() {
    let browser = sample_browser().expect("sample browser should load");
    let ids = browser
        .visible_entries()
        .into_iter()
        .map(|entry| entry.manifest.id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        ids,
        vec![
            "ui.command_palette",
            "logic.search_index",
            "workflow.review_packet_flow",
            "ui.theme_editor",
            "ui.right_inspector",
        ]
    );
}

#[test]
fn query_matches_id_name_summary_and_tags() {
    let mut browser = sample_browser().expect("sample browser should load");
    browser.set_query("workflow.review_packet_flow");
    assert_eq!(
        browser
            .selected_manifest()
            .map(|manifest| manifest.id.as_str()),
        Some("workflow.review_packet_flow")
    );

    browser.set_query("theme_editor");
    assert_eq!(
        browser
            .selected_manifest()
            .map(|manifest| manifest.id.as_str()),
        Some("ui.theme_editor")
    );

    browser.set_query("detail panel");
    assert_eq!(
        browser
            .selected_manifest()
            .map(|manifest| manifest.id.as_str()),
        Some("ui.right_inspector")
    );

    browser.set_query("search");
    let ids = browser
        .visible_entries()
        .into_iter()
        .map(|entry| entry.manifest.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["ui.command_palette", "logic.search_index"]);
}

#[test]
fn kind_filter_works() {
    let mut browser = sample_browser().expect("sample browser should load");
    browser.set_kind_filter(Some(FeatureKind::UiPattern));

    let ids = browser
        .visible_entries()
        .into_iter()
        .map(|entry| entry.manifest.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "ui.command_palette",
            "ui.theme_editor",
            "ui.right_inspector"
        ]
    );
}

#[test]
fn status_filter_works() {
    let mut browser = sample_browser().expect("sample browser should load");
    browser.set_status_filter(Some(FeatureStatus::Stable));

    let ids = browser
        .visible_entries()
        .into_iter()
        .map(|entry| entry.manifest.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["ui.right_inspector"]);
}

#[test]
fn tag_filter_works() {
    let mut browser = sample_browser().expect("sample browser should load");
    browser.set_tag_filter(Some("search".into()));

    let ids = browser
        .visible_entries()
        .into_iter()
        .map(|entry| entry.manifest.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["ui.command_palette", "logic.search_index"]);
}

#[test]
fn selection_clamps_when_visible_set_shrinks() {
    let mut browser = sample_browser().expect("sample browser should load");
    browser.set_selected_index(4);
    assert_eq!(browser.selected_index(), 4);

    browser.set_status_filter(Some(FeatureStatus::Tested));
    assert_eq!(browser.selected_index(), 0);
    assert_eq!(
        browser
            .selected_manifest()
            .map(|manifest| manifest.id.as_str()),
        Some("workflow.review_packet_flow")
    );
}

#[test]
fn selection_reconciles_when_active_feature_falls_out_of_view() {
    let mut browser = sample_browser().expect("sample browser should load");
    assert!(browser.select_feature("logic.search_index"));
    assert_eq!(browser.selected_index(), 1);

    browser.set_kind_filter(Some(FeatureKind::UiPattern));

    assert_eq!(browser.selected_index(), 1);
    assert_eq!(browser.selected_feature_id(), Some("ui.theme_editor"));
}

#[test]
fn selection_preserves_active_feature_when_it_remains_visible() {
    let mut browser = sample_browser().expect("sample browser should load");
    assert!(browser.select_feature("ui.right_inspector"));

    browser.set_kind_filter(Some(FeatureKind::UiPattern));

    assert_eq!(browser.selected_feature_id(), Some("ui.right_inspector"));
}

#[test]
fn selected_preview_accessors_follow_selected_entry() {
    let mut browser = sample_browser().expect("sample browser should load");
    browser.set_query("theme_editor");

    assert_eq!(
        browser.selected_feature_dir(),
        Some("/Users/kogaryu/dev/features/feature_packs/ui/theme_editor")
    );
    assert_eq!(
        browser.selected_readme_path(),
        Some("/Users/kogaryu/dev/features/feature_packs/ui/theme_editor/README.md")
    );
    assert_eq!(
        browser.selected_fixtures_dir(),
        Some("/Users/kogaryu/dev/features/feature_packs/ui/theme_editor/fixtures")
    );
    assert!(
        browser
            .selected_readme_text()
            .expect("selected readme should exist")
            .contains("Theme editor preview state")
    );
    assert!(
        browser
            .selected_fixture_preview()
            .expect("selected fixture preview should exist")
            .contains("codex_dark")
    );
    assert_eq!(browser.selected_feature_id(), Some("ui.theme_editor"));
}

#[test]
fn selection_by_feature_id_uses_visible_browser_entries() {
    let mut browser = sample_browser().expect("sample browser should load");
    browser.set_query("search");

    assert!(browser.select_feature("logic.search_index"));
    assert_eq!(browser.selected_feature_id(), Some("logic.search_index"));
    assert_eq!(
        browser
            .selected_manifest()
            .map(|manifest| manifest.name.as_str()),
        Some("search_index")
    );

    assert!(!browser.select_feature("ui.right_inspector"));
    assert_eq!(browser.selected_feature_id(), Some("logic.search_index"));
}

#[test]
fn empty_state_is_explicit() {
    let mut browser = sample_browser().expect("sample browser should load");
    browser.set_query("definitely-missing");

    assert!(browser.visible_entries().is_empty());
    assert_eq!(
        browser.empty_state_message(),
        Some("No features match the current browser state.")
    );
    assert_eq!(browser.selected_manifest(), None);
}

#[test]
fn registry_adapter_converts_known_feature() {
    let registry = FeatureRegistry::discover().expect("registry should load");
    let feature = registry
        .get("ui.command_palette")
        .expect("command palette should exist");
    let entry = WikiBrowserEntry::from_registered_feature(feature)
        .expect("registered feature should adapt");

    assert_eq!(entry.manifest.id, "ui.command_palette");
    assert_eq!(entry.package_name, "command_palette");
    assert!(entry.readme_text.contains("reusable command palette"));
    assert!(entry.fixture_preview.is_some());
}

#[test]
fn registry_can_build_browser_state() {
    let registry = FeatureRegistry::discover().expect("registry should load");
    let browser = WikiBrowserState::from_registry(&registry).expect("browser should build");

    assert!(browser.entries().len() >= 5);
    assert!(
        browser
            .entries()
            .iter()
            .any(|entry| entry.manifest.id == "ui.right_inspector")
    );
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "wiki_browser");
    assert_eq!(manifest.inputs.items.len(), 4);
    assert_eq!(manifest.outputs.items.len(), 3);
}
