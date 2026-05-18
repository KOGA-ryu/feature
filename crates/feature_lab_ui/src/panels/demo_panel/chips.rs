use eframe::egui;

pub(super) fn stats_row(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
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

pub(super) fn stat_chip(ui: &mut egui::Ui, text: impl Into<String>, color: egui::Color32) {
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

pub(super) fn activation_chip(ui: &mut egui::Ui, message: &str) {
    let lowered = message.to_lowercase();
    let color = if lowered.contains("blocked") {
        egui::Color32::from_rgb(255, 133, 133)
    } else if lowered.contains("activated") {
        egui::Color32::from_rgb(126, 217, 140)
    } else {
        egui::Color32::from_rgb(188, 199, 220)
    };
    stat_chip(ui, message, color);
}

pub(super) fn status_chip(ui: &mut egui::Ui, text: &str, color: egui::Color32) {
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

pub(super) fn metric_tile(ui: &mut egui::Ui, label: &str, value: usize, color: egui::Color32) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18),
        ))
        .corner_radius(14.0)
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.small(egui::RichText::new(label).color(color).strong());
            ui.label(
                egui::RichText::new(value.to_string())
                    .heading()
                    .color(color),
            );
        });
}
