# Error And Empty State Policy

## Purpose

Define how editor actions behave when the document, selection, cursor, or host
state is empty, invalid, or unsupported.

Builders should not invent per-action failure behavior.

## Result Rules

- Pure read/export actions should return an explicit empty output when empty
  input is valid.
- Mutating actions should be no-op only when the action contract says no-op is
  safe and expected.
- Invalid ranges must not panic.
- Unsupported host actions must be disabled or reported as unsupported.
- Errors must be deterministic and testable.

## Empty Document

Expected behavior:

- copy full document returns an empty string
- selected-or-full export returns an empty string
- search returns no matches
- select all creates an empty selection or cursor-only state, as specified by
  the selection contract
- undo on empty history is a safe no-op or explicit no-op result

## Empty Selection

Expected behavior:

- selected-or-full export uses the full document
- copy selection returns an empty string unless the action explicitly falls back
- formatting commands insert wrappers or no-op only if the action contract says
  so
- delete selection no-ops
- replace selection inserts at cursor

## Invalid Cursor Or Range

V1 should prefer validation before mutation.

Allowed responses:

- clamp to valid document boundary when documented
- return structured error without mutating text
- reject the command before it reaches the engine

Not allowed:

- panic
- silently drop text
- create invalid UTF-8
- move cursor outside document bounds

## Unsupported Host Features

Host adapters must not fake support.

- unsupported actions are hidden or disabled
- disabled actions should have an honest tooltip where possible
- missing system clipboard access must not mutate document state
- missing formatter/language server must not block headless editor use

## Tests

- empty document copy/export
- empty selection copy/export
- invalid range rejection
- undo/redo empty history
- unsupported host action disabled mapping
- no mutation after failed command

## Acceptance

Empty and error states are accepted when they are boring, deterministic, and
covered by exact tests.
