use activity_stream::sample_stream;
use command_palette::sample_palette;
use eframe::egui;
use left_rail::sample_rail;
use theme_editor::{
    ThemeMode, ThemePreset, sample_invalid_fixture, sample_light_fixture,
    sample_low_contrast_fixture,
};
use validation_pipeline::{
    ValidationPipeline, sample_invalid_input, sample_valid_input, validate_input,
};

use crate::app::FeatureLabApp;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.heading("Demo / Documentation");
    ui.separator();

    let Some(feature) = app.selected_feature() else {
        ui.label("Select a feature to preview its documentation or demo surface.");
        return;
    };

    ui.heading(&feature.manifest.name);
    ui.label(&feature.manifest.summary);
    ui.add_space(8.0);

    if feature.manifest.id == "logic.validation_pipeline" {
        show_validation_pipeline_demo(ui, app);
        ui.add_space(12.0);
    } else if feature.manifest.id == "ui.activity_stream" {
        show_activity_stream_demo(ui, app);
        ui.add_space(12.0);
    } else if feature.manifest.id == "ui.command_palette" {
        show_command_palette_demo(ui, app);
        ui.add_space(12.0);
    } else if feature.manifest.id == "ui.left_rail" {
        show_left_rail_demo(ui, app);
        ui.add_space(12.0);
    } else if feature.manifest.id == "ui.theme_editor" {
        show_theme_editor_demo(ui, app);
        ui.add_space(12.0);
    } else if feature.manifest.id == "ui.right_inspector" {
        ui.group(|ui| {
            ui.strong("Demo placeholder");
            ui.label("This feature represents a persistent right-side inspector.");
            ui.label("Expected actions:");
            for action in [
                "inspect selected object",
                "show linked artifacts",
                "surface contextual actions",
            ] {
                ui.label(format!("- {action}"));
            }
        });
        ui.add_space(12.0);
    }

    egui::CollapsingHeader::new("README preview")
        .default_open(true)
        .show(ui, |ui| match feature.readme_text() {
            Ok(readme) => {
                egui::ScrollArea::vertical()
                    .max_height(260.0)
                    .show(ui, |ui| {
                        ui.monospace(readme);
                    });
            }
            Err(error) => {
                ui.label(error.to_string());
            }
        });

    egui::CollapsingHeader::new("Fixture preview")
        .default_open(true)
        .show(ui, |ui| match feature.first_fixture_preview() {
            Ok(Some(fixture)) => {
                egui::ScrollArea::vertical()
                    .max_height(220.0)
                    .show(ui, |ui| {
                        ui.monospace(fixture);
                    });
            }
            Ok(None) => {
                ui.label("No fixture files found.");
            }
            Err(error) => {
                ui.label(error.to_string());
            }
        });
}

