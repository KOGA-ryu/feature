# Dex Lane PCB Routing Template

## Purpose

Use this template to define parallel Dex ownership with explicit keep-out zones
and controlled handoff points.

## When to Use

Use when:

- more than one Dex writes code in parallel
- shared files create merge risk
- a coordinator needs exact lane ownership

Avoid when:

- only one implementer is writing

## Required Fields

- `lanes`
- `allowed writes`
- `forbidden zones`
- `transfer points`
- `merge checkpoints`
- `review points`

## Template

```md
# dex lane routing: {wave_name}

## purpose

## when to use

## required fields
- lanes:
- allowed writes:
- forbidden zones:
- transfer points:
- merge checkpoints:
- review points:
```

## Example

```md
# dex lane routing: five_worker_wave

## required fields
- lanes:
  - W1: feature_packs/logic/search_index/**
  - W2: feature_packs/workflows/review_packet_flow/**
  - INT: Cargo.toml and feature_lab_ui shared files
- forbidden zones:
  - workers may not edit Cargo.toml
  - workers may not edit crates/feature_lab_ui/**
- review points:
  - cargo test --workspace after integrator merge
```

## Dex Usage Notes

- Treat forbidden zones as electrical keep-out areas.
- Transfer shared-file pressure to the integrator instead of crossing lanes.
