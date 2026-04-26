# State Machine Template

## Purpose

Use this template to describe legal states and transitions for a feature,
workflow, or Dex session.

## When to Use

Use when:

- lifecycle correctness matters
- invalid transitions need to be explicit
- status labels drive user behavior or review flow

Avoid when:

- the feature is stateless

## Required Fields

- `states`
- `transitions`
- `triggers`
- `invalid transitions`
- `terminal states`

## Template

```md
# state machine: {subject}

## purpose

## when to use

## required fields
- states:
- transitions:
- triggers:
- invalid transitions:
- terminal states:
```

## Example

```md
# state machine: feature_maturity

## required fields
- states:
  - draft
  - experimental
  - tested
  - stable
  - deprecated
- transitions:
  - draft -> experimental
  - experimental -> tested
  - tested -> stable
```

## Dex Usage Notes

- Use exact transition arrows so Dex does not infer illegal state jumps.
- If a state is read-only or terminal, say so directly.
