# Feature Blueprint Template

## Purpose

Use this template for the full buildable packet for one reusable feature. It is
the default blueprint when Dex needs identity, contract, flow, tests, and
integration rules in one place.

## When to Use

Use when:

- the feature is meant to be reused
- the feature needs both human summary and Dex build rules
- the feature has enough behavior that a simple README is too thin

Avoid when:

- you only need a narrow flow sketch
- you are documenting a single helper function

## Required Fields

- title block:
  - `id`
  - `name`
  - `category`
  - `status`
  - `version`
  - `owner`
  - `created_at`
  - `updated_at`
  - `source_inspiration`
  - `maturity`
- `purpose`
- `use when`
- `avoid when`
- `inputs`
- `outputs`
- `parts list`
- `schematic`
- `contract`
- `test bench`
- `integration rules`
- `compatibility`
- `failure modes`
- `extraction notes`

## Template

```md
# feature blueprint: {feature_id}

## title block
- id:
- name:
- category:
- status:
- version:
- owner:
- created_at:
- updated_at:
- source_inspiration:
- maturity:

## purpose

## use when

## avoid when

## inputs

## outputs

## parts list

## schematic

## contract

## test bench

## integration rules

## compatibility
- works with:
- conflicts with:

## failure modes

## extraction notes
```

## Example

```md
# feature blueprint: ui.command_palette

## title block
- id: ui.command_palette
- name: command_palette
- category: ui_pattern
- status: experimental
- version: 0.1.0
- owner: feature_lab
- created_at: 2026-04-26T00:00:00Z
- updated_at: 2026-04-26T00:00:00Z
- source_inspiration: Codex command surfaces
- maturity: component

## purpose
Provide searchable command selection with grouped results and safe activation.

## use when
Global actions or navigational commands need a compact discovery surface.

## avoid when
A fixed toolbar is enough and no search or keyboard selection is needed.
```

## Dex Usage Notes

- Treat this as the master packet when implementing a new feature crate.
- Keep the contract section decision-complete.
- If a field is unknown, mark it explicitly instead of letting Dex guess.
