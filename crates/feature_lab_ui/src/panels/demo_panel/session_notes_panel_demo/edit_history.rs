use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::{empty_state_card, result_card, section_card};
use super::super::chips::stat_chip;

pub(super) fn show_edit_history(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let revisions = app
        .session_notes_panel_demo
        .selected_history()
        .revisions
        .clone();
    let selected_revision_id = app
        .session_notes_panel_demo
        .selected_history()
        .selected_revision()
        .map(|revision| revision.id);

    if revisions.is_empty() {
        empty_state_card(ui, "No edit snapshots recorded yet.");
        return;
    }

    section_card(
        ui,
        "Edit history",
        "Revision history for the active selected-note edit session.",
        |ui| {
            for (index, revision) in revisions.iter().enumerate() {
                let selected = Some(revision.id) == selected_revision_id;
                result_card(ui, selected, egui::Color32::from_rgb(176, 197, 255), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Select").clicked() {
                            app.session_notes_panel_demo
                                .selected_history
                                .set_selected_revision_index(index);
                        }
                        if ui.button("Restore").clicked() {
                            app.session_notes_panel_demo
                                .selected_history
                                .set_selected_revision_index(index);
                            if app.session_notes_panel_demo.restore_selected_revision() {
                                app.push_log(format!(
                                    "Session notes panel demo restored revision {}.",
                                    revision.id
                                ));
                            }
                        }
                        if selected {
                            stat_chip(ui, "selected", egui::Color32::from_rgb(126, 188, 255));
                        }
                        stat_chip(
                            ui,
                            format!("id {}", revision.id),
                            egui::Color32::from_rgb(188, 199, 220),
                        );
                        stat_chip(
                            ui,
                            &revision.recorded_at,
                            egui::Color32::from_rgb(126, 217, 140),
                        );
                        ui.strong(&revision.label);
                    });
                    ui.add_space(4.0);
                    ui.label(&revision.text);
                });
                ui.add_space(6.0);
            }
        },
    );
}
