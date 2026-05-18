use eframe::egui;

use crate::app::FeatureLabApp;

pub(super) fn show_preview(ui: &mut egui::Ui, app: &FeatureLabApp) {
    ui.group(|ui| {
        ui.strong("Preview");
        ui.label(format!(
            "Preset: {} | Resolved mode: {}",
            app.theme_editor_demo.preset().label(),
            app.theme_editor_demo.resolved_mode()
        ));
        ui.label(&app.theme_editor_last_action);
        ui.add_space(8.0);
        show_theme_preview(ui, app);
    });
}

fn show_theme_preview(ui: &mut egui::Ui, app: &FeatureLabApp) {
    let preview = app.theme_editor_demo.preview();
    let app_background = color_from_hex(&preview.app_background, egui::Color32::from_gray(24));
    let focus_ring = color_from_hex(&preview.focus_ring, egui::Color32::LIGHT_BLUE);
    let selection_bg = color_from_hex(&preview.selection_bg, egui::Color32::DARK_BLUE);

    egui::Frame::default()
        .fill(app_background)
        .stroke(egui::Stroke::new(1.0, focus_ring))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "UI font: {} | Code font: {}",
                    preview.ui_font, preview.code_font
                ))
                .color(color_from_hex(
                    &preview.muted_text,
                    egui::Color32::LIGHT_GRAY,
                )),
            );
            ui.add_space(6.0);

            for surface in &preview.surfaces {
                let background = color_from_hex(&surface.background, egui::Color32::from_gray(40));
                let foreground = color_from_hex(&surface.foreground, egui::Color32::WHITE);
                let border = color_from_hex(&surface.border, egui::Color32::GRAY);
                egui::Frame::default()
                    .fill(background)
                    .stroke(egui::Stroke::new(1.0, border))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&surface.title)
                                .strong()
                                .color(foreground),
                        );
                        ui.label(egui::RichText::new(&surface.detail).color(foreground));
                        if surface.id == "command_button" {
                            let button_fill = surface
                                .accent
                                .as_deref()
                                .map(|hex| color_from_hex(hex, selection_bg))
                                .unwrap_or(selection_bg);
                            let button = egui::Button::new(
                                egui::RichText::new("Run command").color(foreground),
                            )
                            .fill(button_fill)
                            .stroke(egui::Stroke::new(1.0, focus_ring));
                            ui.add(button);
                        } else if let Some(accent) = &surface.accent {
                            ui.label(
                                egui::RichText::new(format!("accent {}", accent))
                                    .color(color_from_hex(accent, egui::Color32::LIGHT_BLUE)),
                            );
                        }
                    });
                ui.add_space(6.0);
            }
        });
}

fn color_from_hex(raw: &str, fallback: egui::Color32) -> egui::Color32 {
    let trimmed = raw.trim().trim_start_matches('#');
    if trimmed.len() != 6 {
        return fallback;
    }

    let Ok(value) = u32::from_str_radix(trimmed, 16) else {
        return fallback;
    };

    egui::Color32::from_rgb(
        ((value >> 16) & 0xff) as u8,
        ((value >> 8) & 0xff) as u8,
        (value & 0xff) as u8,
    )
}
