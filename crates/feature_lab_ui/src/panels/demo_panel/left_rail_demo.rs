use eframe::egui;
use left_rail::sample_rail;

use crate::app::FeatureLabApp;

use super::cards::{empty_state_card, result_card, sandbox_toolbar, section_card};
use super::chips::{activation_chip, stat_chip, stats_row};

pub(super) fn show_left_rail_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut move_up = false;
    let mut move_down = false;
    let mut toggle_collapsed = false;
    let mut activate = false;

    sandbox_toolbar(
        ui,
        "Left rail demo",
        "Pure Rust navigation grouping, selection, collapse, and activation state.",
        |ui| {
            ui.horizontal(|ui| {
                move_up = ui.button("Up").clicked();
                move_down = ui.button("Down").clicked();
                toggle_collapsed = ui.button("Toggle Collapse").clicked();
                activate = ui.button("Activate").clicked();
            });
        },
    );
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

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            if rail.is_collapsed() {
                "collapsed"
            } else {
                "expanded"
            },
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!("selected {}", rail.selected_index()),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("badges {}", rail.total_badge_count()),
            egui::Color32::from_rgb(255, 201, 110),
        );
        activation_chip(ui, &app.left_rail_last_activation);
    });
    ui.add_space(8.0);

    if let Some(message) = rail.empty_state_message() {
        empty_state_card(ui, message);
    } else {
        let selected_index = rail.selected_index();
        let mut flat_index = 0usize;
        for section in rail.grouped_sections() {
            section_card(
                ui,
                &section.name,
                "Navigation items in the current rail section.",
                |ui| {
                    for item in section.items {
                        let selected = flat_index == selected_index;
                        let availability = if item.enabled { "enabled" } else { "disabled" };
                        let label = if rail.is_collapsed() {
                            item.icon.clone()
                        } else {
                            item.title.clone()
                        };
                        result_card(
                            ui,
                            selected,
                            if item.enabled {
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
                                        if item.enabled {
                                            egui::Color32::from_rgb(126, 217, 140)
                                        } else {
                                            egui::Color32::from_rgb(255, 133, 133)
                                        },
                                    );
                                    if let Some(count) = item.badge_count {
                                        stat_chip(
                                            ui,
                                            format!("badge {count}"),
                                            egui::Color32::from_rgb(255, 201, 110),
                                        );
                                    }
                                });
                                ui.add_space(4.0);
                                ui.strong(label);
                                ui.small(item.id);
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
