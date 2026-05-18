use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.scroll_reader_capsule_v1";

pub const OVERLAY_LARGE_WIDTH: u16 = 400;
pub const OVERLAY_LARGE_HEIGHT: u16 = 100;
pub const OVERLAY_LARGE_RADIUS: u16 = 28;
pub const OVERLAY_COMPACT_WIDTH: u16 = 200;
pub const OVERLAY_COMPACT_HEIGHT: u16 = 50;
pub const OVERLAY_COMPACT_RADIUS: u16 = 14;
pub const LENS_WIDTH_LARGE: u16 = 184;
pub const LENS_HEIGHT_LARGE: u16 = 54;
pub const LENS_WIDTH_COMPACT: u16 = 92;
pub const LENS_HEIGHT_COMPACT: u16 = 30;
pub const FADE_WIDTH: u16 = 56;
pub const CONTROL_ROW_HEIGHT: u16 = 20;
pub const PROGRESS_MARKER_HEIGHT: u16 = 3;
pub const OVERLAY_PADDING: u16 = 10;
pub const FOCUS_CONTEXT_TARGET_CHARS_LARGE: usize = 90;
pub const FOCUS_CONTEXT_TARGET_CHARS_COMPACT: usize = 44;
pub const LARGE_CONTEXT_FONT_SIZE: u16 = 18;
pub const LARGE_FOCUS_FONT_SIZE: u16 = 20;
pub const COMPACT_CONTEXT_FONT_SIZE: u16 = 11;
pub const COMPACT_FOCUS_FONT_SIZE: u16 = 12;
pub const MAX_FOCUS_TYPE_SCALE_PERCENT: u16 = 112;
pub const FOCUS_LETTER_SPACING_TENTHS_PX: i16 = 2;
pub const CONTINUOUS_TAPE_WORD_GAP_PX: i16 = 6;
pub const CONTEXT_TEXT_OPACITY_PERCENT: u8 = 72;
pub const FOCUS_TEXT_OPACITY_PERCENT: u8 = 100;
pub const LENS_BG_OPACITY_PERCENT: u8 = 96;
pub const TAPE_TRANSITION_DURATION_MS: u16 = 160;
pub const SMOOTHSTEP_MIN_PERMILLE: u16 = 0;
pub const SMOOTHSTEP_MAX_PERMILLE: u16 = 1000;
pub const MIN_READABLE_CONTEXT_WORDS_EACH_SIDE: usize = 3;
pub const AHEAD_CONTEXT_BUDGET_PERCENT: u8 = 60;
pub const BEHIND_CONTEXT_BUDGET_PERCENT: u8 = 40;
pub const DEFAULT_WORDS_PER_MINUTE: u32 = 420;
pub const MIN_WORDS_PER_MINUTE: u32 = 120;
pub const MAX_WORDS_PER_MINUTE: u32 = 900;
pub const SPEED_STEP_WORDS_PER_MINUTE: i32 = 30;
pub const DEFAULT_WHEEL_PIXEL_STEP: i32 = 18;
pub const DEFAULT_THEME_CONTRAST: u8 = 76;

pub const THEME_USER_CONTROLS: [&str; 4] = ["accent", "background", "foreground", "contrast"];
pub const BUILT_IN_THEME_OPTION_IDS: [&str; 2] = ["warm_lens", "eye_comfort"];
pub const CUSTOM_THEME_CONTROLS: [&str; 4] = [
    "accent_color_wheel",
    "background_color_wheel",
    "foreground_color_wheel",
    "contrast_slider",
];
pub const DEFAULT_DESKTOP_SHORTCUT_ID: &str = "summon_scroll_reader_capsule";
pub const DEFAULT_DESKTOP_SHORTCUT_LABEL: &str = "Summon Scroll Reader";
pub const DEFAULT_DESKTOP_SHORTCUT_SCOPE: &str = "desktop_global";
pub const DEFAULT_DESKTOP_SHORTCUT_MODIFIERS: [&str; 2] = ["ctrl", "option"];
pub const DEFAULT_DESKTOP_SHORTCUT_KEY: &str = "r";
pub const DEFAULT_DESKTOP_SHORTCUT_ACTION: &str =
    "capture_selected_or_copied_text_and_open_capsule";

pub const COLOR_TOKENS: [&str; 11] = [
    "base_bg",
    "capsule_bg",
    "lens_bg",
    "lens_edge",
    "text_context",
    "text_focus",
    "text_muted",
    "accent_speed",
    "accent_progress",
    "control_idle",
    "control_hover",
];

pub const BACKGROUND_TREATMENTS: [&str; 2] = ["base_or_capsule_bg", "lens_bg"];

pub const DISPLAY_LAYERS: [&str; 5] = [
    "base_capsule",
    "continuous_sentence_tape",
    "center_lens",
    "fade_zones",
    "optional_function_marks",
];

pub const REQUIRED_CONTROLS: [&str; 4] = [
    "space_play_pause",
    "esc_close",
    "wheel_trackpad_scrub",
    "up_down_speed",
];

pub const HARD_BOUNDARIES: [&str; 11] = [
    "document_library",
    "persistence",
    "history_storage",
    "ai_summary",
    "midi_controls",
    "dashboard",
    "sidebar",
    "full_text_editor",
    "large_settings_pages",
    "account_auth",
    "repo_scanner_integration",
];

pub const OPTIONAL_SURFACES: [&str; 1] = ["separate_options_window"];

pub const RUNTIME_SURFACE_RULES: [&str; 4] = [
    "runtime_displays_pill_only",
    "no_app_shell",
    "no_persistent_play_pause_indicator",
    "no_numeric_wpm_readout",
];

pub const BEAUTY_PRINCIPLES: [&str; 4] = [
    "beauty_is_functional",
    "typographic_focus_not_literal_glow",
    "subtle_format_preservation",
    "minimal_visible_indicators",
];

