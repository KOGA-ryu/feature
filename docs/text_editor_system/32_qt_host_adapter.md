# Qt Host Adapter

## Purpose

Prepare for native Qt apps that need the same editor system.

## Responsibilities

- map action ids to Qt actions/buttons
- map icon names to Qt icon resources
- map shortcut profiles to `QShortcut` or `QAction`
- keep clipboard/filesystem logic in the host layer

## Boundaries

- Do not port the editor engine into Qt-specific logic.
- Do not hardcode labels or hotkeys in every widget.
- Do not add Qt host work until the headless action registry is stable.

## Tests

- action metadata round-trips into Qt action properties
- disabled state is visible
- copy/export returns exact strings from the engine/clipboard layer
