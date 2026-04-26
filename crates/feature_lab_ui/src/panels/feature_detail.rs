use eframe::egui;

use crate::app::FeatureLabApp;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.heading("Metadata Inspector");
    ui.separator();

    let Some(feature) = app.selected_feature() else {
        ui.label("Select a feature to inspect metadata.");
        return;
    };

    ui.label(format!("id: {}", feature.manifest.id));
    ui.label(format!("kind: {}", feature.manifest.kind));
    ui.label(format!("status: {}", feature.manifest.status));
    ui.label(format!("owner: {}", feature.manifest.owner));
    ui.label(format!("created_at: {}", feature.manifest.created_at));
    ui.label(format!("updated_at: {}", feature.manifest.updated_at));
    ui.separator();
    ui.label(format!("crate: {}", feature.package_name));
    ui.label(format!("dir: {}", feature.feature_dir.display()));
    ui.separator();

    let metadata_json =
        serde_json::to_string_pretty(&feature.manifest).unwrap_or_else(|error| error.to_string());
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.monospace(metadata_json);
    });
}
