use activity_stream::sample_stream;
use command_palette::sample_palette;
use eframe::egui;
use left_rail::sample_rail;
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
