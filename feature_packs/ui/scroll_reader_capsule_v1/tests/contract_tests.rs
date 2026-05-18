use feature_core::parse_feature_manifest;
use scroll_reader_capsule_v1::{
    AHEAD_CONTEXT_BUDGET_PERCENT, BACKGROUND_TREATMENTS, BEAUTY_PRINCIPLES,
    BUILT_IN_THEME_OPTION_IDS, COLOR_TOKENS, CONTEXT_TEXT_OPACITY_PERCENT, CUSTOM_THEME_CONTROLS,
    CapsuleDesignTokens, CapsuleGeometry, CapsuleMode, CapsulePalette, CapsuleTypographyTokens,
    CapsuleUiContract, ClipboardWatchDecision, ClipboardWatchIngestState,
    DEFAULT_DESKTOP_SHORTCUT_ACTION, DEFAULT_DESKTOP_SHORTCUT_ID, DEFAULT_DESKTOP_SHORTCUT_KEY,
    DEFAULT_DESKTOP_SHORTCUT_MODIFIERS, DEFAULT_DESKTOP_SHORTCUT_SCOPE, DISPLAY_LAYERS, FEATURE_ID,
    FOCUS_CONTEXT_TARGET_CHARS_LARGE, FOCUS_LETTER_SPACING_TENTHS_PX, FOCUS_TEXT_OPACITY_PERCENT,
    FadeZoneSide, HARD_BOUNDARIES, LARGE_CONTEXT_FONT_SIZE, LARGE_FOCUS_FONT_SIZE,
    LENS_BG_OPACITY_PERCENT, MAX_FOCUS_TYPE_SCALE_PERCENT, MIN_READABLE_CONTEXT_WORDS_EACH_SIDE,
    OPTIONAL_SURFACES, OVERLAY_COMPACT_HEIGHT, OVERLAY_COMPACT_RADIUS, OVERLAY_COMPACT_WIDTH,
    OVERLAY_LARGE_HEIGHT, OVERLAY_LARGE_RADIUS, OVERLAY_LARGE_WIDTH, OverlayStatus,
    REQUIRED_CONTROLS, RUNTIME_SURFACE_RULES, ReaderInputPayload, ReaderInputSource,
    ReaderThemeOptionsContract, ReaderThemeSeed, SMOOTHSTEP_MAX_PERMILLE, SMOOTHSTEP_MIN_PERMILLE,
    ScrollReaderCapsule, TAPE_FORMAT_RULES, TAPE_TRANSITION_DURATION_MS, THEME_USER_CONTROLS,
    TapeMotionConfig, WheelScrubber, design_spec_preview, normalize_selected_text, render_plan_svg,
    sample_binder_theme_seed, sample_binder_theme_seed_fixture, sample_capsule,
    sample_design_tokens, sample_desktop_shortcut, sample_desktop_shortcut_fixture, sample_fixture,
    sample_theme_options_contract, sample_theme_options_fixture, smoothstep_permille,
    tokenize_words,
};

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "scroll_reader_capsule_v1");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "draft");
    assert_eq!(
        manifest.inputs.items,
        vec![
            "reader_input_payload",
            "selected_text",
            "clipboard_watch_snapshot",
            "capsule_mode",
            "desktop_shortcut",
            "host_launch_action",
            "wheel_delta",
            "elapsed_ms",
        ]
    );
    assert_eq!(
        manifest.outputs.items,
        vec![
            "geometry_tokens",
            "color_token_names",
            "theme_options_contract",
            "desktop_shortcut_contract",
            "display_layers",
            "beauty_contract",
            "interaction_contract",
            "input_ingest_contract",
            "clipboard_watch_ingest_report",
            "capsule_runtime_model",
            "tape_motion_model",
            "capsule_render_plan",
            "svg_preview",
            "svg_golden_snapshots",
        ]
    );
}

#[test]
fn canonical_geometry_matches_design_slice() {
    let tokens = CapsuleDesignTokens::canonical();

    assert_eq!(
        tokens.large,
        CapsuleGeometry {
            width: 400,
            height: 100,
            radius: 28,
            lens_width: 184,
            lens_height: 54,
        }
    );
    assert_eq!(
        tokens.compact,
        CapsuleGeometry {
            width: 200,
            height: 50,
            radius: 14,
            lens_width: 92,
            lens_height: 30,
        }
    );
    assert_eq!(tokens.geometry_for_mode(CapsuleMode::Large).width, 400);
    assert_eq!(tokens.geometry_for_mode(CapsuleMode::Compact).width, 200);
}

#[test]
fn exported_constants_include_required_capsule_dimensions() {
    assert_eq!(OVERLAY_LARGE_WIDTH, 400);
    assert_eq!(OVERLAY_LARGE_HEIGHT, 100);
    assert_eq!(OVERLAY_LARGE_RADIUS, 28);
    assert_eq!(OVERLAY_COMPACT_WIDTH, 200);
    assert_eq!(OVERLAY_COMPACT_HEIGHT, 50);
    assert_eq!(OVERLAY_COMPACT_RADIUS, 14);
}

