use feature_core::parse_feature_manifest;
use theme_editor::{
    FEATURE_ID, ThemeEditor, ThemePreset, sample_editor, sample_invalid_fixture,
    sample_light_fixture,
};
use theme_token_generator::{ThemeMode, default_dark_config, parse_config_fixture};

#[test]
fn controls_update_config_and_regenerate_tokens() {
    let mut editor = sample_editor().expect("sample editor should load");

    editor.set_accent("#00AAEE");
    editor.set_contrast(17);
    editor.set_translucent_sidebar(false);
    editor.set_mode(ThemeMode::Light);

    assert_eq!(editor.config().accent, "#00AAEE");
    assert_eq!(editor.config().contrast, 17);
    assert!(!editor.config().translucent_sidebar);
    assert_eq!(editor.config().mode, ThemeMode::Light);
    assert_eq!(editor.tokens().accent, "#00AAEE");
    assert_eq!(editor.preset(), ThemePreset::Custom);
}

#[test]
fn preview_reflects_generated_tokens() {
    let editor = sample_editor().expect("sample editor should load");
    let preview = editor.preview();
    let tokens = editor.tokens();

    assert_eq!(preview.app_background, tokens.app_bg);
    assert_eq!(
        preview
            .surface("left_rail")
            .expect("left rail preview should exist")
            .background,
        tokens.sidebar_bg
    );
    assert_eq!(
        preview
            .surface("code_block")
            .expect("code preview should exist")
            .background,
        tokens.code_bg
    );
}

#[test]
fn invalid_import_surfaces_findings() {
    let mut editor = sample_editor().expect("sample editor should load");
    editor
        .import_theme(sample_invalid_fixture())
        .expect("invalid fixture should still parse as config");

    assert_eq!(editor.preset(), ThemePreset::Imported);
    assert_eq!(editor.findings().len(), 3);
    assert_eq!(editor.findings()[0].field, "accent");
}

#[test]
fn import_populates_controls() {
    let mut editor = sample_editor().expect("sample editor should load");
    editor
        .import_theme(sample_light_fixture())
        .expect("light fixture should import");

    assert_eq!(editor.config().mode, ThemeMode::Light);
    assert_eq!(editor.config().accent, "#2563EB");
    assert_eq!(editor.config().ui_font, "Geist");
}

#[test]
fn copy_export_uses_current_config_payload() {
    let mut editor = sample_editor().expect("sample editor should load");
    editor.set_background("#20242B");
    let exported = editor
        .copy_theme_payload()
        .expect("copy payload should serialize");
    let parsed = parse_config_fixture(&exported).expect("exported theme should parse");

    assert_eq!(parsed.background, "#20242B");
    assert_eq!(parsed.accent, editor.config().accent);
}

#[test]
fn reset_restores_default_preset() {
    let mut editor = sample_editor().expect("sample editor should load");
    editor
        .apply_preset(ThemePreset::HighAccent)
        .expect("high accent preset should apply");
    editor.reset_to_default();

    assert_eq!(editor.preset(), ThemePreset::DefaultDark);
    assert_eq!(editor.config(), &default_dark_config());
}

#[test]
fn preset_switch_reloads_fixture_config() {
    let mut editor = ThemeEditor::default();
    editor
        .apply_preset(ThemePreset::LowContrast)
        .expect("low contrast preset should apply");

    assert_eq!(editor.preset(), ThemePreset::LowContrast);
    assert_eq!(editor.config().contrast, 22);
    assert_eq!(editor.config().background, "#353944");
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "theme_editor");
    assert_eq!(manifest.inputs.items.len(), 4);
    assert_eq!(manifest.outputs.items.len(), 3);
}
