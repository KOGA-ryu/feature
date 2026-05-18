use eframe::egui;
use theme_editor::{
    ThemePreset, sample_invalid_fixture, sample_light_fixture, sample_low_contrast_fixture,
};

use crate::app::FeatureLabApp;

pub(super) fn show_action_bar(
    ui: &mut egui::Ui,
    app: &mut FeatureLabApp,
    action_log: &mut Option<String>,
) {
    ui.group(|ui| {
        ui.strong("Theme editor demo");
        ui.label(
            "Preset switching, compact config edits, live preview, and export over semantic tokens.",
        );
        ui.horizontal_wrapped(|ui| {
            if ui.button("Dark preset").clicked() {
                apply_preset(
                    app,
                    ThemePreset::DefaultDark,
                    "Applied default dark preset.",
                    "Theme editor applied default dark preset.",
                    action_log,
                );
            }
            if ui.button("Light preset").clicked() {
                apply_preset(
                    app,
                    ThemePreset::DefaultLight,
                    "Applied default light preset.",
                    "Theme editor applied default light preset.",
                    action_log,
                );
            }
            if ui.button("Low contrast").clicked() {
                apply_preset(
                    app,
                    ThemePreset::LowContrast,
                    "Applied low contrast preset.",
                    "Theme editor applied low contrast preset.",
                    action_log,
                );
            }
            if ui.button("High accent").clicked() {
                apply_preset(
                    app,
                    ThemePreset::HighAccent,
                    "Applied high accent preset.",
                    "Theme editor applied high accent preset.",
                    action_log,
                );
            }
            if ui.button("Import light").clicked() {
                import_fixture(
                    app,
                    sample_light_fixture(),
                    "Imported default light fixture into editor.",
                    "Theme editor imported light fixture.",
                    action_log,
                );
            }
            if ui.button("Import invalid").clicked() {
                import_fixture(
                    app,
                    sample_invalid_fixture(),
                    "Imported invalid fixture to surface findings.",
                    "Theme editor imported invalid fixture.",
                    action_log,
                );
            }
            if ui.button("Import low contrast").clicked() {
                import_fixture(
                    app,
                    sample_low_contrast_fixture(),
                    "Imported low contrast fixture into editor.",
                    "Theme editor imported low contrast fixture.",
                    action_log,
                );
            }
            if ui.button("Copy theme").clicked() {
                match app.theme_editor_demo.copy_theme_payload() {
                    Ok(payload) => {
                        app.theme_editor_last_action =
                            format!("Prepared theme payload copy ({} bytes).", payload.len());
                        *action_log = Some("Theme editor prepared export payload.".into());
                    }
                    Err(error) => {
                        app.theme_editor_last_action = error.clone();
                        *action_log = Some(format!("Theme export failed: {error}"));
                    }
                }
            }
            if ui.button("Reset").clicked() {
                app.theme_editor_demo.reset_to_default();
                app.theme_editor_last_action = "Reset to default dark preset.".into();
                *action_log = Some("Theme editor reset to default dark preset.".into());
            }
        });
    });
}

fn apply_preset(
    app: &mut FeatureLabApp,
    preset: ThemePreset,
    success_action: &str,
    success_log: &str,
    action_log: &mut Option<String>,
) {
    match app.theme_editor_demo.apply_preset(preset) {
        Ok(()) => {
            app.theme_editor_last_action = success_action.into();
            *action_log = Some(success_log.into());
        }
        Err(error) => {
            app.theme_editor_last_action = error.clone();
            *action_log = Some(format!("Theme editor preset failed: {error}"));
        }
    }
}

fn import_fixture(
    app: &mut FeatureLabApp,
    raw: &str,
    success_action: &str,
    success_log: &str,
    action_log: &mut Option<String>,
) {
    match app.theme_editor_demo.import_theme(raw) {
        Ok(()) => {
            app.theme_editor_last_action = success_action.into();
            *action_log = Some(success_log.into());
        }
        Err(error) => {
            app.theme_editor_last_action = error.clone();
            *action_log = Some(format!("Theme import failed: {error}"));
        }
    }
}