#[test]
fn color_contract_preserves_two_background_law() {
    let contract = CapsuleUiContract::canonical();
    let tokens = CapsuleDesignTokens::canonical();

    assert_eq!(BACKGROUND_TREATMENTS.len(), 2);
    assert!(contract.uses_only_two_background_treatments());
    assert_eq!(
        THEME_USER_CONTROLS,
        ["accent", "background", "foreground", "contrast"]
    );
    assert_eq!(
        tokens.theme_user_controls,
        vec!["accent", "background", "foreground", "contrast"]
    );
    assert_eq!(BUILT_IN_THEME_OPTION_IDS, ["warm_lens", "eye_comfort"]);
    assert_eq!(
        tokens.built_in_theme_option_ids,
        vec!["warm_lens", "eye_comfort"]
    );
    assert_eq!(
        CUSTOM_THEME_CONTROLS,
        [
            "accent_color_wheel",
            "background_color_wheel",
            "foreground_color_wheel",
            "contrast_slider"
        ]
    );
    assert_eq!(
        tokens.custom_theme_controls,
        vec![
            "accent_color_wheel",
            "background_color_wheel",
            "foreground_color_wheel",
            "contrast_slider"
        ]
    );
    assert_eq!(tokens.default_theme_contrast, 76);
    assert!(COLOR_TOKENS.contains(&"base_bg"));
    assert!(COLOR_TOKENS.contains(&"capsule_bg"));
    assert!(COLOR_TOKENS.contains(&"lens_bg"));
    assert!(COLOR_TOKENS.contains(&"accent_speed"));
    assert!(COLOR_TOKENS.contains(&"accent_progress"));
}

#[test]
fn three_color_theme_seed_resolves_all_internal_role_colors() {
    let seed = ReaderThemeSeed::new("#39E83C", "#6B6F78", "#33085D", 76);
    let palette = seed.to_palette();

    assert_eq!(palette.capsule_bg, "#6b6f78");
    assert_eq!(palette.accent_speed, "#39e83c");
    assert_eq!(palette.accent_progress, "#39e83c");
    assert_ne!(palette.base_bg, palette.lens_bg);
    assert_ne!(palette.text_focus, palette.text_context);

    for token in COLOR_TOKENS {
        let color = palette
            .color_for_token(token)
            .unwrap_or_else(|| panic!("{token} should resolve to a derived color"));
        assert!(color.starts_with('#'));
        assert_eq!(color.len(), 7);
    }
}

#[test]
fn reader_theme_seed_is_binder_compatible_four_field_payload() {
    let seed = sample_binder_theme_seed().expect("theme seed fixture should parse");
    let fixture: serde_json::Value = serde_json::from_str(sample_binder_theme_seed_fixture())
        .expect("theme seed fixture should be valid json");
    let serialized = serde_json::to_value(&seed).expect("theme seed should serialize");

    assert_eq!(seed.accent, "#39E83C");
    assert_eq!(seed.background, "#6B6F78");
    assert_eq!(seed.foreground, "#33085D");
    assert_eq!(seed.contrast, 76);
    assert_eq!(serialized, fixture);
    let fixture_object = fixture
        .as_object()
        .expect("theme seed fixture should be an object");
    assert_eq!(fixture_object.len(), 4);
    assert!(fixture_object.contains_key("accent"));
    assert!(fixture_object.contains_key("background"));
    assert!(fixture_object.contains_key("foreground"));
    assert!(fixture_object.contains_key("contrast"));
    assert!(!fixture_object.contains_key("reader_only"));
    assert!(!fixture_object.contains_key("palette"));

    for token in COLOR_TOKENS {
        assert!(seed.to_palette().color_for_token(token).is_some());
    }
}

#[test]
fn theme_options_contract_has_two_presets_plus_custom_color_wheel() {
    let contract = ReaderThemeOptionsContract::canonical();

    assert!(contract.has_exactly_two_builtin_options());
    assert!(contract.uses_color_wheel_seed_controls());
    assert_eq!(contract.built_in_options[0].label, "Warm Lens");
    assert_eq!(contract.built_in_options[1].label, "Eye Comfort");
    assert_eq!(contract.built_in_options[1].seed.accent, "#39E83C");
    assert_eq!(contract.built_in_options[1].seed.background, "#6B6F78");
    assert_eq!(contract.built_in_options[1].seed.foreground, "#33085D");
    assert!(!contract.exposes_internal_role_color_editor);
}

#[test]
fn theme_options_fixture_round_trips_canonical_contract() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_theme_options_fixture()).expect("theme options valid json");
    let contract = sample_theme_options_contract().expect("theme options fixture should parse");

    assert_eq!(contract, ReaderThemeOptionsContract::canonical());
    assert_eq!(
        fixture["built_in_options"]
            .as_array()
            .expect("built-in options should be an array")
            .len(),
        2
    );
    assert_eq!(fixture["exposes_internal_role_color_editor"], false);
}

#[test]
fn custom_color_wheel_seed_still_derives_internal_reader_roles() {
    let custom = ReaderThemeSeed::new("#A8FF00", "#111827", "#F8FAFC", 64);
    let palette = custom.to_palette();

    assert_eq!(
        ReaderThemeOptionsContract::canonical().custom_seed_fields,
        vec!["accent", "background", "foreground", "contrast"]
    );
    assert_eq!(palette.accent_progress, "#a8ff00");
    assert_eq!(palette.capsule_bg, "#111827");
    assert_eq!(palette.text_focus.len(), 7);
    for token in COLOR_TOKENS {
        assert!(palette.color_for_token(token).is_some());
    }
}

