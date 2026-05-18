# ui.wiki_browser

`ui.wiki_browser` is a reusable browser-state feature crate for the Rust feature
lab.

It owns feature entry metadata, simple case-insensitive filtering, selection
clamping, and selected-preview access for manifests, README text, fixture text,
and feature paths. It does not render an egui surface and it does not discover
the filesystem outside the `feature_registry` adapter seam.

## Contract

- Feature id: `ui.wiki_browser`
- Kind: `ui_pattern`
- Status: `experimental`
- Inputs: browser entries, query, filters, selected index
- Outputs: visible entries, selected manifest, selected preview assets

## Behavior

- Empty queries return all entries in their original order.
- Query matching is case-insensitive over feature id, name, summary, and tags.
- Kind and status filters are exact enum filters.
- Tag filtering is exact and case-insensitive.
- Selection clamps to the visible entry set whenever query or filters change.
- Empty result state is explicit.
- Registry integration is limited to adapting `RegisteredFeature` values into
  browser entries.

## Harness / Integration Note

This crate is intentionally isolated. A later integrator can add it to the root
workspace and replace the current harness-local browse state with this reusable
contract.
