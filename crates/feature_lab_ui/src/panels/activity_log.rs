use eframe::egui;

use crate::app::FeatureLabApp;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.heading("Activity Log");
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        for entry in app.activity_log.iter().rev() {
            ui.label(entry);
        }
    });
}
