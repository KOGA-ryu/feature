use eframe::egui;
use session_notes::SessionNoteEntry;

use crate::app::FeatureLabApp;

use super::super::cards::{empty_state_card, result_card, section_card};
use super::super::chips::stat_chip;

pub(super) fn show_visible_notes(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let selected_id = app
        .session_notes_demo
        .selected_entry()
        .map(|entry| entry.id);
    let visible_snapshot = app
        .session_notes_demo
        .visible_entries()
        .into_iter()
        .cloned()
        .collect::<Vec<SessionNoteEntry>>();

    if visible_snapshot.is_empty() {
        empty_state_card(ui, "No notes match the current query.");
        return;
    }

    section_card(
        ui,
        "Visible notes",
        "Select, pin, or delete notes inside the filtered ledger.",
        |ui| {
            for (index, entry) in visible_snapshot.iter().enumerate() {
                show_visible_note_row(ui, app, index, entry, selected_id);
                ui.add_space(6.0);
            }
        },
    );
}

fn show_visible_note_row(
    ui: &mut egui::Ui,
    app: &mut FeatureLabApp,
    index: usize,
    entry: &SessionNoteEntry,
    selected_id: Option<u64>,
) {
    let selected = Some(entry.id) == selected_id;
    result_card(ui, selected, note_accent(entry.pinned), |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui
                .button(if entry.pinned { "Unpin" } else { "Pin" })
                .clicked()
            {
                app.session_notes_demo.toggle_pinned(entry.id);
            }
            if ui.button("Select").clicked() {
                app.session_notes_demo.set_selected_index(index);
            }
            if ui.button("Delete").clicked() {
                app.session_notes_demo.delete_entry(entry.id);
            }
            if selected {
                stat_chip(ui, "selected", egui::Color32::from_rgb(126, 188, 255));
            }
            stat_chip(
                ui,
                format!("id {}", entry.id),
                egui::Color32::from_rgb(188, 199, 220),
            );
            stat_chip(
                ui,
                &entry.created_at,
                egui::Color32::from_rgb(188, 199, 220),
            );
        });
        ui.add_space(4.0);
        ui.label(&entry.text);
    });
}

fn note_accent(pinned: bool) -> egui::Color32 {
    if pinned {
        egui::Color32::from_rgb(255, 201, 110)
    } else {
        egui::Color32::from_rgb(188, 199, 220)
    }
}
