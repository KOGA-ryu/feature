# logic.session_notes

`logic.session_notes` is a reusable headless note ledger for lightweight
session notes inside operator tools and workbenches.

It supports:

- trimmed note creation
- deterministic insertion order
- selected note tracking against a filtered view
- note text updates
- pin and unpin actions
- delete actions with safe selection clamping
- visibility counts and latest-entry helpers

It does not do persistence, markdown, tagging, background time, or file I/O in
v1.

## Contract

- Feature id: `logic.session_notes`
- Kind: `logic_pattern`
- Status: `tested`
- Inputs: entries, query, selection, note actions
- Outputs: visible entries, selected entry, counts, latest entry

## Ledger Law

- blank or whitespace-only adds are ignored
- blank updates are rejected
- new entries append to the end
- toggling `pinned` does not reorder entries
- query matching is case-insensitive against note text and `created_at`
- selection is always clamped against the visible filtered result set
- latest entry follows insertion order, not pin state

## Harness / Integration Note

This crate is intentionally headless-first. Hosts can adapt it with a thin note
panel, capture inbox, or session sidebar later without changing the core state
model.
