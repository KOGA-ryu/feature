# Component Exploded View Template

## Purpose

Use this template to separate a feature into parts before implementation or
refactor.

## When to Use

Use when:

- the feature has multiple internal modules
- you need a parts list and assembly order
- Dex must not confuse storage, state, fixtures, and tests

Avoid when:

- the feature is only one small function

## Required Fields

- `parts list`
- `responsibilities`
- `connectors`
- `assembly order`
- `replaceable modules`

## Template

```md
# exploded view: {feature_id}

## purpose

## when to use

## required fields
- parts list:
- responsibilities:
- connectors:
- assembly order:
- replaceable modules:
```

## Example

```md
# exploded view: ui.command_palette

## required fields
- parts list:
  - command item
  - filter engine
  - grouping state
  - selection state
  - activation result
  - fixture set
  - contract tests
- assembly order:
  - load items
  - filter
  - group
  - clamp selection
  - activate
```

## Dex Usage Notes

- Use this to split a feature into safe implementation steps.
- Keep parts aligned with actual crate seams, not imaginary future abstractions.
