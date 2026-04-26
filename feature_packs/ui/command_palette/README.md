# ui.command_palette

`ui.command_palette` is a reusable command palette feature crate for the Rust feature lab.

It owns command item metadata, simple case-insensitive filtering, category grouping, selection clamping, disabled-command handling, and activation results. It does not execute commands; activation only returns the selected enabled command id.

## Contract

- Feature id: `ui.command_palette`
- Kind: `ui_pattern`
- Status: `experimental`
- Inputs: query, command items, selected index
- Outputs: filtered commands, grouped results, activated command id

## Behavior

- Empty queries return all commands, including disabled ones.
- Filtering matches title, subtitle, category, id, and keywords.
- Grouped results preserve original command order within each category.
- Selection clamps to the visible result set.
- Disabled commands remain visible and cannot activate.
- Empty result state is explicit.

## Harness

The feature lab UI uses this crate for a small center-panel demo that exercises search, selection movement, grouping, and activation without building a full popup surface.
