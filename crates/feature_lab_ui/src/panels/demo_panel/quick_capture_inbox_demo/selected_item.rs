use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::section_card;
use super::super::chips::stat_chip;
use super::super::status_colors::{inbox_status_color, inbox_status_label};

pub(super) fn show_selected_item(ui: &mut egui::Ui, app: &FeatureLabApp) {
    let Some(item) = app.quick_capture_inbox_demo.selected_item() else {
        return;
    };

    ui.add_space(12.0);
    section_card(
        ui,
        "Selected inbox item",
        "Current selected capture from the filtered inbox queue.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                stat_chip(
                    ui,
                    format!("id {}", item.id),
                    egui::Color32::from_rgb(188, 199, 220),
                );
                stat_chip(
                    ui,
                    inbox_status_label(item.status),
                    inbox_status_color(item.status),
                );
                stat_chip(
                    ui,
                    &item.captured_at,
                    egui::Color32::from_rgb(126, 217, 140),
                );
            });
            ui.add_space(6.0);
            ui.label(&item.text);
        },
    );
}
