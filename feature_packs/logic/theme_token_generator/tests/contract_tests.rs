use feature_core::parse_feature_manifest;
use theme_token_generator::{
    FEATURE_ID, ThemeConfig, ThemeFindingSeverity, ThemeMode, generate_theme_tokens,
    sample_dark_config, sample_invalid_config, sample_light_config, sample_low_contrast_config,
};

#[test]
fn valid_dark_fixture_generates_tokens_without_findings() {
    let config = sample_dark_config().expect("dark fixture should parse");
    let result = generate_theme_tokens(&config);

    assert_eq!(result.resolved_mode, ThemeMode::Dark);
    assert!(result.findings.is_empty());
    assert_eq!(result.tokens.accent, "#FF66D4");
    assert_eq!(result.tokens.app_bg, "#181A20");
}

#[test]
fn valid_light_fixture_generates_light_mode_tokens() {
    let config = sample_light_config().expect("light fixture should parse");
    let result = generate_theme_tokens(&config);

    assert_eq!(result.resolved_mode, ThemeMode::Light);
    assert!(result.findings.is_empty());
    assert_eq!(result.tokens.app_bg, "#F6F7FB");
    assert_eq!(result.tokens.text_primary, "#161A23");
}

#[test]
fn invalid_hex_fixture_emits_errors_and_uses_fallback_tokens() {
    let config = sample_invalid_config().expect("invalid fixture should parse");
    let result = generate_theme_tokens(&config);

    assert_eq!(result.resolved_mode, ThemeMode::Dark);
    assert_eq!(result.findings.len(), 3);
    assert!(
        result
            .findings
            .iter()
            .all(|finding| finding.severity == ThemeFindingSeverity::Error)
    );
    assert_eq!(result.tokens.app_bg, "#181A20");
    assert_eq!(result.tokens.accent, "#FF66D4");
    assert_eq!(result.tokens.text_primary, "#E8ECF4");
}

#[test]
fn generation_is_deterministic_for_the_same_input() {
    let config = sample_dark_config().expect("dark fixture should parse");
    let first = generate_theme_tokens(&config);
    let second = generate_theme_tokens(&config);

    assert_eq!(first, second);
}

#[test]
fn contrast_changes_multiple_token_groups() {
    let mut low = sample_dark_config().expect("dark fixture should parse");
    low.contrast = 10;
    let mut high = low.clone();
    high.contrast = 90;

    let low_tokens = generate_theme_tokens(&low).tokens;
    let high_tokens = generate_theme_tokens(&high).tokens;

    assert_ne!(low_tokens.panel_elevated_bg, high_tokens.panel_elevated_bg);
    assert_ne!(low_tokens.text_secondary, high_tokens.text_secondary);
    assert_ne!(low_tokens.border_subtle, high_tokens.border_subtle);
}

#[test]
fn translucent_sidebar_changes_only_sidebar_tokens() {
    let mut opaque = sample_dark_config().expect("dark fixture should parse");
    opaque.translucent_sidebar = false;
    let mut translucent = opaque.clone();
    translucent.translucent_sidebar = true;

    let opaque_tokens = generate_theme_tokens(&opaque).tokens;
    let translucent_tokens = generate_theme_tokens(&translucent).tokens;

    assert_ne!(opaque_tokens.sidebar_bg, translucent_tokens.sidebar_bg);
    assert_ne!(
        opaque_tokens.sidebar_elevated_bg,
        translucent_tokens.sidebar_elevated_bg
    );
    assert_eq!(opaque_tokens.panel_bg, translucent_tokens.panel_bg);
    assert_eq!(
        opaque_tokens.panel_elevated_bg,
        translucent_tokens.panel_elevated_bg
    );
    assert_eq!(
        opaque_tokens.border_subtle,
        translucent_tokens.border_subtle
    );
}

#[test]
fn low_contrast_fixture_emits_warning() {
    let config = sample_low_contrast_config().expect("low contrast fixture should parse");
    let result = generate_theme_tokens(&config);

    assert_eq!(result.resolved_mode, ThemeMode::Dark);
    assert_eq!(result.findings.len(), 1);
    assert_eq!(result.findings[0].severity, ThemeFindingSeverity::Warning);
    assert_eq!(result.findings[0].field, "foreground");
}

#[test]
fn system_mode_resolves_from_background_luminance() {
    let config = ThemeConfig {
        mode: ThemeMode::System,
        accent: "#00A3E9".into(),
        background: "#11131A".into(),
        foreground: "#F3F6FB".into(),
        contrast: 50,
        translucent_sidebar: false,
        ui_font: "Geist".into(),
        code_font: "Geist Mono".into(),
    };
    let result = generate_theme_tokens(&config);

    assert_eq!(result.resolved_mode, ThemeMode::Dark);
}

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "theme_token_generator");
    assert_eq!(manifest.inputs.items.len(), 7);
    assert_eq!(manifest.outputs.items.len(), 3);
}
