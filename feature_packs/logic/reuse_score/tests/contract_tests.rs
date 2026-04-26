use feature_core::parse_feature_manifest;
use reuse_score::{
    FEATURE_ID, FeatureLifecycleStatus, ReuseScoreBand, ReuseScoreInput, ReuseScoreSeverity,
    ReuseScorer, sample_deprecated_inputs, sample_invalid_inputs, sample_mixed_report, score_reuse,
};

#[test]
fn stable_feature_with_tests_and_docs_scores_above_experimental_one() {
    let report = sample_mixed_report().expect("mixed report should build");
    let stable = report
        .cards
        .iter()
        .find(|card| card.feature_id == "logic.search_index")
        .expect("stable feature should exist");
    let experimental = report
        .cards
        .iter()
        .find(|card| card.feature_id == "ui.right_inspector")
        .expect("experimental feature should exist");

    assert!(stable.score > experimental.score);
}

#[test]
fn deprecated_feature_is_heavily_penalized() {
    let inputs = sample_deprecated_inputs().expect("deprecated fixture should parse");
    let report = score_reuse(inputs).expect("deprecated inputs should still score");

    assert_eq!(report.cards[0].feature_id, "ui.legacy_theme_picker");
    assert_eq!(report.cards[0].score, 0);
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.severity == ReuseScoreSeverity::Warning)
    );
}

#[test]
fn failure_count_lowers_score_deterministically() {
    let base = ReuseScoreInput {
        feature_id: "logic.search_index".into(),
        status: FeatureLifecycleStatus::Stable,
        usage_count: 2,
        has_tests: true,
        has_docs: true,
        has_examples: false,
        has_fixtures: true,
        reviewed_recently: true,
        failure_count: 0,
    };
    let regressed = ReuseScoreInput {
        failure_count: 3,
        ..base.clone()
    };

    let report = score_reuse(vec![base, regressed]).expect("scores should build");

    assert!(report.cards[0].score > report.cards[1].score);
}

#[test]
fn band_thresholds_map_correctly() {
    let report = score_reuse(vec![
        ReuseScoreInput {
            feature_id: "a.low".into(),
            status: FeatureLifecycleStatus::Experimental,
            usage_count: 0,
            has_tests: false,
            has_docs: false,
            has_examples: false,
            has_fixtures: false,
            reviewed_recently: false,
            failure_count: 0,
        },
        ReuseScoreInput {
            feature_id: "b.medium".into(),
            status: FeatureLifecycleStatus::Tested,
            usage_count: 0,
            has_tests: false,
            has_docs: false,
            has_examples: false,
            has_fixtures: false,
            reviewed_recently: false,
            failure_count: 0,
        },
        ReuseScoreInput {
            feature_id: "c.high".into(),
            status: FeatureLifecycleStatus::Tested,
            usage_count: 1,
            has_tests: true,
            has_docs: true,
            has_examples: false,
            has_fixtures: false,
            reviewed_recently: false,
            failure_count: 0,
        },
        ReuseScoreInput {
            feature_id: "d.preferred".into(),
            status: FeatureLifecycleStatus::Stable,
            usage_count: 0,
            has_tests: true,
            has_docs: true,
            has_examples: true,
            has_fixtures: false,
            reviewed_recently: false,
            failure_count: 0,
        },
    ])
    .expect("scores should build");

    let low = report
        .cards
        .iter()
        .find(|card| card.feature_id == "a.low")
        .expect("low card should exist");
    let medium = report
        .cards
        .iter()
        .find(|card| card.feature_id == "b.medium")
        .expect("medium card should exist");
    let high = report
        .cards
        .iter()
        .find(|card| card.feature_id == "c.high")
        .expect("high card should exist");
    let preferred = report
        .cards
        .iter()
        .find(|card| card.feature_id == "d.preferred")
        .expect("preferred card should exist");

    assert_eq!(low.band, ReuseScoreBand::Low);
    assert_eq!(medium.band, ReuseScoreBand::Medium);
    assert_eq!(high.band, ReuseScoreBand::High);
    assert_eq!(preferred.band, ReuseScoreBand::Preferred);
}

#[test]
fn batch_output_sorts_by_score_then_feature_id() {
    let report = score_reuse(vec![
        ReuseScoreInput {
            feature_id: "b.tie".into(),
            status: FeatureLifecycleStatus::Experimental,
            usage_count: 0,
            has_tests: false,
            has_docs: false,
            has_examples: false,
            has_fixtures: false,
            reviewed_recently: false,
            failure_count: 0,
        },
        ReuseScoreInput {
            feature_id: "a.tie".into(),
            status: FeatureLifecycleStatus::Experimental,
            usage_count: 0,
            has_tests: false,
            has_docs: false,
            has_examples: false,
            has_fixtures: false,
            reviewed_recently: false,
            failure_count: 0,
        },
        ReuseScoreInput {
            feature_id: "z.top".into(),
            status: FeatureLifecycleStatus::Stable,
            usage_count: 0,
            has_tests: false,
            has_docs: false,
            has_examples: false,
            has_fixtures: false,
            reviewed_recently: false,
            failure_count: 0,
        },
    ])
    .expect("scores should build");

    assert_eq!(report.cards[0].feature_id, "z.top");
    assert_eq!(report.cards[1].feature_id, "a.tie");
    assert_eq!(report.cards[2].feature_id, "b.tie");
}

#[test]
fn invalid_feature_id_fails_validation() {
    let inputs = sample_invalid_inputs().expect("invalid fixture should parse");
    let validation = score_reuse(inputs).expect_err("blank feature id should fail");

    assert!(
        validation
            .findings
            .iter()
            .any(|finding| finding.path == "inputs[0].feature_id")
    );
}

#[test]
fn scorer_object_scores_inputs() {
    let scorer = ReuseScorer;
    let inputs = reuse_score::sample_mixed_inputs().expect("mixed fixture should parse");
    let validation = scorer.validate(&inputs);
    let report = scorer.score(inputs).expect("scorer should build report");

    assert!(validation.is_valid());
    assert_eq!(report.cards.len(), 3);
}

#[test]
fn manifest_contract_matches_expected_shape() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "reuse_score");
    assert_eq!(manifest.inputs.items.len(), 9);
    assert_eq!(manifest.outputs.items.len(), 3);
}
