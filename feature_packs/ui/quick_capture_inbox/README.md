# `ui.quick_capture_inbox`

`ui.quick_capture_inbox` is a headless quick-capture queue for operator ideas, follow-ups, and rough notes that need lightweight status tracking.

## Purpose

Provide a small inbox state model that supports draft capture, filtering, selection, and simple status progression without reordering captured items.

## V1 Behavior

- plain-text capture with trimmed blank rejection
- append-only item insertion order
- filtered visibility by text, timestamp, or status
- explicit `pending`, `processed`, and `archived` item states
- stable selection clamping across filters and deletes

## Non-goals

- persistence
- tags, categories, or multi-inbox support
- drag reordering
- rich text or markdown

## Intended Host Pattern

Use this crate as the state engine behind a thin operator inbox panel in `feature_lab_ui` or another shell that needs fast capture and simple triage.
