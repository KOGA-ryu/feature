# Features Binder Baseline Inventory V1

## Scope
- Inspected path: `/Users/kogaryu/dev/features/feature_packs`
- Timestamp (UTC): `2026-05-18T04:09:44Z`
- Inspection method: direct filesystem walk over `feature_packs/{logic,sim,ui,workflows}/*/feature.toml`

## Readability cleanup
- Baseline facts were unchanged from the inspected inventory.
- Inventory rows in `features_feature_foundry_v1.json` were split into shorter per-group rows for UI readability.

## Feature pack totals
- Total feature packs: `48`
- logic: `17`
- sim: `6`
- ui: `16`
- workflows: `9`

## Feature packs by group
- logic
  - `actor_state`
  - `ai_intent_model`
  - `ai_interaction_event`
  - `ai_interaction_grader`
  - `ai_perception_model`
  - `ai_target_state`
  - `compatibility_matrix`
  - `document_history`
  - `hit_resolution`
  - `prompt_generator`
  - `reuse_score`
  - `search_index`
  - `session_notes`
  - `spec_generator`
  - `spec_selector`
  - `theme_token_generator`
  - `validation_pipeline`
- sim
  - `beat_em_up_plane`
  - `checkpoint_flow`
  - `crash_recovery`
  - `duel_arena`
  - `layer_escalation`
  - `runner_track`
- ui
  - `activity_stream`
  - `ai_grade_hud`
  - `calculator_basic`
  - `checklist_single`
  - `command_palette`
  - `health_hud`
  - `left_rail`
  - `quick_capture_inbox`
  - `right_inspector`
  - `runbook_panel`
  - `scratchpad`
  - `session_notes_panel`
  - `text_editor_plain`
  - `theme_editor`
  - `timer_basic`
  - `wiki_browser`
- workflows
  - `feature_batch_packet_flow`
  - `feature_build_packet_flow`
  - `feature_extraction_flow`
  - `feature_wave_packet_flow`
  - `lineage_tracker`
  - `packet_archive_index`
  - `packet_archive_reader`
  - `packet_archive_writer`
  - `review_packet_flow`

## Missing README files
- none

## Missing test markers (based on observed crate layout)
- Missing `README.md`: none
- Missing `tests/contract_tests.rs` where `tests/` exists: none
- Missing `tests/` directory entirely: none

## Generated / index folders observed
- `feature_packs/by_subject/**`
- `feature_packs/logic/document_history/target`
- `feature_packs/ui/quick_capture_inbox/target`

## Protected integration paths
- `/Users/kogaryu/dev/features/AGENTS.md`
- `/Users/kogaryu/dev/features/Cargo.toml`
- `/Users/kogaryu/dev/features/Cargo.lock`
- `/Users/kogaryu/dev/features/README.md`
- `/Users/kogaryu/dev/features/docs/contract.md`
- `/Users/kogaryu/dev/features/docs/feature_contract.md`
- `/Users/kogaryu/dev/features/feature_packs/<group>/<pack>/Cargo.toml` for each feature pack

## Unknown or suspicious folders
- `feature_packs/by_subject` (generated subject mirror, not a source feature group directory)

## What changed in binder
- Updated `/Users/kogaryu/dev/features/tools/features_binder/data/binder_templates/features_feature_foundry_v1.json` with real inventory rows from this inspection:
  - logic/sim/ui/workflow example lists
  - generated index paths
  - test-marker scan rows
  - scanner-vs-contract source note row
- Added this report: `/Users/kogaryu/dev/features/tools/features_binder/docs/features_baseline_inventory_v1.md`
- `projects.json` feature_pack_summary was inspected but left unchanged because counts match live inventory

## What was intentionally not changed
- No feature crate source files under `/Users/kogaryu/dev/features/feature_packs/**`
- No JSONL files written
- No worker execution (`cargo test` for individual worker crates)
- No changes to `/Users/kogaryu/dev/dex_home` or `/Users/kogaryu/dev/dex_home_final`
