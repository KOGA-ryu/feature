use eframe::egui;

use crate::app::FeatureLabApp;

pub(super) fn show_inspector(ui: &mut egui::Ui, app: &FeatureLabApp) {
    ui.group(|ui| {
        ui.strong("Inspector");
        ui.label(format!(
            "Findings: {}",
            app.theme_editor_demo.findings().len()
        ));
        for finding in app.theme_editor_demo.findings() {
            ui.group(|ui| {
                ui.monospace(format!("{} | {}", finding.severity, finding.field));
                ui.label(&finding.message);
            });
            ui.add_space(4.0);
        }
        if app.theme_editor_demo.findings().is_empty() {
            ui.label("No findings.");
        }

        egui::CollapsingHeader::new("Export payload")
            .default_open(true)
            .show(ui, |ui| match app.theme_editor_demo.export_theme_json() {
                Ok(payload) => {
                    egui::ScrollArea::vertical()
                        .id_salt("theme_editor_export_payload_scroll")
                        .max_height(220.0)
                        .show(ui, |ui| {
                            ui.monospace(payload);
                        });
                }
                Err(error) => {
                    ui.label(error);
                }
            });
    });
}
