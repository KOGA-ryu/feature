# ui.left_rail

`ui.left_rail` is a reusable left-side navigation rail feature crate for the Rust feature lab.

It owns navigation item metadata, section grouping, selection movement, collapse state, badge totals, and activation behavior. It does not navigate to real pages; activation only returns the selected enabled item id.

## Contract

- Feature id: `ui.left_rail`
- Kind: `ui_pattern`
- Status: `experimental`
- Inputs: navigation items, selected index, collapsed state
- Outputs: grouped navigation, active item id, collapsed state

## Behavior

- Items stay grouped by section in insertion order.
- Selection clamps to the visible item set.
- Collapse state hides long labels in a compact surface but keeps item identity.
- Disabled items remain visible and cannot activate.
- Empty-item state is explicit.
- Badge counts are preserved and can be aggregated for summary surfaces.

## Harness

The feature lab UI uses this crate for a small center-panel demo that exercises navigation movement, collapse toggling, badge totals, and activation without building a full application shell.
