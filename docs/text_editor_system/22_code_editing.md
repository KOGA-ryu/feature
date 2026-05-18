# Code Editing

## Purpose

Define programming-oriented editor features without turning the text editor
engine into an IDE.

Most code intelligence is future work or host-provided. The headless editor may
support text-safe primitives such as indentation and comment toggling once they
are specified.

## Regular Applications

- edit Rust, config, scripts, and markdown-adjacent code blocks
- keep indentation stable
- comment and uncomment lines
- format text through a host-provided formatter
- surface diagnostics supplied by another tool

## Actions

- syntax highlighting
- auto-indent
- format document
- format selection
- comment line
- uncomment line
- toggle line comment
- block comment
- bracket matching
- auto-close brackets
- snippets
- autocomplete
- go to definition
- find references
- hover docs
- diagnostics
- code actions
- rename symbol

## V1 Boundary

V1 does not implement IDE intelligence.

Allowed early primitives:

- indentation helpers
- comment toggles for configured line comment markers
- bracket/quote surround helpers if covered by tests

Host or future crates own:

- language server calls
- formatter execution
- diagnostics
- symbol search
- autocomplete
- refactors

## User Access Pattern

- menu: `Code`, `Edit`, or host-specific command palette
- toolbar: avoid crowding; code actions usually belong in a palette or gutter
- right-click: comment, format selection, code action where available

## Default Hotkeys

- comment line: `Ctrl+/` on Linux, `Cmd+/` on macOS
- format document: host-specific, commonly `Shift+Alt+F` or formatter command
- go to definition: host-specific, commonly `F12`

Mark host-specific behavior as `needs verification` before coding.

## Button And Icon

- comment: `message-square`
- format: `wand-sparkles` or `align-left`
- diagnostics: `circle-alert`
- code action: `lightbulb`

## Tests

- line comment toggles selected lines
- indentation helper preserves existing text
- no language-server behavior in headless crate
- host-provided formatter errors do not mutate text unless accepted

## Acceptance

Code editing is accepted only when headless primitives stay pure and advanced
IDE behavior is isolated behind host or future adapter contracts.
