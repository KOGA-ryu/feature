use eframe::egui;

use crate::app::FeatureLabApp;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.heading("Metadata Inspector");
    ui.small("Contract, ownership, and reusable shape.");
    ui.separator();

    let Some(feature) = app.selected_feature() else {
        ui.label("Select a feature to inspect metadata.");
        return;
    };

    egui::ScrollArea::vertical()
        .id_salt("metadata_inspector_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            section_card(
                ui,
                "Contract snapshot",
                "Canonical metadata and ownership fields from feature.toml.",
                |ui| {
                    ui.horizontal_wrapped(|ui| {
                        info_chip(
                            ui,
                            &feature.manifest.kind.to_string(),
                            kind_color(&feature.manifest.id),
                        );
                        info_chip(
                            ui,
                            &feature.manifest.status.to_string(),
                            lifecycle_color(&feature.manifest.status.to_string()),
                        );
                        info_chip(
                            ui,
                            &feature.manifest.owner,
                            egui::Color32::from_rgb(188, 199, 220),
                        );
                    });
                    ui.add_space(10.0);
                    egui::Grid::new("feature_metadata_grid")
                        .num_columns(2)
                        .spacing([12.0, 8.0])
                        .striped(true)
                        .show(ui, |ui| {
                            metadata_row(ui, "id", &feature.manifest.id);
                            metadata_row(ui, "created", &feature.manifest.created_at);
                            metadata_row(ui, "updated", &feature.manifest.updated_at);
                            metadata_row(ui, "crate", &feature.package_name);
                        });
                    ui.add_space(10.0);
                    ui.small(egui::RichText::new("feature directory").strong());
                    ui.add_space(4.0);
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
                        .stroke(egui::Stroke::new(
                            1.0,
                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18),
                        ))
                        .corner_radius(12.0)
                        .inner_margin(egui::Margin::symmetric(10, 8))
                        .show(ui, |ui| {
                            ui.small(
                                egui::RichText::new(feature.feature_dir.display().to_string())
                                    .monospace(),
                            );
                        });
                },
            );

            ui.add_space(10.0);
            section_card(
                ui,
                "Signals",
                "Inputs, outputs, dependencies, compatibility, and reusable tags.",
                |ui| {
                    chip_group(ui, "tags", &feature.manifest.tags);
                    chip_group(ui, "dependencies", &feature.manifest.dependencies);
                    chip_group(ui, "compatible", &feature.manifest.compatible_features);
                    chip_group(ui, "inputs", &feature.manifest.inputs.items);
                    chip_group(ui, "outputs", &feature.manifest.outputs.items);
                },
            );

            ui.add_space(10.0);
            section_card(
                ui,
                "JSON manifest",
                "Raw machine-readable payload used by registry and CLI tooling.",
                |ui| {
                    let metadata_json = serde_json::to_string_pretty(&feature.manifest)
                        .unwrap_or_else(|error| error.to_string());
                    egui::ScrollArea::vertical()
                        .max_height(280.0)
                        .show(ui, |ui| {
                            ui.monospace(metadata_json);
                        });
                },
            );
        });
}

fn metadata_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.small(egui::RichText::new(label).strong());
    ui.small(value);
    ui.end_row();
}

fn chip_group(ui: &mut egui::Ui, label: &str, values: &[String]) {
    ui.small(egui::RichText::new(label).strong());
    ui.add_space(4.0);
    if values.is_empty() {
        ui.small("none");
        ui.add_space(8.0);
        return;
    }
    ui.horizontal_wrapped(|ui| {
        for value in values {
            egui::Frame::new()
                .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 12))
                .stroke(egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 24),
                ))
                .corner_radius(999.0)
                .inner_margin(egui::Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    ui.small(egui::RichText::new(value).monospace());
                });
        }
    });
    ui.add_space(8.0);
}

fn section_card(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20),
        ))
        .corner_radius(16.0)
        .inner_margin(egui::Margin::symmetric(14, 12))
        .show(ui, |ui| {
            ui.strong(title);
            ui.small(subtitle);
            ui.add_space(10.0);
            add_contents(ui);
        });
}

fn info_chip(ui: &mut egui::Ui, text: &str, color: egui::Color32) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            22,
        ))
        .stroke(egui::Stroke::new(1.0, color))
        .corner_radius(999.0)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.small(egui::RichText::new(text).color(color).strong());
        });
}

fn kind_color(feature_id: &str) -> egui::Color32 {
    if feature_id.starts_with("ui.") {
        egui::Color32::from_rgb(126, 188, 255)
    } else if feature_id.starts_with("logic.") {
        egui::Color32::from_rgb(255, 201, 110)
    } else if feature_id.starts_with("workflow.") {
        egui::Color32::from_rgb(177, 150, 255)
    } else {
        egui::Color32::from_gray(190)
    }
}

fn lifecycle_color(status: &str) -> egui::Color32 {
    match status {
        "stable" => egui::Color32::from_rgb(126, 217, 140),
        "tested" => egui::Color32::from_rgb(126, 188, 255),
        "experimental" => egui::Color32::from_rgb(255, 201, 110),
        "deprecated" => egui::Color32::from_rgb(255, 133, 133),
        _ => egui::Color32::from_gray(190),
    }
}
