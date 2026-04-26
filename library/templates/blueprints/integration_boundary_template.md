# Integration Boundary Template

## Purpose

Use this template to lock what a feature may connect to and what it must not
touch.

## When to Use

Use when:

- integration risk is higher than implementation risk
- a worker may need to stop before touching shared files
- the repo has shared choke points

Avoid when:

- there is no real boundary or ownership concern

## Required Fields

- `allowed connections`
- `forbidden connections`
- `ownership boundaries`
- `shared choke points`
- `escalation path`

## Template

```md
# integration boundary: {feature_id}

## purpose

## when to use

## required fields
- allowed connections:
- forbidden connections:
- ownership boundaries:
- shared choke points:
- escalation path:
```

## Example

```md
# integration boundary: logic.search_index

## required fields
- allowed connections:
  - feature_packs/logic/search_index/**
- forbidden connections:
  - Cargo.toml
  - crates/feature_lab_ui/**
- shared choke points:
  - root workspace members
  - harness demo wiring
- escalation path:
  - emit shared-change packet for integrator
```

## Dex Usage Notes

- Use this before parallel work.
- If a needed file is in a forbidden zone, stop and escalate instead of
  improvising.
