use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::sandbox_toolbar;

pub(super) fn show_toolbar(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut record_snapshot = false;
    let mut restore_selected = false;
    let mut mark_clean = false;

    sandbox_toolbar(
        ui,
        "Document history demo",
        "Single-document revision ledger with deterministic snapshot recording, restore, and clean-baseline dirty tracking.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Mark Clean").clicked() {
                    mark_clean = true;
                }
                if ui.button("Restore Selected").clicked() {
                    restore_selected = true;
                }
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Snapshot");
                ui.add_sized(
                    [140.0, 28.0],
                    egui::TextEdit::singleline(&mut app.document_history_demo_snapshot_label)
                        .hint_text("label"),
                );
                ui.add_sized(
                    [168.0, 28.0],
                    egui::TextEdit::singleline(&mut app.document_history_demo_recorded_at)
                        .hint_text("recorded_at"),
                );
                record_snapshot = ui.button("Record Snapshot").clicked();
            });
        },
    );
    ui.add_space(8.0);

    if mark_clean {
        app.document_history_demo.mark_clean();
        app.push_log("Document history demo marked the working text clean.");
    }
    if restore_selected && app.document_history_demo.restore_selected_revision() {
        app.push_log("Document history demo restored the selected revision.");
    }
    if record_snapshot
        && app.document_history_demo.record_snapshot(
            app.document_history_demo_snapshot_label.clone(),
            app.document_history_demo_recorded_at.clone(),
        )
    {
        app.push_log("Document history demo recorded a snapshot.");
    }
}
