# Parallel Dex Prompt Packets

Use these packets when running a `5`-worker wave in this repo.

## Coordinator Kickoff Packet

```text
Run a 5-worker feature wave in /Users/kogaryu/dev/features.

Wave:
- W1: logic.search_index
- W2: workflow.review_packet_flow
- W3: logic.spec_generator
- W4: logic.prompt_generator
- W5: workflow.feature_extraction_flow

Operating rules:
- each worker owns only its feature crate path
- shared files are owned by the integrator lane only
- if a worker needs shared-file changes, stop and emit the escalation packet
- do not let workers edit each other’s crates
- do not start reviewer proof until the integrator finishes
- keep proof terminal-first unless a feature explicitly needs live harness smoke
```

## Worker Packet Template

```text
Implement one isolated feature in the Rust feature lab.

Feature:
{feature_id}

Repo:
/Users/kogaryu/dev/features

Location:
{feature_path}

Lane:
{lane_name}

Allowed writes:
- {feature_path}/**

Forbidden writes:
- /Users/kogaryu/dev/features/Cargo.toml
- /Users/kogaryu/dev/features/crates/**
- any other feature_packs path outside {feature_path}

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

Required output:
- feature crate implemented
- contract tests added
- local feature commands proposed for the integrator/reviewer

Stop after this one feature.

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

Lane:
{lane_name}

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

## Integrator Packet

```text
Integrate a completed 5-worker feature wave in /Users/kogaryu/dev/features.

You own shared files only:
- /Users/kogaryu/dev/features/Cargo.toml
- /Users/kogaryu/dev/features/crates/feature_lab_ui/Cargo.toml
- /Users/kogaryu/dev/features/crates/feature_lab_ui/src/app.rs
- /Users/kogaryu/dev/features/crates/feature_lab_ui/src/panels/demo_panel.rs

Workers have already built:
- logic.search_index
- workflow.review_packet_flow
- logic.spec_generator
- logic.prompt_generator
- workflow.feature_extraction_flow

Rules:
- do not redesign feature crates
- patch feature crates only if a compile fix is absolutely required, and explain why
- batch workspace member additions once
- add minimal harness/demo wiring only where needed
- preserve isolated feature contracts

Verification:
- cargo fmt --all --check
- cargo test --workspace
- cargo run -p feature_cli -- list
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
Review an integrated feature wave in /Users/kogaryu/dev/features.

Do not modify files.

Wave features:
- logic.search_index
- workflow.review_packet_flow
- logic.spec_generator
- logic.prompt_generator
- workflow.feature_extraction_flow

Run:
- cargo fmt --all --check
- cargo test --workspace
- cargo run -p feature_cli -- list
- cargo run -p feature_cli -- show logic.search_index
- cargo run -p feature_cli -- show workflow.review_packet_flow
- cargo run -p feature_cli -- show logic.spec_generator
- cargo run -p feature_cli -- show logic.prompt_generator
- cargo run -p feature_cli -- show workflow.feature_extraction_flow
- cargo run -p feature_cli -- test logic.search_index
- cargo run -p feature_cli -- test workflow.review_packet_flow
- cargo run -p feature_cli -- test logic.spec_generator
- cargo run -p feature_cli -- test logic.prompt_generator
- cargo run -p feature_cli -- test workflow.feature_extraction_flow
- git diff --check

Optional manual smoke only if a wave feature explicitly depends on live harness behavior:
- scripts/launch_feature_lab_ui_app.sh

Report:
1. findings by severity
2. commands run
3. tests passed/failed
4. known risks
5. merge recommendation
```

## Current Safe Batch Size

- `5` worker Dex
- `1` integrator Dex
- `1` reviewer Dex

Do not let all `7` write code at once. Only the `5` workers write in parallel. The integrator writes after worker completion, and the reviewer stays read-only.
