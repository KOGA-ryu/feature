# Fixture Example Policy

## Purpose

Keep examples, fixtures, and golden tests organized so text editor behavior can
be proven by exact input/output instead of screenshots or vibes.

## Fixture Principles

- Fixtures must be small.
- Fixtures must be readable.
- Fixtures must test one behavior unless they are integration fixtures.
- Expected output should be exact.
- Sensitive data must be fake.
- Line endings and trailing whitespace must be intentional.

## Suggested Fixture Families

- plain text
- markdown
- code block
- terminal transcript
- Codex status-style output
- soft-wrapped terminal output
- empty document
- Unicode text
- CRLF text
- tab-indented text
- redaction examples with fake secrets

## Naming Rules

Use boring names:

```text
fixtures/copy_plain_input.txt
fixtures/copy_plain_expected.txt
fixtures/terminal_soft_wrap_input.txt
fixtures/terminal_soft_wrap_expected.txt
fixtures/prompt_block_input.md
fixtures/prompt_block_expected.md
```

## Test Record Shape

Every golden test should state:

- fixture input
- action
- options
- expected text
- expected cursor
- expected selection
- expected dirty state
- expected undo/redo state when mutating
- expected errors or warnings

## Example Policy

Docs may include short inline examples. Longer examples should become fixtures
once code exists.

Examples should show:

- before
- action
- after
- why the behavior matters

## Tests

- fixture loads successfully
- expected output matches exactly
- no real secret-like values are present
- trailing whitespace tests make whitespace visible in test names or comments
- CRLF fixtures are not silently normalized by the test harness

## Acceptance

Fixture policy is accepted when another builder can add a new behavior test
without deciding where examples live or how expected output should be named.
