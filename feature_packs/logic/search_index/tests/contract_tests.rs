use feature_core::parse_feature_manifest;
use search_index::{FEATURE_ID, SearchRequest, sample_index};

#[test]
fn empty_query_returns_all_fixture_items() {
    let index = sample_index().expect("fixture index should load");
    let results = index.search(&SearchRequest::new());

    assert_eq!(results.len(), 5);
    assert_eq!(results[0].id, "logic.validation_pipeline");
    assert_eq!(results[4].id, "workflow.review_packet_flow");
}

#[test]
fn title_search_works() {
    let index = sample_index().expect("fixture index should load");
    let results = index.search(&SearchRequest::new().with_query("PALETTE"));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "ui.command_palette");
}

#[test]
fn summary_search_works() {
    let index = sample_index().expect("fixture index should load");
    let results = index.search(&SearchRequest::new().with_query("review packets"));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "workflow.review_packet_flow");
}

#[test]
fn tag_search_works() {
    let index = sample_index().expect("fixture index should load");
    let results = index.search(&SearchRequest::new().with_query("timeline"));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "ui.activity_stream");
}

#[test]
fn kind_filter_works() {
    let index = sample_index().expect("fixture index should load");
    let results = index.search(&SearchRequest::new().with_kind("LOGIC_PATTERN"));

    let ids = results
        .iter()
        .map(|item| item.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["logic.validation_pipeline", "logic.search_index"]);
}

#[test]
fn tag_filter_works() {
    let index = sample_index().expect("fixture index should load");
    let results = index.search(&SearchRequest::new().with_tag("search"));

    let ids = results
        .iter()
        .map(|item| item.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["ui.command_palette", "logic.search_index"]);
}

#[test]
fn no_match_query_returns_empty_result() {
    let index = sample_index().expect("fixture index should load");
    let results = index.search(&SearchRequest::new().with_query("definitely-missing"));

    assert!(results.is_empty());
}

#[test]
fn result_ordering_is_stable() {
    let index = sample_index().expect("fixture index should load");
    let results = index.search(&SearchRequest::new().with_query("search"));

    let ids = results
        .iter()
        .map(|item| item.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, vec!["ui.command_palette", "logic.search_index"]);
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "search_index");
    assert_eq!(manifest.inputs.items.len(), 3);
    assert_eq!(manifest.outputs.items.len(), 3);
}
