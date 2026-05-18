# Undo Redo History

## Purpose

Make editing safe by keeping reversible document changes.

Undo/redo belongs in the headless engine because every host needs the same
history behavior.

## Regular Applications

- undo accidental typing
- redo reverted changes
- group a paste as one undo step
- revert to saved state
- track whether the document is dirty

## Actions

- undo
- redo
- clear redo after new edit
- group edit step
- mark clean
- revert to clean snapshot
- report undo depth
- report redo depth

## User Access Pattern

- menu: `Edit / Undo`, `Edit / Redo`
- toolbar: curved-arrow icons when space allows
- command palette: `Undo`, `Redo`
- context menu: optional, host dependent

## Default Hotkeys

- Linux desktop editor: `Ctrl+Z`, `Ctrl+Shift+Z`
- Linux terminal-like host: `Ctrl+Z` may be reserved; use host profile override
- macOS: `Cmd+Z`, `Cmd+Shift+Z`
- AI prompt box: match host convention when embedded

## Button And Icon

- undo: `undo-2`
- redo: `redo-2`

## State Rules

- Mutating text commands create undo entries.
- Pure copy/export commands do not create undo entries.
- Selection-only commands should not create content undo entries in V1.
- A new edit after undo clears redo history.
- History limit must be explicit and testable.

## Tests

- undo insert text
- redo insert text
- undo replace selection
- undo paste as one grouped step
- redo cleared after new edit
- dirty state changes after edit
- mark clean resets dirty state without losing text
- history limit does not panic

## Acceptance

Undo/redo is accepted when exact tests prove text, cursor, selection, dirty
state, undo depth, and redo depth after each step.
