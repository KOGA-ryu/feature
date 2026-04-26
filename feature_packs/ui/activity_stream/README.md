# ui.activity_stream

`ui.activity_stream` is a reusable activity timeline feature crate for the Rust feature lab.

It owns activity entry metadata, simple case-insensitive filtering, chronological order preservation, selection movement, unread counting, empty-state signaling, and activation rules. It does not open real artifacts; activation only returns the selected actionable entry id.

## Contract

- Feature id: `ui.activity_stream`
- Kind: `ui_pattern`
- Status: `experimental`
- Inputs: activity entries, query, selected index
- Outputs: visible entries, activated entry id, visible unread count

## Behavior

- Empty queries return all entries in their original order.
- Filtering matches title, detail, actor, kind, status, and id.
- Selection clamps to the visible entry set.
- Non-actionable entries remain visible but cannot activate.
- Empty result state is explicit.
- Visible unread count updates with filtering.

## Harness

The feature lab UI uses this crate for a small center-panel demo that exercises filtering, ordering, selection, unread summaries, and activation without building a full shell timeline.