pub const TAPE_FORMAT_RULES: [&str; 12] = [
    "continuous_sentence_tape",
    "highlight_exactly_one_focus_anchor_word",
    "focus_anchor_not_reading_unit",
    "context_band_is_reading_unit",
    "preserve_readable_ahead_context",
    "preserve_readable_behind_context",
    "support_peripheral_context_reading",
    "preserve_word_order",
    "preserve_punctuation",
    "neighbor_words_remain_visible",
    "allow_subtle_lens_type_scale",
    "allow_subtle_lens_word_spacing",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleMode {
    Large,
    Compact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleGeometry {
    pub width: u16,
    pub height: u16,
    pub radius: u16,
    pub lens_width: u16,
    pub lens_height: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleDesignTokens {
    pub large: CapsuleGeometry,
    pub compact: CapsuleGeometry,
    pub fade_width: u16,
    pub control_row_height: u16,
    pub progress_marker_height: u16,
    pub overlay_padding: u16,
    pub focus_context_target_chars_large: usize,
    pub focus_context_target_chars_compact: usize,
    pub tape_transition_duration_ms: u16,
    pub typography: CapsuleTypographyTokens,
    pub color_tokens: Vec<String>,
    pub theme_user_controls: Vec<String>,
    pub built_in_theme_option_ids: Vec<String>,
    pub custom_theme_controls: Vec<String>,
    pub default_theme_contrast: u8,
    pub display_layers: Vec<String>,
    pub beauty_principles: Vec<String>,
    pub tape_format_rules: Vec<String>,
    pub optional_surfaces: Vec<String>,
    pub runtime_surface_rules: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleTypographyTokens {
    pub large_context_font_size: u16,
    pub large_focus_font_size: u16,
    pub compact_context_font_size: u16,
    pub compact_focus_font_size: u16,
    pub max_focus_type_scale_percent: u16,
    pub focus_letter_spacing_tenths_px: i16,
    pub continuous_tape_word_gap_px: i16,
    pub context_text_opacity_percent: u8,
    pub focus_text_opacity_percent: u8,
    pub lens_bg_opacity_percent: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleUiContract {
    pub geometry: CapsuleDesignTokens,
    pub background_treatments: Vec<String>,
    pub desktop_shortcut: DesktopShortcutBinding,
    pub required_controls: Vec<String>,
    pub hard_boundaries: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopShortcutBinding {
    pub id: String,
    pub label: String,
    pub scope: String,
    pub modifiers: Vec<String>,
    pub key: String,
    pub action: String,
    pub enabled_by_default: bool,
    pub host_owned: bool,
}

impl CapsuleGeometry {
    pub const fn large() -> Self {
        Self {
            width: OVERLAY_LARGE_WIDTH,
            height: OVERLAY_LARGE_HEIGHT,
            radius: OVERLAY_LARGE_RADIUS,
            lens_width: LENS_WIDTH_LARGE,
            lens_height: LENS_HEIGHT_LARGE,
        }
    }

    pub const fn compact() -> Self {
        Self {
            width: OVERLAY_COMPACT_WIDTH,
            height: OVERLAY_COMPACT_HEIGHT,
            radius: OVERLAY_COMPACT_RADIUS,
            lens_width: LENS_WIDTH_COMPACT,
            lens_height: LENS_HEIGHT_COMPACT,
        }
    }
}

impl CapsuleDesignTokens {
    pub fn canonical() -> Self {
        Self {
            large: CapsuleGeometry::large(),
            compact: CapsuleGeometry::compact(),
            fade_width: FADE_WIDTH,
            control_row_height: CONTROL_ROW_HEIGHT,
            progress_marker_height: PROGRESS_MARKER_HEIGHT,
            overlay_padding: OVERLAY_PADDING,
            focus_context_target_chars_large: FOCUS_CONTEXT_TARGET_CHARS_LARGE,
            focus_context_target_chars_compact: FOCUS_CONTEXT_TARGET_CHARS_COMPACT,
            tape_transition_duration_ms: TAPE_TRANSITION_DURATION_MS,
            typography: CapsuleTypographyTokens::canonical(),
            color_tokens: COLOR_TOKENS
                .iter()
                .map(|token| (*token).to_owned())
                .collect(),
            theme_user_controls: THEME_USER_CONTROLS
                .iter()
                .map(|control| (*control).to_owned())
                .collect(),
            built_in_theme_option_ids: BUILT_IN_THEME_OPTION_IDS
                .iter()
                .map(|option| (*option).to_owned())
                .collect(),
            custom_theme_controls: CUSTOM_THEME_CONTROLS
                .iter()
                .map(|control| (*control).to_owned())
                .collect(),
            default_theme_contrast: DEFAULT_THEME_CONTRAST,
            display_layers: DISPLAY_LAYERS
                .iter()
                .map(|layer| (*layer).to_owned())
                .collect(),
            beauty_principles: BEAUTY_PRINCIPLES
                .iter()
                .map(|principle| (*principle).to_owned())
                .collect(),
            tape_format_rules: TAPE_FORMAT_RULES
                .iter()
                .map(|rule| (*rule).to_owned())
                .collect(),
            optional_surfaces: OPTIONAL_SURFACES
                .iter()
                .map(|surface| (*surface).to_owned())
                .collect(),
            runtime_surface_rules: RUNTIME_SURFACE_RULES
                .iter()
                .map(|rule| (*rule).to_owned())
                .collect(),
        }
    }

    pub fn geometry_for_mode(&self, mode: CapsuleMode) -> CapsuleGeometry {
        match mode {
            CapsuleMode::Large => self.large,
            CapsuleMode::Compact => self.compact,
        }
    }
}

impl CapsuleTypographyTokens {
    pub const fn canonical() -> Self {
        Self {
            large_context_font_size: LARGE_CONTEXT_FONT_SIZE,
            large_focus_font_size: LARGE_FOCUS_FONT_SIZE,
            compact_context_font_size: COMPACT_CONTEXT_FONT_SIZE,
            compact_focus_font_size: COMPACT_FOCUS_FONT_SIZE,
            max_focus_type_scale_percent: MAX_FOCUS_TYPE_SCALE_PERCENT,
            focus_letter_spacing_tenths_px: FOCUS_LETTER_SPACING_TENTHS_PX,
            continuous_tape_word_gap_px: CONTINUOUS_TAPE_WORD_GAP_PX,
            context_text_opacity_percent: CONTEXT_TEXT_OPACITY_PERCENT,
            focus_text_opacity_percent: FOCUS_TEXT_OPACITY_PERCENT,
            lens_bg_opacity_percent: LENS_BG_OPACITY_PERCENT,
        }
    }

    pub fn context_font_size_for_mode(&self, mode: CapsuleMode) -> u16 {
        match mode {
            CapsuleMode::Large => self.large_context_font_size,
            CapsuleMode::Compact => self.compact_context_font_size,
        }
    }

    pub fn focus_font_size_for_mode(&self, mode: CapsuleMode) -> u16 {
        match mode {
            CapsuleMode::Large => self.large_focus_font_size,
            CapsuleMode::Compact => self.compact_focus_font_size,
        }
    }

    pub fn focus_type_scale_percent_for_mode(&self, mode: CapsuleMode) -> u16 {
        let context = self.context_font_size_for_mode(mode).max(1) as u32;
        let focus = self.focus_font_size_for_mode(mode) as u32;
        ((focus * 100) / context) as u16
    }

    pub fn respects_subtle_focus_scale(&self) -> bool {
        self.focus_type_scale_percent_for_mode(CapsuleMode::Large)
            <= self.max_focus_type_scale_percent
            && self.focus_type_scale_percent_for_mode(CapsuleMode::Compact)
                <= self.max_focus_type_scale_percent
    }
}

impl TapeMotionConfig {
    pub fn for_mode(mode: CapsuleMode) -> Self {
        let tokens = CapsuleDesignTokens::canonical();
        let geometry = tokens.geometry_for_mode(mode);

        Self {
            mode,
            center_x: (geometry.width / 2) as i32,
            word_gap: tokens.typography.continuous_tape_word_gap_px,
            context_character_target: match mode {
                CapsuleMode::Large => tokens.focus_context_target_chars_large,
                CapsuleMode::Compact => tokens.focus_context_target_chars_compact,
            },
            transition_duration_ms: tokens.tape_transition_duration_ms,
        }
    }
}

impl CapsuleUiContract {
    pub fn canonical() -> Self {
        Self {
            geometry: CapsuleDesignTokens::canonical(),
            background_treatments: BACKGROUND_TREATMENTS
                .iter()
                .map(|treatment| (*treatment).to_owned())
                .collect(),
            desktop_shortcut: DesktopShortcutBinding::default_summon(),
            required_controls: REQUIRED_CONTROLS
                .iter()
                .map(|control| (*control).to_owned())
                .collect(),
            hard_boundaries: HARD_BOUNDARIES
                .iter()
                .map(|boundary| (*boundary).to_owned())
                .collect(),
        }
    }

    pub fn uses_only_two_background_treatments(&self) -> bool {
        self.background_treatments.len() == 2
    }

    pub fn excludes_large_app_shell(&self) -> bool {
        [
            "document_library",
            "persistence",
            "dashboard",
            "sidebar",
            "full_text_editor",
            "large_settings_pages",
            "repo_scanner_integration",
        ]
        .iter()
        .all(|boundary| self.hard_boundaries.iter().any(|known| known == boundary))
    }

    pub fn supports_beauty_as_function(&self) -> bool {
        self.geometry
            .beauty_principles
            .iter()
            .any(|principle| principle == "beauty_is_functional")
            && self
                .geometry
                .beauty_principles
                .iter()
                .any(|principle| principle == "typographic_focus_not_literal_glow")
    }

    pub fn allows_separate_options_window_only(&self) -> bool {
        self.geometry.optional_surfaces == ["separate_options_window"]
            && self
                .hard_boundaries
                .iter()
                .any(|boundary| boundary == "large_settings_pages")
    }

    pub fn runtime_is_pill_only(&self) -> bool {
        self.geometry
            .runtime_surface_rules
            .iter()
            .any(|rule| rule == "runtime_displays_pill_only")
            && self
                .geometry
                .runtime_surface_rules
                .iter()
                .any(|rule| rule == "no_app_shell")
            && self
                .geometry
                .runtime_surface_rules
                .iter()
                .any(|rule| rule == "no_persistent_play_pause_indicator")
            && self
                .geometry
                .runtime_surface_rules
                .iter()
                .any(|rule| rule == "no_numeric_wpm_readout")
    }
}

impl DesktopShortcutBinding {
    pub fn default_summon() -> Self {
        Self {
            id: DEFAULT_DESKTOP_SHORTCUT_ID.to_owned(),
            label: DEFAULT_DESKTOP_SHORTCUT_LABEL.to_owned(),
            scope: DEFAULT_DESKTOP_SHORTCUT_SCOPE.to_owned(),
            modifiers: DEFAULT_DESKTOP_SHORTCUT_MODIFIERS
                .iter()
                .map(|modifier| (*modifier).to_owned())
                .collect(),
            key: DEFAULT_DESKTOP_SHORTCUT_KEY.to_owned(),
            action: DEFAULT_DESKTOP_SHORTCUT_ACTION.to_owned(),
            enabled_by_default: true,
            host_owned: true,
        }
    }

    pub fn display_accelerator(&self) -> String {
        let mut parts = self.modifiers.clone();
        parts.push(self.key.to_uppercase());
        parts.join("+")
    }

    pub fn is_host_registered_desktop_shortcut(&self) -> bool {
        self.scope == "desktop_global" && self.host_owned && self.enabled_by_default
    }
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn design_spec_preview() -> &'static str {
    include_str!("../../../../docs/scroll_reader_capsule/01_visual_contract.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/design_tokens.json")
}

pub fn sample_binder_theme_seed_fixture() -> &'static str {
    include_str!("../fixtures/theme_seed_binder_compatible.json")
}

pub fn sample_theme_options_fixture() -> &'static str {
    include_str!("../fixtures/theme_options.json")
}

pub fn sample_desktop_shortcut_fixture() -> &'static str {
    include_str!("../fixtures/desktop_shortcut.json")
}

pub fn sample_design_tokens() -> Result<CapsuleDesignTokens, String> {
    serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())
}

pub fn sample_binder_theme_seed() -> Result<ReaderThemeSeed, String> {
    serde_json::from_str(sample_binder_theme_seed_fixture()).map_err(|error| error.to_string())
}

pub fn sample_theme_options_contract() -> Result<ReaderThemeOptionsContract, String> {
    serde_json::from_str(sample_theme_options_fixture()).map_err(|error| error.to_string())
}

pub fn sample_desktop_shortcut() -> Result<DesktopShortcutBinding, String> {
    serde_json::from_str(sample_desktop_shortcut_fixture()).map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlayStatus {
    Paused,
    Playing,
    Finished,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderInputSource {
    SelectedText,
    ClipboardSnapshot,
    ClipboardWatch,
    TextEditorClipboardPipeline,
}

impl Default for ReaderInputSource {
    fn default() -> Self {
        Self::SelectedText
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderInputPayload {
    pub source: ReaderInputSource,
    pub text: String,
}

impl ReaderInputPayload {
    pub fn new(source: ReaderInputSource, text: impl Into<String>) -> Self {
        Self {
            source,
            text: text.into(),
        }
    }

    pub fn selected_text(text: impl Into<String>) -> Self {
        Self::new(ReaderInputSource::SelectedText, text)
    }

    pub fn clipboard_snapshot(text: impl Into<String>) -> Self {
        Self::new(ReaderInputSource::ClipboardSnapshot, text)
    }

    pub fn clipboard_watch(text: impl Into<String>) -> Self {
        Self::new(ReaderInputSource::ClipboardWatch, text)
    }

    pub fn text_editor_clipboard_pipeline(text: impl Into<String>) -> Self {
        Self::new(ReaderInputSource::TextEditorClipboardPipeline, text)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardWatchDecision {
    Accepted,
    IgnoredDisabled,
    IgnoredUnchanged,
    IgnoredEmpty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardWatchIngestReport {
    pub decision: ClipboardWatchDecision,
    pub status_label: String,
    pub payload: Option<ReaderInputPayload>,
}

impl ClipboardWatchIngestReport {
    pub fn accepted(text: String) -> Self {
        Self {
            decision: ClipboardWatchDecision::Accepted,
            status_label: "captured copied text".to_owned(),
            payload: Some(ReaderInputPayload::clipboard_watch(text)),
        }
    }

    pub fn ignored(decision: ClipboardWatchDecision, status_label: impl Into<String>) -> Self {
        Self {
            decision,
            status_label: status_label.into(),
            payload: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardWatchIngestState {
    enabled: bool,
    last_seen_text: String,
}

impl Default for ClipboardWatchIngestState {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardWatchIngestState {
    pub fn new() -> Self {
        Self {
            enabled: false,
            last_seen_text: String::new(),
        }
    }

    pub fn enabled() -> Self {
        Self {
            enabled: true,
            last_seen_text: String::new(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn last_seen_text(&self) -> &str {
        &self.last_seen_text
    }

    pub fn observe_clipboard_text(
        &mut self,
        text: impl Into<String>,
    ) -> ClipboardWatchIngestReport {
        let text = text.into();
        if !self.enabled {
            return ClipboardWatchIngestReport::ignored(
                ClipboardWatchDecision::IgnoredDisabled,
                "clipboard watch is off",
            );
        }
        if text == self.last_seen_text {
            return ClipboardWatchIngestReport::ignored(
                ClipboardWatchDecision::IgnoredUnchanged,
                "clipboard text unchanged",
            );
        }

        self.last_seen_text = text.clone();
        if text.trim().is_empty() {
            return ClipboardWatchIngestReport::ignored(
                ClipboardWatchDecision::IgnoredEmpty,
                "clipboard text is empty",
            );
        }

        ClipboardWatchIngestReport::accepted(text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderToken {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderViewport {
    pub before_fade: Vec<String>,
    pub lens: Option<String>,
    pub after_fade: Vec<String>,
    pub current_index: usize,
    pub token_count: usize,
    pub progress_percent: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TapeMotionConfig {
    pub mode: CapsuleMode,
    pub center_x: i32,
    pub word_gap: i16,
    pub context_character_target: usize,
    pub transition_duration_ms: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TapeMotionFrame {
    pub focus_anchor_index: usize,
    pub center_x: i32,
    pub progress_permille: u16,
    pub placements: Vec<TapeWordPlacement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TapeWordPlacement {
    pub token_index: usize,
    pub text: String,
    pub x: i32,
    pub opacity_percent: u8,
    pub font_size: u16,
    pub focus_anchor: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WheelScrubber {
    pixel_step: i32,
    accumulator_y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrollReaderCapsule {
    input_source: ReaderInputSource,
    normalized_text: String,
    tokens: Vec<ReaderToken>,
    current_index: usize,
    words_per_minute: u32,
    status: OverlayStatus,
    wheel_scrubber: WheelScrubber,
    elapsed_ms_accumulator: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleRenderModel {
    pub mode: CapsuleMode,
    pub geometry: CapsuleGeometry,
    pub status: OverlayStatus,
    pub before_text: String,
    pub lens_text: String,
    pub after_text: String,
    pub progress_percent: u8,
    pub empty_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FadeZoneSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderRect {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
    pub radius: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleFadeZone {
    pub side: FadeZoneSide,
    pub rect: RenderRect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleProgressMarker {
    pub track: RenderRect,
    pub fill: RenderRect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleRenderPlan {
    pub mode: CapsuleMode,
    pub status: OverlayStatus,
    pub geometry: CapsuleGeometry,
    pub capsule_rect: RenderRect,
    pub lens_rect: RenderRect,
    pub text_clip_rect: RenderRect,
    pub fade_zones: Vec<CapsuleFadeZone>,
    pub progress_marker: CapsuleProgressMarker,
    pub text_baseline_y: i32,
    pub motion_frame: TapeMotionFrame,
    pub empty_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderThemeSeed {
    pub accent: String,
    pub background: String,
    pub foreground: String,
    pub contrast: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderThemeOption {
    pub id: String,
    pub label: String,
    pub seed: ReaderThemeSeed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderThemeOptionsContract {
    pub built_in_options: Vec<ReaderThemeOption>,
    pub custom_theme_controls: Vec<String>,
    pub custom_seed_fields: Vec<String>,
    pub exposes_internal_role_color_editor: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsulePalette {
    pub base_bg: String,
    pub capsule_bg: String,
    pub lens_bg: String,
    pub lens_edge: String,
    pub text_context: String,
    pub text_focus: String,
    pub text_muted: String,
    pub accent_speed: String,
    pub accent_progress: String,
    pub control_idle: String,
    pub control_hover: String,
}

impl Default for WheelScrubber {
    fn default() -> Self {
        Self {
            pixel_step: DEFAULT_WHEEL_PIXEL_STEP,
            accumulator_y: 0,
        }
    }
}

impl WheelScrubber {
    pub fn new(pixel_step: i32) -> Self {
        Self {
            pixel_step: pixel_step.max(1),
            accumulator_y: 0,
        }
    }

    pub fn consume(&mut self, angle_delta_y: i32, pixel_delta_y: i32) -> isize {
        if angle_delta_y != 0 {
            self.accumulator_y = 0;
            return if angle_delta_y > 0 { -1 } else { 1 };
        }

        if pixel_delta_y == 0 {
            return 0;
        }

        self.accumulator_y += pixel_delta_y;
        let mut emitted_steps = 0isize;

        while self.accumulator_y >= self.pixel_step {
            emitted_steps -= 1;
            self.accumulator_y -= self.pixel_step;
        }
        while self.accumulator_y <= -self.pixel_step {
            emitted_steps += 1;
            self.accumulator_y += self.pixel_step;
        }

        emitted_steps
    }

    pub fn accumulator_y(&self) -> i32 {
        self.accumulator_y
    }
}

impl Default for ScrollReaderCapsule {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollReaderCapsule {
    pub fn new() -> Self {
        Self {
            input_source: ReaderInputSource::default(),
            normalized_text: String::new(),
            tokens: Vec::new(),
            current_index: 0,
            words_per_minute: DEFAULT_WORDS_PER_MINUTE,
            status: OverlayStatus::Paused,
            wheel_scrubber: WheelScrubber::default(),
            elapsed_ms_accumulator: 0,
        }
    }

    pub fn from_selected_text(selected_text: &str) -> Self {
        Self::from_input_payload(ReaderInputPayload::selected_text(selected_text))
    }

    pub fn from_input_payload(payload: ReaderInputPayload) -> Self {
        let mut capsule = Self::new();
        capsule.load_input_payload(payload);
        capsule
    }

    pub fn load_selected_text(&mut self, selected_text: &str) {
        self.load_input_payload(ReaderInputPayload::selected_text(selected_text));
    }

    pub fn load_input_payload(&mut self, payload: ReaderInputPayload) {
        self.input_source = payload.source;
        self.normalized_text = normalize_selected_text(&payload.text);
        self.tokens = tokenize_words(&self.normalized_text);
        self.current_index = 0;
        self.status = OverlayStatus::Paused;
        self.elapsed_ms_accumulator = 0;
        self.wheel_scrubber = WheelScrubber::default();
    }

    pub fn toggle_play(&mut self) {
        match self.status {
            OverlayStatus::Paused => {
                if !self.tokens.is_empty() {
                    self.status = OverlayStatus::Playing;
                }
            }
            OverlayStatus::Playing => self.status = OverlayStatus::Paused,
            OverlayStatus::Finished => {
                if !self.tokens.is_empty() {
                    self.current_index = 0;
                    self.status = OverlayStatus::Playing;
                }
            }
            OverlayStatus::Closed => {}
        }
    }

    pub fn close(&mut self) {
        self.status = OverlayStatus::Closed;
    }

    pub fn step_by(&mut self, steps: isize) {
        if self.status == OverlayStatus::Closed || self.tokens.is_empty() || steps == 0 {
            return;
        }

        if steps < 0 {
            self.current_index = self.current_index.saturating_sub(steps.unsigned_abs());
            if self.status == OverlayStatus::Finished {
                self.status = OverlayStatus::Paused;
            }
            return;
        }

        let last_index = self.tokens.len() - 1;
        let requested = self.current_index.saturating_add(steps as usize);
        if requested >= last_index {
            self.current_index = last_index;
            if requested > last_index && self.status == OverlayStatus::Playing {
                self.status = OverlayStatus::Finished;
            }
        } else {
            self.current_index = requested;
        }
    }

    pub fn scrub_wheel(&mut self, angle_delta_y: i32, pixel_delta_y: i32) -> isize {
        let steps = self.wheel_scrubber.consume(angle_delta_y, pixel_delta_y);
        self.step_by(steps);
        steps
    }

    pub fn tick(&mut self, elapsed_ms: u64) {
        if self.status != OverlayStatus::Playing || self.tokens.is_empty() {
            return;
        }

        self.elapsed_ms_accumulator = self.elapsed_ms_accumulator.saturating_add(elapsed_ms);
        let step_ms = self.milliseconds_per_token();

        while self.elapsed_ms_accumulator >= step_ms {
            self.elapsed_ms_accumulator -= step_ms;
            let previous_index = self.current_index;
            self.step_by(1);
            if self.current_index == previous_index {
                self.status = OverlayStatus::Finished;
                self.elapsed_ms_accumulator = 0;
                break;
            }
        }
    }

    pub fn adjust_speed(&mut self, delta_words_per_minute: i32) {
        let next = self.words_per_minute as i32 + delta_words_per_minute;
        self.words_per_minute =
            next.clamp(MIN_WORDS_PER_MINUTE as i32, MAX_WORDS_PER_MINUTE as i32) as u32;
    }

    pub fn viewport(&self, mode: CapsuleMode, radius: usize) -> ReaderViewport {
        if self.tokens.is_empty() {
            return ReaderViewport {
                before_fade: Vec::new(),
                lens: None,
                after_fade: Vec::new(),
                current_index: 0,
                token_count: 0,
                progress_percent: 0,
            };
        }

        let _ = mode;
        let lens_end = self.current_index + 1;
        let before_start = self.current_index.saturating_sub(radius);
        let after_end = (lens_end + radius).min(self.tokens.len());

        ReaderViewport {
            before_fade: self.tokens[before_start..self.current_index]
                .iter()
                .map(|token| token.text.clone())
                .collect(),
            lens: Some(self.tokens[self.current_index].text.clone()),
            after_fade: self.tokens[lens_end..after_end]
                .iter()
                .map(|token| token.text.clone())
                .collect(),
            current_index: self.current_index,
            token_count: self.tokens.len(),
            progress_percent: self.progress_percent(),
        }
    }

    pub fn viewport_for_mode(&self, mode: CapsuleMode) -> ReaderViewport {
        let target_chars = match mode {
            CapsuleMode::Large => FOCUS_CONTEXT_TARGET_CHARS_LARGE,
            CapsuleMode::Compact => FOCUS_CONTEXT_TARGET_CHARS_COMPACT,
        };
        self.viewport_by_context_chars(target_chars)
    }

    pub fn viewport_by_context_chars(&self, target_chars: usize) -> ReaderViewport {
        if self.tokens.is_empty() {
            return ReaderViewport {
                before_fade: Vec::new(),
                lens: None,
                after_fade: Vec::new(),
                current_index: 0,
                token_count: 0,
                progress_percent: 0,
            };
        }

        let (before_start, after_end) =
            self.context_bounds_for_budget(self.current_index, target_chars);

        ReaderViewport {
            before_fade: self.tokens[before_start..self.current_index]
                .iter()
                .map(|token| token.text.clone())
                .collect(),
            lens: Some(self.tokens[self.current_index].text.clone()),
            after_fade: self.tokens[self.current_index + 1..after_end]
                .iter()
                .map(|token| token.text.clone())
                .collect(),
            current_index: self.current_index,
            token_count: self.tokens.len(),
            progress_percent: self.progress_percent(),
        }
    }

    pub fn render_model(&self, mode: CapsuleMode) -> CapsuleRenderModel {
        let geometry = CapsuleDesignTokens::canonical().geometry_for_mode(mode);
        let viewport = self.viewport_for_mode(mode);

        CapsuleRenderModel {
            mode,
            geometry,
            status: self.status,
            before_text: viewport.before_fade.join(" "),
            lens_text: viewport
                .lens
                .unwrap_or_else(|| "Select text to read".to_owned()),
            after_text: viewport.after_fade.join(" "),
            progress_percent: viewport.progress_percent,
            empty_message: if self.tokens.is_empty() {
                Some("Select text to read".to_owned())
            } else {
                None
            },
        }
    }

    pub fn render_plan(&self, mode: CapsuleMode) -> CapsuleRenderPlan {
        let model = self.render_model(mode);
        let frame = self.tape_motion_frame(mode);
        render_plan_from_model_and_frame(model, frame)
    }

    pub fn tape_motion_frame(&self, mode: CapsuleMode) -> TapeMotionFrame {
        self.motion_frame_for_index(mode, self.current_index, 0)
    }

    pub fn tape_transition_frame(
        &self,
        mode: CapsuleMode,
        from_index: usize,
        to_index: usize,
        progress_permille: i32,
    ) -> TapeMotionFrame {
        if self.tokens.is_empty() {
            return self.empty_motion_frame(mode, 0);
        }

        let from_index = from_index.min(self.tokens.len() - 1);
        let to_index = to_index.min(self.tokens.len() - 1);
        let progress = clamp_permille(progress_permille);

        if progress == SMOOTHSTEP_MIN_PERMILLE {
            return self.motion_frame_for_index(mode, from_index, 0);
        }
        if progress == SMOOTHSTEP_MAX_PERMILLE {
            return self.motion_frame_for_index(mode, to_index, 0);
        }

        let config = TapeMotionConfig::for_mode(mode);
        let typography = CapsuleTypographyTokens::canonical();
        let eased = smoothstep_permille(progress as i32) as i32;
        let focus_anchor_index = if eased < 500 { from_index } else { to_index };
        let (from_start, from_end) =
            self.context_bounds_for_budget(from_index, config.context_character_target);
        let (to_start, to_end) =
            self.context_bounds_for_budget(to_index, config.context_character_target);
        let start = from_start.min(to_start);
        let end = from_end.max(to_end);

        let placements = (start..end)
            .map(|token_index| {
                let from_x = self.word_center_x_for_index(from_index, token_index, &config);
                let to_x = self.word_center_x_for_index(to_index, token_index, &config);
                let x = interpolate_i32(from_x, to_x, eased);
                self.word_placement_for_index(token_index, focus_anchor_index, x, mode, &typography)
            })
            .collect();

        TapeMotionFrame {
            focus_anchor_index,
            center_x: config.center_x,
            progress_permille: progress,
            placements,
        }
    }

    pub fn render_transition_plan(
        &self,
        mode: CapsuleMode,
        from_index: usize,
        to_index: usize,
        progress_permille: i32,
    ) -> CapsuleRenderPlan {
        let model = self.render_model(mode);
        let frame = self.tape_transition_frame(mode, from_index, to_index, progress_permille);
        render_plan_from_model_and_frame(model, frame)
    }

    pub fn render_svg(&self, mode: CapsuleMode) -> String {
        render_plan_svg(&self.render_plan(mode), &CapsulePalette::default())
    }

    pub fn normalized_text(&self) -> &str {
        &self.normalized_text
    }

    pub fn input_source(&self) -> ReaderInputSource {
        self.input_source
    }

    pub fn tokens(&self) -> &[ReaderToken] {
        &self.tokens
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn status(&self) -> OverlayStatus {
        self.status
    }

    pub fn words_per_minute(&self) -> u32 {
        self.words_per_minute
    }

    pub fn milliseconds_per_token(&self) -> u64 {
        (60_000 / self.words_per_minute.max(1) as u64).max(1)
    }

    pub fn progress_percent(&self) -> u8 {
        if self.tokens.is_empty() {
            return 0;
        }
        if self.tokens.len() == 1 {
            return 100;
        }
        let ratio = self.current_index as f32 / (self.tokens.len() - 1) as f32;
        (ratio.clamp(0.0, 1.0) * 100.0).round() as u8
    }

    fn motion_frame_for_index(
        &self,
        mode: CapsuleMode,
        focus_anchor_index: usize,
        progress_permille: u16,
    ) -> TapeMotionFrame {
        if self.tokens.is_empty() {
            return self.empty_motion_frame(mode, progress_permille);
        }

        let config = TapeMotionConfig::for_mode(mode);
        let typography = CapsuleTypographyTokens::canonical();
        let focus_anchor_index = focus_anchor_index.min(self.tokens.len() - 1);
        let (start, end) =
            self.context_bounds_for_budget(focus_anchor_index, config.context_character_target);
        let placements = (start..end)
            .map(|token_index| {
                let x = self.word_center_x_for_index(focus_anchor_index, token_index, &config);
                self.word_placement_for_index(token_index, focus_anchor_index, x, mode, &typography)
            })
            .collect();

        TapeMotionFrame {
            focus_anchor_index,
            center_x: config.center_x,
            progress_permille,
            placements,
        }
    }

    fn empty_motion_frame(&self, mode: CapsuleMode, progress_permille: u16) -> TapeMotionFrame {
        TapeMotionFrame {
            focus_anchor_index: 0,
            center_x: TapeMotionConfig::for_mode(mode).center_x,
            progress_permille,
            placements: Vec::new(),
        }
    }

    fn word_placement_for_index(
        &self,
        token_index: usize,
        focus_anchor_index: usize,
        x: i32,
        mode: CapsuleMode,
        typography: &CapsuleTypographyTokens,
    ) -> TapeWordPlacement {
        let focus_anchor = token_index == focus_anchor_index;

        TapeWordPlacement {
            token_index,
            text: self.tokens[token_index].text.clone(),
            x,
            opacity_percent: if focus_anchor {
                typography.focus_text_opacity_percent
            } else {
                typography.context_text_opacity_percent
            },
            font_size: if focus_anchor {
                typography.focus_font_size_for_mode(mode)
            } else {
                typography.context_font_size_for_mode(mode)
            },
            focus_anchor,
        }
    }

    fn word_center_x_for_index(
        &self,
        focus_anchor_index: usize,
        token_index: usize,
        config: &TapeMotionConfig,
    ) -> i32 {
        if token_index == focus_anchor_index {
            return config.center_x;
        }

        let typography = CapsuleTypographyTokens::canonical();
        let gap = config.word_gap as i32;
        let focus_anchor_width =
            self.estimated_token_width(focus_anchor_index, config.mode, true, &typography);

        if token_index < focus_anchor_index {
            let mut x = config.center_x - focus_anchor_width / 2;
            for index in (token_index..focus_anchor_index).rev() {
                let width = self.estimated_token_width(index, config.mode, false, &typography);
                x -= gap + width;
            }
            x + self.estimated_token_width(token_index, config.mode, false, &typography) / 2
        } else {
            let mut x = config.center_x + focus_anchor_width / 2;
            for index in focus_anchor_index + 1..=token_index {
                let width = self.estimated_token_width(index, config.mode, false, &typography);
                x += gap + width;
            }
            x - self.estimated_token_width(token_index, config.mode, false, &typography) / 2
        }
    }

    fn estimated_token_width(
        &self,
        token_index: usize,
        mode: CapsuleMode,
        focus_anchor: bool,
        typography: &CapsuleTypographyTokens,
    ) -> i32 {
        let font_size = if focus_anchor {
            typography.focus_font_size_for_mode(mode)
        } else {
            typography.context_font_size_for_mode(mode)
        };
        estimate_text_width_px(&self.tokens[token_index].text, font_size)
    }

    fn context_bounds_for_budget(
        &self,
        focus_anchor_index: usize,
        target_chars: usize,
    ) -> (usize, usize) {
        let before_budget = target_chars * BEHIND_CONTEXT_BUDGET_PERCENT as usize / 100;
        let after_budget = target_chars.saturating_sub(before_budget);
        (
            self.context_start_for_budget(focus_anchor_index, before_budget),
            self.context_end_for_budget(focus_anchor_index, after_budget),
        )
    }

    fn context_start_for_budget(&self, focus_anchor_index: usize, budget: usize) -> usize {
        let mut used = 0usize;
        let mut start = focus_anchor_index;

        while start > 0 {
            let next_len = self.tokens[start - 1].text.chars().count();
            let separator = if used == 0 { 0 } else { 1 };
            if used + separator + next_len > budget {
                break;
            }
            used += separator + next_len;
            start -= 1;
        }

        start
    }

    fn context_end_for_budget(&self, focus_anchor_index: usize, budget: usize) -> usize {
        let mut used = 0usize;
        let mut end = focus_anchor_index + 1;

        while end < self.tokens.len() {
            let next_len = self.tokens[end].text.chars().count();
            let separator = if used == 0 { 0 } else { 1 };
            if used + separator + next_len > budget {
                break;
            }
            used += separator + next_len;
            end += 1;
        }

        end
    }
}

impl Default for ReaderThemeSeed {
    fn default() -> Self {
        Self {
            accent: "#d9ad70".to_owned(),
            background: "#25221d".to_owned(),
            foreground: "#efe2c7".to_owned(),
            contrast: DEFAULT_THEME_CONTRAST,
        }
    }
}

impl ReaderThemeSeed {
    pub fn new(
        accent: impl Into<String>,
        background: impl Into<String>,
        foreground: impl Into<String>,
        contrast: u8,
    ) -> Self {
        Self {
            accent: accent.into(),
            background: background.into(),
            foreground: foreground.into(),
            contrast,
        }
    }

    pub fn to_palette(&self) -> CapsulePalette {
        CapsulePalette::from_theme_seed(self)
    }
}

impl ReaderThemeOption {
    pub fn warm_lens() -> Self {
        Self {
            id: "warm_lens".to_owned(),
            label: "Warm Lens".to_owned(),
            seed: ReaderThemeSeed::default(),
        }
    }

    pub fn eye_comfort() -> Self {
        Self {
            id: "eye_comfort".to_owned(),
            label: "Eye Comfort".to_owned(),
            seed: ReaderThemeSeed::new("#39E83C", "#6B6F78", "#33085D", DEFAULT_THEME_CONTRAST),
        }
    }
}

impl ReaderThemeOptionsContract {
    pub fn canonical() -> Self {
        Self {
            built_in_options: vec![
                ReaderThemeOption::warm_lens(),
                ReaderThemeOption::eye_comfort(),
            ],
            custom_theme_controls: CUSTOM_THEME_CONTROLS
                .iter()
                .map(|control| (*control).to_owned())
                .collect(),
            custom_seed_fields: THEME_USER_CONTROLS
                .iter()
                .map(|field| (*field).to_owned())
                .collect(),
            exposes_internal_role_color_editor: false,
        }
    }

    pub fn has_exactly_two_builtin_options(&self) -> bool {
        self.built_in_options.len() == BUILT_IN_THEME_OPTION_IDS.len()
            && self
                .built_in_options
                .iter()
                .map(|option| option.id.as_str())
                .eq(BUILT_IN_THEME_OPTION_IDS)
    }

    pub fn uses_color_wheel_seed_controls(&self) -> bool {
        self.custom_theme_controls
            .iter()
            .map(String::as_str)
            .eq(CUSTOM_THEME_CONTROLS)
            && self
                .custom_seed_fields
                .iter()
                .map(String::as_str)
                .eq(THEME_USER_CONTROLS)
            && !self.exposes_internal_role_color_editor
    }
}

impl CapsulePalette {
    pub fn from_theme_seed(seed: &ReaderThemeSeed) -> Self {
        let default = ReaderThemeSeed::default();
        let accent = parse_hex_color(&seed.accent).unwrap_or_else(|| {
            parse_hex_color(&default.accent).expect("default accent should parse")
        });
        let background = parse_hex_color(&seed.background).unwrap_or_else(|| {
            parse_hex_color(&default.background).expect("default background should parse")
        });
        let foreground = parse_hex_color(&seed.foreground).unwrap_or_else(|| {
            parse_hex_color(&default.foreground).expect("default foreground should parse")
        });
        let contrast = seed.contrast.min(100);
        let lens_lift = ((100 - contrast) / 4).min(18);

        Self {
            base_bg: color_to_hex(mix_colors(background, RgbColor::BLACK, 45)),
            capsule_bg: color_to_hex(background),
            lens_bg: color_to_hex(mix_colors(foreground, RgbColor::WHITE, lens_lift)),
            lens_edge: color_to_hex(mix_colors(foreground, accent, 35)),
            text_context: color_to_hex(mix_colors(foreground, background, 38)),
            text_focus: color_to_hex(mix_colors(background, RgbColor::BLACK, 35)),
            text_muted: color_to_hex(mix_colors(foreground, background, 25)),
            accent_speed: color_to_hex(accent),
            accent_progress: color_to_hex(accent),
            control_idle: color_to_hex(mix_colors(foreground, background, 15)),
            control_hover: color_to_hex(mix_colors(foreground, accent, 45)),
        }
    }

    pub fn color_for_token(&self, token: &str) -> Option<&str> {
        match token {
            "base_bg" => Some(&self.base_bg),
            "capsule_bg" => Some(&self.capsule_bg),
            "lens_bg" => Some(&self.lens_bg),
            "lens_edge" => Some(&self.lens_edge),
            "text_context" => Some(&self.text_context),
            "text_focus" => Some(&self.text_focus),
            "text_muted" => Some(&self.text_muted),
            "accent_speed" => Some(&self.accent_speed),
            "accent_progress" => Some(&self.accent_progress),
            "control_idle" => Some(&self.control_idle),
            "control_hover" => Some(&self.control_hover),
            _ => None,
        }
    }
}

impl Default for CapsulePalette {
    fn default() -> Self {
        ReaderThemeSeed::default().to_palette()
    }
}

pub fn normalize_selected_text(text: &str) -> String {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    let trimmed = normalized.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    trimmed
        .split('\n')
        .map(collapse_spaces_and_tabs)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn tokenize_words(text: &str) -> Vec<ReaderToken> {
    let mut tokens = Vec::new();
    let mut start: Option<usize> = None;

    for (index, character) in text.char_indices() {
        if character.is_whitespace() {
            if let Some(token_start) = start.take() {
                push_token(&mut tokens, text, token_start, index);
            }
        } else if start.is_none() {
            start = Some(index);
        }
    }

    if let Some(token_start) = start {
        push_token(&mut tokens, text, token_start, text.len());
    }

    tokens
}

pub fn render_plan_from_model_and_frame(
    model: CapsuleRenderModel,
    motion_frame: TapeMotionFrame,
) -> CapsuleRenderPlan {
    let width = model.geometry.width;
    let height = model.geometry.height;
    let center_x = (width / 2) as i32;
    let center_y = (height / 2) as i32;
    let lens_x = center_x - (model.geometry.lens_width / 2) as i32;
    let lens_y = center_y - (model.geometry.lens_height / 2) as i32;
    let progress_width = width.saturating_sub(OVERLAY_PADDING * 2);
    let progress_fill = ((progress_width as u32 * model.progress_percent as u32) / 100) as u16;
    let text_baseline_y = center_y
        + if model.mode == CapsuleMode::Large {
            8
        } else {
            5
        };

    let lens_rect = RenderRect {
        x: lens_x,
        y: lens_y,
        width: model.geometry.lens_width,
        height: model.geometry.lens_height,
        radius: model.geometry.lens_height / 4,
    };
    let text_clip_rect = RenderRect {
        x: OVERLAY_PADDING as i32,
        y: lens_rect.y,
        width: width.saturating_sub(OVERLAY_PADDING * 2),
        height: lens_rect.height,
        radius: 0,
    };
    let fade_height = lens_rect.height;
    let left_fade = CapsuleFadeZone {
        side: FadeZoneSide::Left,
        rect: RenderRect {
            x: text_clip_rect.x,
            y: lens_rect.y,
            width: FADE_WIDTH,
            height: fade_height,
            radius: 0,
        },
    };
    let right_fade = CapsuleFadeZone {
        side: FadeZoneSide::Right,
        rect: RenderRect {
            x: text_clip_rect.x + text_clip_rect.width as i32 - FADE_WIDTH as i32,
            y: lens_rect.y,
            width: FADE_WIDTH,
            height: fade_height,
            radius: 0,
        },
    };

    CapsuleRenderPlan {
        mode: model.mode,
        status: model.status,
        geometry: model.geometry,
        capsule_rect: RenderRect {
            x: 0,
            y: 0,
            width,
            height,
            radius: model.geometry.radius,
        },
        lens_rect,
        text_clip_rect,
        fade_zones: vec![left_fade, right_fade],
        progress_marker: CapsuleProgressMarker {
            track: RenderRect {
                x: OVERLAY_PADDING as i32,
                y: (OVERLAY_PADDING / 2) as i32,
                width: progress_width,
                height: PROGRESS_MARKER_HEIGHT,
                radius: 1,
            },
            fill: RenderRect {
                x: OVERLAY_PADDING as i32,
                y: (OVERLAY_PADDING / 2) as i32,
                width: progress_fill,
                height: PROGRESS_MARKER_HEIGHT,
                radius: 1,
            },
        },
        text_baseline_y,
        motion_frame,
        empty_message: model.empty_message,
    }
}

pub fn render_plan_svg(plan: &CapsuleRenderPlan, palette: &CapsulePalette) -> String {
    let typography = CapsuleTypographyTokens::canonical();
    let status_glyph = status_glyph(plan.status);
    let text_nodes = if plan.motion_frame.placements.is_empty() {
        let text = plan
            .empty_message
            .as_deref()
            .unwrap_or("Select text to read");
        format!(
            r##"    <text x="{center_x}" y="{text_y}" text-anchor="middle" font-family="Georgia, serif" font-size="{font_size}" font-weight="400" letter-spacing="0" opacity="{opacity}" fill="{fill}">{text}</text>"##,
            center_x = plan.motion_frame.center_x,
            text_y = plan.text_baseline_y,
            font_size = typography.context_font_size_for_mode(plan.mode),
            opacity = percent_to_opacity(typography.context_text_opacity_percent),
            fill = escape_attr(&palette.text_muted),
            text = escape_text(text),
        )
    } else {
        plan.motion_frame
            .placements
            .iter()
            .map(|placement| {
                let fill = if placement.focus_anchor {
                    &palette.text_focus
                } else {
                    &palette.text_context
                };
                let font_weight = if placement.focus_anchor { 600 } else { 400 };
                let letter_spacing = if placement.focus_anchor {
                    tenths_to_decimal(typography.focus_letter_spacing_tenths_px)
                } else {
                    "0".to_owned()
                };

                format!(
                    r##"    <text x="{x}" y="{text_y}" text-anchor="middle" font-family="Georgia, serif" font-size="{font_size}" font-weight="{font_weight}" letter-spacing="{letter_spacing}" opacity="{opacity}" fill="{fill}">{text}</text>"##,
                    x = placement.x,
                    text_y = plan.text_baseline_y,
                    font_size = placement.font_size,
                    opacity = percent_to_opacity(placement.opacity_percent),
                    fill = escape_attr(fill),
                    text = escape_text(&placement.text),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let fade_nodes = plan
        .fade_zones
        .iter()
        .map(|zone| {
            let opacity = match zone.side {
                FadeZoneSide::Left => "0.42",
                FadeZoneSide::Right => "0.32",
            };
            format!(
                r##"  <rect x="{x}" y="{y}" width="{width}" height="{height}" rx="{radius}" fill="{fill}" opacity="{opacity}"/>"##,
                x = zone.rect.x,
                y = zone.rect.y,
                width = zone.rect.width,
                height = zone.rect.height,
                radius = zone.rect.radius,
                fill = escape_attr(&palette.capsule_bg),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" role="img" aria-label="scroll reader capsule {status_glyph}">
  <defs>
    <clipPath id="scroll-reader-text-clip">
      <rect x="{clip_x}" y="{clip_y}" width="{clip_width}" height="{clip_height}"/>
    </clipPath>
  </defs>
  <rect x="{capsule_x}" y="{capsule_y}" width="{width}" height="{height}" rx="{radius}" fill="{capsule_bg}" stroke="{lens_edge}" stroke-opacity="0.34"/>
  <rect x="{progress_x}" y="{progress_y}" width="{progress_width}" height="{progress_h}" rx="{progress_radius}" fill="{base_bg}" opacity="0.45"/>
  <rect x="{progress_fill_x}" y="{progress_fill_y}" width="{progress_fill_width}" height="{progress_fill_h}" rx="{progress_fill_radius}" fill="{accent_progress}" opacity="0.9"/>
  <rect x="{lens_x}" y="{lens_y}" width="{lens_width}" height="{lens_height}" rx="{lens_radius}" fill="{lens_bg}" opacity="{lens_bg_opacity}" stroke="{lens_edge}" stroke-width="1"/>
  <g clip-path="url(#scroll-reader-text-clip)">
{text_nodes}
  </g>
{fade_nodes}
</svg>"##,
        width = plan.geometry.width,
        height = plan.geometry.height,
        capsule_x = plan.capsule_rect.x,
        capsule_y = plan.capsule_rect.y,
        radius = plan.capsule_rect.radius,
        capsule_bg = escape_attr(&palette.capsule_bg),
        lens_bg = escape_attr(&palette.lens_bg),
        lens_edge = escape_attr(&palette.lens_edge),
        base_bg = escape_attr(&palette.base_bg),
        accent_progress = escape_attr(&palette.accent_progress),
        progress_x = plan.progress_marker.track.x,
        progress_y = plan.progress_marker.track.y,
        progress_width = plan.progress_marker.track.width,
        progress_h = plan.progress_marker.track.height,
        progress_radius = plan.progress_marker.track.radius,
        progress_fill_x = plan.progress_marker.fill.x,
        progress_fill_y = plan.progress_marker.fill.y,
        progress_fill_width = plan.progress_marker.fill.width,
        progress_fill_h = plan.progress_marker.fill.height,
        progress_fill_radius = plan.progress_marker.fill.radius,
        lens_x = plan.lens_rect.x,
        lens_y = plan.lens_rect.y,
        lens_width = plan.lens_rect.width,
        lens_height = plan.lens_rect.height,
        lens_radius = plan.lens_rect.radius,
        lens_bg_opacity = percent_to_opacity(typography.lens_bg_opacity_percent),
        clip_x = plan.text_clip_rect.x,
        clip_y = plan.text_clip_rect.y,
        clip_width = plan.text_clip_rect.width,
        clip_height = plan.text_clip_rect.height,
    )
}

pub fn render_capsule_svg(model: &CapsuleRenderModel, palette: &CapsulePalette) -> String {
    let frame = TapeMotionFrame {
        focus_anchor_index: 0,
        center_x: (model.geometry.width / 2) as i32,
        progress_permille: 0,
        placements: Vec::new(),
    };
    render_plan_svg(
        &render_plan_from_model_and_frame(model.clone(), frame),
        palette,
    )
}

pub fn render_motion_svg(
    frame: &TapeMotionFrame,
    model: &CapsuleRenderModel,
    palette: &CapsulePalette,
) -> String {
    render_plan_svg(
        &render_plan_from_model_and_frame(model.clone(), frame.clone()),
        palette,
    )
}

pub fn sample_capsule() -> ScrollReaderCapsule {
    ScrollReaderCapsule::from_selected_text(
        "The quick brown fox jumps over the lazy dog. True power knowledge comes from practice.",
    )
}

fn collapse_spaces_and_tabs(line: &str) -> String {
    let mut collapsed = String::new();
    let mut previous_was_space = false;

    for character in line.chars() {
        if character == ' ' || character == '\t' {
            if !previous_was_space {
                collapsed.push(' ');
                previous_was_space = true;
            }
            continue;
        }

        collapsed.push(character);
        previous_was_space = false;
    }

    collapsed
}

fn push_token(tokens: &mut Vec<ReaderToken>, text: &str, start: usize, end: usize) {
    if start < end {
        tokens.push(ReaderToken {
            text: text[start..end].to_owned(),
            start,
            end,
        });
    }
}

fn estimate_text_width_px(text: &str, font_size: u16) -> i32 {
    let char_count = text.chars().count() as i32;
    ((char_count * font_size as i32 * 11) / 20).max(1)
}

pub fn smoothstep_permille(progress_permille: i32) -> u16 {
    let progress = clamp_permille(progress_permille) as i64;
    ((progress * progress * (3000 - 2 * progress)) / 1_000_000) as u16
}

pub fn clamp_permille(progress_permille: i32) -> u16 {
    progress_permille.clamp(
        SMOOTHSTEP_MIN_PERMILLE as i32,
        SMOOTHSTEP_MAX_PERMILLE as i32,
    ) as u16
}

fn interpolate_i32(from: i32, to: i32, progress_permille: i32) -> i32 {
    from + (((to - from) * progress_permille) / SMOOTHSTEP_MAX_PERMILLE as i32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl RgbColor {
    const BLACK: Self = Self {
        red: 0,
        green: 0,
        blue: 0,
    };
    const WHITE: Self = Self {
        red: 255,
        green: 255,
        blue: 255,
    };
}

fn parse_hex_color(value: &str) -> Option<RgbColor> {
    let hex = value.trim().strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }

    Some(RgbColor {
        red: u8::from_str_radix(&hex[0..2], 16).ok()?,
        green: u8::from_str_radix(&hex[2..4], 16).ok()?,
        blue: u8::from_str_radix(&hex[4..6], 16).ok()?,
    })
}

fn mix_colors(base: RgbColor, overlay: RgbColor, overlay_percent: u8) -> RgbColor {
    let overlay_percent = overlay_percent.min(100) as u16;
    let base_percent = 100 - overlay_percent;

    RgbColor {
        red: mix_channel(base.red, overlay.red, base_percent, overlay_percent),
        green: mix_channel(base.green, overlay.green, base_percent, overlay_percent),
        blue: mix_channel(base.blue, overlay.blue, base_percent, overlay_percent),
    }
}

fn mix_channel(base: u8, overlay: u8, base_percent: u16, overlay_percent: u16) -> u8 {
    (((base as u16 * base_percent) + (overlay as u16 * overlay_percent)) / 100) as u8
}

fn color_to_hex(color: RgbColor) -> String {
    format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
}

fn status_glyph(status: OverlayStatus) -> &'static str {
    match status {
        OverlayStatus::Playing => "pause",
        OverlayStatus::Paused => "play",
        OverlayStatus::Finished => "done",
        OverlayStatus::Closed => "closed",
    }
}

fn percent_to_opacity(percent: u8) -> String {
    if percent >= 100 {
        return "1".to_owned();
    }

    format!("0.{:02}", percent)
}

fn tenths_to_decimal(tenths: i16) -> String {
    if tenths == 0 {
        return "0".to_owned();
    }

    let sign = if tenths < 0 { "-" } else { "" };
    let abs = tenths.abs();
    format!("{sign}{}.{:01}", abs / 10, abs % 10)
}

fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(value: &str) -> String {
    escape_text(value).replace('"', "&quot;")
}
