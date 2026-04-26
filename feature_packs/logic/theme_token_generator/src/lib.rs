use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "logic.theme_token_generator";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

impl fmt::Display for ThemeMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Light => formatter.write_str("light"),
            Self::Dark => formatter.write_str("dark"),
            Self::System => formatter.write_str("system"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub mode: ThemeMode,
    pub accent: String,
    pub background: String,
    pub foreground: String,
    pub contrast: u8,
    pub translucent_sidebar: bool,
    pub ui_font: String,
    pub code_font: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        default_dark_config()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeTokens {
    pub app_bg: String,
    pub panel_bg: String,
    pub panel_elevated_bg: String,
    pub sidebar_bg: String,
    pub sidebar_elevated_bg: String,
    pub border_subtle: String,
    pub border_strong: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
    pub text_disabled: String,
    pub accent: String,
    pub accent_hover: String,
    pub accent_active: String,
    pub accent_soft: String,
    pub focus_ring: String,
    pub selection_bg: String,
    pub code_bg: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeFindingSeverity {
    Error,
    Warning,
}

impl fmt::Display for ThemeFindingSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeValidationFinding {
    pub severity: ThemeFindingSeverity,
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeGenerationResult {
    pub resolved_mode: ThemeMode,
    pub tokens: ThemeTokens,
    pub findings: Vec<ThemeValidationFinding>,
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_dark_fixture() -> &'static str {
    include_str!("../fixtures/default_dark_theme.json")
}

pub fn sample_light_fixture() -> &'static str {
    include_str!("../fixtures/default_light_theme.json")
}

pub fn sample_low_contrast_fixture() -> &'static str {
    include_str!("../fixtures/low_contrast_theme.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/bad_hex_theme.json")
}

pub fn parse_config_fixture(raw: &str) -> Result<ThemeConfig, String> {
    serde_json::from_str(raw).map_err(|error| error.to_string())
}

pub fn sample_dark_config() -> Result<ThemeConfig, String> {
    parse_config_fixture(sample_dark_fixture())
}

pub fn sample_light_config() -> Result<ThemeConfig, String> {
    parse_config_fixture(sample_light_fixture())
}

pub fn sample_low_contrast_config() -> Result<ThemeConfig, String> {
    parse_config_fixture(sample_low_contrast_fixture())
}

pub fn sample_invalid_config() -> Result<ThemeConfig, String> {
    parse_config_fixture(sample_invalid_fixture())
}

pub fn default_dark_config() -> ThemeConfig {
    ThemeConfig {
        mode: ThemeMode::Dark,
        accent: "#FF66D4".into(),
        background: "#181A20".into(),
        foreground: "#E8ECF4".into(),
        contrast: 84,
        translucent_sidebar: true,
        ui_font: "Geist".into(),
        code_font: "Geist Mono".into(),
    }
}

pub fn default_light_config() -> ThemeConfig {
    ThemeConfig {
        mode: ThemeMode::Light,
        accent: "#2563EB".into(),
        background: "#F6F7FB".into(),
        foreground: "#161A23".into(),
        contrast: 58,
        translucent_sidebar: false,
        ui_font: "Geist".into(),
        code_font: "Geist Mono".into(),
    }
}

pub fn generate_theme_tokens(config: &ThemeConfig) -> ThemeGenerationResult {
    let mut findings = Vec::new();
    let parsed_background = parse_hex_color(&config.background);
    let resolved_mode = match config.mode {
        ThemeMode::System => parsed_background
            .as_ref()
            .map(|background| {
                if background.relative_luminance() < 0.45 {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                }
            })
            .unwrap_or(ThemeMode::Dark),
        explicit_mode => explicit_mode,
    };
    let resolved_defaults = fallback_for_mode(resolved_mode);
    let accent = parse_color_field(
        "accent",
        &config.accent,
        &mut findings,
        parse_hex_color(&resolved_defaults.accent).expect("default accent should be valid"),
    );
    let background = resolve_color(
        "background",
        parsed_background,
        &mut findings,
        parse_hex_color(&resolved_defaults.background).expect("default background should be valid"),
    );
    let foreground = parse_color_field(
        "foreground",
        &config.foreground,
        &mut findings,
        parse_hex_color(&resolved_defaults.foreground).expect("default foreground should be valid"),
    );

    let spread = (config.contrast.min(100) as f32) / 100.0;
    let contrast_ratio = contrast_ratio(background, foreground);
    if contrast_ratio < 4.5 {
        findings.push(ThemeValidationFinding {
            severity: ThemeFindingSeverity::Warning,
            field: "foreground".into(),
            message: format!(
                "Foreground/background contrast ratio {:.2} is below the recommended 4.5:1.",
                contrast_ratio
            ),
        });
    }

    findings.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.field.cmp(&right.field))
            .then_with(|| left.message.cmp(&right.message))
    });

    let tokens = derive_tokens(
        accent,
        background,
        foreground,
        spread,
        config.translucent_sidebar,
    );

    ThemeGenerationResult {
        resolved_mode,
        tokens,
        findings,
    }
}

fn fallback_for_mode(mode: ThemeMode) -> ThemeConfig {
    match mode {
        ThemeMode::Dark | ThemeMode::System => default_dark_config(),
        ThemeMode::Light => default_light_config(),
    }
}

