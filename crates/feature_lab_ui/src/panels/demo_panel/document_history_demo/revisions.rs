use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::{empty_state_card, result_card, section_card};
use super::super::chips::stat_chip;

pub(super) fn show_revisions(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.add_space(12.0);
    let selected_revision_id = app
        .document_history_demo
        .selected_revision()
        .map(|revision| revision.id);
    let revisions = app.document_history_demo.revisions.clone();

    if revisions.is_empty() {
        empty_state_card(ui, "No document revisions recorded yet.");
        return;
    }

    section_card(
        ui,
        "Revisions",
        "Select a revision, inspect its metadata, and restore it into the working document.",
        |ui| {
            for (index, revision) in revisions.iter().enumerate() {
                let selected = Some(revision.id) == selected_revision_id;
                result_card(ui, selected, egui::Color32::from_rgb(126, 188, 255), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Select").clicked() {
                            app.document_history_demo.set_selected_revision_index(index);
                        }
                        if ui.button("Restore").clicked() {
                            app.document_history_demo.set_selected_revision_index(index);
                            if app.document_history_demo.restore_selected_revision() {
                                app.push_log(format!(
                                    "Document history demo restored revision {}.",
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
                            egui::Color32::from_rgb(188, 199, 220),
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
