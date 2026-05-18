use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::chips::{stat_chip, stats_row};

pub(super) fn show_stats(ui: &mut egui::Ui, app: &FeatureLabApp) {
    let counts = app.document_history_demo.counts();
    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("revisions {}", counts.revisions),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            if counts.dirty { "dirty" } else { "clean" },
            if counts.dirty {
                egui::Color32::from_rgb(255, 133, 133)
            } else {
                egui::Color32::from_rgb(126, 217, 140)
            },
        );
        stat_chip(
            ui,
            format!(
                "selected {}",
                app.document_history_demo.selected_revision_index + 1
            ),
            egui::Color32::from_rgb(188, 199, 220),
        );
    });
    ui.add_space(8.0);
}
