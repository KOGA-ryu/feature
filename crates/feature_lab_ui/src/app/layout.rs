mod bottom_panels;
mod central_panel;
mod side_panels;
mod top_bar;

use eframe::egui;

use self::bottom_panels::show_bottom_panels;
use self::central_panel::show_central_panel;
use self::side_panels::show_side_panels;
use self::top_bar::show_top_bar;
use super::FeatureLabApp;

impl eframe::App for FeatureLabApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Self::apply_visuals(ctx, self.glass_enabled);
        let style = ctx.style();
        let counts = self.feature_counts();
        let selected_summary = self
            .browser
            .selected_manifest()
            .map(|manifest| format!("{} · {}", manifest.id, manifest.status.to_string()))
            .unwrap_or_else(|| "No feature selected".into());

        show_top_bar(ctx, self, style.as_ref(), &selected_summary, counts);
        show_side_panels(ctx, self, style.as_ref());
        show_bottom_panels(ctx, self, style.as_ref());
        show_central_panel(ctx, self, style.as_ref());
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        #[cfg(target_os = "macos")]
        {
            if self.glass_enabled {
                egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
            } else {
                egui::Color32::from_rgb(14, 16, 20).to_normalized_gamma_f32()
            }
        }
        #[cfg(not(target_os = "macos"))]
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
    }
}
