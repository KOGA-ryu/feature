# ui.text_editor_plain

`ui.text_editor_plain` is a reusable plain-text editor core for the Rust
feature lab.

## Purpose

Provide one in-memory multi-line document model that hosts can adapt into real
editing surfaces without making egui or another UI toolkit the source of truth.

## Current Scope

- multi-line plain-text document state
- cursor and anchor/caret selection state
- insert, newline, backspace, and forward-delete editing
- arrow, home, and end navigation
- undo/redo with bounded history
- dirty-state tracking against a clean baseline
- pure copy/export helpers for selected text or full-document fallback
- line lookup helpers for current, single-line, and line-range reads
- basic text cleanup for normalized line endings and trimmed trailing whitespace

## Non-goals

- file system I/O
- system clipboard integration
- mouse editing or drag selection
- markdown, rich text, or syntax highlighting
- multi-document tabs or collaborative editing

## Copy / Export Helpers

The crate exposes headless helpers that return strings only. Hosts can put those
strings on a clipboard later, but this crate does not call the system clipboard.

- `selected_text_or_all()`: selected text, or full document when no text is selected
- `copy_plain()`: exact selected-or-all text
- `copy_markdown_block(language)`: selected-or-all text wrapped in a Markdown fence
- `copy_prompt_block(source)`: source line plus fenced exact text for AI prompts
- `current_line_text()`, `line_text(index)`, `line_range_text(start, end)`: line reads
- `trim_trailing_whitespace_text(input)`: pure cleanup helper for imported text

## Intended Host / Use Cases

- keyboard-driven editor demos in `feature_lab_ui`
- document state inside command palettes, inspectors, or sidecars
- future reusable editing shells that need a headless text engine
