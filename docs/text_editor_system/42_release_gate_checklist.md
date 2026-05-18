# Release Gate Checklist

## Purpose

Define when a text editor feature slice is ready to become part of the reusable
Features library.

This checklist is the final gate after implementation review.

## Required Before Accept

- assigned spec doc exists
- action contract exists for user-visible behavior
- implementation stays inside allowed files
- headless behavior is separated from host behavior
- exact tests pass
- edge cases are tested or explicitly deferred
- README or feature docs explain the behavior
- review bucket records the verdict
- no unrelated files changed

## Feature Crate Gate

For a new crate, require:

- `Cargo.toml`
- `feature.toml`
- `README.md`
- `src/lib.rs`
- `tests/`
- `fixtures/` when exact examples are needed
- workspace membership only when the crate is ready to build
- feature inventory update only in the integration slice

## Headless Behavior Gate

Require exact tests for:

- text output
- cursor state
- selection state
- dirty state when relevant
- undo/redo state when mutating
- empty document behavior
- empty selection behavior
- Unicode/newline behavior where relevant

## Host Adapter Gate

Require proof that:

- host renders actions from metadata
- labels/tooltips/hotkeys come from the action registry
- unsupported actions are hidden or disabled honestly
- screenshots prove layout only
- headless tests prove behavior

## Security Gate

Require:

- no real secrets in fixtures
- redaction behavior tested when added
- generated artifacts kept out of source unless intentionally committed
- no JSONL writes unless explicitly assigned by a future workflow

## Review Gate

Use `41_spark_code_review_rubric.md`.

Accepted statuses:

- `ACCEPT`
- `ACCEPT_WITH_NOTES` only if follow-up notes do not block the next slice

Blocking statuses:

- `CHANGES_REQUIRED`
- `REJECT_AND_REBUILD`

## Acceptance

A release gate passes when a reviewer can say the feature is documented,
tested, reusable, bounded, and safe to build on.
