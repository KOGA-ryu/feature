use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::sandbox_toolbar;
use super::chips::{stat_chip, stats_row};

pub(super) fn show_scratchpad_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    sandbox_toolbar(
        ui,
        "Scratchpad demo",
        "Headless plain-text note state with clear and count helpers.",
        |ui| {
            if ui.button("Clear").clicked() {
                app.scratchpad_demo.clear();
                app.push_log("Scratchpad demo cleared.");
            }
        },
    );
    ui.add_space(8.0);

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("chars {}", app.scratchpad_demo.character_count()),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("lines {}", app.scratchpad_demo.line_count()),
            egui::Color32::from_rgb(126, 217, 140),
        );
    });
    ui.add_space(8.0);

    ui.add_sized(
        [ui.available_width(), 220.0],
        egui::TextEdit::multiline(&mut app.scratchpad_demo.text)
            .hint_text("Write quick notes here.")
            .desired_rows(10),
    );
}