#[test]
fn desktop_shortcut_contract_uses_host_owned_global_summon_key() {
    let contract = CapsuleUiContract::canonical();
    let shortcut = contract.desktop_shortcut;

    assert_eq!(DEFAULT_DESKTOP_SHORTCUT_ID, "summon_scroll_reader_capsule");
    assert_eq!(DEFAULT_DESKTOP_SHORTCUT_SCOPE, "desktop_global");
    assert_eq!(DEFAULT_DESKTOP_SHORTCUT_MODIFIERS, ["ctrl", "option"]);
    assert_eq!(DEFAULT_DESKTOP_SHORTCUT_KEY, "r");
    assert_eq!(
        DEFAULT_DESKTOP_SHORTCUT_ACTION,
        "capture_selected_or_copied_text_and_open_capsule"
    );
    assert_eq!(shortcut.display_accelerator(), "ctrl+option+R");
    assert!(shortcut.is_host_registered_desktop_shortcut());
}

#[test]
fn desktop_shortcut_fixture_round_trips_default_binding() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_desktop_shortcut_fixture()).expect("shortcut fixture valid");
    let shortcut = sample_desktop_shortcut().expect("shortcut fixture should parse");

    assert_eq!(
        shortcut,
        scroll_reader_capsule_v1::DesktopShortcutBinding::default_summon()
    );
    assert_eq!(fixture["scope"], "desktop_global");
    assert_eq!(fixture["host_owned"], true);
    assert_eq!(fixture["enabled_by_default"], true);
}

#[test]
fn beauty_is_part_of_functional_contract() {
    let contract = CapsuleUiContract::canonical();

    assert!(contract.supports_beauty_as_function());
    assert!(BEAUTY_PRINCIPLES.contains(&"beauty_is_functional"));
    assert!(BEAUTY_PRINCIPLES.contains(&"typographic_focus_not_literal_glow"));
    assert!(BEAUTY_PRINCIPLES.contains(&"subtle_format_preservation"));
    assert!(BEAUTY_PRINCIPLES.contains(&"minimal_visible_indicators"));
}

#[test]
fn tape_format_preserves_text_with_subtle_lens_adjustments() {
    assert!(TAPE_FORMAT_RULES.contains(&"continuous_sentence_tape"));
    assert!(TAPE_FORMAT_RULES.contains(&"highlight_exactly_one_focus_anchor_word"));
    assert!(TAPE_FORMAT_RULES.contains(&"focus_anchor_not_reading_unit"));
    assert!(TAPE_FORMAT_RULES.contains(&"context_band_is_reading_unit"));
    assert!(TAPE_FORMAT_RULES.contains(&"preserve_readable_ahead_context"));
    assert!(TAPE_FORMAT_RULES.contains(&"preserve_readable_behind_context"));
    assert!(TAPE_FORMAT_RULES.contains(&"support_peripheral_context_reading"));
    assert!(TAPE_FORMAT_RULES.contains(&"preserve_word_order"));
    assert!(TAPE_FORMAT_RULES.contains(&"preserve_punctuation"));
    assert!(TAPE_FORMAT_RULES.contains(&"neighbor_words_remain_visible"));
    assert!(TAPE_FORMAT_RULES.contains(&"allow_subtle_lens_type_scale"));
    assert!(TAPE_FORMAT_RULES.contains(&"allow_subtle_lens_word_spacing"));
    assert!(!TAPE_FORMAT_RULES.contains(&"allow_multi_word_lens_beats"));
}

#[test]
fn typography_tokens_keep_magnification_subtle() {
    let typography = CapsuleTypographyTokens::canonical();

    assert_eq!(LARGE_CONTEXT_FONT_SIZE, 18);
    assert_eq!(LARGE_FOCUS_FONT_SIZE, 20);
    assert_eq!(MAX_FOCUS_TYPE_SCALE_PERCENT, 112);
    assert_eq!(FOCUS_LETTER_SPACING_TENTHS_PX, 2);
    assert_eq!(CONTEXT_TEXT_OPACITY_PERCENT, 72);
    assert_eq!(FOCUS_TEXT_OPACITY_PERCENT, 100);
    assert_eq!(LENS_BG_OPACITY_PERCENT, 96);
    assert!(typography.respects_subtle_focus_scale());
    assert_eq!(
        typography.focus_type_scale_percent_for_mode(CapsuleMode::Large),
        111
    );
}

#[test]
fn tape_motion_config_uses_fixed_center_and_transition_duration() {
    let config = TapeMotionConfig::for_mode(CapsuleMode::Large);

    assert_eq!(config.center_x, 200);
    assert_eq!(config.word_gap, 6);
    assert_eq!(config.context_character_target, 90);
    assert_eq!(config.transition_duration_ms, TAPE_TRANSITION_DURATION_MS);
    assert_eq!(TAPE_TRANSITION_DURATION_MS, 160);
}

#[test]
fn display_layers_match_capsule_visual_stack() {
    assert_eq!(
        DISPLAY_LAYERS,
        [
            "base_capsule",
            "continuous_sentence_tape",
            "center_lens",
            "fade_zones",
            "optional_function_marks",
        ]
    );
}

