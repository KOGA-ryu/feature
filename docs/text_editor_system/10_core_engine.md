# Core Engine

## Purpose

The core engine owns document text, cursor, selection, edit commands, dirty
state, and undo/redo.

## Regular Applications

- notes
- prompt staging
- command palette inputs
- sidecar editors
- game/dev tool consoles
- app-specific document fields

## Current State

`ui.text_editor_plain` already provides the first engine:

- multi-line text
- cursor and selection
- insert/delete/navigation commands
- select all
- undo/redo
- dirty tracking

## Headless API

The engine should remain UI-free. It may expose helpers for reading text and
creating export strings, but it must not own clipboard, filesystem, or widget
logic.

## Tests

- exact command-to-state tests
- unicode-safe cursor movement
- selection bounds
- undo/redo history
- dirty-state transitions
