# logic.hit_resolution

`logic.hit_resolution` is the reusable combat-resolution layer for applying one
already-selected hit request to one actor.

## Purpose

Provide a deterministic, headless hit application rule set for:

- damage
- invulnerability checks
- hitstun
- knockdown
- crash state entry
- defeat outcomes

## V1 Behavior

- works over `logic.actor_state`
- no geometry or target-finding
- no physics engine coupling
- deterministic outcome record per hit request

## Non-goals

- hitbox overlap
- combo scoring
- crowd AI
- knockback physics

## Intended Host Pattern

Use this crate after a host or simulation layer has already decided which actor
was hit. It owns the deterministic state transition, not the collision search.
