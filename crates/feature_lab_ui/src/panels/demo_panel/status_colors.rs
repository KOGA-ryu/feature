use eframe::egui;
use quick_capture_inbox::InboxItemStatus;
use runbook_panel::RunbookStepStatus;
use session_notes_panel::SessionNotesPanelMode;
use timer_basic::TimerStatus;

pub(super) fn has_live_demo(feature_id: &str) -> bool {
    matches!(
        feature_id,
        "logic.validation_pipeline"
            | "ui.activity_stream"
            | "ui.timer_basic"
            | "ui.runbook_panel"
            | "ui.quick_capture_inbox"
            | "ui.scratchpad"
            | "logic.session_notes"
            | "logic.document_history"
            | "ui.calculator_basic"
            | "ui.checklist_single"
            | "ui.text_editor_plain"
            | "ui.session_notes_panel"
            | "ui.command_palette"
            | "ui.left_rail"
            | "ui.theme_editor"
            | "logic.actor_state"
            | "logic.ai_target_state"
            | "logic.ai_perception_model"
            | "logic.ai_intent_model"
            | "logic.ai_interaction_event"
            | "logic.ai_interaction_grader"
            | "logic.hit_resolution"
            | "sim.beat_em_up_plane"
            | "sim.duel_arena"
            | "sim.layer_escalation"
            | "sim.runner_track"
            | "sim.crash_recovery"
            | "sim.checkpoint_flow"
            | "ui.ai_grade_hud"
            | "ui.health_hud"
    )
}

pub(super) fn kind_color(feature_id: &str) -> egui::Color32 {
    if feature_id.starts_with("ui.") {
        egui::Color32::from_rgb(126, 188, 255)
    } else if feature_id.starts_with("logic.") {
        egui::Color32::from_rgb(255, 201, 110)
    } else if feature_id.starts_with("sim.") {
        egui::Color32::from_rgb(110, 214, 188)
    } else if feature_id.starts_with("workflow.") {
        egui::Color32::from_rgb(177, 150, 255)
    } else {
        egui::Color32::from_gray(190)
    }
}

pub(super) fn lifecycle_color(status: &str) -> egui::Color32 {
    match status {
        "stable" => egui::Color32::from_rgb(126, 217, 140),
        "tested" => egui::Color32::from_rgb(126, 188, 255),
        "experimental" => egui::Color32::from_rgb(255, 201, 110),
        "deprecated" => egui::Color32::from_rgb(255, 133, 133),
        _ => egui::Color32::from_gray(190),
    }
}

pub(super) fn timer_status_label(status: TimerStatus) -> &'static str {
    match status {
        TimerStatus::Idle => "idle",
        TimerStatus::Running => "running",
        TimerStatus::Paused => "paused",
        TimerStatus::Completed => "completed",
    }
}

pub(super) fn timer_status_color(status: TimerStatus) -> egui::Color32 {
    match status {
        TimerStatus::Idle => egui::Color32::from_rgb(188, 199, 220),
        TimerStatus::Running => egui::Color32::from_rgb(126, 188, 255),
        TimerStatus::Paused => egui::Color32::from_rgb(255, 201, 110),
        TimerStatus::Completed => egui::Color32::from_rgb(126, 217, 140),
    }
}

pub(super) fn runbook_status_label(status: RunbookStepStatus) -> &'static str {
    match status {
        RunbookStepStatus::Pending => "pending",
        RunbookStepStatus::Active => "active",
        RunbookStepStatus::Completed => "completed",
        RunbookStepStatus::Blocked => "blocked",
    }
}

pub(super) fn runbook_status_color(status: RunbookStepStatus) -> egui::Color32 {
    match status {
        RunbookStepStatus::Pending => egui::Color32::from_rgb(188, 199, 220),
        RunbookStepStatus::Active => egui::Color32::from_rgb(126, 188, 255),
        RunbookStepStatus::Completed => egui::Color32::from_rgb(126, 217, 140),
        RunbookStepStatus::Blocked => egui::Color32::from_rgb(255, 133, 133),
    }
}

pub(super) fn inbox_status_label(status: InboxItemStatus) -> &'static str {
    match status {
        InboxItemStatus::Pending => "pending",
        InboxItemStatus::Processed => "processed",
        InboxItemStatus::Archived => "archived",
    }
}

pub(super) fn inbox_status_color(status: InboxItemStatus) -> egui::Color32 {
    match status {
        InboxItemStatus::Pending => egui::Color32::from_rgb(255, 201, 110),
        InboxItemStatus::Processed => egui::Color32::from_rgb(126, 217, 140),
        InboxItemStatus::Archived => egui::Color32::from_rgb(188, 199, 220),
    }
}

pub(super) fn session_notes_panel_mode_label(mode: SessionNotesPanelMode) -> &'static str {
    match mode {
        SessionNotesPanelMode::Browse => "browse",
        SessionNotesPanelMode::Compose => "compose",
        SessionNotesPanelMode::EditSelected => "edit_selected",
    }
}
