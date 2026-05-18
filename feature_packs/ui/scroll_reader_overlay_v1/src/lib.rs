use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.scroll_reader_overlay_v1";
pub const DEFAULT_WORDS_PER_MINUTE: u32 = 420;
pub const MIN_WORDS_PER_MINUTE: u32 = 120;
pub const MAX_WORDS_PER_MINUTE: u32 = 900;
pub const SPEED_STEP_WORDS_PER_MINUTE: i32 = 30;
pub const DEFAULT_WHEEL_PIXEL_STEP: i32 = 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlayStatus {
    Idle,
    Playing,
    Paused,
    Finished,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderToken {
    pub index: usize,
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
    pub status: OverlayStatus,
    pub words_per_minute: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrollReaderOverlay {
    normalized_text: String,
    tokens: Vec<ReaderToken>,
    current_index: usize,
    status: OverlayStatus,
    words_per_minute: u32,
    elapsed_ms_accumulator: u64,
    wheel_scrubber: WheelScrubber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WheelScrubber {
    pixel_step: i32,
    accumulator_y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SampleInput {
    pub selected_text: String,
    pub words_per_minute: u32,
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

impl Default for ScrollReaderOverlay {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollReaderOverlay {
    pub fn new() -> Self {
        Self {
            normalized_text: String::new(),
            tokens: Vec::new(),
            current_index: 0,
            status: OverlayStatus::Idle,
            words_per_minute: DEFAULT_WORDS_PER_MINUTE,
            elapsed_ms_accumulator: 0,
            wheel_scrubber: WheelScrubber::default(),
        }
    }

    pub fn from_selected_text(selected_text: &str) -> Self {
        let mut reader = Self::new();
        reader.load_selected_text(selected_text);
        reader
    }

    pub fn load_selected_text(&mut self, selected_text: &str) {
        self.normalized_text = normalize_selected_text(selected_text);
        self.tokens = tokenize_words(&self.normalized_text);
        self.current_index = 0;
        self.elapsed_ms_accumulator = 0;
        self.wheel_scrubber = WheelScrubber::default();
        self.status = if self.tokens.is_empty() {
            OverlayStatus::Idle
        } else {
            OverlayStatus::Paused
        };
    }

    pub fn play(&mut self) {
        if self.status == OverlayStatus::Closed || self.tokens.is_empty() {
            return;
        }
        if self.status == OverlayStatus::Finished {
            self.current_index = 0;
        }
        self.status = OverlayStatus::Playing;
    }

    pub fn pause(&mut self) {
        if self.status == OverlayStatus::Playing {
            self.status = OverlayStatus::Paused;
        }
    }

    pub fn toggle_play(&mut self) {
        match self.status {
            OverlayStatus::Playing => self.pause(),
            OverlayStatus::Idle | OverlayStatus::Paused | OverlayStatus::Finished => self.play(),
            OverlayStatus::Closed => {}
        }
    }

    pub fn close(&mut self) {
        self.status = OverlayStatus::Closed;
    }

    pub fn adjust_speed(&mut self, delta_words_per_minute: i32) {
        let next = self.words_per_minute as i32 + delta_words_per_minute;
        self.words_per_minute =
            next.clamp(MIN_WORDS_PER_MINUTE as i32, MAX_WORDS_PER_MINUTE as i32) as u32;
    }

    pub fn speed_up(&mut self) {
        self.adjust_speed(SPEED_STEP_WORDS_PER_MINUTE);
    }

    pub fn slow_down(&mut self) {
        self.adjust_speed(-SPEED_STEP_WORDS_PER_MINUTE);
    }

    pub fn scrub_wheel(&mut self, angle_delta_y: i32, pixel_delta_y: i32) -> isize {
        let steps = self.wheel_scrubber.consume(angle_delta_y, pixel_delta_y);
        if steps != 0 {
            self.step_by(steps);
        }
        steps
    }

    pub fn step_by(&mut self, steps: isize) {
        if self.tokens.is_empty() || self.status == OverlayStatus::Closed || steps == 0 {
            return;
        }

        let last_index = self.tokens.len() - 1;
        if steps > 0 {
            let requested = self.current_index.saturating_add(steps as usize);
            if requested >= last_index {
                self.current_index = last_index;
                if requested > last_index {
                    self.status = OverlayStatus::Finished;
                }
            } else {
                self.current_index = requested;
                if self.status == OverlayStatus::Finished {
                    self.status = OverlayStatus::Paused;
                }
            }
            return;
        }

        let rewind = steps.unsigned_abs();
        self.current_index = self.current_index.saturating_sub(rewind);
        if self.status == OverlayStatus::Finished {
            self.status = OverlayStatus::Paused;
        }
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
            if self.status == OverlayStatus::Finished || self.current_index == previous_index {
                self.status = OverlayStatus::Finished;
                self.elapsed_ms_accumulator = 0;
                break;
            }
        }
    }

    pub fn viewport(&self, fade_radius: usize) -> ReaderViewport {
        let lens = self.current_token().map(|token| token.text.clone());
        let before_start = self.current_index.saturating_sub(fade_radius);
        let before_fade = self.tokens[before_start..self.current_index]
            .iter()
            .map(|token| token.text.clone())
            .collect();
        let after_end = self
            .current_index
            .saturating_add(fade_radius + 1)
            .min(self.tokens.len());
        let after_fade = if self.tokens.is_empty() {
            Vec::new()
        } else {
            self.tokens[self.current_index + 1..after_end]
                .iter()
                .map(|token| token.text.clone())
                .collect()
        };

        ReaderViewport {
            before_fade,
            lens,
            after_fade,
            current_index: self.current_index,
            token_count: self.tokens.len(),
            status: self.status,
            words_per_minute: self.words_per_minute,
        }
    }

    pub fn current_token(&self) -> Option<&ReaderToken> {
        self.tokens.get(self.current_index)
    }

    pub fn normalized_text(&self) -> &str {
        &self.normalized_text
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
        let wpm = self.words_per_minute.max(1) as u64;
        (60_000 / wpm).max(1)
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
    let mut token_start: Option<usize> = None;

    for (index, character) in text.char_indices() {
        if character.is_whitespace() {
            if let Some(start) = token_start.take() {
                push_token(&mut tokens, text, start, index);
            }
            continue;
        }

        if token_start.is_none() {
            token_start = Some(index);
        }
    }

    if let Some(start) = token_start {
        push_token(&mut tokens, text, start, text.len());
    }

    tokens
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_input.json")
}

pub fn sample_input() -> Result<SampleInput, String> {
    serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())
}

pub fn sample_overlay() -> Result<ScrollReaderOverlay, String> {
    let input = sample_input()?;
    let mut overlay = ScrollReaderOverlay::from_selected_text(&input.selected_text);
    overlay.words_per_minute = input
        .words_per_minute
        .clamp(MIN_WORDS_PER_MINUTE, MAX_WORDS_PER_MINUTE);
    Ok(overlay)
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
    if start >= end {
        return;
    }

    tokens.push(ReaderToken {
        index: tokens.len(),
        text: text[start..end].to_owned(),
        start,
        end,
    });
}
