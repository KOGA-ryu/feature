# Draft And File State

## Purpose

Keep editing safe before real filesystem integration.

## Actions

- new draft
- load draft text
- mark clean
- dirty-state check
- export draft
- restore unsaved draft
- save through host adapter

## Boundaries

The headless engine does not own filesystem I/O. Hosts own open/save dialogs,
stale-write checks, autosave policy, and permissions.

## Tests

- loading text resets undo history
- mark clean updates dirty baseline
- export does not mark dirty
- host save errors do not mutate document state
