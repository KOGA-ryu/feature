use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::chips::{stat_chip, stats_row};

pub(super) fn show_stats(ui: &mut egui::Ui, app: &FeatureLabApp) {
    let counts = app.quick_capture_inbox_demo.counts();
    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("total {}", counts.total),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("pending {}", counts.pending),
            egui::Color32::from_rgb(255, 201, 110),
        );
        stat_chip(
            ui,
            format!("visible {}", counts.visible),
            egui::Color32::from_rgb(126, 217, 140),
        );
    });
    ui.add_space(8.0);
}
