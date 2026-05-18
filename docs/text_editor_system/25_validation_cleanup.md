# Validation Cleanup

## Purpose

Detect risky or messy text and provide safe cleanup actions.

Validation should report facts first. Cleanup should be explicit and testable.

## Regular Applications

- avoid broken JSON, YAML, TOML, and Markdown
- catch leaked secrets before sharing text
- remove trailing whitespace
- normalize line endings
- detect duplicate lines and suspicious characters

## Actions

- spellcheck
- grammar check
- validate markdown
- validate JSON
- validate YAML
- validate TOML
- validate links
- detect duplicate lines
- warn on long lines
- detect invalid characters
- detect secrets or tokens
- detect encoding problems
- trim trailing whitespace
- normalize line endings
- normalize blank lines

## V1 Boundary

Early cleanup helpers may be pure string transforms.

Validators that require external parsers, dictionaries, network access, or
secret scanners belong in later crates or host integrations.

## User Access Pattern

- status strip: validation summary
- side panel: issue list
- command palette: run validation, apply cleanup
- toolbar: only compact warning indicator

## Default Hotkeys

- validate document: host-specific
- apply cleanup: host-specific
- command palette access is preferred in V1

## Button And Icon

- validate: `shield-check`
- warning: `triangle-alert`
- cleanup: `sparkles`
- secrets: `key-round`

## State Rules

- Validation reports do not mutate text.
- Cleanup actions are mutating and must be undoable.
- Cleanup must show or document exactly what it changes.

## Tests

- trailing whitespace cleanup exact output
- line ending normalization exact output
- duplicate line detection locations
- long line warning threshold
- secret/token detection fixtures use fake tokens only
- validation failure does not mutate text

## Acceptance

Validation cleanup is accepted when fact detection and mutation are separate,
with exact tests for every cleanup transform.
