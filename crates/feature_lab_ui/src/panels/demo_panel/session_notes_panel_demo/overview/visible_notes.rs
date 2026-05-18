use eframe::egui;
use session_notes::SessionNoteEntry;
use session_notes_panel::SessionNotesPanelMode;

use crate::app::FeatureLabApp;

use super::super::super::cards::{empty_state_card, result_card, section_card};
use super::super::super::chips::stat_chip;
use super::note_meta::{note_accent, show_note_meta_chips};

pub(super) fn show_visible_notes(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let selected_id = app
        .session_notes_panel_demo
        .notes()
        .selected_entry()
        .map(|entry| entry.id);
    let visible_snapshot = app
        .session_notes_panel_demo
        .notes()
        .visible_entries()
        .into_iter()
        .cloned()
        .collect::<Vec<SessionNoteEntry>>();

    if visible_snapshot.is_empty() {
        empty_state_card(ui, "No notes match the current session-notes-panel query.");
        return;
    }

    section_card(
        ui,
        "Visible notes",
        "Select, pin, delete, or edit notes while the ledger remains the source of truth.",
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
    let editing_selected =
        selected && app.session_notes_panel_demo.mode == SessionNotesPanelMode::EditSelected;
    result_card(ui, selected, note_accent(entry.pinned), |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui.button("Select").clicked() {
                app.session_notes_panel_demo.set_selected_index(index);
            }
            if ui.button("Pin").clicked() {
                app.session_notes_panel_demo.set_selected_index(index);
                app.session_notes_panel_demo.toggle_pinned_selected();
            }
            if ui.button("Edit").clicked() {
                app.session_notes_panel_demo.set_selected_index(index);
                let _ = app.session_notes_panel_demo.start_edit_selected();
            }
            if ui.button("Delete").clicked() {
                app.session_notes_panel_demo.set_selected_index(index);
                app.session_notes_panel_demo.delete_selected();
            }
            if selected {
                stat_chip(ui, "selected", egui::Color32::from_rgb(126, 188, 255));
            }
            if editing_selected {
                stat_chip(ui, "editing", egui::Color32::from_rgb(176, 197, 255));
            }
            show_note_meta_chips(ui, entry);
        });
        ui.add_space(4.0);
        ui.label(&entry.text);
    });
}
