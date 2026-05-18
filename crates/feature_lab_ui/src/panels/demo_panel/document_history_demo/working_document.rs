use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::section_card;

pub(super) fn show_working_document(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    section_card(
        ui,
        "Working document",
        "Edit the current in-memory document text, then record or restore named snapshots.",
        |ui| {
            let mut working_text = app.document_history_demo.working_text().to_owned();
            let response = ui.add_sized(
                [ui.available_width(), 180.0],
                egui::TextEdit::multiline(&mut working_text)
                    .hint_text("Working document text")
                    .desired_rows(8),
            );
            if response.changed() {
                app.document_history_demo.set_working_text(working_text);
            }
        },
    );
}
