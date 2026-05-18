use eframe::egui;

use super::FeatureLabApp;

impl FeatureLabApp {
    pub fn configure_platform_visuals(ctx: &egui::Context) {
        Self::apply_visuals(ctx, cfg!(target_os = "macos"));
    }

    pub(super) fn apply_visuals(ctx: &egui::Context, glass_enabled: bool) {
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();

        if glass_enabled {
            style.visuals.panel_fill = egui::Color32::from_rgba_unmultiplied(18, 20, 24, 102);
            style.visuals.window_fill = egui::Color32::from_rgba_unmultiplied(28, 31, 36, 140);
            style.visuals.faint_bg_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18);
            style.visuals.extreme_bg_color = egui::Color32::from_rgba_unmultiplied(8, 10, 14, 182);
            style.visuals.widgets.noninteractive.bg_fill =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 14);
            style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 32),
            );
            style.visuals.widgets.inactive.bg_fill =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20);
            style.visuals.widgets.inactive.weak_bg_fill =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 10);
            style.visuals.widgets.active.bg_fill =
                egui::Color32::from_rgba_unmultiplied(140, 176, 255, 58);
            style.visuals.widgets.hovered.bg_fill =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 28);
            style.visuals.window_stroke = egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 38),
            );
        } else {
            style.visuals.panel_fill = egui::Color32::from_rgb(23, 25, 30);
            style.visuals.window_fill = egui::Color32::from_rgb(34, 37, 44);
            style.visuals.faint_bg_color = egui::Color32::from_rgb(46, 50, 58);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(12, 13, 16);
            style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(34, 37, 44);
            style.visuals.widgets.noninteractive.bg_stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(62, 68, 78));
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(44, 48, 56);
            style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(34, 37, 44);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(64, 88, 140);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(52, 58, 68);
            style.visuals.window_stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(62, 68, 78));
        }

        style.spacing.item_spacing = egui::vec2(10.0, 8.0);
        style.spacing.button_padding = egui::vec2(10.0, 6.0);
        ctx.set_style(style);
    }

    pub(super) fn top_bar_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(18, 20, 24, 124)
        } else {
            egui::Color32::from_rgb(20, 22, 27)
        }
    }

    pub(super) fn left_panel_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(20, 22, 28, 104)
        } else {
            egui::Color32::from_rgb(24, 26, 32)
        }
    }

    pub(super) fn right_panel_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(22, 24, 30, 112)
        } else {
            egui::Color32::from_rgb(27, 29, 36)
        }
    }

    pub(super) fn bottom_panel_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(18, 20, 24, 116)
        } else {
            egui::Color32::from_rgb(21, 23, 29)
        }
    }

    pub(super) fn central_panel_fill(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(10, 12, 16, 72)
        } else {
            egui::Color32::from_rgb(17, 19, 24)
        }
    }

    pub(crate) fn chrome_stroke(&self) -> egui::Color32 {
        if self.glass_enabled {
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 28)
        } else {
            egui::Color32::from_rgb(64, 70, 82)
        }
    }
}

pub(super) fn glass_panel_frame(
    style: &egui::Style,
    fill: egui::Color32,
    stroke: egui::Color32,
) -> egui::Frame {
    egui::Frame::side_top_panel(style)
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
}

pub(super) fn glass_central_frame(
    style: &egui::Style,
    fill: egui::Color32,
    stroke: egui::Color32,
) -> egui::Frame {
    egui::Frame::central_panel(style)
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
}

pub(super) fn pill(ui: &mut egui::Ui, text: impl Into<String>, color: egui::Color32) {
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
            ui.small(egui::RichText::new(text).color(color).strong());
        });
}

pub(super) fn toolbar_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20),
        ))
        .corner_radius(14.0)
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| add_contents(ui));
        });
}

pub(super) fn status_pill(ui: &mut egui::Ui, text: impl Into<String>, color: egui::Color32) {
    let text = text.into();
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            20,
        ))
        .stroke(egui::Stroke::new(1.0, color))
        .corner_radius(999.0)
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.small(egui::RichText::new(text).color(color).strong().monospace());
        });
}