fn show_command_palette_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut move_up = false;
    let mut move_down = false;
    let mut activate = false;

    ui.group(|ui| {
        ui.strong("Command palette demo");
        ui.label("Pure Rust filtering, grouping, selection, and activation state.");
        ui.horizontal(|ui| {
            ui.label("Query");
            ui.text_edit_singleline(&mut app.command_palette_demo_query);
            move_up = ui.button("Up").clicked();
            move_down = ui.button("Down").clicked();
            activate = ui.button("Activate").clicked();
        });
    });
    ui.add_space(8.0);

    let mut palette = match sample_palette() {
        Ok(palette) => palette,
        Err(error) => {
            ui.label(error);
            return;
        }
    };

    palette.set_query(app.command_palette_demo_query.clone());
    palette.set_selected_index(app.command_palette_demo_selected_index);

    if move_up {
        palette.move_up();
    }
    if move_down {
        palette.move_down();
    }

    let mut log_message = None;
    if activate {
        match palette.activate_selected() {
            Some(command_id) => {
                app.command_palette_last_activation = format!("Activated: {command_id}");
                log_message = Some(format!("Command palette activated {command_id}"));
            }
            None => {
                app.command_palette_last_activation =
                    "Activation blocked: selected command is disabled or missing.".into();
                log_message = Some("Command palette activation was blocked.".into());
            }
        }
    }

    app.command_palette_demo_selected_index = palette.selected_index();

    ui.label(format!(
        "Selected index: {}",
        app.command_palette_demo_selected_index
    ));
    ui.label(&app.command_palette_last_activation);
    ui.add_space(8.0);

    if let Some(message) = palette.empty_state_message() {
        ui.label(message);
    } else {
        let selected_index = palette.selected_index();
        let mut flat_index = 0usize;
        for group in palette.grouped_results() {
            ui.group(|ui| {
                ui.strong(&group.category);
                for command in group.commands {
                    let marker = if flat_index == selected_index {
                        ">"
                    } else {
                        " "
                    };
                    let availability = if command.enabled {
                        "enabled"
                    } else {
                        "disabled"
                    };
                    let shortcut = command.shortcut.as_deref().unwrap_or("no shortcut");
                    ui.monospace(format!(
                        "{marker} {} [{availability}] ({shortcut})",
                        command.title
                    ));
                    ui.small(format!("{} | {}", command.subtitle, command.id));
                    flat_index += 1;
                }
            });
            ui.add_space(6.0);
        }
    }

    if let Some(message) = log_message {
        app.push_log(message);
    }
}

fn show_validation_pipeline_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut use_valid = false;
    let mut use_invalid = false;

    ui.group(|ui| {
        ui.strong("Validation pipeline demo");
        ui.label("Pure Rust validation rules, findings, severities, and aggregated output.");
        ui.horizontal(|ui| {
            use_valid = ui.button("Valid Fixture").clicked();
            use_invalid = ui.button("Invalid Fixture").clicked();
        });
    });
    ui.add_space(8.0);

    if use_valid {
        app.validation_pipeline_demo_use_invalid = false;
        app.push_log("Validation pipeline switched to valid fixture.");
    }
    if use_invalid {
        app.validation_pipeline_demo_use_invalid = true;
        app.push_log("Validation pipeline switched to invalid fixture.");
    }

    let input = if app.validation_pipeline_demo_use_invalid {
        match sample_invalid_input() {
            Ok(input) => input,
            Err(error) => {
                ui.label(error);
                return;
            }
        }
    } else {
        match sample_valid_input() {
            Ok(input) => input,
            Err(error) => {
                ui.label(error);
                return;
            }
        }
    };

    let pipeline = ValidationPipeline::default();
    let result = validate_input(&input);

    ui.label(format!(
        "Fixture: {}",
        if app.validation_pipeline_demo_use_invalid {
            "invalid"
        } else {
            "valid"
        }
    ));
    ui.label(format!("Rules active: {}", pipeline.rules().len()));
    ui.label(format!("Outcome: {}", result.summary.outcome));
    ui.label(format!(
        "Errors: {} | Warnings: {} | Total findings: {}",
        result.summary.error_count, result.summary.warning_count, result.summary.total_findings
    ));
    ui.label(format!(
        "Input counts: required={} collections={} timestamps={} dependencies={} compatible={}",
        input.required_fields.len(),
        input.collections.len(),
        input.timestamps.len(),
        input.dependencies.len(),
        input.compatible_features.len()
    ));
    ui.add_space(8.0);

    if result.findings.is_empty() {
        ui.label("No findings.");
    } else {
        for finding in result.findings {
            ui.group(|ui| {
                ui.monospace(format!(
                    "{} | {} | {}",
                    finding.severity, finding.code, finding.path
                ));
                ui.label(finding.message);
            });
            ui.add_space(6.0);
        }
    }
}

