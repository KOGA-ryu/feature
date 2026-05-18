# Spark Code Review Rubric

## Purpose

This rubric grades Spark code slices for the text editor package before they are
accepted as reusable feature-library code.

A passing build is not enough. A Spark patch must match the assigned contract,
stay in scope, prove behavior, and leave the next slice easier.

## Review Verdicts

Use one of these review statuses:

- `ACCEPT`: ready to keep and build on.
- `ACCEPT_WITH_NOTES`: usable now, with minor follow-up notes that do not block
  the next slice.
- `CHANGES_REQUIRED`: do not build the next slice until the required fixes land.
- `REJECT_AND_REBUILD`: patch is too far from the contract or too risky to
  salvage as the base.

## Hard Gates

If any hard gate fails, the review status is automatically
`CHANGES_REQUIRED` or `REJECT_AND_REBUILD`.

- Scope: touched only files allowed by the Spark order.
- Contract: implemented the assigned spec without invented behavior.
- Build: the target crate or workspace builds as required by the order.
- Tests: required tests pass and cover the new behavior.
- No pollution: no JSONL writes, generated junk, unrelated repo changes, or
  binder data changes unless explicitly assigned.
- No hidden coupling: no host UI, filesystem, backend, global state, or
  persistence coupling added to a headless crate unless explicitly assigned.
- Readable diff: reviewer can understand the change without archaeology.

## Score Model

Score only after hard gates pass.

| Category | Points | What It Means |
| --- | ---: | --- |
| Contract fidelity | 25 | Matches the assigned docs, action names, boundaries, and V1 scope. |
| Functional correctness | 20 | Behavior is correct for normal use and preserves editor state. |
| Tests and proof | 20 | Tests cover exact text, cursor, selection, output, and undo expectations. |
| Architecture cleanliness | 15 | Code stays headless/core where required and keeps host adapters thin. |
| Reuse/readability | 10 | API and implementation are simple, named clearly, and reusable by future hosts. |
| Edge/error handling | 5 | Empty input, empty selection, Unicode/newline cases, and invalid options are handled honestly. |
| Review hygiene | 5 | Review bucket states changed files, commands, risks, and next slice clearly. |

Suggested grade bands:

- `95-100`: A, promote-ready.
- `85-94`: B, good with minor cleanup.
- `70-84`: C, functional but not yet a pattern.
- `50-69`: D, salvage only parts.
- `0-49`: F, reject or rebuild.

## Required Review Shape

Every Spark code review must include:

```text
review_status:
grade:
score:
changed_files:
allowed_scope_match:
contract_match:
tests_run:
test_result:
proof_artifacts:
hard_gate_failures:
risks:
required_fixes:
recommended_followups:
recommended_next_slice:
reviewer_notes:
```

Use `none` for empty fields. Do not omit fields.

## Text Editor Specific Rules

### Headless First

`text_editor_plain` is the document truth engine. It may own text state,
cursor/selection state, undo/redo state, dirty state, and pure string helpers.

It must not own:

- UI widgets
- clipboard system calls
- filesystem save dialogs
- host-specific menus
- Qt or egui imports
- background workers
- persistence outside its own in-memory document state

### Action Metadata Before Host Invention

Host UI behavior should come from the action registry contract once
`text_editor_actions` exists. A host adapter may render actions, but it must not
silently rename actions, invent hotkeys, or change semantics.

### Golden Behavior

Headless tests are the source of truth. Screenshots are useful for host adapter
proof, but screenshots do not replace exact string/state tests.

Required test evidence for editor behavior:

- input fixture
- action or command
- expected text
- expected cursor
- expected selection
- expected output string when applicable
- undo/redo expectation when mutating
- host profile expectation only when UI-facing

## First Code Slice Gate

The first code slice after the docs passes is:

```text
text_editor_plain copy/export helpers
```

That slice should add only headless helpers and contract tests for exact text
output. It should not create host UI, clipboard system integration, or a new
crate.

Minimum expected behavior:

- selected-or-full text output
- line/current-line text helpers if specified by the docs
- markdown block output if specified by the docs
- prompt/context block output if specified by the docs
- exact whitespace and newline preservation tests
- empty selection behavior test
- empty document behavior test

Automatic `CHANGES_REQUIRED` for the first code slice:

- helper output changes document state
- helper inserts visual-wrap newlines into copied text
- helper drops trailing newlines without the spec saying so
- helper adds UI or system clipboard calls to `text_editor_plain`
- tests assert only that strings are non-empty instead of exact output

## Example Reviews

### Accept

```text
review_status: ACCEPT
grade: A-
score: 92
changed_files: feature_packs/ui/text_editor_plain/src/lib.rs; feature_packs/ui/text_editor_plain/tests/contract_tests.rs
allowed_scope_match: yes
contract_match: yes
tests_run: cargo test -p text_editor_plain
test_result: passed
proof_artifacts: none
hard_gate_failures: none
risks: markdown block fence language is plain text only in V1
required_fixes: none
recommended_followups: add richer prompt block metadata in clipboard crate
recommended_next_slice: text_editor_actions registry crate
reviewer_notes: headless boundary held; exact output tests are readable
```

### Changes Required

```text
review_status: CHANGES_REQUIRED
grade: C
score: 76
changed_files: feature_packs/ui/text_editor_plain/src/lib.rs; crates/feature_lab_ui/src/app.rs
allowed_scope_match: no
contract_match: partial
tests_run: cargo test -p text_editor_plain
test_result: passed
proof_artifacts: none
hard_gate_failures: touched host UI outside assigned scope
risks: host behavior now depends on unfinished helper names
required_fixes: revert host UI edit; add empty selection exact-output test
recommended_followups: move UI work to host adapter slice
recommended_next_slice: repeat current copy/export helper slice after fixes
reviewer_notes: behavior is close, but scope failed
```

## Acceptance

A Spark patch is accepted only when the review can say:

- the assigned contract was implemented
- the headless/host boundary was preserved
- exact tests prove the behavior
- no unrelated files were changed
- the next slice is clearer than before
