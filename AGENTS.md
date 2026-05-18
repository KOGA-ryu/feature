# AGENTS

This repo is a Rust feature lab at `/Users/kogaryu/dev/features`.

The unit of work is one feature crate. Do not build loose feature files at repo
root.

## Start Here

Before editing anything, run:

```bash
pwd
git branch --show-current
git status --short
```

Then read:

1. `README.md`
2. `docs/feature_contract.md`
3. `docs/testing_ground_rules.md`
4. `docs/chatgpt_coding_window_bootstrap.md`
5. `docs/chatgpt_feature_packets.md`
6. `docs/sequential_dex_prompt_packets.md`
7. `docs/terminal_first_proof.md`

If `pwd` is not `/Users/kogaryu/dev/features`, switch repos before doing any
work.

## Repo Rule

One feature, one crate, one contract, one test ground, one demo surface only
when explicitly requested.

Default behavior:

- worker tasks edit only one feature crate under `feature_packs/**`
- shared files stay untouched unless the task explicitly includes integration
- do not modify unrelated features
- do not redesign existing generators or the harness unless the task requires it

## Feature Crate Contract

Every feature crate must include:

- `Cargo.toml`
- `feature.toml`
- `README.md`
- `src/lib.rs`
- `tests/contract_tests.rs`
- `fixtures/`

Required `feature.toml` metadata:

- `id`
- `name`
- `kind`
- `status`
- `summary`
- `inputs`
- `outputs`
- `dependencies`
- `compatible_features`
- `tags`
- `owner`
- `created_at`
- `updated_at`

## Worker Mode

Use worker mode when building one feature.

Allowed writes:

- `feature_packs/<category>/<feature_name>/**`

Forbidden writes by default:

- `/Users/kogaryu/dev/features/Cargo.toml`
- `/Users/kogaryu/dev/features/Cargo.lock`
- `/Users/kogaryu/dev/features/crates/**`
- any other `feature_packs` path outside the assigned feature crate

If a shared change is truly required, stop and report:

- blocked file
- why it is needed
- minimal required change
- what you did not edit

Do not make the shared change silently.

## Integrator Mode

Use integrator mode only after worker completion.

Default shared ownership:

- `/Users/kogaryu/dev/features/Cargo.toml`
- `/Users/kogaryu/dev/features/Cargo.lock`

Only touch `crates/feature_lab_ui/**` when the task explicitly requires harness
demo wiring.

Do not redesign worker crates during integration. Patch a worker crate only for
a compile-fix emergency and explain why.

## Verification

Proof is terminal-first by default.

- use `cargo` and `feature_cli` as the canonical acceptance path
- treat `feature_lab_ui` as optional manual smoke only
- only require live harness interaction when the task explicitly depends on it

Worker-local proof for an isolated crate:

```bash
cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/<category>/<feature_name>/Cargo.toml
```

Workspace proof after integration:

```bash
cargo fmt --all --check
cargo test --workspace
cargo run -p feature_cli -- show <feature_id>
cargo run -p feature_cli -- test <feature_id>
git diff --check
```

Optional harness smoke for accessibility automation and Computer Use targeting:

```bash
scripts/launch_feature_lab_ui_app.sh
```

This builds and launches a disposable repo-local bundle at:

- `/Users/kogaryu/dev/features/target/macos-app/Feature Lab.app`

Canonical automation identifiers:

- preferred app name: `Feature Lab`
- fallback bundle id: `dev.kogaryu.feature-lab`

## Useful Paths

- `docs/chatgpt_coding_window_bootstrap.md`: fresh-window startup packet
- `docs/terminal_first_proof.md`: canonical proof policy for terminal-first work
- `docs/chatgpt_feature_packets.md`: copy-paste worker, integrator, and reviewer prompts
- `docs/sequential_dex_prompt_packets.md`: sequential multi-feature packet patterns
- `docs/parallel_dex_prompt_packets.md`: parallel multi-agent packet patterns
- `docs/roadmap.md`: repo roadmap and next backlog items
