# ChatGPT Coding Window Bootstrap

Use this when opening a fresh ChatGPT coding window against the feature lab.

## Goal

Make the new coding window self-sufficient before it edits code.

## Startup Sequence

Run:

```bash
pwd
git branch --show-current
git status --short
```

Then read:

1. `/Users/kogaryu/dev/features/AGENTS.md`
2. `/Users/kogaryu/dev/features/README.md`
3. `/Users/kogaryu/dev/features/docs/feature_contract.md`
4. `/Users/kogaryu/dev/features/docs/testing_ground_rules.md`
5. `/Users/kogaryu/dev/features/docs/chatgpt_feature_packets.md`
6. `/Users/kogaryu/dev/features/docs/terminal_first_proof.md`

## What The Repo Is

This repo is not a generic docs wiki.

It is a Rust workspace where each reusable feature lives in its own crate under
`feature_packs/`.

Core pieces:

- `crates/feature_core`: manifest parsing and shared types
- `crates/feature_registry`: filesystem-based feature discovery
- `crates/feature_cli`: `list`, `show`, and `test`
- `crates/feature_lab_ui`: browse and optional smoke-test harness
- `feature_packs/`: one crate per feature

## Choose The Right Mode

Use worker mode when the task is:

- one feature crate
- one isolated contract
- no shared workspace edits yet

Use integrator mode when the task is:

- add workspace membership
- refresh `Cargo.lock`
- optionally wire a requested harness demo branch

Use reviewer mode when the task is:

- read-only proof
- run workspace commands
- report findings and risks

Default proof rule:

- prove features in the terminal first
- use `feature_lab_ui` only when the task explicitly needs a live harness smoke

## Worker Acceptance Checklist

Before stopping, the coding window should ensure:

- the feature crate exists with all required files
- `feature.toml` matches repo conventions
- fixtures parse
- contract tests exist
- `cargo test --manifest-path .../Cargo.toml` passes
- shared files are untouched unless the task explicitly allowed them

## Integration Acceptance Checklist

Before stopping, the coding window should ensure:

- root `Cargo.toml` includes the new crate if integration was requested
- `Cargo.lock` is refreshed if workspace membership changed
- `cargo fmt --all --check` passes
- `cargo test --workspace` passes
- `cargo run -p feature_cli -- show <feature_id>` passes
- `cargo run -p feature_cli -- test <feature_id>` passes
- `git diff --check` passes

## Fresh-Window Prompt Stub

Use this as the opening message in a coding window when starting a new feature:

```text
Work in /Users/kogaryu/dev/features.

Before editing run:
- pwd
- git branch --show-current
- git status --short

Then read:
- AGENTS.md
- README.md
- docs/feature_contract.md
- docs/testing_ground_rules.md
- docs/chatgpt_feature_packets.md

After that, implement the assigned feature using worker mode unless the task explicitly says integrator or reviewer.
```
