use eframe::egui;
use session_notes_panel::SessionNotesPanelMode;

use crate::app::FeatureLabApp;

use super::super::cards::section_card;
use super::super::session_notes_sync::sync_session_notes_panel_composer;

pub(super) fn show_compose_mode(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.add_space(12.0);
    let mut commit_compose = false;
    let mut cancel_compose = false;
    let mut composer_text = app.session_notes_panel_demo.composer().text().to_owned();

    section_card(
        ui,
        "Compose note",
        "Dedicated compose editor routed through the panel adapter.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("created_at");
                ui.add_sized(
                    [220.0, 28.0],
                    egui::TextEdit::singleline(&mut app.session_notes_panel_demo_created_at)
                        .hint_text("created_at"),
                );
                commit_compose = ui.button("Commit Compose").clicked();
                cancel_compose = ui.button("Cancel").clicked();
            });
            ui.add_space(8.0);
            let response = ui.add_sized(
                [ui.available_width(), 180.0],
                egui::TextEdit::multiline(&mut composer_text)
                    .hint_text("Compose a new note")
                    .desired_rows(8),
            );
            if response.changed() {
                sync_session_notes_panel_composer(
                    &mut app.session_notes_panel_demo,
                    &composer_text,
                );
            }
        },
    );

    if commit_compose
        && app
            .session_notes_panel_demo
            .commit_compose(app.session_notes_panel_demo_created_at.clone())
    {
        app.push_log("Session notes panel demo committed a composed note.");
    }
    if cancel_compose {
        app.session_notes_panel_demo.mode = SessionNotesPanelMode::Browse;
        app.session_notes_panel_demo
            .composer
            .load_text(String::new());
        app.push_log("Session notes panel demo cancelled compose mode.");
    }
}
