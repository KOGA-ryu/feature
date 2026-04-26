# Logic Flow Schematic Template

## Purpose

Use this template to describe how inputs move through transforms, guards, and
outputs inside a logic feature.

## When to Use

Use when:

- the feature is primarily a pipeline or decision flow
- data validation or transformation is the main job
- Dex needs to know order of operations

Avoid when:

- layout is the main concern

## Required Fields

- `inputs`
- `transforms`
- `guards`
- `outputs`
- `failure points`
- `test points`

## Template

```md
# logic flow: {feature_id}

## purpose

## when to use

## required fields
- inputs:
- transforms:
- guards:
- outputs:
- failure points:
- test points:
```

## Example

```md
# logic flow: spec_generation_pipeline

## required fields
- inputs:
  - app idea
  - matching library items
- transforms:
  - search library
  - filter incompatible patterns
  - assemble selected parts
- guards:
  - reject missing required inputs
  - reject incompatible pairings
- outputs:
  - generated spec
```

## Dex Usage Notes

- Prefer explicit ordered steps over prose paragraphs.
- Mark guards separately so Dex does not bury validation rules inside transforms.
