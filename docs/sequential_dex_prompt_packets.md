# Sequential Dex Prompt Packets

Use this flow when you want Dex or Codex to build several features in one
queue, one feature at a time, end-to-end.

This is the sequential middle lane between:

- isolated single-feature packets in `docs/chatgpt_feature_packets.md`
- parallel multi-worker waves in `docs/parallel_dex_prompt_packets.md`

## Queue Law

- queue order is explicit and canonical
- run one queued feature at a time
- finish worker proof, integration proof, reviewer proof, review packet, and
  receipt before moving to the next feature
- stop only on blocker
- terminal proof is canonical; live harness smoke is optional unless explicitly needed

## Default Stop Rule

Continue unless blocked.

A blocker means:

- required proof command fails and you cannot fix it inside the current step
- the feature needs an unsafe or ambiguous shared-file change not covered by the
  packet
- the review packet would recommend `do_not_merge`
- the repo state is too dirty or conflicted to produce a trustworthy receipt

## Per-Step Contract

Each queued feature step should produce:

1. worker packet execution
2. integration packet execution
3. reviewer packet execution
4. one review packet with these required sections:
   - `files_changed`
   - `commands_run`
   - `tests_run`
   - `risks`
   - `known_issues`
   - `follow_up_tasks`
   - `merge_recommendation`
5. one receipt that says the step passed and the queue may advance

## Resume Law

After a successful step:

- archive or save the review packet
- record the receipt
- resume at the next queued feature id

If a step blocks:

- stop on that feature
- report the blocker, exact failing command, and why it cannot be cleared inside
  the current step
- do not skip ahead silently

## Operator Prompt Template

```text
Implement this feature queue in /Users/kogaryu/dev/features.

Queue:
- {feature_id_1}
- {feature_id_2}
- {feature_id_3}

Rules:
- do them one at a time, in order
- do not stop after each feature unless blocked
- for each feature, complete worker, integrator, reviewer, review packet, and receipt before moving on
- stop on blocker
- optional feature_lab_ui wiring is only allowed when the active feature truly needs a live harness surface

Per-feature proof:
- cargo test --manifest-path <feature crate Cargo.toml>
- cargo fmt --all --check
- cargo test --workspace
- cargo run -p feature_cli -- show <feature id>
- cargo run -p feature_cli -- test <feature id>
- git diff --check

Optional manual smoke only when the active feature truly depends on the harness:
- scripts/launch_feature_lab_ui_app.sh

Final report:
1. feature-by-feature results
2. proof run for each feature
3. any blockers or deferred follow-up
4. final batch summary
```

## Related Docs

- `docs/chatgpt_feature_packets.md`
- `docs/parallel_dex_prompt_packets.md`
- `workflow.feature_batch_packet_flow`
