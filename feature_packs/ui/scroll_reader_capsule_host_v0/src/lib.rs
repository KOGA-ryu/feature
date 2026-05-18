use std::{
    fs,
    path::{Path, PathBuf},
};

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use scroll_reader_capsule_v1::{
    CapsuleMode, CapsuleRenderPlan, OverlayStatus, ReaderInputPayload, ScrollReaderCapsule,
};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.scroll_reader_capsule_host_v0";
pub const SAMPLE_TEXT: &str = "The quick brown fox jumps over the lazy dog while the scroll reader keeps one focus word centered.";
pub const HOST_WINDOW_TITLE: &str = "Scroll Reader Capsule";
pub const HOST_RUNTIME_SURFACE: &str = "pill_only";
pub const HOST_LAUNCH_PATH: &str = "command_line_or_sample_text";
pub const HOST_REPOSITION_BEHAVIOR: &str = "click_drag_capsule_to_move_window";
pub const HOST_PREFERENCES_FILE_NAME: &str = "preferences.json";
pub const HOST_PREFERENCES_VERSION: u8 = 1;

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_host_session_fixture() -> &'static str {
    include_str!("../fixtures/sample_host_session.json")
}

pub fn sample_host_session_contract() -> Result<HostSessionFixture, String> {
    serde_json::from_str(sample_host_session_fixture()).map_err(|error| error.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSessionFixture {
    pub mode: CapsuleMode,
    pub sample_text: String,
    pub launch_path: String,
    pub clipboard_load: String,
    pub reposition: String,
    pub remembers_position: bool,
    pub remembers_mode: bool,
    pub desktop_shortcut_registered: bool,
    pub runtime_surface: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostWindowPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostWindowPreferences {
    pub version: u8,
    pub mode: CapsuleMode,
    pub window_position: Option<HostWindowPosition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardLoadDecision {
    Accepted,
    IgnoredEmpty,
    IgnoredClosed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardLoadReport {
    pub decision: ClipboardLoadDecision,
    pub status_label: String,
    pub token_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostCommand {
    TogglePlay,
    Close,
    StepForward,
    StepBackward,
    SpeedUp,
    SlowDown,
    ToggleMode,
    Tick(u64),
    LoadClipboardText,
    Wheel {
        angle_delta_y: i32,
        pixel_delta_y: i32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostKeyboardContract {
    pub load_clipboard: String,
    pub toggle_mode: String,
    pub play_pause: String,
    pub close: String,
    pub speed_up: String,
    pub slow_down: String,
    pub wheel: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostPointerContract {
    pub reposition: String,
    pub wheel: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleHostSession {
    capsule: ScrollReaderCapsule,
    mode: CapsuleMode,
    launch_path: String,
    desktop_shortcut_registered: bool,
}

impl Default for HostKeyboardContract {
    fn default() -> Self {
        Self {
            load_clipboard: "Cmd+V or Ctrl+V".to_owned(),
            toggle_mode: "M".to_owned(),
            play_pause: "Space".to_owned(),
            close: "Esc".to_owned(),
            speed_up: "Up".to_owned(),
            slow_down: "Down".to_owned(),
            wheel: "mouse wheel or trackpad".to_owned(),
        }
    }
}

impl Default for HostPointerContract {
    fn default() -> Self {
        Self {
            reposition: HOST_REPOSITION_BEHAVIOR.to_owned(),
            wheel: "mouse wheel or trackpad".to_owned(),
        }
    }
}

impl Default for CapsuleHostSession {
    fn default() -> Self {
        Self::from_text(SAMPLE_TEXT)
    }
}

impl Default for HostWindowPreferences {
    fn default() -> Self {
        Self {
            version: HOST_PREFERENCES_VERSION,
            mode: CapsuleMode::Large,
            window_position: None,
        }
    }
}

impl HostWindowPreferences {
    pub fn new(mode: CapsuleMode, window_position: Option<HostWindowPosition>) -> Self {
        Self {
            version: HOST_PREFERENCES_VERSION,
            mode,
            window_position,
        }
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, String> {
        let text = fs::read_to_string(path.as_ref()).map_err(|error| error.to_string())?;
        let preferences: Self = serde_json::from_str(&text).map_err(|error| error.to_string())?;
        if preferences.version != HOST_PREFERENCES_VERSION {
            return Err(format!(
                "unsupported preferences version {}",
                preferences.version
            ));
        }
        Ok(preferences)
    }

    pub fn load_or_default(path: impl AsRef<Path>) -> Self {
        Self::load_from_path(path).unwrap_or_default()
    }

    pub fn save_to_path(&self, path: impl AsRef<Path>) -> Result<(), String> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        let text = serde_json::to_string_pretty(self).map_err(|error| error.to_string())?;
        fs::write(path.as_ref(), format!("{text}\n")).map_err(|error| error.to_string())
    }

    pub fn default_path() -> PathBuf {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("scroll_reader_capsule_host_v0")
                .join(HOST_PREFERENCES_FILE_NAME);
        }

        std::env::temp_dir()
            .join("scroll_reader_capsule_host_v0")
            .join(HOST_PREFERENCES_FILE_NAME)
    }
}

impl CapsuleHostSession {
    pub fn from_text(text: impl Into<String>) -> Self {
        Self {
            capsule: ScrollReaderCapsule::from_input_payload(ReaderInputPayload::selected_text(
                text.into(),
            )),
            mode: CapsuleMode::Large,
            launch_path: HOST_LAUNCH_PATH.to_owned(),
            desktop_shortcut_registered: false,
        }
    }

    pub fn from_args(args: impl IntoIterator<Item = String>) -> Self {
        let text = args
            .into_iter()
            .skip(1)
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_owned();
        if text.is_empty() {
            Self::default()
        } else {
            Self::from_text(text)
        }
    }

    pub fn apply_command(&mut self, command: HostCommand) {
        match command {
            HostCommand::TogglePlay => self.capsule.toggle_play(),
            HostCommand::Close => self.capsule.close(),
            HostCommand::StepForward => self.capsule.step_by(1),
            HostCommand::StepBackward => self.capsule.step_by(-1),
            HostCommand::SpeedUp => self
                .capsule
                .adjust_speed(scroll_reader_capsule_v1::SPEED_STEP_WORDS_PER_MINUTE),
            HostCommand::SlowDown => self
                .capsule
                .adjust_speed(-scroll_reader_capsule_v1::SPEED_STEP_WORDS_PER_MINUTE),
            HostCommand::ToggleMode => self.toggle_mode(),
            HostCommand::Tick(elapsed_ms) => self.capsule.tick(elapsed_ms),
            HostCommand::LoadClipboardText => {}
            HostCommand::Wheel {
                angle_delta_y,
                pixel_delta_y,
            } => {
                self.capsule.scrub_wheel(angle_delta_y, pixel_delta_y);
            }
        }
    }

    pub fn load_clipboard_text(&mut self, text: impl Into<String>) -> ClipboardLoadReport {
        if self.is_closed() {
            return ClipboardLoadReport {
                decision: ClipboardLoadDecision::IgnoredClosed,
                status_label: "capsule is closed".to_owned(),
                token_count: self.capsule.tokens().len(),
            };
        }

        let text = text.into();
        if text.trim().is_empty() {
            return ClipboardLoadReport {
                decision: ClipboardLoadDecision::IgnoredEmpty,
                status_label: "clipboard text is empty".to_owned(),
                token_count: self.capsule.tokens().len(),
            };
        }

        self.capsule
            .load_input_payload(ReaderInputPayload::clipboard_snapshot(text));
        ClipboardLoadReport {
            decision: ClipboardLoadDecision::Accepted,
            status_label: "loaded clipboard text".to_owned(),
            token_count: self.capsule.tokens().len(),
        }
    }

    pub fn render_plan(&self) -> CapsuleRenderPlan {
        self.capsule.render_plan(self.mode)
    }

    pub fn apply_preferences(&mut self, preferences: &HostWindowPreferences) {
        self.mode = preferences.mode;
    }

    pub fn preferences_with_position(
        &self,
        window_position: Option<HostWindowPosition>,
    ) -> HostWindowPreferences {
        HostWindowPreferences::new(self.mode, window_position)
    }

    pub fn mode(&self) -> CapsuleMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: CapsuleMode) {
        self.mode = mode;
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            CapsuleMode::Large => CapsuleMode::Compact,
            CapsuleMode::Compact => CapsuleMode::Large,
        };
    }

    pub fn status(&self) -> OverlayStatus {
        self.capsule.status()
    }

    pub fn current_index(&self) -> usize {
        self.capsule.current_index()
    }

    pub fn words_per_minute(&self) -> u32 {
        self.capsule.words_per_minute()
    }

    pub fn is_closed(&self) -> bool {
        self.status() == OverlayStatus::Closed
    }

    pub fn desktop_shortcut_registered(&self) -> bool {
        self.desktop_shortcut_registered
    }

    pub fn launch_path(&self) -> &str {
        &self.launch_path
    }

    pub fn capsule(&self) -> &ScrollReaderCapsule {
        &self.capsule
    }
}
