use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};
pub use theme_token_generator::{ThemeConfig, ThemeMode, ThemeTokens, ThemeValidationFinding};
use theme_token_generator::{
    ThemeGenerationResult, default_dark_config, default_light_config, generate_theme_tokens,
    parse_config_fixture,
};

pub const FEATURE_ID: &str = "ui.theme_editor";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemePreset {
    DefaultDark,
    DefaultLight,
    LowContrast,
    HighAccent,
    Imported,
    Custom,
}

impl ThemePreset {
    pub fn label(self) -> &'static str {
        match self {
            Self::DefaultDark => "default dark",
            Self::DefaultLight => "default light",
            Self::LowContrast => "low contrast",
            Self::HighAccent => "high accent",
            Self::Imported => "imported",
            Self::Custom => "custom",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewSurface {
    pub id: String,
    pub title: String,
    pub background: String,
    pub foreground: String,
    pub border: String,
    pub accent: Option<String>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePreviewModel {
    pub app_background: String,
    pub surfaces: Vec<PreviewSurface>,
    pub focus_ring: String,
    pub selection_bg: String,
    pub code_background: String,
    pub muted_text: String,
    pub ui_font: String,
    pub code_font: String,
}

impl ThemePreviewModel {
    pub fn surface(&self, id: &str) -> Option<&PreviewSurface> {
        self.surfaces.iter().find(|surface| surface.id == id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeEditor {
    config: ThemeConfig,
    preset: ThemePreset,
    generation: ThemeGenerationResult,
    preview: ThemePreviewModel,
}

impl Default for ThemeEditor {
    fn default() -> Self {
        Self::new(default_dark_config())
    }
}

impl ThemeEditor {
    pub fn new(config: ThemeConfig) -> Self {
        let generation = generate_theme_tokens(&config);
        let preview = build_preview(&config, &generation.tokens);
        Self {
            config,
            preset: ThemePreset::DefaultDark,
            generation,
            preview,
        }
    }

    pub fn from_fixture_str(raw: &str) -> Result<Self, String> {
        let config = parse_config_fixture(raw)?;
        Ok(Self::new(config))
    }

    pub fn config(&self) -> &ThemeConfig {
        &self.config
    }

    pub fn preset(&self) -> ThemePreset {
        self.preset
    }

    pub fn tokens(&self) -> &ThemeTokens {
        &self.generation.tokens
    }

    pub fn findings(&self) -> &[ThemeValidationFinding] {
        &self.generation.findings
    }

    pub fn preview(&self) -> &ThemePreviewModel {
        &self.preview
    }

    pub fn resolved_mode(&self) -> ThemeMode {
        self.generation.resolved_mode
    }

    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.config.mode = mode;
        self.mark_custom_and_refresh();
    }

    pub fn set_accent(&mut self, accent: impl Into<String>) {
        self.config.accent = accent.into();
        self.mark_custom_and_refresh();
    }

    pub fn set_background(&mut self, background: impl Into<String>) {
        self.config.background = background.into();
        self.mark_custom_and_refresh();
    }

    pub fn set_foreground(&mut self, foreground: impl Into<String>) {
        self.config.foreground = foreground.into();
        self.mark_custom_and_refresh();
    }

    pub fn set_contrast(&mut self, contrast: u8) {
        self.config.contrast = contrast.min(100);
        self.mark_custom_and_refresh();
    }

    pub fn set_translucent_sidebar(&mut self, translucent_sidebar: bool) {
        self.config.translucent_sidebar = translucent_sidebar;
        self.mark_custom_and_refresh();
    }

    pub fn set_ui_font(&mut self, ui_font: impl Into<String>) {
        self.config.ui_font = ui_font.into();
        self.mark_custom_and_refresh();
    }

    pub fn set_code_font(&mut self, code_font: impl Into<String>) {
        self.config.code_font = code_font.into();
        self.mark_custom_and_refresh();
    }

    pub fn apply_preset(&mut self, preset: ThemePreset) -> Result<(), String> {
        let config = match preset {
            ThemePreset::DefaultDark => default_dark_config(),
            ThemePreset::DefaultLight => default_light_config(),
            ThemePreset::LowContrast => parse_config_fixture(sample_low_contrast_fixture())?,
            ThemePreset::HighAccent => parse_config_fixture(sample_high_accent_fixture())?,
            ThemePreset::Imported | ThemePreset::Custom => {
                return Err("Imported/custom presets must come from config input.".into());
            }
        };
        self.config = config;
        self.preset = preset;
        self.refresh();
        Ok(())
    }

    pub fn reset_to_default(&mut self) {
        self.config = default_dark_config();
        self.preset = ThemePreset::DefaultDark;
        self.refresh();
    }

    pub fn import_theme(&mut self, raw: &str) -> Result<(), String> {
        let config = parse_config_fixture(raw)?;
        self.config = config;
        self.preset = ThemePreset::Imported;
        self.refresh();
        Ok(())
    }

    pub fn export_theme_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.config).map_err(|error| error.to_string())
    }

    pub fn copy_theme_payload(&self) -> Result<String, String> {
        self.export_theme_json()
    }

    fn mark_custom_and_refresh(&mut self) {
        self.preset = ThemePreset::Custom;
        self.refresh();
    }

    fn refresh(&mut self) {
        self.generation = generate_theme_tokens(&self.config);
        self.preview = build_preview(&self.config, &self.generation.tokens);
    }
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

pub fn sample_high_accent_fixture() -> &'static str {
    include_str!("../fixtures/high_accent_theme.json")
}

pub fn sample_invalid_fixture() -> &'static str {
    include_str!("../fixtures/bad_hex_theme.json")
}

pub fn sample_editor() -> Result<ThemeEditor, String> {
    ThemeEditor::from_fixture_str(sample_dark_fixture())
}

fn build_preview(config: &ThemeConfig, tokens: &ThemeTokens) -> ThemePreviewModel {
    ThemePreviewModel {
        app_background: tokens.app_bg.clone(),
        surfaces: vec![
            PreviewSurface {
                id: "left_rail".into(),
                title: "Left rail".into(),
                background: tokens.sidebar_bg.clone(),
                foreground: tokens.text_secondary.clone(),
                border: tokens.border_subtle.clone(),
                accent: Some(tokens.selection_bg.clone()),
                detail: "Feature groups, navigation, and lightweight status.".into(),
            },
            PreviewSurface {
                id: "main_panel".into(),
                title: "Main panel".into(),
                background: tokens.panel_bg.clone(),
                foreground: tokens.text_primary.clone(),
                border: tokens.border_subtle.clone(),
                accent: None,
                detail: "Core workspace surface for build content.".into(),
            },
            PreviewSurface {
                id: "right_inspector".into(),
                title: "Right inspector".into(),
                background: tokens.panel_elevated_bg.clone(),
                foreground: tokens.text_primary.clone(),
                border: tokens.border_strong.clone(),
                accent: Some(tokens.accent_soft.clone()),
                detail: "Context, metadata, and object detail.".into(),
            },
            PreviewSurface {
                id: "command_button".into(),
                title: "Command button".into(),
                background: tokens.accent.clone(),
                foreground: tokens.panel_bg.clone(),
                border: tokens.focus_ring.clone(),
                accent: Some(tokens.accent_hover.clone()),
                detail: "Primary action and focus treatment.".into(),
            },
            PreviewSurface {
                id: "code_block".into(),
                title: "Code block".into(),
                background: tokens.code_bg.clone(),
                foreground: tokens.text_primary.clone(),
                border: tokens.border_subtle.clone(),
                accent: None,
                detail: "Monospace content and derived code surface.".into(),
            },
        ],
        focus_ring: tokens.focus_ring.clone(),
        selection_bg: tokens.selection_bg.clone(),
        code_background: tokens.code_bg.clone(),
        muted_text: tokens.text_muted.clone(),
        ui_font: config.ui_font.clone(),
        code_font: config.code_font.clone(),
    }
}
