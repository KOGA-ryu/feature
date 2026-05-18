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

## Non-goals

- file system I/O
- clipboard integration
- mouse editing or drag selection
- markdown, rich text, or syntax highlighting
- multi-document tabs or collaborative editing

## Intended Host / Use Cases

- keyboard-driven editor demos in `feature_lab_ui`
- document state inside command palettes, inspectors, or sidecars
- future reusable editing shells that need a headless text engine
