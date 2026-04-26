# Test Bench Template

## Purpose

Use this template to specify fixtures, proof commands, and expected assertions
for a feature or workflow.

## When to Use

Use when:

- the feature needs repeatable proof
- fixtures drive the behavior
- Dex must know what "done" means

Avoid when:

- the work is only narrative documentation

## Required Fields

- `fixtures`
- `commands`
- `expected outputs`
- `failure probes`
- `required assertions`

## Template

```md
# test bench: {feature_id}

## purpose

## when to use

## required fields
- fixtures:
- commands:
- expected outputs:
- failure probes:
- required assertions:
```

## Example

```md
# test bench: logic.validation_pipeline

## required fields
- fixtures:
  - valid_input.json
  - invalid_input.json
- commands:
  - cargo test -p validation_pipeline
- required assertions:
  - valid fixture passes
  - invalid fixture emits multiple findings
  - severity ordering is stable
```

## Dex Usage Notes

- Prefer explicit fixture names and exact proof commands.
- Include at least one success case and one failure case.
