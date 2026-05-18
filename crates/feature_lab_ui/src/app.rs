mod actions;
mod bootstrap;
mod browser_state;
mod layout;
mod style;

use actor_state::ActorState;
use ai_grade_hud::AIGradeHudState;
use ai_intent_model::AIIntentState;
use ai_interaction_event::AIInteractionEvent;
use ai_interaction_grader::AIInteractionGradeCard;
use ai_perception_model::AIPerceptionSnapshot;
use ai_target_state::AITargetState;
use beat_em_up_plane::BeatEmUpPlaneState;
use checkpoint_flow::CheckpointFlowState;
use crash_recovery::CrashRecoveryState;
use document_history::DocumentHistoryState;
use duel_arena::DuelArenaState;
use feature_core::FeatureKind;
use feature_registry::FeatureRegistry;
use health_hud::HealthHudState;
use hit_resolution::{HitRequest, HitResult};
use layer_escalation::LayerEscalationDecision;
use quick_capture_inbox::QuickCaptureInboxState;
use runbook_panel::RunbookState;
use runner_track::RunnerTrackState;
use scratchpad::ScratchpadState;
use session_notes::SessionNotesState;
use session_notes_panel::SessionNotesPanelState;
use text_editor_plain::TextEditorPlain;
use theme_editor::ThemeEditor;
use timer_basic::TimerState;
use wiki_browser::WikiBrowserState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KindFilter {
    All,
    Ui,
    Logic,
    Sim,
    Workflow,
}

impl KindFilter {
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Ui => "ui",
            Self::Logic => "logic",
            Self::Sim => "sim",
            Self::Workflow => "workflow",
        }
    }

    pub fn from_browser_kind(kind: Option<&FeatureKind>) -> Self {
        match kind {
            None => Self::All,
            Some(FeatureKind::UiPattern) => Self::Ui,
            Some(FeatureKind::LogicPattern) => Self::Logic,
            Some(FeatureKind::SimulationPattern) => Self::Sim,
            Some(FeatureKind::Workflow) => Self::Workflow,
        }
    }

    pub fn into_browser_kind(self) -> Option<FeatureKind> {
        match self {
            Self::All => None,
            Self::Ui => Some(FeatureKind::UiPattern),
            Self::Logic => Some(FeatureKind::LogicPattern),
            Self::Sim => Some(FeatureKind::SimulationPattern),
            Self::Workflow => Some(FeatureKind::Workflow),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FeatureCounts {
    pub total: usize,
    pub ui: usize,
    pub logic: usize,
    pub sim: usize,
    pub workflow: usize,
}

pub struct FeatureLabApp {
    pub(crate) registry: FeatureRegistry,
    pub(crate) browser: WikiBrowserState,
    pub(crate) glass_enabled: bool,
    pub(crate) test_output: String,
    pub(crate) activity_log: Vec<String>,
    pub(crate) command_palette_demo_query: String,
    pub(crate) command_palette_demo_selected_index: usize,
    pub(crate) command_palette_last_activation: String,
    pub(crate) activity_stream_demo_query: String,
    pub(crate) activity_stream_demo_selected_index: usize,
    pub(crate) activity_stream_last_activation: String,
    pub(crate) validation_pipeline_demo_use_invalid: bool,
    pub(crate) left_rail_demo_selected_index: usize,
    pub(crate) left_rail_demo_collapsed: bool,
    pub(crate) left_rail_last_activation: String,
    pub(crate) timer_demo: TimerState,
    pub(crate) runbook_demo: RunbookState,
    pub(crate) quick_capture_inbox_demo: QuickCaptureInboxState,
    pub(crate) quick_capture_inbox_demo_captured_at: String,
    pub(crate) scratchpad_demo: ScratchpadState,
    pub(crate) calculator_demo: calculator_basic::CalculatorState,
    pub(crate) checklist_demo: checklist_single::ChecklistState,
    pub(crate) checklist_demo_draft: String,
    pub(crate) actor_state_demo: ActorState,
    pub(crate) ai_target_state_demo: AITargetState,
    pub(crate) ai_perception_demo: AIPerceptionSnapshot,
    pub(crate) ai_intent_demo: AIIntentState,
    pub(crate) ai_interaction_event_demo: Vec<AIInteractionEvent>,
    pub(crate) ai_interaction_grade_demo: AIInteractionGradeCard,
    pub(crate) layer_escalation_demo: LayerEscalationDecision,
    pub(crate) duel_arena_demo: DuelArenaState,
    pub(crate) ai_grade_hud_demo: AIGradeHudState,
    pub(crate) hit_resolution_demo_target: ActorState,
    pub(crate) hit_resolution_demo_request: HitRequest,
    pub(crate) hit_resolution_demo_last_result: Option<HitResult>,
    pub(crate) beat_em_up_plane_demo: BeatEmUpPlaneState,
    pub(crate) runner_track_demo: RunnerTrackState,
    pub(crate) crash_recovery_demo: CrashRecoveryState,
    pub(crate) checkpoint_flow_demo: CheckpointFlowState,
    pub(crate) health_hud_demo_actor: ActorState,
    pub(crate) health_hud_demo: HealthHudState,
    pub(crate) document_history_demo: DocumentHistoryState,
    pub(crate) document_history_demo_snapshot_label: String,
    pub(crate) document_history_demo_recorded_at: String,
    pub(crate) session_notes_demo: SessionNotesState,
    pub(crate) session_notes_demo_draft: String,
    pub(crate) session_notes_demo_created_at: String,
    pub(crate) text_editor_plain_demo: TextEditorPlain,
    pub(crate) session_notes_panel_demo: SessionNotesPanelState,
    pub(crate) session_notes_panel_demo_created_at: String,
    pub(crate) session_notes_panel_demo_snapshot_label: String,
    pub(crate) session_notes_panel_demo_recorded_at: String,
    pub(crate) theme_editor_demo: ThemeEditor,
    pub(crate) theme_editor_last_action: String,
}
