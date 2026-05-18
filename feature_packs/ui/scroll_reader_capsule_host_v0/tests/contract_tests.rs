use feature_core::parse_feature_manifest;
use scroll_reader_capsule_host_v0::{
    CapsuleHostSession, ClipboardLoadDecision, FEATURE_ID, HOST_LAUNCH_PATH,
    HOST_REPOSITION_BEHAVIOR, HOST_RUNTIME_SURFACE, HostCommand, HostKeyboardContract,
    HostPointerContract, HostWindowPosition, HostWindowPreferences, SAMPLE_TEXT, manifest,
    sample_host_session_contract, sample_host_session_fixture,
};
use scroll_reader_capsule_v1::{CapsuleMode, OverlayStatus, ReaderInputSource};

#[test]
fn feature_manifest_matches_contract() {
    let parsed =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(parsed.id, FEATURE_ID);
    assert_eq!(parsed.name, "scroll_reader_capsule_host_v0");
    assert_eq!(parsed.kind.to_string(), "ui_pattern");
    assert_eq!(parsed.status.to_string(), "draft");
    assert_eq!(
        parsed.dependencies,
        vec!["feature_core", "ui.scroll_reader_capsule_v1"]
    );
    assert!(
        parsed
            .inputs
            .items
            .contains(&"command_line_text".to_owned())
    );
    assert!(
        parsed
            .inputs
            .items
            .contains(&"clipboard_text_snapshot".to_owned())
    );
    assert!(parsed.inputs.items.contains(&"pointer_drag".to_owned()));
    assert!(parsed.inputs.items.contains(&"window_position".to_owned()));
    assert!(
        parsed
            .inputs
            .items
            .contains(&"capsule_mode_preference".to_owned())
    );
    assert!(
        parsed
            .outputs
            .items
            .contains(&"runnable_capsule_window".to_owned())
    );
    assert!(
        parsed
            .outputs
            .items
            .contains(&"clipboard_load_report".to_owned())
    );
    assert!(
        parsed
            .outputs
            .items
            .contains(&"window_preferences".to_owned())
    );
    assert!(
        parsed
            .outputs
            .items
            .contains(&"host_pointer_contract".to_owned())
    );
    assert_eq!(manifest().expect("public manifest helper").id, FEATURE_ID);
}

#[test]
fn sample_fixture_describes_local_runnable_host() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_host_session_fixture()).expect("fixture should be json");
    let session = sample_host_session_contract().expect("fixture should parse");

    assert_eq!(session.mode, CapsuleMode::Large);
    assert_eq!(session.sample_text, SAMPLE_TEXT);
    assert_eq!(session.launch_path, HOST_LAUNCH_PATH);
    assert_eq!(session.clipboard_load, "paste_on_demand");
    assert_eq!(session.reposition, HOST_REPOSITION_BEHAVIOR);
    assert!(session.remembers_position);
    assert!(session.remembers_mode);
    assert_eq!(session.runtime_surface, HOST_RUNTIME_SURFACE);
    assert!(!session.desktop_shortcut_registered);
    assert_eq!(fixture["runtime_surface"], "pill_only");
}

#[test]
fn default_session_renders_capsule_plan_from_headless_reader() {
    let session = CapsuleHostSession::default();
    let plan = session.render_plan();

    assert_eq!(session.launch_path(), HOST_LAUNCH_PATH);
    assert!(!session.desktop_shortcut_registered());
    assert_eq!(plan.geometry.width, 400);
    assert_eq!(plan.motion_frame.center_x, 200);
    assert_eq!(plan.empty_message, None);
    assert!(!plan.motion_frame.placements.is_empty());
}

#[test]
fn window_preferences_round_trip_position_and_mode() {
    let path = temp_preferences_path("round_trip_position_and_mode");
    let preferences = HostWindowPreferences::new(
        CapsuleMode::Compact,
        Some(HostWindowPosition { x: 420, y: 96 }),
    );

    preferences
        .save_to_path(&path)
        .expect("preferences should save");
    let loaded = HostWindowPreferences::load_from_path(&path).expect("preferences should load");

    assert_eq!(loaded, preferences);
    assert_eq!(loaded.version, 1);
    assert_eq!(loaded.mode, CapsuleMode::Compact);
    assert_eq!(
        loaded.window_position,
        Some(HostWindowPosition { x: 420, y: 96 })
    );

    let _ = std::fs::remove_file(path);
}

#[test]
fn session_applies_and_emits_window_preferences_without_text_persistence() {
    let mut session = CapsuleHostSession::from_text("temporary reader text");
    let preferences = HostWindowPreferences::new(
        CapsuleMode::Compact,
        Some(HostWindowPosition { x: 10, y: 20 }),
    );

    session.apply_preferences(&preferences);
    assert_eq!(session.mode(), CapsuleMode::Compact);

    let emitted = session.preferences_with_position(Some(HostWindowPosition { x: 11, y: 22 }));
    assert_eq!(emitted.mode, CapsuleMode::Compact);
    assert_eq!(
        emitted.window_position,
        Some(HostWindowPosition { x: 11, y: 22 })
    );
    assert!(
        !serde_json::to_string(&emitted)
            .expect("preferences serialize")
            .contains("temporary reader text")
    );
}