#[test]
fn controls_stay_minimal_and_hotkey_first() {
    assert_eq!(
        REQUIRED_CONTROLS,
        [
            "space_play_pause",
            "esc_close",
            "wheel_trackpad_scrub",
            "up_down_speed",
        ]
    );
}

#[test]
fn hard_boundaries_exclude_full_app_growth() {
    let contract = CapsuleUiContract::canonical();

    assert!(contract.excludes_large_app_shell());
    assert!(HARD_BOUNDARIES.contains(&"document_library"));
    assert!(HARD_BOUNDARIES.contains(&"persistence"));
    assert!(HARD_BOUNDARIES.contains(&"ai_summary"));
    assert!(HARD_BOUNDARIES.contains(&"midi_controls"));
    assert!(HARD_BOUNDARIES.contains(&"repo_scanner_integration"));
    assert!(HARD_BOUNDARIES.contains(&"large_settings_pages"));
}

#[test]
fn separate_options_window_is_allowed_but_not_part_of_capsule_body() {
    let contract = CapsuleUiContract::canonical();

    assert_eq!(OPTIONAL_SURFACES, ["separate_options_window"]);
    assert!(contract.allows_separate_options_window_only());
}

#[test]
fn runtime_surface_is_pill_only_without_persistent_status_chrome() {
    let contract = CapsuleUiContract::canonical();

    assert!(contract.runtime_is_pill_only());
    assert!(RUNTIME_SURFACE_RULES.contains(&"runtime_displays_pill_only"));
    assert!(RUNTIME_SURFACE_RULES.contains(&"no_app_shell"));
    assert!(RUNTIME_SURFACE_RULES.contains(&"no_persistent_play_pause_indicator"));
    assert!(RUNTIME_SURFACE_RULES.contains(&"no_numeric_wpm_readout"));
}

#[test]
fn fixture_round_trips_design_tokens() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["large"]["width"].as_u64(), Some(400));
    assert_eq!(fixture["compact"]["height"].as_u64(), Some(50));

    let tokens = sample_design_tokens().expect("sample tokens should load");
    assert_eq!(tokens, CapsuleDesignTokens::canonical());
}

#[test]
fn crate_references_design_spec_without_copying_engine_logic() {
    let spec = design_spec_preview();

    assert!(spec.contains("overlay_large_width"));
    assert!(spec.contains("Only two background treatments"));
    assert!(spec.contains("continuous sentence tape"));
    assert!(spec.contains("typographically, not graphically"));
    assert!(spec.contains("runtime surface shows only the pill"));
}

#[test]
fn selected_text_normalization_preserves_format_rhythm() {
    assert_eq!(
        normalize_selected_text("  First\t\tline.\r\nSecond   line.  "),
        "First line.\nSecond line."
    );
    assert_eq!(normalize_selected_text(" \t\n "), "");
}

#[test]
fn input_payload_sources_load_like_selected_text_without_clipboard_ownership() {
    let selected = ScrollReaderCapsule::from_selected_text("  Alpha\tbeta\nGamma  ");
    let clipboard = ScrollReaderCapsule::from_input_payload(
        ReaderInputPayload::clipboard_snapshot("  Alpha\tbeta\nGamma  "),
    );
    let editor = ScrollReaderCapsule::from_input_payload(
        ReaderInputPayload::text_editor_clipboard_pipeline("  Alpha\tbeta\nGamma  "),
    );

    assert_eq!(selected.input_source(), ReaderInputSource::SelectedText);
    assert_eq!(
        clipboard.input_source(),
        ReaderInputSource::ClipboardSnapshot
    );
    assert_eq!(
        editor.input_source(),
        ReaderInputSource::TextEditorClipboardPipeline
    );
    assert_eq!(selected.normalized_text(), "Alpha beta\nGamma");
    assert_eq!(clipboard.normalized_text(), selected.normalized_text());
    assert_eq!(editor.normalized_text(), selected.normalized_text());
    assert_eq!(clipboard.tokens(), selected.tokens());
    assert_eq!(editor.tokens(), selected.tokens());
}

#[test]
fn clipboard_watch_ingest_is_explicit_and_host_fed() {
    let mut watch = ClipboardWatchIngestState::new();

    let disabled = watch.observe_clipboard_text("Copied text");
    assert_eq!(disabled.decision, ClipboardWatchDecision::IgnoredDisabled);
    assert_eq!(disabled.payload, None);
    assert!(!watch.is_enabled());

    watch.set_enabled(true);
    let accepted = watch.observe_clipboard_text("Copied text");
    assert_eq!(accepted.decision, ClipboardWatchDecision::Accepted);
    let payload = accepted
        .payload
        .expect("accepted watch text should emit payload");
    assert_eq!(payload.source, ReaderInputSource::ClipboardWatch);
    assert_eq!(payload.text, "Copied text");
    assert_eq!(watch.last_seen_text(), "Copied text");
}

#[test]
fn clipboard_watch_ignores_unchanged_and_empty_snapshots() {
    let mut watch = ClipboardWatchIngestState::enabled();

    assert_eq!(
        watch.observe_clipboard_text("same").decision,
        ClipboardWatchDecision::Accepted
    );
    assert_eq!(
        watch.observe_clipboard_text("same").decision,
        ClipboardWatchDecision::IgnoredUnchanged
    );
    assert_eq!(
        watch.observe_clipboard_text(" \n\t ").decision,
        ClipboardWatchDecision::IgnoredEmpty
    );
}

