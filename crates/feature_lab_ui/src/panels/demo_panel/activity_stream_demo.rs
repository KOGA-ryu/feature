use activity_stream::sample_stream;
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{empty_state_card, result_card, sandbox_toolbar};
use super::chips::{activation_chip, stat_chip, stats_row};

pub(super) fn show_activity_stream_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut move_up = false;
    let mut move_down = false;
    let mut activate = false;

    sandbox_toolbar(
        ui,
        "Activity stream demo",
        "Pure Rust activity filtering, ordering, selection, and activation state.",
        |ui| {
            ui.horizontal(|ui| {
                ui.label("Query");
                ui.text_edit_singleline(&mut app.activity_stream_demo_query);
                move_up = ui.button("Up").clicked();
                move_down = ui.button("Down").clicked();
                activate = ui.button("Activate").clicked();
            });
        },
    );
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

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("selected {}", app.activity_stream_demo_selected_index),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("unread {}", stream.visible_unread_count()),
            egui::Color32::from_rgb(255, 201, 110),
        );
        activation_chip(ui, &app.activity_stream_last_activation);
    });
    ui.add_space(8.0);

    if let Some(message) = stream.empty_state_message() {
        empty_state_card(ui, message);
    } else {
        let selected_index = stream.selected_index();
        for (index, entry) in stream.visible_entries().into_iter().enumerate() {
            let selected = index == selected_index;
            result_card(
                ui,
                selected,
                if entry.unread {
                    egui::Color32::from_rgb(255, 201, 110)
                } else {
                    egui::Color32::from_rgb(126, 188, 255)
                },
                |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if selected {
                            stat_chip(ui, "selected", egui::Color32::from_rgb(126, 188, 255));
                        }
                        stat_chip(
                            ui,
                            entry.kind.clone(),
                            egui::Color32::from_rgb(188, 199, 220),
                        );
                        stat_chip(
                            ui,
                            entry.status.clone(),
                            if entry.status == "failed" {
                                egui::Color32::from_rgb(255, 133, 133)
                            } else {
                                egui::Color32::from_rgb(126, 217, 140)
                            },
                        );
                        stat_chip(
                            ui,
                            if entry.unread { "unread" } else { "read" },
                            if entry.unread {
                                egui::Color32::from_rgb(255, 201, 110)
                            } else {
                                egui::Color32::from_rgb(188, 199, 220)
                            },
                        );
                        stat_chip(
                            ui,
                            if entry.actionable {
                                "actionable"
                            } else {
                                "static"
                            },
                            if entry.actionable {
                                egui::Color32::from_rgb(126, 217, 140)
                            } else {
                                egui::Color32::from_rgb(188, 199, 220)
                            },
                        );
                    });
                    ui.add_space(4.0);
                    ui.strong(&entry.title);
                    ui.small(format!(
                        "{} | {} | {}",
                        entry.actor, entry.timestamp, entry.id
                    ));
                    ui.label(&entry.detail);
                },
            );
            ui.add_space(6.0);
        }
    }

    if let Some(message) = log_message {
        app.push_log(message);
    }
}
