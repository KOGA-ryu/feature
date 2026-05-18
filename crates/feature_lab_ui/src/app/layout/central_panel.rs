use eframe::egui;

use crate::app::FeatureLabApp;
use crate::panels::demo_panel;

use super::super::style::glass_central_frame;

pub(super) fn show_central_panel(
    ctx: &egui::Context,
    app: &mut FeatureLabApp,
    style: &egui::Style,
) {
    egui::CentralPanel::default()
        .frame(glass_central_frame(
            style,
            app.central_panel_fill(),
            app.chrome_stroke(),
        ))
        .show(ctx, |ui| demo_panel::show(ui, app));
}