#[test]
fn clipboard_watch_payload_loads_reader_without_clipboard_ownership() {
    let mut watch = ClipboardWatchIngestState::enabled();
    let report = watch.observe_clipboard_text("  Alpha\tbeta\nGamma  ");
    let payload = report.payload.expect("watch should create payload");
    let capsule = ScrollReaderCapsule::from_input_payload(payload);

    assert_eq!(capsule.input_source(), ReaderInputSource::ClipboardWatch);
    assert_eq!(capsule.normalized_text(), "Alpha beta\nGamma");
    assert_eq!(
        capsule
            .tokens()
            .iter()
            .map(|token| token.text.as_str())
            .collect::<Vec<_>>(),
        vec!["Alpha", "beta", "Gamma"]
    );
}

#[test]
fn input_payload_reload_resets_state_and_preserves_source() {
    let mut capsule = ScrollReaderCapsule::from_selected_text("one two three");
    capsule.step_by(2);
    capsule.toggle_play();

    capsule.load_input_payload(ReaderInputPayload::clipboard_snapshot("fresh text"));

    assert_eq!(capsule.input_source(), ReaderInputSource::ClipboardSnapshot);
    assert_eq!(capsule.current_index(), 0);
    assert_eq!(capsule.status(), OverlayStatus::Paused);
    assert_eq!(
        capsule
            .tokens()
            .iter()
            .map(|token| token.text.as_str())
            .collect::<Vec<_>>(),
        vec!["fresh", "text"]
    );
}

#[test]
fn empty_input_payload_uses_empty_reader_state() {
    let capsule =
        ScrollReaderCapsule::from_input_payload(ReaderInputPayload::clipboard_snapshot(" \n\t "));

    assert_eq!(capsule.input_source(), ReaderInputSource::ClipboardSnapshot);
    assert!(capsule.tokens().is_empty());
    assert_eq!(capsule.status(), OverlayStatus::Paused);
    assert_eq!(
        capsule
            .render_model(CapsuleMode::Compact)
            .empty_message
            .as_deref(),
        Some("Select text to read")
    );
}

#[test]
fn copied_extraction_corruption_is_preserved_as_text() {
    let capsule = ScrollReaderCapsule::from_input_payload(ReaderInputPayload::clipboard_snapshot(
        "  PDF-\nartifact  ##broken@@  OCR|glyph  ",
    ));

    assert_eq!(
        capsule.normalized_text(),
        "PDF-\nartifact ##broken@@ OCR|glyph"
    );
    assert_eq!(
        capsule
            .tokens()
            .iter()
            .map(|token| token.text.as_str())
            .collect::<Vec<_>>(),
        vec!["PDF-", "artifact", "##broken@@", "OCR|glyph"]
    );
}

#[test]
fn tokenization_preserves_valid_spans() {
    let normalized = normalize_selected_text("quick brown fox jumps");
    let tokens = tokenize_words(&normalized);

    assert_eq!(
        tokens
            .iter()
            .map(|token| token.text.as_str())
            .collect::<Vec<_>>(),
        vec!["quick", "brown", "fox", "jumps"]
    );
    assert_eq!(&normalized[tokens[2].start..tokens[2].end], "fox");
}

#[test]
fn viewport_centers_one_focus_anchor_word_on_continuous_tape() {
    let mut capsule = ScrollReaderCapsule::from_selected_text("the quick brown fox jumps over");
    capsule.step_by(3);

    let viewport = capsule.viewport(CapsuleMode::Large, 3);
    assert_eq!(viewport.before_fade, vec!["the", "quick", "brown"]);
    assert_eq!(viewport.lens.as_deref(), Some("fox"));
    assert_eq!(viewport.after_fade, vec!["jumps", "over"]);
}

#[test]
fn mode_viewport_uses_large_context_target_while_highlighting_one_word() {
    let mut capsule = sample_capsule();
    capsule.step_by(3);

    let viewport = capsule.viewport_for_mode(CapsuleMode::Large);
    assert_eq!(FOCUS_CONTEXT_TARGET_CHARS_LARGE, 90);
    assert_eq!(viewport.lens.as_deref(), Some("fox"));
    assert!(viewport.before_fade.join(" ").contains("The quick brown"));
    assert!(
        viewport
            .after_fade
            .join(" ")
            .contains("jumps over the lazy dog.")
    );
    assert!(!viewport.lens.as_deref().unwrap().contains(' '));
}

