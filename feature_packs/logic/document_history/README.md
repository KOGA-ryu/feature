# logic.document_history

`logic.document_history` is a reusable headless revision ledger for one
in-memory document inside operator tools and workbenches.

It supports:

- one working document text buffer
- deterministic snapshot recording
- selected revision tracking
- restoring working text from a selected revision
- revision counts and clean-baseline dirty tracking

It does not do persistence, file I/O, branching history, diff generation, or
background time in v1.

## Contract

- Feature id: `logic.document_history`
- Kind: `logic_pattern`
- Status: `tested`
- Inputs: working text, revisions, selection, history actions
- Outputs: working text, selected revision, counts, dirty state

## History Law

- one document only
- blank working text is allowed
- first snapshot is allowed even if it matches the clean baseline
- unchanged snapshot against the latest recorded revision is a no-op
- snapshots preserve insertion order
- restore updates `working_text` and keeps history intact
- selection is always clamped against the revision list
- dirty compares `working_text` against `clean_text`

## Harness / Integration Note

This crate is intentionally headless-first. Hosts can adapt it into editor
sidebars, review receipts, or draft-history panels later without changing the
core state model.