fn show_activity_stream_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut move_up = false;
    let mut move_down = false;
    let mut activate = false;

    ui.group(|ui| {
        ui.strong("Activity stream demo");
        ui.label("Pure Rust activity filtering, ordering, selection, and activation state.");
        ui.horizontal(|ui| {
            ui.label("Query");
            ui.text_edit_singleline(&mut app.activity_stream_demo_query);
            move_up = ui.button("Up").clicked();
            move_down = ui.button("Down").clicked();
            activate = ui.button("Activate").clicked();
        });
    });
    ui.add_space(8.0);

    let mut stream = match sample_stream() {
        Ok(stream) => stream,
        Err(error) => {
            ui.label(error);
            return;
        }
    };

    stream.set_query(app.activity_stream_demo_query.clone());
    stream.set_selected_index(app.activity_stream_demo_selected_index);

    if move_up {
        stream.move_up();
    }
    if move_down {
        stream.move_down();
    }

    let mut log_message = None;
    if activate {
        match stream.activate_selected() {
            Some(entry_id) => {
                app.activity_stream_last_activation = format!("Activated: {entry_id}");
                log_message = Some(format!("Activity stream activated {entry_id}"));
            }
            None => {
                app.activity_stream_last_activation =
                    "Activation blocked: selected entry is not actionable or missing.".into();
                log_message = Some("Activity stream activation was blocked.".into());
            }
        }
    }

    app.activity_stream_demo_selected_index = stream.selected_index();

    ui.label(format!(
        "Selected index: {}",
        app.activity_stream_demo_selected_index
    ));
    ui.label(format!(
        "Unread visible entries: {}",
        stream.visible_unread_count()
    ));
    ui.label(&app.activity_stream_last_activation);
    ui.add_space(8.0);

    if let Some(message) = stream.empty_state_message() {
        ui.label(message);
    } else {
        let selected_index = stream.selected_index();
        for (index, entry) in stream.visible_entries().into_iter().enumerate() {
            ui.group(|ui| {
                let marker = if index == selected_index { ">" } else { " " };
                let unread = if entry.unread { "unread" } else { "read" };
                let actionable = if entry.actionable {
                    "actionable"
                } else {
                    "static"
                };
                ui.monospace(format!(
                    "{marker} {} [{} | {} | {} | {}]",
                    entry.title, entry.kind, entry.status, unread, actionable
                ));
                ui.small(format!(
                    "{} | {} | {}",
                    entry.actor, entry.timestamp, entry.id
                ));
                ui.label(&entry.detail);
            });
            ui.add_space(6.0);
        }
    }

    if let Some(message) = log_message {
        app.push_log(message);
    }
}