#[test]
fn tape_motion_frame_keeps_focus_anchor_centered_with_deterministic_neighbors() {
    let mut capsule = sample_capsule();
    capsule.step_by(3);

    let frame = capsule.tape_motion_frame(CapsuleMode::Large);
    let focus_anchor = frame
        .placements
        .iter()
        .find(|placement| placement.focus_anchor)
        .expect("focus_anchor placement should exist");
    let brown = frame
        .placements
        .iter()
        .find(|placement| placement.text == "brown")
        .expect("previous word should exist");
    let jumps = frame
        .placements
        .iter()
        .find(|placement| placement.text == "jumps")
        .expect("next word should exist");

    assert_eq!(frame.center_x, 200);
    assert_eq!(focus_anchor.text, "fox");
    assert_eq!(focus_anchor.x, frame.center_x);
    assert_eq!(focus_anchor.font_size, 20);
    assert_eq!(focus_anchor.opacity_percent, 100);
    assert_eq!(brown.x, 153);
    assert_eq!(jumps.x, 247);
    assert_eq!(brown.font_size, 18);
    assert_eq!(jumps.opacity_percent, 72);
}

#[test]
fn context_band_preserves_readable_words_ahead_and_behind_focus_anchor() {
    let mut capsule = sample_capsule();
    capsule.step_by(3);

    let frame = capsule.tape_motion_frame(CapsuleMode::Large);
    let before_count = frame
        .placements
        .iter()
        .filter(|placement| placement.x < frame.center_x)
        .count();
    let after_count = frame
        .placements
        .iter()
        .filter(|placement| placement.x > frame.center_x)
        .count();

    assert!(before_count >= MIN_READABLE_CONTEXT_WORDS_EACH_SIDE);
    assert!(after_count >= MIN_READABLE_CONTEXT_WORDS_EACH_SIDE);
    assert!(
        frame
            .placements
            .iter()
            .any(|placement| placement.text == "quick")
    );
    assert!(
        frame
            .placements
            .iter()
            .any(|placement| placement.text == "jumps")
    );
    assert!(
        frame
            .placements
            .iter()
            .any(|placement| placement.text == "over")
    );
}

#[test]
fn context_budget_is_slightly_ahead_weighted_for_left_to_right_reading() {
    let mut capsule = ScrollReaderCapsule::from_selected_text(
        "zero one two three four five six seven eight nine ten eleven twelve thirteen fourteen",
    );
    capsule.step_by(7);

    let viewport = capsule.viewport_for_mode(CapsuleMode::Compact);

    assert_eq!(AHEAD_CONTEXT_BUDGET_PERCENT, 60);
    assert_eq!(viewport.lens.as_deref(), Some("seven"));
    assert!(viewport.after_fade.len() >= viewport.before_fade.len());
    assert!(viewport.before_fade.len() >= MIN_READABLE_CONTEXT_WORDS_EACH_SIDE);
    assert!(viewport.after_fade.len() >= MIN_READABLE_CONTEXT_WORDS_EACH_SIDE);
}

#[test]
fn tape_transition_endpoints_match_static_frames() {
    let capsule = sample_capsule();
    let from = capsule.tape_transition_frame(CapsuleMode::Large, 3, 4, 0);
    let to = capsule.tape_transition_frame(CapsuleMode::Large, 3, 4, 1000);

    assert_eq!(from, {
        let mut capsule = sample_capsule();
        capsule.step_by(3);
        capsule.tape_motion_frame(CapsuleMode::Large)
    });
    assert_eq!(to, {
        let mut capsule = sample_capsule();
        capsule.step_by(4);
        capsule.tape_motion_frame(CapsuleMode::Large)
    });
}

#[test]
fn smoothstep_permille_is_monotonic_and_clamped() {
    assert_eq!(smoothstep_permille(-50), SMOOTHSTEP_MIN_PERMILLE);
    assert_eq!(smoothstep_permille(0), 0);
    assert_eq!(smoothstep_permille(250), 156);
    assert_eq!(smoothstep_permille(500), 500);
    assert_eq!(smoothstep_permille(750), 843);
    assert_eq!(smoothstep_permille(1000), 1000);
    assert_eq!(smoothstep_permille(1200), SMOOTHSTEP_MAX_PERMILLE);
    assert!(smoothstep_permille(250) < smoothstep_permille(500));
    assert!(smoothstep_permille(500) < smoothstep_permille(750));
}

#[test]
fn play_pause_finished_and_closed_follow_small_state_machine() {
    let mut capsule = ScrollReaderCapsule::from_selected_text("one two");
    assert_eq!(capsule.status(), OverlayStatus::Paused);

    capsule.toggle_play();
    assert_eq!(capsule.status(), OverlayStatus::Playing);

    capsule.toggle_play();
    assert_eq!(capsule.status(), OverlayStatus::Paused);

    capsule.toggle_play();
    capsule.tick(capsule.milliseconds_per_token() * 3);
    assert_eq!(capsule.status(), OverlayStatus::Finished);

    capsule.close();
    capsule.toggle_play();
    capsule.step_by(-1);
    assert_eq!(capsule.status(), OverlayStatus::Closed);
    assert_eq!(capsule.current_index(), 1);
}

#[test]
fn empty_text_never_enters_playing() {
    let mut capsule = ScrollReaderCapsule::from_selected_text(" \n\t ");
    assert_eq!(capsule.tokens().len(), 0);
    assert_eq!(capsule.status(), OverlayStatus::Paused);

    capsule.toggle_play();
    assert_eq!(capsule.status(), OverlayStatus::Paused);

    let model = capsule.render_model(CapsuleMode::Compact);
    assert_eq!(model.empty_message.as_deref(), Some("Select text to read"));
    assert_eq!(model.lens_text, "Select text to read");

    let frame = capsule.tape_motion_frame(CapsuleMode::Compact);
    assert_eq!(frame.center_x, 100);
    assert!(frame.placements.is_empty());
}

