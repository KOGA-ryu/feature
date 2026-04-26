# Blueprint Teardown

A blueprint is not a picture of a thing. It is a compressed instruction system
for building the thing correctly.

For the Rust feature lab, a blueprint should tell Dex and humans:

- what the feature is
- where it belongs
- what it connects to
- what it depends on
- what can change
- what must not change
- how to prove it works

## What a Blueprint Must Carry

A useful blueprint carries several layers of instruction at once:

- `shape`: the visible layout or arrangement
- `relationships`: what connects to what
- `constraints`: limits, scale, allowed boundaries
- `sequence`: what gets built first and what depends on it
- `authority`: ownership, revision, approval state
- `symbols`: compressed meaning through standard labels
- `layers`: separate views for UI, logic, data, state, tests, and integration

## Feature Blueprint Mapping

In this repo, a feature blueprint maps to:

- `identity`: id, name, category, version, owner, timestamps
- `purpose`: what problem the feature solves
- `inputs`: data, config, and state needed to use it
- `outputs`: results, side effects, or returned state
- `dependencies`: required crates, patterns, or adjacent features
- `layout`: panel or surface arrangement when UI exists
- `logic flow`: how data or events move through it
- `states`: selection, lifecycle, error, or status behavior
- `tests`: fixture-driven proof commands and assertions
- `fixtures`: input materials for repeatable checks
- `risks`: known failure modes and tricky constraints
- `reuse notes`: what should be extracted and reused later
- `integration rules`: where the feature may and may not connect

## Core Blueprint Concepts

### Title Block

The title block tells the builder what they are looking at and whether it can be
trusted.

Use these fields when possible:

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

### Legend

Dex should never guess what kind of item it is reading. Use standard feature
type labels:

- `ui_pattern`
- `logic_pattern`
- `workflow_pattern`
- `component_contract`
- `app_archetype`
- `build_recipe`
- `review_packet`
- `teardown_reference`
- `data_model`
- `test_pattern`

### Scale

Use conceptual scale so Dex does not confuse a helper with a subsystem:

- `micro`: helper or small function
- `component`: reusable UI or logic module
- `surface`: full panel or page
- `workflow`: multi-step process
- `system`: app-wide architecture

### Layers

Separate blueprint concerns instead of mixing them into one blob:

- `intent layer`
- `ui layer`
- `logic layer`
- `data layer`
- `state layer`
- `test layer`
- `integration layer`
- `reuse layer`

This repo should preserve layer boundaries. For example, `logic.search_index`
should not require random edits to `feature_lab_ui` just to explain its logic.

### Callouts

Use callouts for small but dangerous behavior decisions.

Example:

```text
Callout: disabled command activation
Disabled command items remain visible in results, but activation returns None.
```

### Section Cuts

A section cut exposes hidden structure. In software, section cuts are the
internal views:

- state flow
- data flow
- event flow
- dependency flow
- error flow
- test flow

### Revision Table

Blueprints need revision history so old doctrine does not masquerade as current
law.

Use:

- `revision`
- `date`
- `change`
- `reason`
- `breaking_change`
- `migration_notes`

## The Five Feature Views

Every reusable feature should be describable through five views:

1. `card view`: human summary
2. `contract view`: build rules for Dex
3. `schematic view`: data, dependency, or state flow
4. `test bench view`: fixtures, commands, and expected proof
5. `integration view`: allowed connections and keep-out zones

## Feature Lab Mapping

The current repo already maps well to blueprint thinking:

- `feature_packs/` = isolated rooms
- `crates/feature_core` = foundation slab
- `crates/feature_registry` = building directory
- `crates/feature_cli` = inspection tool
- `crates/feature_lab_ui` = test bench showroom
- `docs/` = permit office
- `library/` = archive and warehouse

Inside each feature crate:

- `feature.toml` = title block
- `README.md` = card view
- `src/lib.rs` = machinery
- `tests/contract_tests.rs` = inspection checklist
- `fixtures/` = test materials

## Why This Exists

Without a shared blueprint language, every new feature invents its own format.
That slows reuse, weakens Dex prompts, and makes the library harder to search,
validate, and extend safely.
