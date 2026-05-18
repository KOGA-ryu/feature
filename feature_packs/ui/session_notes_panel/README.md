# ui.session_notes_panel

`ui.session_notes_panel` is a reusable host-state adapter for richer operator
notes flows in the Rust feature lab.

## Purpose

Adapt the existing note ledger, plain-text editor core, and document revision
history into one panel-oriented state object without moving storage ownership
away from `logic.session_notes`.

## Current Scope

- browse and filter session notes
- compose a new note through a dedicated editor surface
- edit the selected note through a dedicated editor surface
- pin and delete selected notes through the ledger owner
- record and restore revision snapshots for the active selected-note edit session

## Non-goals

- file system I/O
- markdown or rich text
- multi-document editing
- external sync or persistence beyond the underlying reusable crates
- changing `logic.session_notes` into a UI-owned storage model

## Intended Host / Use Cases

- side panels and operator workbenches that need richer note editing
- session-review shells that want compose plus edit history
- future document-aware note hosts that still keep note storage and editor logic
  separated
