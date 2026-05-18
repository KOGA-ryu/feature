# Collaboration Review

## Purpose

Define review-oriented editor features without making collaboration state part
of the core text engine too early.

These features are useful for specs, packets, and code review, but most are
later-phase host or app features.

## Regular Applications

- comment on selected text
- suggest edits
- compare versions
- review changes before accepting
- preserve author and receipt metadata

## Actions

- add comment
- resolve comment
- suggested edit
- accept suggestion
- reject suggestion
- track changes
- diff view
- compare versions
- review annotation
- show author attribution
- show change history

## V1 Boundary

V1 does not implement collaborative editing.

Allowed early planning:

- metadata shape for selected text references
- copy selected text with source metadata
- future action IDs and host placement

Host or app-owned:

- comment storage
- author identity
- sync
- track changes
- version history

## User Access Pattern

- right context panel: comments and suggestions
- inline marker: comment anchors
- toolbar: accept/reject when reviewing
- command palette: add comment, show changes

## Default Hotkeys

- add comment: host-specific
- accept/reject suggestion: host-specific
- diff navigation: host-specific

Mark all collaboration hotkeys as `needs verification` before coding.

## Button And Icon

- comment: `message-circle`
- suggest edit: `pencil-line`
- accept: `check`
- reject: `x`
- diff: `git-compare`
- history: `history`

## Tests

- selected text reference survives simple non-mutating copy
- suggestion model does not mutate source text until accepted
- diff view is read-only unless explicit action applies a change

## Acceptance

Collaboration review is accepted only when review records are explicit,
auditable, and separate from headless document truth.
