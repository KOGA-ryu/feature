use eframe::egui;

use crate::app::FeatureLabApp;
use crate::panels::{feature_detail, feature_list};

use super::super::style::glass_panel_frame;

pub(super) fn show_side_panels(ctx: &egui::Context, app: &mut FeatureLabApp, style: &egui::Style) {
    egui::SidePanel::left("feature_list")
        .resizable(true)
        .default_width(260.0)
        .frame(glass_panel_frame(
            style,
            app.left_panel_fill(),
            app.chrome_stroke(),
        ))
        .show(ctx, |ui| feature_list::show(ui, app));

    egui::SidePanel::right("feature_detail")
        .resizable(true)
        .default_width(320.0)
        .frame(glass_panel_frame(
            style,
            app.right_panel_fill(),
            app.chrome_stroke(),
        ))
        .show(ctx, |ui| feature_detail::show(ui, app));
}
