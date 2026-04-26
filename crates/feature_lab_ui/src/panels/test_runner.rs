use eframe::egui;

use crate::app::FeatureLabApp;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.heading("Test Runner");
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.monospace(&app.test_output);
    });
}
