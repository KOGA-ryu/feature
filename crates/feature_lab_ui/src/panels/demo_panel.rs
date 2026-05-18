mod activity_stream_demo;
mod actor_state_demo;
mod ai_grade_hud_demo;
mod ai_intent_model_demo;
mod ai_interaction_event_demo;
mod ai_interaction_grader_demo;
mod ai_perception_model_demo;
mod ai_target_state_demo;
mod beat_em_up_plane_demo;
mod calculator_demo;
mod cards;
mod checklist_demo;
mod checkpoint_flow_demo;
mod chips;
mod command_palette_demo;
mod crash_recovery_demo;
mod document_history_demo;
mod duel_arena_demo;
mod health_hud_demo;
mod hit_resolution_demo;
mod layer_escalation_demo;
mod left_rail_demo;
mod quick_capture_inbox_demo;
mod runbook_demo;
mod runner_track_demo;
mod scratchpad_demo;
mod session_notes_demo;
mod session_notes_panel_demo;
mod session_notes_sync;
mod status_colors;
mod text_editor_plain_demo;
mod text_editor_surface;
mod theme_editor_demo;
mod timer_demo;
mod validation_pipeline_demo;

use eframe::egui;

use crate::app::FeatureLabApp;

use self::activity_stream_demo::show_activity_stream_demo;
use self::actor_state_demo::show_actor_state_demo;
use self::ai_grade_hud_demo::show_ai_grade_hud_demo;
use self::ai_intent_model_demo::show_ai_intent_model_demo;
use self::ai_interaction_event_demo::show_ai_interaction_event_demo;
use self::ai_interaction_grader_demo::show_ai_interaction_grader_demo;
use self::ai_perception_model_demo::show_ai_perception_model_demo;
use self::ai_target_state_demo::show_ai_target_state_demo;
use self::beat_em_up_plane_demo::show_beat_em_up_plane_demo;
use self::calculator_demo::show_calculator_basic_demo;
use self::cards::{hero_card, section_card};
use self::checklist_demo::show_checklist_single_demo;
use self::checkpoint_flow_demo::show_checkpoint_flow_demo;
use self::command_palette_demo::show_command_palette_demo;
use self::crash_recovery_demo::show_crash_recovery_demo;
use self::document_history_demo::show_document_history_demo;
use self::duel_arena_demo::show_duel_arena_demo;
use self::health_hud_demo::show_health_hud_demo;
use self::hit_resolution_demo::show_hit_resolution_demo;
use self::layer_escalation_demo::show_layer_escalation_demo;
use self::left_rail_demo::show_left_rail_demo;
use self::quick_capture_inbox_demo::show_quick_capture_inbox_demo;
use self::runbook_demo::show_runbook_panel_demo;
use self::runner_track_demo::show_runner_track_demo;
use self::scratchpad_demo::show_scratchpad_demo;
use self::session_notes_demo::show_session_notes_demo;
use self::session_notes_panel_demo::show_session_notes_panel_demo;
use self::status_colors::has_live_demo;
use self::text_editor_plain_demo::show_text_editor_plain_demo;
use self::theme_editor_demo::show_theme_editor_demo;
use self::timer_demo::show_timer_basic_demo;
use self::validation_pipeline_demo::show_validation_pipeline_demo;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.heading("Workbench");
    ui.small("Live demo surfaces, contract context, and preview assets.");
    ui.separator();

    let Some(feature) = app.selected_browser_entry().cloned() else {
        ui.label("Select a feature to preview its documentation or demo surface.");
        return;
    };

    egui::ScrollArea::vertical()
        .id_salt("workbench_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            hero_card(ui, &feature);
            ui.add_space(12.0);

            section_card(
                ui,
                "Demo bench",
                if has_live_demo(&feature.manifest.id) {
                    "Interactive feature-specific harness wired into the lab."
                } else {
                    "Documentation-driven preview for a reusable feature without a dedicated live harness yet."
                },
                |ui| {
                    if feature.manifest.id == "logic.validation_pipeline" {
                        show_validation_pipeline_demo(ui, app);
                    } else if feature.manifest.id == "ui.activity_stream" {
                        show_activity_stream_demo(ui, app);
                    } else if feature.manifest.id == "logic.ai_target_state" {
                        show_ai_target_state_demo(ui, app);
                    } else if feature.manifest.id == "logic.ai_perception_model" {
                        show_ai_perception_model_demo(ui, app);
                    } else if feature.manifest.id == "logic.ai_intent_model" {
                        show_ai_intent_model_demo(ui, app);
                    } else if feature.manifest.id == "logic.ai_interaction_event" {
                        show_ai_interaction_event_demo(ui, app);
                    } else if feature.manifest.id == "logic.ai_interaction_grader" {
                        show_ai_interaction_grader_demo(ui, app);
                    } else if feature.manifest.id == "ui.timer_basic" {
                        show_timer_basic_demo(ui, app);
                    } else if feature.manifest.id == "ui.runbook_panel" {
                        show_runbook_panel_demo(ui, app);
                    } else if feature.manifest.id == "ui.quick_capture_inbox" {
                        show_quick_capture_inbox_demo(ui, app);
                    } else if feature.manifest.id == "ui.scratchpad" {
                        show_scratchpad_demo(ui, app);
                    } else if feature.manifest.id == "logic.session_notes" {
                        show_session_notes_demo(ui, app);
                    } else if feature.manifest.id == "logic.document_history" {
                        show_document_history_demo(ui, app);
                    } else if feature.manifest.id == "logic.actor_state" {
                        show_actor_state_demo(ui, app);
                    } else if feature.manifest.id == "logic.hit_resolution" {
                        show_hit_resolution_demo(ui, app);
                    } else if feature.manifest.id == "sim.beat_em_up_plane" {
                        show_beat_em_up_plane_demo(ui, app);
                    } else if feature.manifest.id == "sim.duel_arena" {
                        show_duel_arena_demo(ui, app);
                    } else if feature.manifest.id == "sim.layer_escalation" {
                        show_layer_escalation_demo(ui, app);
                    } else if feature.manifest.id == "sim.runner_track" {
                        show_runner_track_demo(ui, app);
                    } else if feature.manifest.id == "sim.crash_recovery" {
                        show_crash_recovery_demo(ui, app);
                    } else if feature.manifest.id == "sim.checkpoint_flow" {
                        show_checkpoint_flow_demo(ui, app);
                    } else if feature.manifest.id == "ui.calculator_basic" {
                        show_calculator_basic_demo(ui, app);
                    } else if feature.manifest.id == "ui.checklist_single" {
                        show_checklist_single_demo(ui, app);
                    } else if feature.manifest.id == "ui.health_hud" {
                        show_health_hud_demo(ui, app);
                    } else if feature.manifest.id == "ui.ai_grade_hud" {
                        show_ai_grade_hud_demo(ui, app);
                    } else if feature.manifest.id == "ui.text_editor_plain" {
                        show_text_editor_plain_demo(ui, app);
                    } else if feature.manifest.id == "ui.session_notes_panel" {
                        show_session_notes_panel_demo(ui, app);
                    } else if feature.manifest.id == "ui.command_palette" {
                        show_command_palette_demo(ui, app);
                    } else if feature.manifest.id == "ui.left_rail" {
                        show_left_rail_demo(ui, app);
                    } else if feature.manifest.id == "ui.theme_editor" {
                        show_theme_editor_demo(ui, app);
                    } else if feature.manifest.id == "ui.right_inspector" {
                        ui.strong("Right inspector placeholder");
                        ui.label("Persistent right-side detail panel for the current selection.");
                        ui.add_space(6.0);
                        for action in [
                            "Inspect selected object identity and metadata",
                            "Show linked artifacts and related context",
                            "Surface contextual actions for the current selection",
                        ] {
                            ui.label(format!("• {action}"));
                        }
                    } else {
                        ui.label("No live demo surface is wired for this feature yet.");
                    }
                },
            );

            ui.add_space(12.0);
            section_card(
                ui,
                "README preview",
                "Human-facing contract and implementation notes from the feature crate.",
                |ui| {
                    ui.small(
                        egui::RichText::new(feature.readme_path.as_str())
                            .monospace()
                            .color(egui::Color32::from_gray(170)),
                    );
                    ui.add_space(6.0);
                    egui::ScrollArea::vertical()
                        .id_salt("workbench_readme_preview_scroll")
                        .max_height(260.0)
                        .show(ui, |ui| {
                            ui.monospace(feature.readme_text.as_str());
                        });
                },
            );

            ui.add_space(12.0);
            section_card(
                ui,
                "Fixture preview",
                "First available sample payload from the feature crate fixtures directory.",
                |ui| match feature.fixture_preview.as_deref() {
                    Some(fixture) => {
                        ui.small(
                            egui::RichText::new(feature.fixtures_dir.as_str())
                                .monospace()
                                .color(egui::Color32::from_gray(170)),
                        );
                        ui.add_space(6.0);
                        egui::ScrollArea::vertical()
                            .id_salt("workbench_fixture_preview_scroll")
                            .max_height(220.0)
                            .show(ui, |ui| {
                                ui.monospace(fixture);
                            });
                    }
                    None => {
                        ui.label("No fixture files found.");
                    }
                },
            );
        });
}
