use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::sandbox_toolbar;

pub(super) fn show_toolbar(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut add_clicked = false;
    let mut should_submit = false;

    sandbox_toolbar(
        ui,
        "Session notes demo",
        "Headless note ledger with query filtering, pinning, selection, and deterministic insertion order.",
        |ui| {
            ui.label("Add note");
            ui.horizontal(|ui| {
                let response = ui.add_sized(
                    [ui.available_width() - 168.0, 28.0],
                    egui::TextEdit::singleline(&mut app.session_notes_demo_draft)
                        .hint_text("Write a session note"),
                );
                should_submit =
                    response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
                ui.add_sized(
                    [148.0, 28.0],
                    egui::TextEdit::singleline(&mut app.session_notes_demo_created_at)
                        .hint_text("created_at"),
                );
                add_clicked = ui.button("Add").clicked();
            });
            ui.add_space(6.0);
            ui.label("Filter");
            let mut query = app.session_notes_demo.query.clone();
            if ui
                .add_sized(
                    [ui.available_width(), 28.0],
                    egui::TextEdit::singleline(&mut query)
                        .hint_text("Filter by note text or timestamp"),
                )
                .changed()
            {
                app.session_notes_demo.set_query(query);
            }
        },
    );
    ui.add_space(8.0);

    if (should_submit || add_clicked)
        && app.session_notes_demo.add_entry(
            app.session_notes_demo_draft.clone(),
            app.session_notes_demo_created_at.clone(),
        )
    {
        app.session_notes_demo_draft.clear();
        app.push_log("Session notes demo added an entry.");
    }
}
