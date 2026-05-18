# Terminal-First Proof

This repo defaults to terminal proof, not live harness clicking.

## Rule

- prove feature behavior through crate tests, workspace tests, and `feature_cli`
- treat `feature_lab_ui` as optional manual smoke only
- only require live UI interaction when:
  - the task explicitly asks for it
  - the feature contract depends on the harness surface itself
  - terminal proof passes but a manual smoke is needed to investigate a harness-only risk

## Canonical Proof Ladder

Worker-local proof:

```bash
cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/<category>/<feature_name>/Cargo.toml
```

Shared integration proof:

```bash
cargo fmt --all --check
cargo test --workspace
cargo run -p feature_cli -- list
cargo run -p feature_cli -- show <feature_id>
cargo run -p feature_cli -- test <feature_id>
git diff --check
```

## Optional Harness Smoke

Run this only when the task truly needs it:

```bash
scripts/launch_feature_lab_ui_app.sh
```

Use the Feature Lab app to confirm:

- the feature appears in registry/browser views
- README and fixture previews render
- any explicitly requested demo surface is reachable

Do not turn routine feature work into an accessibility-automation task when the
terminal already proves the contract.

## Why

This repo is built around:

- headless-first feature crates
- explicit contracts
- deterministic tests
- searchable CLI discovery

The harness is valuable, but it is downstream of the contract. The contract
must stay provable without a flaky live UI loop.
