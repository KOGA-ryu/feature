use eframe::egui;

use crate::app::FeatureLabApp;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.heading("Activity Log");
    ui.small("Recent operator and demo actions.");
    ui.add_space(8.0);

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("{} entries", app.activity_log.len()),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(ui, "newest first", egui::Color32::from_rgb(126, 188, 255));
    });

    ui.add_space(8.0);
    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18),
        ))
        .corner_radius(14.0)
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("activity_log_scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for entry in app.activity_log.iter().rev() {
                        let severity = entry_severity(entry);
                        egui::Frame::group(ui.style())
                            .fill(egui::Color32::from_rgba_unmultiplied(
                                severity.color.r(),
                                severity.color.g(),
                                severity.color.b(),
                                18,
                            ))
                            .stroke(egui::Stroke::new(1.0, severity.color))
                            .corner_radius(12.0)
                            .inner_margin(egui::Margin::symmetric(10, 8))
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    stat_chip(ui, severity.label, severity.color);
                                });
                                ui.add_space(4.0);
                                ui.label(entry);
                            });
                        ui.add_space(6.0);
                    }
                });
        });
}

struct SeverityStyle {
    label: &'static str,
    color: egui::Color32,
}

fn entry_severity(entry: &str) -> SeverityStyle {
    let lowered = entry.to_lowercase();

    if lowered.contains("failed")
        || lowered.contains("error")
        || lowered.contains("blocked")
        || lowered.contains("invalid")
    {
        SeverityStyle {
            label: "error",
            color: egui::Color32::from_rgb(255, 133, 133),
        }
    } else if lowered.contains("opened")
        || lowered.contains("ran tests")
        || lowered.contains("activated")
        || lowered.contains("applied")
        || lowered.contains("imported")
        || lowered.contains("reset")
        || lowered.contains("selected")
        || lowered.contains("loaded")
    {
        SeverityStyle {
            label: "activity",
            color: egui::Color32::from_rgb(126, 217, 140),
        }
    } else {
        SeverityStyle {
            label: "info",
            color: egui::Color32::from_rgb(188, 199, 220),
        }
    }
}

fn stats_row(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18),
        ))
        .corner_radius(12.0)
        .inner_margin(egui::Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| add_contents(ui));
        });
}

fn stat_chip(ui: &mut egui::Ui, text: impl Into<String>, color: egui::Color32) {
    let text = text.into();
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
            ui.small(egui::RichText::new(text).color(color).strong().monospace());
        });
}
