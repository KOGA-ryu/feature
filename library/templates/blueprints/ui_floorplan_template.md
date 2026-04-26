# UI Floorplan Template

## Purpose

Use this template to describe UI layout, adjacency, and navigation structure
without dropping into implementation code.

## When to Use

Use when:

- panel placement matters
- users move between distinct surfaces
- operator attention routing depends on layout

Avoid when:

- the problem is mainly data flow or validation logic

## Required Fields

- `surface name`
- `primary operator goal`
- `panels or rooms`
- `adjacency`
- `navigation paths`
- `layout constraints`
- `focus order`
- `responsive notes`
- `keep-out zones`

## Template

```md
# ui floorplan: {surface_id}

## purpose

## when to use

## required fields
- surface name:
- primary operator goal:
- panels:
- adjacency:
- navigation paths:
- layout constraints:
- focus order:
- responsive notes:
- keep-out zones:
```

## Example

```md
# ui floorplan: feature_lab_ui

## purpose
Provide a test-bench showroom for browsing and proving features.

## required fields
- panels:
  - left rail: feature list
  - center: demo or docs
  - right inspector: metadata detail
  - bottom stream: test output and activity log
- navigation paths:
  - left rail -> center demo
  - left rail -> right inspector
- layout constraints:
  - bottom panel must stay available for proof output
```

## Dex Usage Notes

- Use this before editing panel-heavy UI crates.
- Keep the layout symbolic; do not over-specify pixel polish unless the feature
  truly depends on it.
