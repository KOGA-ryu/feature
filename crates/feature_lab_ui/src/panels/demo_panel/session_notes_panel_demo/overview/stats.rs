use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::super::chips::{stat_chip, stats_row};
use super::super::super::status_colors::session_notes_panel_mode_label;

pub(super) fn show_stats(ui: &mut egui::Ui, app: &FeatureLabApp) {
    let counts = app.session_notes_panel_demo.notes().counts();
    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("total {}", counts.total),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("pinned {}", counts.pinned),
            egui::Color32::from_rgb(255, 201, 110),
        );
        stat_chip(
            ui,
            format!("visible {}", counts.visible),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            session_notes_panel_mode_label(app.session_notes_panel_demo.mode),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!(
                "history {}",
                app.session_notes_panel_demo
                    .selected_history()
                    .revision_count()
            ),
            egui::Color32::from_rgb(176, 197, 255),
        );
    });
    ui.add_space(8.0);
}
