use command_palette::{FEATURE_ID, sample_palette};
use feature_core::parse_feature_manifest;

#[test]
fn empty_query_returns_enabled_and_disabled_commands() {
    let palette = sample_palette().expect("sample palette should load");
    let visible = palette.visible_commands();

    assert_eq!(visible.len(), 5);
    assert!(visible.iter().any(|command| command.enabled));
    assert!(visible.iter().any(|command| !command.enabled));
}

#[test]
fn query_filters_by_title() {
    let mut palette = sample_palette().expect("sample palette should load");
    palette.set_query("settings");

    let visible = palette.visible_commands();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, "open_settings");
}

#[test]
fn query_filters_by_keyword() {
    let mut palette = sample_palette().expect("sample palette should load");
    palette.set_query("telemetry");

    let visible = palette.visible_commands();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, "open_activity_stream");
}

#[test]
fn disabled_command_cannot_activate() {
    let mut palette = sample_palette().expect("sample palette should load");
    palette.set_query("snapshot");

    assert_eq!(
        palette
            .selected_command()
            .map(|command| command.id.as_str()),
        Some("export_snapshot")
    );
    assert_eq!(palette.activate_selected(), None);
}

#[test]
fn selected_index_clamps_correctly() {
    let mut palette = sample_palette().expect("sample palette should load");
    palette.set_selected_index(99);
    assert_eq!(palette.selected_index(), 4);

    palette.move_down();
    assert_eq!(palette.selected_index(), 4);

    for _ in 0..10 {
        palette.move_up();
    }
    assert_eq!(palette.selected_index(), 0);

    palette.set_query("settings");
    assert_eq!(palette.selected_index(), 0);
}

#[test]
fn grouped_results_preserve_category_grouping() {
    let palette = sample_palette().expect("sample palette should load");
    let groups = palette.grouped_results();

    assert_eq!(groups.len(), 4);
    assert_eq!(groups[0].category, "View");
    assert_eq!(groups[0].commands.len(), 2);
    assert_eq!(groups[0].commands[0].id, "toggle_right_inspector");
    assert_eq!(groups[0].commands[1].id, "open_activity_stream");
    assert_eq!(groups[1].category, "Workspace");
    assert_eq!(groups[2].category, "Review");
    assert_eq!(groups[3].category, "Artifacts");
}

#[test]
fn empty_result_state_works() {
    let mut palette = sample_palette().expect("sample palette should load");
    palette.set_query("no-such-command");

    assert!(palette.visible_commands().is_empty());
    assert!(palette.grouped_results().is_empty());
    assert_eq!(
        palette.empty_state_message(),
        Some("No commands match the current query.")
    );
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "command_palette");
    assert_eq!(manifest.inputs.items.len(), 3);
    assert_eq!(manifest.outputs.items.len(), 3);
}
