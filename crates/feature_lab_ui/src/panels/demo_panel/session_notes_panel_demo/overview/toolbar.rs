use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::super::cards::sandbox_toolbar;

pub(super) fn show_toolbar(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut start_compose = false;
    let mut start_edit = false;
    let mut toggle_pin = false;
    let mut delete_selected = false;

    sandbox_toolbar(
        ui,
        "Session notes panel demo",
        "Host-state adapter over note storage, plain-text editing, and revision history for richer operator notes workflows.",
        |ui| {
            ui.label("Filter");
            let mut query = app.session_notes_panel_demo.notes().query.clone();
            if ui
                .add_sized(
                    [ui.available_width(), 28.0],
                    egui::TextEdit::singleline(&mut query)
                        .hint_text("Filter by note text or timestamp"),
                )
                .changed()
            {
                app.session_notes_panel_demo.set_filter_query(query);
            }
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                start_compose = ui.button("Start Compose").clicked();
                start_edit = ui.button("Edit Selected").clicked();
                toggle_pin = ui.button("Toggle Pin").clicked();
                delete_selected = ui.button("Delete Selected").clicked();
            });
        },
    );
    ui.add_space(8.0);

    if start_compose {
        app.session_notes_panel_demo.start_compose();
        app.push_log("Session notes panel demo entered compose mode.");
    }
    if start_edit {
        if app.session_notes_panel_demo.start_edit_selected() {
            app.push_log("Session notes panel demo entered selected-note edit mode.");
        } else {
            app.push_log("Session notes panel demo could not start edit mode.");
        }
    }
    if toggle_pin && app.session_notes_panel_demo.toggle_pinned_selected() {
        app.push_log("Session notes panel demo toggled the selected note pin state.");
    }
    if delete_selected && app.session_notes_panel_demo.delete_selected() {
        app.push_log("Session notes panel demo deleted the selected note.");
    }
}
