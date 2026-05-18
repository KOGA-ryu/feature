use ai_interaction_event::sample_events as sample_ai_events;
use ai_interaction_grader::sample_grade_card as sample_ai_grade_card;
use feature_registry::{FeatureRegistry, default_workspace_root};
use hit_resolution::sample_scenario as sample_hit_scenario;
use text_editor_plain::sample_editor;
use theme_editor::ThemeEditor;
use wiki_browser::WikiBrowserState;

use super::FeatureLabApp;

impl FeatureLabApp {
    fn new_shell(
        registry: FeatureRegistry,
        browser: WikiBrowserState,
        activity_log: Vec<String>,
    ) -> Self {
        let hit_resolution_scenario = sample_hit_scenario().unwrap_or_default();
        Self {
            registry,
            browser,
            glass_enabled: cfg!(target_os = "macos"),
            test_output: "No tests run yet.".into(),
            activity_log,
            command_palette_demo_query: String::new(),
            command_palette_demo_selected_index: 0,
            command_palette_last_activation: "No command activated yet.".into(),
            activity_stream_demo_query: String::new(),
            activity_stream_demo_selected_index: 0,
            activity_stream_last_activation: "No activity entry activated yet.".into(),
            validation_pipeline_demo_use_invalid: false,
            left_rail_demo_selected_index: 0,
            left_rail_demo_collapsed: false,
            left_rail_last_activation: "No navigation item activated yet.".into(),
            timer_demo: timer_basic::sample_state().unwrap_or_default(),
            runbook_demo: runbook_panel::sample_state().unwrap_or_default(),
            quick_capture_inbox_demo: quick_capture_inbox::sample_state().unwrap_or_default(),
            quick_capture_inbox_demo_captured_at: "2026-04-27T10:00:00Z".into(),
            scratchpad_demo: scratchpad::sample_state().unwrap_or_default(),
            calculator_demo: calculator_basic::CalculatorState::default(),
            checklist_demo: checklist_single::sample_state().unwrap_or_default(),
            checklist_demo_draft: String::new(),
            actor_state_demo: actor_state::sample_state().unwrap_or_default(),
            ai_target_state_demo: ai_target_state::sample_state().unwrap_or_default(),
            ai_perception_demo: ai_perception_model::sample_state().unwrap_or_default(),
            ai_intent_demo: ai_intent_model::sample_state().unwrap_or_default(),
            ai_interaction_event_demo: sample_ai_events().unwrap_or_default(),
            ai_interaction_grade_demo: sample_ai_grade_card().unwrap_or_else(|_| {
                ai_interaction_grader::AIInteractionGradeCard {
                    read_quality: 50,
                    timing: 50,
                    control: 50,
                    adaptation: 50,
                    efficiency: 50,
                    style: 50,
                    overall_band: ai_interaction_grader::SkillBand::Competent,
                    confidence: ai_interaction_grader::GradeConfidence::Low,
                    event_count: 0,
                    poor_read_streak: 0,
                }
            }),
            layer_escalation_demo: layer_escalation::sample_state().unwrap_or_else(|_| {
                layer_escalation::LayerEscalationDecision {
                    stay_coarse: true,
                    offer_zoom: false,
                    force_zoom: false,
                    next_layer: None,
                    reward_modifier: 0,
                    reason: layer_escalation::EscalationReason::LowConfidence,
                }
            }),
            duel_arena_demo: duel_arena::sample_state().unwrap_or_default(),
            ai_grade_hud_demo: ai_grade_hud::sample_state().unwrap_or_else(|_| {
                ai_grade_hud::AIGradeHudState {
                    visible_band: ai_interaction_grader::SkillBand::Competent,
                    confidence: ai_interaction_grader::GradeConfidence::Low,
                    summary_label: "competent read".into(),
                    emphasis_label: "best at read_quality".into(),
                    zoom_ready: false,
                }
            }),
            hit_resolution_demo_target: hit_resolution_scenario.target,
            hit_resolution_demo_request: hit_resolution_scenario.request,
            hit_resolution_demo_last_result: None,
            beat_em_up_plane_demo: beat_em_up_plane::sample_state().unwrap_or_default(),
            runner_track_demo: runner_track::sample_state().unwrap_or_default(),
            crash_recovery_demo: crash_recovery::sample_state().unwrap_or_default(),
            checkpoint_flow_demo: checkpoint_flow::sample_state().unwrap_or_default(),
            health_hud_demo_actor: actor_state::sample_state().unwrap_or_default(),
            health_hud_demo: health_hud::sample_state().unwrap_or_default(),
            document_history_demo: document_history::sample_state().unwrap_or_default(),
            document_history_demo_snapshot_label: "draft".into(),
            document_history_demo_recorded_at: "2026-04-27T10:05:00Z".into(),
            session_notes_demo: session_notes::sample_state().unwrap_or_default(),
            session_notes_demo_draft: String::new(),
            session_notes_demo_created_at: "2026-04-27T10:00:00Z".into(),
            text_editor_plain_demo: sample_editor().unwrap_or_default(),
            session_notes_panel_demo: session_notes_panel::sample_state().unwrap_or_default(),
            session_notes_panel_demo_created_at: "2026-04-27T10:10:00Z".into(),
            session_notes_panel_demo_snapshot_label: "edit checkpoint".into(),
            session_notes_panel_demo_recorded_at: "2026-04-27T10:15:00Z".into(),
            theme_editor_demo: ThemeEditor::default(),
            theme_editor_last_action: "No theme action yet.".into(),
        }
    }

    pub fn bootstrap() -> Self {
        match FeatureRegistry::discover() {
            Ok(registry) => match WikiBrowserState::from_registry(&registry) {
                Ok(browser) => Self::new_shell(
                    registry,
                    browser,
                    vec!["Feature registry loaded successfully.".into()],
                ),
                Err(error) => Self::new_shell(
                    registry,
                    WikiBrowserState::new(Vec::new()),
                    vec![format!("Failed to build browser state: {error}")],
                ),
            },
            Err(error) => Self::new_shell(
                FeatureRegistry::empty(default_workspace_root()),
                WikiBrowserState::new(Vec::new()),
                vec![format!("Failed to load registry: {error}")],
            ),
        }
    }
}