fn show_left_rail_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut move_up = false;
    let mut move_down = false;
    let mut toggle_collapsed = false;
    let mut activate = false;

    ui.group(|ui| {
        ui.strong("Left rail demo");
        ui.label("Pure Rust navigation grouping, selection, collapse, and activation state.");
        ui.horizontal(|ui| {
            move_up = ui.button("Up").clicked();
            move_down = ui.button("Down").clicked();
            toggle_collapsed = ui.button("Toggle Collapse").clicked();
            activate = ui.button("Activate").clicked();
        });
    });
    ui.add_space(8.0);

    let mut rail = match sample_rail() {
        Ok(rail) => rail,
        Err(error) => {
            ui.label(error);
            return;
        }
    };

    rail.set_selected_index(app.left_rail_demo_selected_index);
    if app.left_rail_demo_collapsed {
        rail.set_collapsed(true);
    }

    if move_up {
        rail.move_up();
    }
    if move_down {
        rail.move_down();
    }
    if toggle_collapsed {
        rail.toggle_collapsed();
        app.push_log(format!(
            "Left rail {}",
            if rail.is_collapsed() {
                "collapsed"
            } else {
                "expanded"
            }
        ));
    }

    let mut log_message = None;
    if activate {
        match rail.activate_selected() {
            Some(item_id) => {
                app.left_rail_last_activation = format!("Activated: {item_id}");
                log_message = Some(format!("Left rail activated {item_id}"));
            }
            None => {
                app.left_rail_last_activation =
                    "Activation blocked: selected item is disabled or missing.".into();
                log_message = Some("Left rail activation was blocked.".into());
            }
        }
    }

    app.left_rail_demo_selected_index = rail.selected_index();
    app.left_rail_demo_collapsed = rail.is_collapsed();

    ui.label(format!(
        "State: {}",
        if rail.is_collapsed() {
            "collapsed"
        } else {
            "expanded"
        }
    ));
    ui.label(format!("Selected index: {}", rail.selected_index()));
    ui.label(format!("Badge total: {}", rail.total_badge_count()));
    ui.label(&app.left_rail_last_activation);
    ui.add_space(8.0);

    if let Some(message) = rail.empty_state_message() {
        ui.label(message);
    } else {
        let selected_index = rail.selected_index();
        let mut flat_index = 0usize;
        for section in rail.grouped_sections() {
            ui.group(|ui| {
                ui.strong(&section.name);
                for item in section.items {
                    let marker = if flat_index == selected_index {
                        ">"
                    } else {
                        " "
                    };
                    let availability = if item.enabled { "enabled" } else { "disabled" };
                    let label = if rail.is_collapsed() {
                        item.icon.clone()
                    } else {
                        item.title.clone()
                    };
                    let badge = item
                        .badge_count
                        .map(|count| format!(" badge:{count}"))
                        .unwrap_or_default();
                    ui.monospace(format!("{marker} {} [{availability}]{}", label, badge));
                    if !rail.is_collapsed() {
                        ui.small(item.id);
                    }
                    flat_index += 1;
                }
            });
            ui.add_space(6.0);
        }
    }

    if let Some(message) = log_message {
        app.push_log(message);
    }
}