fn derive_tokens(
    accent: Rgb,
    background: Rgb,
    foreground: Rgb,
    spread: f32,
    translucent_sidebar: bool,
) -> ThemeTokens {
    let panel_bg = background.mix(foreground, 0.04 + spread * 0.08);
    let panel_elevated_bg = background.mix(foreground, 0.08 + spread * 0.14);
    let mut sidebar_bg = background.mix(foreground, 0.03 + spread * 0.05);
    let mut sidebar_elevated_bg = background.mix(foreground, 0.07 + spread * 0.10);

    if translucent_sidebar {
        sidebar_bg = sidebar_bg.mix(background, 0.22);
        sidebar_elevated_bg = sidebar_elevated_bg.mix(background, 0.18);
    }

    ThemeTokens {
        app_bg: background.to_hex(),
        panel_bg: panel_bg.to_hex(),
        panel_elevated_bg: panel_elevated_bg.to_hex(),
        sidebar_bg: sidebar_bg.to_hex(),
        sidebar_elevated_bg: sidebar_elevated_bg.to_hex(),
        border_subtle: background.mix(foreground, 0.16 + spread * 0.12).to_hex(),
        border_strong: background.mix(foreground, 0.28 + spread * 0.18).to_hex(),
        text_primary: foreground.to_hex(),
        text_secondary: foreground
            .mix(background, 0.12 + (1.0 - spread) * 0.24)
            .to_hex(),
        text_muted: foreground
            .mix(background, 0.34 + (1.0 - spread) * 0.16)
            .to_hex(),
        text_disabled: foreground
            .mix(background, 0.56 + (1.0 - spread) * 0.12)
            .to_hex(),
        accent: accent.to_hex(),
        accent_hover: accent.mix(foreground, 0.08 + spread * 0.08).to_hex(),
        accent_active: accent
            .mix(background, 0.10 + (1.0 - spread) * 0.08)
            .to_hex(),
        accent_soft: accent.mix(background, 0.72).to_hex(),
        focus_ring: accent.mix(foreground, 0.12).to_hex(),
        selection_bg: accent.mix(background, 0.58).to_hex(),
        code_bg: panel_bg.mix(background, 0.40).to_hex(),
    }
}

fn parse_color_field(
    field: &str,
    raw: &str,
    findings: &mut Vec<ThemeValidationFinding>,
    fallback: Rgb,
) -> Rgb {
    match parse_hex_color(raw) {
        Ok(color) => color,
        Err(error) => {
            findings.push(ThemeValidationFinding {
                severity: ThemeFindingSeverity::Error,
                field: field.into(),
                message: error,
            });
            fallback
        }
    }
}

fn resolve_color(
    field: &str,
    parsed: Result<Rgb, String>,
    findings: &mut Vec<ThemeValidationFinding>,
    fallback: Rgb,
) -> Rgb {
    match parsed {
        Ok(color) => color,
        Err(error) => {
            findings.push(ThemeValidationFinding {
                severity: ThemeFindingSeverity::Error,
                field: field.into(),
                message: error,
            });
            fallback
        }
    }
}

fn parse_hex_color(raw: &str) -> Result<Rgb, String> {
    let value = raw.trim();
    let Some(value) = value.strip_prefix('#') else {
        return Err(format!("Color '{value}' must start with '#'."));
    };

    if value.len() != 6 {
        return Err(format!(
            "Color '#{value}' must use 6 hexadecimal digits in #RRGGBB format."
        ));
    }

    let red = u8::from_str_radix(&value[0..2], 16)
        .map_err(|_| format!("Color '#{value}' contains invalid hexadecimal digits."))?;
    let green = u8::from_str_radix(&value[2..4], 16)
        .map_err(|_| format!("Color '#{value}' contains invalid hexadecimal digits."))?;
    let blue = u8::from_str_radix(&value[4..6], 16)
        .map_err(|_| format!("Color '#{value}' contains invalid hexadecimal digits."))?;

    Ok(Rgb::new(red, green, blue))
}

fn contrast_ratio(left: Rgb, right: Rgb) -> f32 {
    let left = left.relative_luminance();
    let right = right.relative_luminance();
    let (lighter, darker) = if left >= right {
        (left, right)
    } else {
        (right, left)
    };

    (lighter + 0.05) / (darker + 0.05)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    fn mix(self, other: Self, amount: f32) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        Self {
            red: mix_channel(self.red, other.red, amount),
            green: mix_channel(self.green, other.green, amount),
            blue: mix_channel(self.blue, other.blue, amount),
        }
    }

    fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    fn relative_luminance(self) -> f32 {
        let red = srgb_channel_to_linear(self.red);
        let green = srgb_channel_to_linear(self.green);
        let blue = srgb_channel_to_linear(self.blue);

        0.2126 * red + 0.7152 * green + 0.0722 * blue
    }
}

fn mix_channel(left: u8, right: u8, amount: f32) -> u8 {
    let left = left as f32;
    let right = right as f32;
    (left + (right - left) * amount).round().clamp(0.0, 255.0) as u8
}

fn srgb_channel_to_linear(channel: u8) -> f32 {
    let channel = channel as f32 / 255.0;
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}
