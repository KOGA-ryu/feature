# View Controls

## Purpose

Define how hosts display editor text without changing document truth.

View controls are host state unless a setting must be serialized by a specific
application.

## Regular Applications

- make long editing sessions readable
- inspect whitespace and line endings
- work with wide code or wrapped prose
- split or preview documents in host UI

## Actions

- zoom in
- zoom out
- reset zoom
- toggle word wrap
- toggle line numbers
- toggle whitespace
- toggle invisibles
- toggle minimap
- toggle ruler or column guide
- set theme
- set font family
- set font size
- split view
- preview mode
- focus mode
- distraction-free mode

## Boundaries

The headless engine does not own rendering, fonts, themes, minimaps, rulers, or
split panes.

Hosts may store view preferences, but view state must not change document text.

## User Access Pattern

- menu: `View`
- toolbar: only common toggles such as wrap, preview, and zoom
- status bar: line/column, wrap state, encoding when available
- command palette: all view commands

## Default Hotkeys

- zoom in: `Ctrl++` Linux, `Cmd++` macOS
- zoom out: `Ctrl+-` Linux, `Cmd+-` macOS
- reset zoom: `Ctrl+0` Linux, `Cmd+0` macOS
- word wrap: host-specific, commonly `Alt+Z`; mark as `needs verification`

## Button And Icon

- zoom in: `zoom-in`
- zoom out: `zoom-out`
- word wrap: `wrap-text`
- line numbers: `list-ordered`
- whitespace: `pilcrow`
- preview: `eye`
- split view: `columns-2`

## Tests

- host adapter maps view actions without mutating document text
- unsupported view action is hidden or disabled honestly
- toggling wrap does not alter copied text
- zoom does not alter cursor/selection state

## Acceptance

View controls are accepted when they are clearly host-owned and cannot corrupt
headless text state.
