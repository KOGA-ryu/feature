# Golden Tests

## Purpose

Define when a text editor feature is ready for reuse.

## Minimum Golden Standard

- exact input fixture
- command/action
- expected text
- expected cursor
- expected selection
- expected output string when applicable
- undo/redo expectation when mutating
- host profile expectation when UI-facing

## Test Families

- text input
- navigation
- selection
- copy/export
- paste cleanup
- search/replace
- markdown formatting
- AI prompt helpers
- terminal cleanup
- host adapter mapping

## Acceptance

A feature is golden when the tests prove both behavior and the public action
contract. Screenshots are useful for hosts but do not replace headless tests.
