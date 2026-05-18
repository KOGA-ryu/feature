use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::section_card;
use super::super::session_notes_sync::sync_session_notes_panel_selected_editor;
use super::edit_history::show_edit_history;

pub(super) fn show_edit_mode(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.add_space(12.0);
    let mut save_edit = false;
    let mut cancel_edit = false;
    let mut record_snapshot = false;
    let mut restore_selected = false;
    let mut editor_text = app
        .session_notes_panel_demo
        .selected_editor()
        .map(|editor| editor.text().to_owned())
        .unwrap_or_default();

    section_card(
        ui,
        "Edit selected note",
        "Selected-note editor plus revision-history controls routed through the adapter state.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                save_edit = ui.button("Save Edit").clicked();
                cancel_edit = ui.button("Cancel Edit").clicked();
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label("snapshot");
                ui.add_sized(
                    [160.0, 28.0],
                    egui::TextEdit::singleline(&mut app.session_notes_panel_demo_snapshot_label)
                        .hint_text("label"),
                );
                ui.add_sized(
                    [220.0, 28.0],
                    egui::TextEdit::singleline(&mut app.session_notes_panel_demo_recorded_at)
                        .hint_text("recorded_at"),
                );
                record_snapshot = ui.button("Record Snapshot").clicked();
                restore_selected = ui.button("Restore Selected").clicked();
            });
            ui.add_space(8.0);
            let response = ui.add_sized(
                [ui.available_width(), 200.0],
                egui::TextEdit::multiline(&mut editor_text)
                    .hint_text("Selected note editor")
                    .desired_rows(9),
            );
            if response.changed() {
                sync_session_notes_panel_selected_editor(
                    &mut app.session_notes_panel_demo,
                    &editor_text,
                );
            }
        },
    );

    if save_edit && app.session_notes_panel_demo.save_selected_edit() {
        app.push_log("Session notes panel demo saved the selected edit.");
    }
    if cancel_edit {
        app.session_notes_panel_demo.cancel_edit();
        app.push_log("Session notes panel demo cancelled selected-note editing.");
    }
    if record_snapshot
        && app.session_notes_panel_demo.record_selected_snapshot(
            app.session_notes_panel_demo_snapshot_label.clone(),
            app.session_notes_panel_demo_recorded_at.clone(),
        )
    {
        app.push_log("Session notes panel demo recorded an edit snapshot.");
    }
    if restore_selected && app.session_notes_panel_demo.restore_selected_revision() {
        app.push_log("Session notes panel demo restored the selected edit snapshot.");
    }

    ui.add_space(12.0);
    show_edit_history(ui, app);
}
