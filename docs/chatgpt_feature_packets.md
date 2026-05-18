# ChatGPT Feature Packets

These are copy-paste packets for a fresh ChatGPT coding window.

For multi-feature queues that should run one feature at a time, end-to-end, use
[docs/sequential_dex_prompt_packets.md](/Users/kogaryu/dev/features/docs/sequential_dex_prompt_packets.md).

For parallel multi-worker waves, use
[docs/parallel_dex_prompt_packets.md](/Users/kogaryu/dev/features/docs/parallel_dex_prompt_packets.md).

Default proof policy:

- terminal-first through `cargo` and `feature_cli`
- `feature_lab_ui` only when the task explicitly requires a live harness smoke

## Isolated Feature Worker Packet

```text
Implement one isolated feature in the Rust feature lab.

Repo:
/Users/kogaryu/dev/features

Before editing run:
- pwd
- git branch --show-current
- git status --short

Then read:
- AGENTS.md
- README.md
- docs/feature_contract.md
- docs/testing_ground_rules.md

Feature:
{feature_id}

Location:
feature_packs/{category}/{feature_name}

Allowed writes:
- /Users/kogaryu/dev/features/feature_packs/{category}/{feature_name}/**

Forbidden writes:
- /Users/kogaryu/dev/features/Cargo.toml
- /Users/kogaryu/dev/features/Cargo.lock
- /Users/kogaryu/dev/features/crates/**
- any other feature_packs path outside feature_packs/{category}/{feature_name}

Goal:
Create the feature as an isolated crate with metadata, docs, fixtures, logic, and contract tests.

Required files:
- Cargo.toml
- feature.toml
- README.md
- src/lib.rs
- tests/contract_tests.rs
- fixtures/*

Rules:
- do not edit shared files
- do not wire the feature into the workspace or harness
- if shared changes are required, stop and emit the shared-change escalation packet
- keep the feature self-contained and testable in isolation
- do not modify unrelated features

Verification:
- cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/{category}/{feature_name}/Cargo.toml

Final report:
1. files changed
2. tests added
3. commands run
4. tests passed/failed
5. shared changes needed, if any
6. known limitations
```

## Shared-Change Escalation Packet

```text
Shared-change escalation.

Repo:
/Users/kogaryu/dev/features

Feature:
{feature_id}

Blocked file:
{shared_file_path}

Why it is needed:
{one short paragraph}

Minimal required change:
{exact narrow change}

What I did not edit:
- shared files
- other feature crates
```

## Integration Packet

```text
Integrate a completed feature in /Users/kogaryu/dev/features.

Before editing run:
- pwd
- git branch --show-current
- git status --short

Then read:
- AGENTS.md
- README.md
- docs/chatgpt_coding_window_bootstrap.md

Feature to integrate:
{feature_id}

You own shared files only:
- /Users/kogaryu/dev/features/Cargo.toml
- /Users/kogaryu/dev/features/Cargo.lock

Optional shared files only if explicitly required by the task:
- /Users/kogaryu/dev/features/crates/feature_lab_ui/**

Rules:
- do not redesign the feature crate
- patch the feature crate only if a compile fix is absolutely required, and explain why
- add workspace membership once
- preserve isolated feature contracts

Verification:
- cargo fmt --all --check
- cargo test --workspace
- cargo run -p feature_cli -- show {feature_id}
- cargo run -p feature_cli -- test {feature_id}
- git diff --check

Final report:
1. shared files changed
2. integration fixes applied
3. commands run
4. tests passed/failed
5. unresolved blockers
```

## Reviewer Packet

```text
Review an integrated feature in /Users/kogaryu/dev/features.

Do not modify files.

Feature:
{feature_id}

Run:
- cargo fmt --all --check
- cargo test --workspace
- cargo run -p feature_cli -- show {feature_id}
- cargo run -p feature_cli -- test {feature_id}
- git diff --check

Optional manual smoke only if the task explicitly depends on the harness:
- scripts/launch_feature_lab_ui_app.sh

Report:
1. findings by severity
2. commands run
3. tests passed/failed
4. known risks
5. merge recommendation
```

## Ready Example: `ui.wiki_browser`

```text
Implement one isolated feature in the Rust feature lab.

Repo:
/Users/kogaryu/dev/features

Before editing run:
- pwd
- git branch --show-current
- git status --short

Then read:
- AGENTS.md
- README.md
- docs/feature_contract.md
- docs/testing_ground_rules.md

Feature:
ui.wiki_browser

Location:
feature_packs/ui/wiki_browser

Allowed writes:
- /Users/kogaryu/dev/features/feature_packs/ui/wiki_browser/**

Forbidden writes:
- /Users/kogaryu/dev/features/Cargo.toml
- /Users/kogaryu/dev/features/Cargo.lock
- /Users/kogaryu/dev/features/crates/**
- any other feature_packs path outside feature_packs/ui/wiki_browser

Goal:
Create a browseable UI feature crate for viewing registered features, README previews, fixture previews, and manifest metadata. Keep it self-contained first.

Required files:
- Cargo.toml
- feature.toml
- README.md
- src/lib.rs
- tests/contract_tests.rs
- fixtures/*

Rules:
- do not edit shared files
- do not wire the feature into the workspace or harness yet
- if shared changes are required, stop and emit the shared-change escalation packet
- keep the feature self-contained and testable in isolation
- do not modify unrelated features

Verification:
- cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/wiki_browser/Cargo.toml
```