#[test]
fn wheel_scrub_matches_chess_scroll_direction_and_accumulates_pixels() {
    let mut scrubber = WheelScrubber::new(18);
    assert_eq!(scrubber.consume(120, 0), -1);
    assert_eq!(scrubber.consume(-120, 0), 1);

    assert_eq!(scrubber.consume(0, 8), 0);
    assert_eq!(scrubber.accumulator_y(), 8);
    assert_eq!(scrubber.consume(0, 10), -1);
    assert_eq!(scrubber.accumulator_y(), 0);
    assert_eq!(scrubber.consume(0, -36), 2);
}

#[test]
fn speed_is_internal_and_clamped() {
    let mut capsule = ScrollReaderCapsule::from_selected_text("one two three");
    assert_eq!(capsule.words_per_minute(), 420);
    assert_eq!(capsule.milliseconds_per_token(), 142);

    capsule.adjust_speed(10_000);
    assert_eq!(capsule.words_per_minute(), 900);

    capsule.adjust_speed(-10_000);
    assert_eq!(capsule.words_per_minute(), 120);
}

#[test]
fn render_plan_exposes_stable_large_geometry_for_host_drawing() {
    let mut capsule = sample_capsule();
    capsule.step_by(3);

    let plan = capsule.render_plan(CapsuleMode::Large);

    assert_eq!(plan.capsule_rect.x, 0);
    assert_eq!(plan.capsule_rect.y, 0);
    assert_eq!(plan.capsule_rect.width, 400);
    assert_eq!(plan.capsule_rect.height, 100);
    assert_eq!(plan.lens_rect.width, 184);
    assert_eq!(plan.lens_rect.height, 54);
    assert_eq!(
        plan.lens_rect.x + plan.lens_rect.width as i32 / 2,
        plan.motion_frame.center_x
    );
    assert_eq!(plan.motion_frame.center_x, 200);
    assert_eq!(plan.text_clip_rect.x, 10);
    assert_eq!(plan.text_clip_rect.width, 380);
    assert!(plan.text_clip_rect.x >= plan.capsule_rect.x);
    assert!(
        plan.text_clip_rect.x + plan.text_clip_rect.width as i32
            <= plan.capsule_rect.x + plan.capsule_rect.width as i32
    );
    assert_eq!(plan.progress_marker.track.height, 3);
    assert_eq!(plan.empty_message, None);
}

#[test]
fn render_plan_fade_zones_do_not_cover_focus_anchor() {
    let mut capsule = sample_capsule();
    capsule.step_by(3);

    let plan = capsule.render_plan(CapsuleMode::Large);
    let focus_x = plan.motion_frame.center_x;
    let left = plan
        .fade_zones
        .iter()
        .find(|zone| zone.side == FadeZoneSide::Left)
        .expect("left fade zone");
    let right = plan
        .fade_zones
        .iter()
        .find(|zone| zone.side == FadeZoneSide::Right)
        .expect("right fade zone");

    assert_eq!(plan.fade_zones.len(), 2);
    assert!(left.rect.x + (left.rect.width as i32) < focus_x);
    assert!(right.rect.x > focus_x);
    assert_eq!(left.rect.height, plan.lens_rect.height);
    assert_eq!(right.rect.height, plan.lens_rect.height);
}

#[test]
fn render_transition_plan_uses_interpolated_tape_motion_frame() {
    let capsule = sample_capsule();

    let start = capsule.render_transition_plan(CapsuleMode::Large, 3, 4, 0);
    let middle = capsule.render_transition_plan(CapsuleMode::Large, 3, 4, 500);
    let end = capsule.render_transition_plan(CapsuleMode::Large, 3, 4, 1000);

    assert_eq!(start.motion_frame.focus_anchor_index, 3);
    assert_eq!(middle.motion_frame.progress_permille, 500);
    assert_eq!(end.motion_frame.focus_anchor_index, 4);
    assert_eq!(start.lens_rect, middle.lens_rect);
    assert_eq!(middle.lens_rect, end.lens_rect);
    assert_eq!(middle.motion_frame.center_x, middle.lens_rect.x + 92);
}

#[test]
fn empty_render_plan_preserves_empty_message_and_centered_lens() {
    let capsule = ScrollReaderCapsule::from_selected_text(" \n\t ");

    let plan = capsule.render_plan(CapsuleMode::Compact);
    let svg = capsule.render_svg(CapsuleMode::Compact);

    assert_eq!(plan.geometry.width, 200);
    assert_eq!(plan.motion_frame.center_x, 100);
    assert!(plan.motion_frame.placements.is_empty());
    assert_eq!(plan.empty_message.as_deref(), Some("Select text to read"));
    assert_eq!(
        plan.lens_rect.x + plan.lens_rect.width as i32 / 2,
        plan.motion_frame.center_x
    );
    assert!(svg.contains(">Select text to read<"));
}

