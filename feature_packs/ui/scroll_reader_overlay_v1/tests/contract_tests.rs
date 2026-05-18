use feature_core::parse_feature_manifest;
use scroll_reader_overlay_v1::{
    DEFAULT_WORDS_PER_MINUTE, FEATURE_ID, MAX_WORDS_PER_MINUTE, MIN_WORDS_PER_MINUTE,
    OverlayStatus, SPEED_STEP_WORDS_PER_MINUTE, ScrollReaderOverlay, WheelScrubber,
    normalize_selected_text, sample_fixture, sample_input, sample_overlay, tokenize_words,
};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "scroll_reader_overlay_v1");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "draft");
    assert_eq!(
        manifest.inputs.items,
        vec![
            "selected_text",
            "launch_command",
            "overlay_commands",
            "wheel_delta",
            "elapsed_ms",
        ]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "normalized_text",
            "reader_tokens",
            "lens_token",
            "fade_tokens",
            "playback_status",
            "words_per_minute",
        ]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");

    assert_eq!(fixture["words_per_minute"].as_u64(), Some(420));
    assert!(
        fixture["selected_text"]
            .as_str()
            .unwrap()
            .contains("Speed reading")
    );
}

#[test]
fn sample_input_builds_overlay() {
    let input = sample_input().expect("sample input should load");
    let overlay = sample_overlay().expect("sample overlay should build");

    assert_eq!(overlay.words_per_minute(), input.words_per_minute);
    assert_eq!(overlay.status(), OverlayStatus::Paused);
    assert!(overlay.tokens().len() > 10);
}

#[test]
fn normalization_matches_vox_reader_shape() {
    assert_eq!(
        normalize_selected_text("  First\t\tline.\r\nSecond   line.  "),
        "First line.\nSecond line."
    );
    assert_eq!(normalize_selected_text(" \t\r\n "), "");
}

#[test]
fn tokenization_keeps_byte_spans_for_lens_highlighting() {
    let normalized = normalize_selected_text(" Alpha  beta.\nGamma ");
    let tokens = tokenize_words(&normalized);

    assert_eq!(
        tokens
            .iter()
            .map(|token| token.text.as_str())
            .collect::<Vec<_>>(),
        vec!["Alpha", "beta.", "Gamma"]
    );
    assert_eq!(&normalized[tokens[1].start..tokens[1].end], "beta.");
}

#[test]
fn viewport_returns_center_lens_and_fade_context() {
    let mut overlay = ScrollReaderOverlay::from_selected_text("one two three four five");
    overlay.step_by(2);

    let viewport = overlay.viewport(2);
    assert_eq!(viewport.before_fade, vec!["one", "two"]);
    assert_eq!(viewport.lens.as_deref(), Some("three"));
    assert_eq!(viewport.after_fade, vec!["four", "five"]);
    assert_eq!(viewport.current_index, 2);
}

#[test]
fn play_pause_close_follow_overlay_controls() {
    let mut overlay = ScrollReaderOverlay::from_selected_text("one two");
    assert_eq!(overlay.status(), OverlayStatus::Paused);

    overlay.toggle_play();
    assert_eq!(overlay.status(), OverlayStatus::Playing);

    overlay.toggle_play();
    assert_eq!(overlay.status(), OverlayStatus::Paused);

    overlay.close();
    overlay.toggle_play();
    assert_eq!(overlay.status(), OverlayStatus::Closed);
}

#[test]
fn tick_advances_by_words_per_minute_until_finished() {
    let mut overlay = ScrollReaderOverlay::from_selected_text("one two three");
    overlay.toggle_play();

    let step_ms = overlay.milliseconds_per_token();
    overlay.tick(step_ms);
    assert_eq!(overlay.current_index(), 1);
    assert_eq!(overlay.status(), OverlayStatus::Playing);

    overlay.tick(step_ms * 3);
    assert_eq!(overlay.current_index(), 2);
    assert_eq!(overlay.status(), OverlayStatus::Finished);
}

#[test]
fn wheel_angle_matches_parlawl_scrub_direction() {
    let mut overlay = ScrollReaderOverlay::from_selected_text("one two three");
    overlay.step_by(1);

    assert_eq!(overlay.scrub_wheel(120, 0), -1);
    assert_eq!(overlay.current_index(), 0);

    assert_eq!(overlay.scrub_wheel(-120, 0), 1);
    assert_eq!(overlay.current_index(), 1);
}

#[test]
fn wheel_pixel_delta_accumulates_trackpad_steps() {
    let mut scrubber = WheelScrubber::new(18);

    assert_eq!(scrubber.consume(0, 8), 0);
    assert_eq!(scrubber.accumulator_y(), 8);
    assert_eq!(scrubber.consume(0, 10), -1);
    assert_eq!(scrubber.accumulator_y(), 0);
    assert_eq!(scrubber.consume(0, -36), 2);
}

#[test]
fn navigation_clamps_like_review_state() {
    let mut overlay = ScrollReaderOverlay::from_selected_text("one two three");

    overlay.step_by(-4);
    assert_eq!(overlay.current_index(), 0);

    overlay.step_by(99);
    assert_eq!(overlay.current_index(), 2);
    assert_eq!(overlay.status(), OverlayStatus::Finished);
}

#[test]
fn speed_adjustment_clamps_to_overlay_bounds() {
    let mut overlay = ScrollReaderOverlay::from_selected_text("one two");
    assert_eq!(overlay.words_per_minute(), DEFAULT_WORDS_PER_MINUTE);

    overlay.adjust_speed(10_000);
    assert_eq!(overlay.words_per_minute(), MAX_WORDS_PER_MINUTE);

    overlay.adjust_speed(-10_000);
    assert_eq!(overlay.words_per_minute(), MIN_WORDS_PER_MINUTE);

    overlay.adjust_speed(SPEED_STEP_WORDS_PER_MINUTE);
    assert_eq!(overlay.words_per_minute(), MIN_WORDS_PER_MINUTE + 30);
}
