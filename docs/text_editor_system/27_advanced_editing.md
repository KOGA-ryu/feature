# Advanced Editing

## Purpose

Define power-user editing tools that make repeated changes fast.

Advanced editing must not enter V1 until the core editor behavior and action
registry are stable.

## Regular Applications

- bulk edit repeated text
- transform casing
- reorder lines
- insert repeated sequences
- wrap or surround selections
- perform repeated actions through command palette or macros

## Actions

- command palette
- keyboard shortcut editor
- macros
- multi-cursor editing
- regex transforms
- case conversion
- sort lines
- reverse lines
- unique lines
- number sequence insertion
- wrap selected text
- surround selection with quotes
- surround selection with brackets
- surround selection with tags
- expand selection
- shrink selection
- transpose characters
- transpose lines
- move line up
- move line down

## V1 Boundary

V1 may include simple line movement or surround helpers only if the core docs
and tests specify exact behavior.

Later-phase features:

- macros
- multi-cursor editing
- shortcut editor
- regex transforms
- structural expand/shrink selection

## User Access Pattern

- command palette first
- menu: `Edit` or `Transform`
- toolbar: avoid exposing advanced actions as permanent buttons unless the host
  is built for power editing
- context menu: surround and transform selected text

## Default Hotkeys

- move line up/down: host-specific
- duplicate line: often `Ctrl+D` in some editors, but conflicts by host
- command palette: host-specific

Mark advanced default hotkeys as `needs verification` before coding.

## Button And Icon

- command palette: `command`
- macro: `play`
- sort: `arrow-down-a-z`
- transform: `case-sensitive`
- move line: `arrow-up-down`
- surround: `brackets`

## Tests

- case conversion exact output
- sort lines exact output
- move line preserves selection or defines how selection moves
- surround selection handles empty selection honestly
- macro behavior is not implemented until deterministic replay rules exist

## Acceptance

Advanced editing is accepted when each transform is deterministic, undoable, and
available through action metadata rather than hidden host behavior.
