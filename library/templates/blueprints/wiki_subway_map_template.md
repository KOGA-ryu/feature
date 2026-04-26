# Wiki Subway Map Template

## Purpose

Use this template to describe workflows or documentation navigation as route
clarity rather than physical layout.

## When to Use

Use when:

- a user needs to know how work moves
- multiple documents or feature types connect through handoffs
- navigation is more important than exact structure

Avoid when:

- panel layout or file ownership is the main question

## Required Fields

- `stations`
- `lines`
- `transfer stations`
- `closed stations`
- `terminal outputs`

## Template

```md
# subway map: {workflow_name}

## purpose

## when to use

## required fields
- stations:
- lines:
- transfer stations:
- closed stations:
- terminal outputs:
```

## Example

```md
# subway map: feature_build_flow

## required fields
- stations:
  - idea intake
  - app archetype
  - build recipe
  - spec
  - prompt
  - feature build
  - review packet
  - extraction
- transfer stations:
  - spec
  - review packet
```

## Dex Usage Notes

- Use route order, not directory order.
- Mark blocked or deprecated stations explicitly so Dex does not route through
  dead doctrine.
