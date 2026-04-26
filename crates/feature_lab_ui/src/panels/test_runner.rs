use eframe::egui;

use crate::app::FeatureLabApp;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let selected_feature = app
        .selected_feature_id
        .as_deref()
        .unwrap_or("No feature selected");
    let selected_feature = selected_feature.to_owned();
    let line_count = app.test_output.lines().count();
    let (state_label, state_color) = app.test_output_state();

    ui.heading("Test Runner");
    ui.small("Latest verification output for the selected feature.");
    ui.add_space(8.0);

    stats_row(ui, |ui| {
        stat_chip(ui, selected_feature, egui::Color32::from_rgb(188, 199, 220));
        stat_chip(
            ui,
            format!("{line_count} lines"),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(ui, state_label, state_color);
        if ui.button("Run Tests").clicked() {
            app.run_selected_tests();
        }
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
                .id_salt("test_output_scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.monospace(&app.test_output);
                });
        });
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