#[test]
fn golden_svg_snapshots_match_render_plan_output() {
    let cases = [
        (
            "large_mid.svg",
            include_str!("../fixtures/render_snapshots/large_mid.svg"),
            snapshot_large_mid(),
        ),
        (
            "compact_mid.svg",
            include_str!("../fixtures/render_snapshots/compact_mid.svg"),
            snapshot_compact_mid(),
        ),
        (
            "large_start.svg",
            include_str!("../fixtures/render_snapshots/large_start.svg"),
            snapshot_large_start(),
        ),
        (
            "large_end.svg",
            include_str!("../fixtures/render_snapshots/large_end.svg"),
            snapshot_large_end(),
        ),
        (
            "compact_empty.svg",
            include_str!("../fixtures/render_snapshots/compact_empty.svg"),
            snapshot_compact_empty(),
        ),
        (
            "large_long_focus_word.svg",
            include_str!("../fixtures/render_snapshots/large_long_focus_word.svg"),
            snapshot_large_long_focus_word(),
        ),
        (
            "large_transition_50.svg",
            include_str!("../fixtures/render_snapshots/large_transition_50.svg"),
            snapshot_large_transition_50(),
        ),
    ];

    for (name, expected, actual) in cases {
        assert_eq!(
            normalize_svg_fixture(expected),
            normalize_svg_fixture(&actual),
            "{name} should match the checked-in render snapshot"
        );
    }
}

#[test]
fn golden_svg_snapshots_keep_runtime_surface_small() {
    let snapshots = [
        snapshot_large_mid(),
        snapshot_compact_mid(),
        snapshot_large_start(),
        snapshot_large_end(),
        snapshot_compact_empty(),
        snapshot_large_long_focus_word(),
        snapshot_large_transition_50(),
    ];

    for svg in snapshots {
        assert!(svg.contains("<clipPath id=\"scroll-reader-text-clip\">"));
        assert!(!svg.contains("wpm"));
        assert!(!svg.contains(">play<"));
        assert!(!svg.contains(">pause<"));
        assert!(!svg.contains("toolbar"));
        assert!(!svg.contains("dashboard"));
        assert!(!svg.contains("app shell"));
    }

    let empty = snapshot_compact_empty();
    assert!(empty.contains(">Select text to read<"));

    let long_word = snapshot_large_long_focus_word();
    assert!(long_word.contains("pneumonoultramicroscopicsilicovolcanoconiosis"));
    assert!(long_word.contains("width=\"400\" height=\"100\""));
    assert!(long_word.contains("<clipPath id=\"scroll-reader-text-clip\">"));

    let transition = snapshot_large_transition_50();
    assert_ne!(transition, snapshot_large_mid());
    assert!(transition.contains(">jumps<"));
    assert!(transition.contains("font-size=\"20\""));
}

#[test]
fn render_model_and_svg_are_capsule_not_app_shell() {
    let mut capsule = sample_capsule();
    capsule.step_by(3);

    let model = capsule.render_model(CapsuleMode::Large);
    assert_eq!(model.geometry.width, 400);
    assert_eq!(model.lens_text, "fox");
    assert!(model.after_text.starts_with("jumps over"));
    assert!(model.empty_message.is_none());

    let svg = capsule.render_svg(CapsuleMode::Large);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("width=\"400\""));
    assert!(svg.contains(">fox<"));
    assert!(svg.contains(">jumps<"));
    assert!(svg.contains(">over<"));
    assert!(svg.contains("font-size=\"20\""));
    assert!(svg.contains("font-size=\"18\""));
    assert!(svg.contains("font-weight=\"600\""));
    assert!(svg.contains("letter-spacing=\"0.2\""));
    assert!(svg.contains("opacity=\"0.72\""));
    assert!(svg.contains("opacity=\"0.96\""));
    assert!(!svg.contains("font-size=\"21\""));
    assert!(!svg.contains("wpm"));
    assert!(!svg.contains(">pause<"));
    assert!(!svg.contains(">play<"));
    assert!(!svg.contains("‹‹"));
}

fn snapshot_large_mid() -> String {
    let mut capsule = sample_capsule();
    capsule.step_by(3);
    capsule.render_svg(CapsuleMode::Large)
}

fn snapshot_compact_mid() -> String {
    let mut capsule = sample_capsule();
    capsule.step_by(3);
    capsule.render_svg(CapsuleMode::Compact)
}

fn snapshot_large_start() -> String {
    sample_capsule().render_svg(CapsuleMode::Large)
}

fn snapshot_large_end() -> String {
    let mut capsule = sample_capsule();
    capsule.step_by(999);
    capsule.render_svg(CapsuleMode::Large)
}

fn snapshot_compact_empty() -> String {
    ScrollReaderCapsule::from_selected_text(" \n\t ").render_svg(CapsuleMode::Compact)
}

fn snapshot_large_long_focus_word() -> String {
    let mut capsule = ScrollReaderCapsule::from_selected_text(
        "before context pneumonoultramicroscopicsilicovolcanoconiosis after context remains readable",
    );
    capsule.step_by(2);
    capsule.render_svg(CapsuleMode::Large)
}

fn snapshot_large_transition_50() -> String {
    let capsule = sample_capsule();
    render_plan_svg(
        &capsule.render_transition_plan(CapsuleMode::Large, 3, 4, 500),
        &CapsulePalette::default(),
    )
}

fn normalize_svg_fixture(svg: &str) -> String {
    svg.replace("\r\n", "\n").trim_end_matches('\n').to_owned()
}
