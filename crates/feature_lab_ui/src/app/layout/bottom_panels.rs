use eframe::egui;

use crate::app::FeatureLabApp;
use crate::panels::{activity_log, test_runner};

use super::super::style::{glass_panel_frame, status_pill};

pub(super) fn show_bottom_panels(
    ctx: &egui::Context,
    app: &mut FeatureLabApp,
    style: &egui::Style,
) {
    show_test_output(ctx, app, style);
    show_status_strip(ctx, app, style);
}

fn show_test_output(ctx: &egui::Context, app: &mut FeatureLabApp, style: &egui::Style) {
    egui::TopBottomPanel::bottom("test_output")
        .resizable(true)
        .default_height(220.0)
        .frame(glass_panel_frame(
            style,
            app.bottom_panel_fill(),
            app.chrome_stroke(),
        ))
        .show(ctx, |ui| {
            let total_size = ui.available_size_before_wrap();
            let spacing = ui.spacing().item_spacing.x;
            let left_width = ((total_size.x - spacing) * 0.6).max(280.0);
            let right_width = (total_size.x - spacing - left_width).max(180.0);

            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(left_width, total_size.y),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| test_runner::show(ui, app),
                );
                ui.allocate_ui_with_layout(
                    egui::vec2(right_width, total_size.y),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| activity_log::show(ui, app),
                );
            });
        });
}

fn show_status_strip(ctx: &egui::Context, app: &mut FeatureLabApp, style: &egui::Style) {
    egui::TopBottomPanel::bottom("status_strip")
        .resizable(false)
        .exact_height(42.0)
        .frame(glass_panel_frame(
            style,
            app.bottom_panel_fill(),
            app.chrome_stroke(),
        ))
        .show(ctx, |ui| {
            let (test_state, test_state_color) = app.test_output_state();
            let visible_count = app.visible_feature_count();
            let selected_feature = app.selected_feature_id().unwrap_or("No feature selected");
            let latest_event = app.latest_activity_entry().unwrap_or("No activity yet.");

            ui.horizontal_wrapped(|ui| {
                status_pill(
                    ui,
                    format!("selected {selected_feature}"),
                    egui::Color32::from_rgb(188, 199, 220),
                );
                status_pill(
                    ui,
                    format!("filter {}", app.kind_filter().label()),
                    egui::Color32::from_rgb(126, 188, 255),
                );
                status_pill(
                    ui,
                    format!("{visible_count} visible"),
                    egui::Color32::from_rgb(255, 201, 110),
                );
                status_pill(ui, format!("tests {test_state}"), test_state_color);
                ui.separator();
                ui.small(
                    egui::RichText::new(format!("latest: {latest_event}"))
                        .monospace()
                        .color(egui::Color32::from_gray(185)),
                );
            });
        });
}
