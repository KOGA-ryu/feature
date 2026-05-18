use command_palette::sample_palette;
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{empty_state_card, result_card, sandbox_toolbar, section_card};
use super::chips::{activation_chip, stat_chip, stats_row};

pub(super) fn show_command_palette_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut move_up = false;
    let mut move_down = false;
    let mut activate = false;

    sandbox_toolbar(
        ui,
        "Command palette demo",
        "Pure Rust filtering, grouping, selection, and activation state.",
        |ui| {
            ui.horizontal(|ui| {
                ui.label("Query");
                ui.text_edit_singleline(&mut app.command_palette_demo_query);
                move_up = ui.button("Up").clicked();
                move_down = ui.button("Down").clicked();
                activate = ui.button("Activate").clicked();
            });
        },
    );
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

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("selected {}", app.command_palette_demo_selected_index),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            if app.command_palette_demo_query.trim().is_empty() {
                "query all".into()
            } else {
                format!("query {}", app.command_palette_demo_query.trim())
            },
            egui::Color32::from_rgb(188, 199, 220),
        );
        activation_chip(ui, &app.command_palette_last_activation);
    });
    ui.add_space(8.0);

    if let Some(message) = palette.empty_state_message() {
        empty_state_card(ui, message);
    } else {
        let selected_index = palette.selected_index();
        let mut flat_index = 0usize;
        for group in palette.grouped_results() {
            section_card(
                ui,
                &group.category,
                "Grouped palette results for the current query.",
                |ui| {
                    for command in group.commands {
                        let selected = flat_index == selected_index;
                        let availability = if command.enabled {
                            "enabled"
                        } else {
                            "disabled"
                        };
                        let shortcut = command.shortcut.as_deref().unwrap_or("no shortcut");
                        result_card(
                            ui,
                            selected,
                            if command.enabled {
                                egui::Color32::from_rgb(126, 188, 255)
                            } else {
                                egui::Color32::from_rgb(255, 133, 133)
                            },
                            |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    if selected {
                                        stat_chip(
                                            ui,
                                            "selected",
                                            egui::Color32::from_rgb(126, 188, 255),
                                        );
                                    }
                                    stat_chip(
                                        ui,
                                        availability,
                                        if command.enabled {
                                            egui::Color32::from_rgb(126, 217, 140)
                                        } else {
                                            egui::Color32::from_rgb(255, 133, 133)
                                        },
                                    );
                                    stat_chip(ui, shortcut, egui::Color32::from_rgb(188, 199, 220));
                                });
                                ui.add_space(4.0);
                                ui.strong(&command.title);
                                ui.small(format!("{} | {}", command.subtitle, command.id));
                            },
                        );
                        ui.add_space(6.0);
                        flat_index += 1;
                    }
                },
            );
            ui.add_space(8.0);
        }
    }

    if let Some(message) = log_message {
        app.push_log(message);
    }
}
