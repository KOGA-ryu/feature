use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::section_card;
use super::super::chips::stat_chip;

pub(super) fn show_selected_note(ui: &mut egui::Ui, app: &FeatureLabApp) {
    let Some(entry) = app.session_notes_demo.selected_entry() else {
        return;
    };

    ui.add_space(12.0);
    section_card(
        ui,
        "Selected note",
        "Current selected entry from the filtered note ledger.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                stat_chip(
                    ui,
                    format!("id {}", entry.id),
                    egui::Color32::from_rgb(188, 199, 220),
                );
                stat_chip(
                    ui,
                    if entry.pinned { "pinned" } else { "unpinned" },
                    note_accent(entry.pinned),
                );
                stat_chip(
                    ui,
                    &entry.created_at,
                    egui::Color32::from_rgb(126, 217, 140),
                );
            });
            ui.add_space(6.0);
            ui.label(&entry.text);
        },
    );
}

fn note_accent(pinned: bool) -> egui::Color32 {
    if pinned {
        egui::Color32::from_rgb(255, 201, 110)
    } else {
        egui::Color32::from_rgb(188, 199, 220)
    }
}
