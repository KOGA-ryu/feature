# Paste Cleanup

## Purpose

Turn damaged terminal/chat/browser text into clean editor text.

## Actions

- paste plain
- paste and normalize line endings
- clean terminal paste
- strip ANSI escape codes
- strip box borders
- unwrap soft-wrapped lines
- preserve intentional blank lines
- remove shell prompts

## User Access Pattern

- toolbar: Clean Paste
- command palette: all cleanup transforms
- paste menu: Paste, Paste Plain, Paste Clean

## State Rules

- Cleanup can produce preview text before insertion.
- Insertion is one undo step.
- Destructive cleanup should be visible in tests and preview examples.

## Tests

- box-drawing status output strips borders
- ANSI sequences are removed
- soft wraps join without adding extra spaces
- blank lines remain intentional
- shell prompts are removed only when configured
