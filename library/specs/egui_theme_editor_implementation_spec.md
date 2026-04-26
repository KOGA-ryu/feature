# egui Theme Editor Implementation Spec

## Purpose

Turn the theme-customization blueprint into an implementation-ready packet for
the Rust feature lab, with one logic feature and one UI feature built around a
stable token contract.

## Scope

This is a spec, not a crate implementation. It defines the seam to build next.

Primary targets:

- `logic.theme_token_generator`
- `ui.theme_editor`

## Build Split

### `logic.theme_token_generator`

Owns:

- `ThemeConfig`
- `ThemeTokens`
- config validation
- token derivation
- preview model generation
- import/export schema versioning

Does not own:

- egui widgets
- application persistence layer
- app-specific component styling

### `ui.theme_editor`

Owns:

- editor layout
- color controls
- contrast slider
- toggle controls
- theme preview
- import/copy actions
- validation surface

Does not own:

- color derivation rules
- token math
- long-term storage format

## Rust Contract

### Theme config

```rust
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

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
```

### Semantic token contract

```rust
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
```

### Validation output

```rust
pub enum ThemeFindingSeverity {
    Info,
    Warning,
    Error,
}

pub struct ThemeValidationFinding {
    pub severity: ThemeFindingSeverity,
    pub field: String,
    pub message: String,
}
```

### Generator seam

```rust
pub struct ThemeGenerationResult {
    pub tokens: ThemeTokens,
    pub findings: Vec<ThemeValidationFinding>,
}

pub fn generate_theme_tokens(config: &ThemeConfig) -> ThemeGenerationResult;
```

## Derivation Rules

- parse the three editable colors first:
  - `accent`
  - `background`
  - `foreground`
- derive a surface ladder from `background`
- derive a text ladder from `foreground`
- derive interaction tokens from `accent`
- apply `contrast` as a spread parameter across ramps
- apply `translucent_sidebar` only to sidebar tokens
- produce findings for malformed input or low readability

## egui Floorplan

```text
top bar
  search presets | import | copy theme | reset

left column
  presets
  mode
  color controls
  contrast
  structural toggles
  font controls

center column
  live preview
  panel stack
  sidebar sample
  code sample
  button/focus sample

right column
  raw config
  derived token inspector
  validation findings
```

## Interaction Contract

- every control edits `ThemeConfig`
- every edit triggers token regeneration
- preview updates from `ThemeTokens`
- validation findings remain visible while editing
- disabled export/copy is not needed; export is allowed with warnings
- malformed colors should show findings and preserve the last valid preview if
  regeneration fails hard

## Minimal Preview Surfaces

The preview should include:

- app background
- left rail sample
- raised panel sample
- right inspector sample
- command button sample
- code block sample
- muted secondary text sample
- focus ring sample

The preview must consume the same token names that production components will
consume later.

## Test Checklist

### `logic.theme_token_generator`

- valid config returns deterministic tokens
- invalid hex returns findings
- low contrast returns findings
- empty fonts fall back to defaults or findings, per chosen rule
- `contrast` changes at least:
  - `panel_elevated_bg`
  - `text_secondary`
  - `border_subtle`
- `translucent_sidebar` changes sidebar tokens only

### `ui.theme_editor`

- controls update `ThemeConfig`
- preview reflects regenerated tokens
- findings render when inputs are invalid
- import populates controls
- copy/export uses current config payload
- reset restores a default preset

## Fixture Suggestions

- `fixtures/default_dark_theme.json`
- `fixtures/default_light_theme.json`
- `fixtures/low_contrast_theme.json`
- `fixtures/bad_hex_theme.json`
- `fixtures/high_accent_theme.json`

## Integration Notes

- build `logic.theme_token_generator` first
- keep `ui.theme_editor` blocked on the token contract, not on app-wide theme
  plumbing
- when integrating into an app shell, provide tokens through one read-only
  shared state seam
- avoid teaching unrelated components to derive or mutate colors locally

## Dex Usage Notes

- implement the logic crate first and prove the token contract with fixtures
- keep color math isolated and testable
- do not start with a full settings application
- do not let the UI crate invent extra token names that the logic crate does
  not emit
