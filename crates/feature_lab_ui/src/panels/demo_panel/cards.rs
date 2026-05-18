use eframe::egui;
use wiki_browser::WikiBrowserEntry;

use super::chips::{metric_tile, status_chip};
use super::status_colors::{has_live_demo, kind_color, lifecycle_color};

pub(super) fn hero_card(ui: &mut egui::Ui, feature: &WikiBrowserEntry) {
    let interactive_label = if has_live_demo(&feature.manifest.id) {
        "interactive bench"
    } else {
        "documentation preview"
    };

    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 10))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 28),
        ))
        .corner_radius(18.0)
        .inner_margin(egui::Margin::symmetric(18, 16))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.vertical(|ui| {
                    ui.heading(&feature.manifest.name);
                    ui.small(egui::RichText::new(&feature.manifest.id).monospace());
                });
                ui.add_space(12.0);
                status_chip(
                    ui,
                    &feature.manifest.kind.to_string(),
                    kind_color(&feature.manifest.id),
                );
                status_chip(
                    ui,
                    &feature.manifest.status.to_string(),
                    lifecycle_color(&feature.manifest.status.to_string()),
                );
                status_chip(
                    ui,
                    interactive_label,
                    egui::Color32::from_rgb(176, 197, 255),
                );
            });

            ui.add_space(10.0);
            ui.label(&feature.manifest.summary);
            ui.add_space(10.0);

            ui.horizontal_wrapped(|ui| {
                metric_tile(
                    ui,
                    "tags",
                    feature.manifest.tags.len(),
                    egui::Color32::from_rgb(126, 188, 255),
                );
                metric_tile(
                    ui,
                    "deps",
                    feature.manifest.dependencies.len(),
                    egui::Color32::from_rgb(255, 201, 110),
                );
                metric_tile(
                    ui,
                    "inputs",
                    feature.manifest.inputs.items.len(),
                    egui::Color32::from_rgb(126, 217, 140),
                );
                metric_tile(
                    ui,
                    "outputs",
                    feature.manifest.outputs.items.len(),
                    egui::Color32::from_rgb(220, 168, 255),
                );
                metric_tile(
                    ui,
                    "compatible",
                    feature.manifest.compatible_features.len(),
                    egui::Color32::from_rgb(255, 150, 195),
                );
            });

            if !feature.manifest.tags.is_empty() {
                ui.add_space(10.0);
                ui.small(egui::RichText::new("Tags").strong());
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    for tag in &feature.manifest.tags {
                        status_chip(ui, tag, egui::Color32::from_rgb(180, 188, 204));
                    }
                });
            }
        });
}

pub(super) fn section_card(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 22),
        ))
        .corner_radius(16.0)
        .inner_margin(egui::Margin::symmetric(16, 14))
        .show(ui, |ui| {
            ui.strong(title);
            ui.small(subtitle);
            ui.add_space(10.0);
            add_contents(ui);
        });
}

pub(super) fn sandbox_toolbar(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    add_controls: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 10))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 24),
        ))
        .corner_radius(14.0)
        .inner_margin(egui::Margin::symmetric(14, 12))
        .show(ui, |ui| {
            ui.strong(title);
            ui.small(subtitle);
            ui.add_space(8.0);
            add_controls(ui);
        });
}

pub(super) fn result_card(
    ui: &mut egui::Ui,
    selected: bool,
    accent: egui::Color32,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    let fill = if selected {
        egui::Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 24)
    } else {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 6)
    };
    let stroke = if selected {
        accent
    } else {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18)
    };
    egui::Frame::group(ui.style())
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
        .corner_radius(12.0)
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| add_contents(ui));
}

pub(super) fn empty_state_card(ui: &mut egui::Ui, message: &str) {
    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18),
        ))
        .corner_radius(14.0)
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.strong("Empty state");
            ui.small(message);
        });
}
