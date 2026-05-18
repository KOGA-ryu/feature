# ui.scratchpad

`ui.scratchpad` is a reusable plain-text note surface for the Rust feature lab.

## Purpose

Provide one durable text body with tiny helper methods that hosts can wire into
quick-note drawers, sidecars, or operator panels.

## V1 Behavior

- owns one plain-text note body
- supports setting and clearing text
- reports character and line counts
- keeps no external file I/O policy inside the feature itself

## Non-goals

- markdown or rich text
- document storage or export
- multiple named notes
- collaborative editing

## Intended Host / Use Cases

- sidecar operator notes in app shells
- live demo surfaces in `feature_lab_ui`
- lightweight session notes inside larger productivity tools