#[test]
fn toggle_mode_switches_between_large_and_compact_size_preferences() {
    let mut session = CapsuleHostSession::from_text("one two three");
    assert_eq!(session.mode(), CapsuleMode::Large);

    session.apply_command(HostCommand::ToggleMode);
    assert_eq!(session.mode(), CapsuleMode::Compact);

    session.apply_command(HostCommand::ToggleMode);
    assert_eq!(session.mode(), CapsuleMode::Large);
}

#[test]
fn command_line_text_replaces_sample_text_when_provided() {
    let session = CapsuleHostSession::from_args([
        "scroll_reader_capsule_host_v0".to_owned(),
        "custom".to_owned(),
        "reader".to_owned(),
        "text".to_owned(),
    ]);

    assert_eq!(
        session
            .capsule()
            .tokens()
            .iter()
            .map(|token| token.text.as_str())
            .collect::<Vec<_>>(),
        vec!["custom", "reader", "text"]
    );
}

#[test]
fn paste_on_demand_loads_clipboard_snapshot_without_history() {
    let mut session = CapsuleHostSession::from_text("old text remains until paste");
    session.apply_command(HostCommand::StepForward);

    let report = session.load_clipboard_text("  fresh\tclipboard\ntext  ");

    assert_eq!(report.decision, ClipboardLoadDecision::Accepted);
    assert_eq!(report.status_label, "loaded clipboard text");
    assert_eq!(report.token_count, 3);
    assert_eq!(session.current_index(), 0);
    assert_eq!(session.status(), OverlayStatus::Paused);
    assert_eq!(
        session.capsule().input_source(),
        ReaderInputSource::ClipboardSnapshot
    );
    assert_eq!(session.capsule().normalized_text(), "fresh clipboard\ntext");
    assert_eq!(
        session
            .capsule()
            .tokens()
            .iter()
            .map(|token| token.text.as_str())
            .collect::<Vec<_>>(),
        vec!["fresh", "clipboard", "text"]
    );
}

#[test]
fn empty_clipboard_snapshot_does_not_wipe_current_reader() {
    let mut session = CapsuleHostSession::from_text("keep this text");
    session.apply_command(HostCommand::StepForward);

    let report = session.load_clipboard_text(" \n\t ");

    assert_eq!(report.decision, ClipboardLoadDecision::IgnoredEmpty);
    assert_eq!(report.status_label, "clipboard text is empty");
    assert_eq!(session.current_index(), 1);
    assert_eq!(session.capsule().normalized_text(), "keep this text");
}

#[test]
fn closed_session_ignores_clipboard_load() {
    let mut session = CapsuleHostSession::from_text("closed text");
    session.apply_command(HostCommand::Close);

    let report = session.load_clipboard_text("new text");

    assert_eq!(report.decision, ClipboardLoadDecision::IgnoredClosed);
    assert_eq!(report.status_label, "capsule is closed");
    assert_eq!(session.capsule().normalized_text(), "closed text");
}

#[test]
fn host_commands_route_to_existing_reader_controls() {
    let mut session = CapsuleHostSession::from_text("one two three four");

    assert_eq!(session.status(), OverlayStatus::Paused);
    session.apply_command(HostCommand::TogglePlay);
    assert_eq!(session.status(), OverlayStatus::Playing);
    session.apply_command(HostCommand::StepForward);
    assert_eq!(session.current_index(), 1);
    session.apply_command(HostCommand::StepBackward);
    assert_eq!(session.current_index(), 0);

    let default_speed = session.words_per_minute();
    session.apply_command(HostCommand::SpeedUp);
    assert!(session.words_per_minute() > default_speed);
    session.apply_command(HostCommand::SlowDown);
    assert_eq!(session.words_per_minute(), default_speed);

    session.apply_command(HostCommand::Close);
    assert!(session.is_closed());
}

#[test]
fn wheel_command_uses_same_scrub_direction_as_reader_core() {
    let mut session = CapsuleHostSession::from_text("one two three");
    session.apply_command(HostCommand::Wheel {
        angle_delta_y: -120,
        pixel_delta_y: 0,
    });
    assert_eq!(session.current_index(), 1);

    session.apply_command(HostCommand::Wheel {
        angle_delta_y: 120,
        pixel_delta_y: 0,
    });
    assert_eq!(session.current_index(), 0);
}

#[test]
fn pointer_contract_allows_click_drag_repositioning_without_extra_chrome() {
    let contract = HostPointerContract::default();

    assert_eq!(
        HOST_REPOSITION_BEHAVIOR,
        "click_drag_capsule_to_move_window"
    );
    assert_eq!(contract.reposition, HOST_REPOSITION_BEHAVIOR);
    assert_eq!(contract.wheel, "mouse wheel or trackpad");
}

#[test]
fn keyboard_contract_stays_small() {
    let contract = HostKeyboardContract::default();

    assert_eq!(contract.load_clipboard, "Cmd+V or Ctrl+V");
    assert_eq!(contract.toggle_mode, "M");
    assert_eq!(contract.play_pause, "Space");
    assert_eq!(contract.close, "Esc");
    assert_eq!(contract.speed_up, "Up");
    assert_eq!(contract.slow_down, "Down");
    assert_eq!(contract.wheel, "mouse wheel or trackpad");
}

fn temp_preferences_path(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "scroll_reader_capsule_host_v0_{label}_{}_{}.json",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos()
    ))
}
