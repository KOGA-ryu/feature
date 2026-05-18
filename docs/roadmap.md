# Roadmap

## Phase 1

- Rust workspace
- shared metadata crate
- feature registry
- CLI list/show/test
- egui harness shell
- one feature crate: `ui.right_inspector`
- workspace tests

## Next Features

1. utility trio foundation
   - `ui.scratchpad`
   - `ui.calculator_basic`
   - `ui.checklist_single`
2. next utility primitive
   - `ui.text_editor_plain`
3. operator cockpit wave
   - `ui.timer_basic`
   - `ui.runbook_panel`
   - `logic.session_notes`
4. notes and documents wave
   - `ui.quick_capture_inbox`
   - `logic.document_history`
   - `ui.session_notes_panel`
5. hybrid brawler-racer gameplay systems foundation
   - `logic.actor_state`
   - `logic.hit_resolution`
   - `sim.beat_em_up_plane`
   - `sim.runner_track`
   - `sim.crash_recovery`
   - `sim.checkpoint_flow`
   - `ui.health_hud`
   - next contract step: `docs/hybrid_brawler_racer_sprite_integration_contract.md`
6. simulation-zoom AI grading wave
   - `logic.ai_target_state`
   - `logic.ai_perception_model`
   - `logic.ai_intent_model`
   - `logic.ai_interaction_event`
   - `logic.ai_interaction_grader`
   - `sim.layer_escalation`
   - `sim.duel_arena`
   - `ui.ai_grade_hud`
7. `ui.command_palette`
8. `ui.left_rail`
9. `ui.activity_stream`
10. `logic.validation_pipeline`
11. `logic.search_index`
12. `workflow.review_packet_flow`
13. `logic.spec_generator`
14. `logic.prompt_generator`
15. `workflow.feature_extraction_flow`
16. `ui.wiki_browser`

Build one at a time.

## Execution Flows

- isolated single-feature flow: `workflow.feature_build_packet_flow`
- sequential batch flow: `workflow.feature_batch_packet_flow`
- parallel wave flow: `workflow.feature_wave_packet_flow`