fn show_theme_editor_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut action_log = None;

    ui.group(|ui| {
        ui.strong("Theme editor demo");
        ui.label("Preset switching, compact config edits, live preview, and export over semantic tokens.");
        ui.horizontal_wrapped(|ui| {
            if ui.button("Dark preset").clicked() {
                match app.theme_editor_demo.apply_preset(ThemePreset::DefaultDark) {
                    Ok(()) => {
                        app.theme_editor_last_action = "Applied default dark preset.".into();
                        action_log = Some("Theme editor applied default dark preset.".into());
                    }
                    Err(error) => {
                        app.theme_editor_last_action = error.clone();
                        action_log = Some(format!("Theme editor preset failed: {error}"));
                    }
                }
            }
            if ui.button("Light preset").clicked() {
                match app.theme_editor_demo.apply_preset(ThemePreset::DefaultLight) {
                    Ok(()) => {
                        app.theme_editor_last_action = "Applied default light preset.".into();
                        action_log = Some("Theme editor applied default light preset.".into());
                    }
                    Err(error) => {
                        app.theme_editor_last_action = error.clone();
                        action_log = Some(format!("Theme editor preset failed: {error}"));
                    }
                }
            }
            if ui.button("Low contrast").clicked() {
                match app.theme_editor_demo.apply_preset(ThemePreset::LowContrast) {
                    Ok(()) => {
                        app.theme_editor_last_action = "Applied low contrast preset.".into();
                        action_log = Some("Theme editor applied low contrast preset.".into());
                    }
                    Err(error) => {
                        app.theme_editor_last_action = error.clone();
                        action_log = Some(format!("Theme editor preset failed: {error}"));
                    }
                }
            }
            if ui.button("High accent").clicked() {
                match app.theme_editor_demo.apply_preset(ThemePreset::HighAccent) {
                    Ok(()) => {
                        app.theme_editor_last_action = "Applied high accent preset.".into();
                        action_log = Some("Theme editor applied high accent preset.".into());
                    }
                    Err(error) => {
                        app.theme_editor_last_action = error.clone();
                        action_log = Some(format!("Theme editor preset failed: {error}"));
                    }
                }
            }
            if ui.button("Import light").clicked() {
                match app.theme_editor_demo.import_theme(sample_light_fixture()) {
                    Ok(()) => {
                        app.theme_editor_last_action =
                            "Imported default light fixture into editor.".into();
                        action_log = Some("Theme editor imported light fixture.".into());
                    }
                    Err(error) => {
                        app.theme_editor_last_action = error.clone();
                        action_log = Some(format!("Theme import failed: {error}"));
                    }
                }
            }
            if ui.button("Import invalid").clicked() {
                match app.theme_editor_demo.import_theme(sample_invalid_fixture()) {
                    Ok(()) => {
                        app.theme_editor_last_action =
                            "Imported invalid fixture to surface findings.".into();
                        action_log = Some("Theme editor imported invalid fixture.".into());
                    }
                    Err(error) => {
                        app.theme_editor_last_action = error.clone();
                        action_log = Some(format!("Theme import failed: {error}"));
                    }
                }
            }
            if ui.button("Import low contrast").clicked() {
                match app.theme_editor_demo.import_theme(sample_low_contrast_fixture()) {
                    Ok(()) => {
                        app.theme_editor_last_action =
                            "Imported low contrast fixture into editor.".into();
                        action_log = Some("Theme editor imported low contrast fixture.".into());
                    }
                    Err(error) => {
                        app.theme_editor_last_action = error.clone();
                        action_log = Some(format!("Theme import failed: {error}"));
                    }
                }
            }
            if ui.button("Copy theme").clicked() {
                match app.theme_editor_demo.copy_theme_payload() {
                    Ok(payload) => {
                        app.theme_editor_last_action = format!(
                            "Prepared theme payload copy ({} bytes).",
                            payload.len()
                        );
                        action_log = Some("Theme editor prepared export payload.".into());
                    }
                    Err(error) => {
                        app.theme_editor_last_action = error.clone();
                        action_log = Some(format!("Theme export failed: {error}"));
                    }
                }
            }
            if ui.button("Reset").clicked() {
                app.theme_editor_demo.reset_to_default();
                app.theme_editor_last_action = "Reset to default dark preset.".into();
                action_log = Some("Theme editor reset to default dark preset.".into());
            }
        });
    });
    ui.add_space(8.0);

    let mut mode = app.theme_editor_demo.config().mode;
    let mut accent = app.theme_editor_demo.config().accent.clone();
    let mut background = app.theme_editor_demo.config().background.clone();
    let mut foreground = app.theme_editor_demo.config().foreground.clone();
    let mut contrast = app.theme_editor_demo.config().contrast;
    let mut translucent_sidebar = app.theme_editor_demo.config().translucent_sidebar;
    let mut ui_font = app.theme_editor_demo.config().ui_font.clone();
    let mut code_font = app.theme_editor_demo.config().code_font.clone();

    ui.columns(3, |columns| {
        columns[0].group(|ui| {
            ui.strong("Controls");
            egui::ComboBox::from_label("Mode")
                .selected_text(mode.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut mode, ThemeMode::Light, ThemeMode::Light.to_string());
                    ui.selectable_value(&mut mode, ThemeMode::Dark, ThemeMode::Dark.to_string());
                    ui.selectable_value(
                        &mut mode,
                        ThemeMode::System,
                        ThemeMode::System.to_string(),
                    );
                });
            ui.label("Accent");
            ui.text_edit_singleline(&mut accent);
            ui.label("Background");
            ui.text_edit_singleline(&mut background);
            ui.label("Foreground");
            ui.text_edit_singleline(&mut foreground);
            ui.add(egui::Slider::new(&mut contrast, 0..=100).text("Contrast"));
            ui.checkbox(&mut translucent_sidebar, "Translucent sidebar");
            ui.label("UI font");
            ui.text_edit_singleline(&mut ui_font);
            ui.label("Code font");
            ui.text_edit_singleline(&mut code_font);
        });

        columns[1].group(|ui| {
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

        columns[2].group(|ui| {
            ui.strong("Inspector");
            ui.label(format!(
                "Findings: {}",
                app.theme_editor_demo.findings().len()
            ));
            for finding in app.theme_editor_demo.findings() {
                ui.group(|ui| {
                    ui.monospace(format!("{} | {}", finding.severity, finding.field));
                    ui.label(&finding.message);
                });
                ui.add_space(4.0);
            }
            if app.theme_editor_demo.findings().is_empty() {
                ui.label("No findings.");
            }

            egui::CollapsingHeader::new("Export payload")
                .default_open(true)
                .show(ui, |ui| match app.theme_editor_demo.export_theme_json() {
                    Ok(payload) => {
                        egui::ScrollArea::vertical()
                            .max_height(220.0)
                            .show(ui, |ui| {
                                ui.monospace(payload);
                            });
                    }
                    Err(error) => {
                        ui.label(error);
                    }
                });
        });
    });

    let previous_mode = app.theme_editor_demo.config().mode;
    if mode != previous_mode {
        app.theme_editor_demo.set_mode(mode);
        app.theme_editor_last_action = format!("Mode set to {}.", mode);
        action_log = Some(format!("Theme editor mode set to {mode}."));
    }
    if accent != app.theme_editor_demo.config().accent {
        app.theme_editor_demo.set_accent(accent);
        app.theme_editor_last_action = "Accent updated.".into();
        action_log = Some("Theme editor updated accent.".into());
    }
    if background != app.theme_editor_demo.config().background {
        app.theme_editor_demo.set_background(background);
        app.theme_editor_last_action = "Background updated.".into();
        action_log = Some("Theme editor updated background.".into());
    }
    if foreground != app.theme_editor_demo.config().foreground {
        app.theme_editor_demo.set_foreground(foreground);
        app.theme_editor_last_action = "Foreground updated.".into();
        action_log = Some("Theme editor updated foreground.".into());
    }
    if contrast != app.theme_editor_demo.config().contrast {
        app.theme_editor_demo.set_contrast(contrast);
        app.theme_editor_last_action = format!("Contrast set to {}.", contrast);
        action_log = Some(format!("Theme editor contrast set to {contrast}."));
    }
    if translucent_sidebar != app.theme_editor_demo.config().translucent_sidebar {
        app.theme_editor_demo
            .set_translucent_sidebar(translucent_sidebar);
        app.theme_editor_last_action = if translucent_sidebar {
            "Enabled translucent sidebar.".into()
        } else {
            "Disabled translucent sidebar.".into()
        };
        action_log = Some("Theme editor toggled translucent sidebar.".into());
    }
    if ui_font != app.theme_editor_demo.config().ui_font {
        app.theme_editor_demo.set_ui_font(ui_font);
        app.theme_editor_last_action = "UI font updated.".into();
        action_log = Some("Theme editor updated UI font.".into());
    }
    if code_font != app.theme_editor_demo.config().code_font {
        app.theme_editor_demo.set_code_font(code_font);
        app.theme_editor_last_action = "Code font updated.".into();
        action_log = Some("Theme editor updated code font.".into());
    }

    if let Some(message) = action_log {
        app.push_log(message);
    }
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
    let value = raw.trim();
    let Some(value) = value.strip_prefix('#') else {
        return fallback;
    };
    if value.len() != 6 {
        return fallback;
    }
    let red = u8::from_str_radix(&value[0..2], 16).ok();
    let green = u8::from_str_radix(&value[2..4], 16).ok();
    let blue = u8::from_str_radix(&value[4..6], 16).ok();
    match (red, green, blue) {
        (Some(red), Some(green), Some(blue)) => egui::Color32::from_rgb(red, green, blue),
        _ => fallback,
    }
}
