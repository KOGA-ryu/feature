use eframe::egui;
use session_notes::SessionNoteEntry;

use super::super::super::chips::stat_chip;

pub(super) fn show_note_meta_chips(ui: &mut egui::Ui, entry: &SessionNoteEntry) {
    stat_chip(
        ui,
        if entry.pinned { "pinned" } else { "unpinned" },
        note_accent(entry.pinned),
    );
    stat_chip(
        ui,
        format!("id {}", entry.id),
        egui::Color32::from_rgb(188, 199, 220),
    );
    stat_chip(
        ui,
        &entry.created_at,
        egui::Color32::from_rgb(126, 217, 140),
    );
}

pub(super) fn note_accent(pinned: bool) -> egui::Color32 {
    if pinned {
        egui::Color32::from_rgb(255, 201, 110)
    } else {
        egui::Color32::from_rgb(188, 199, 220)
    }
}
