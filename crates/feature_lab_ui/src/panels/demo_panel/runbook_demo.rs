use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{result_card, sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};
use super::status_colors::{runbook_status_color, runbook_status_label};

pub(super) fn show_runbook_panel_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    sandbox_toolbar(
        ui,
        "Runbook demo",
        "Single-selection runbook state with explicit status transitions and summary counts.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Move Up").clicked() {
                    app.runbook_demo.move_up();
                }
                if ui.button("Move Down").clicked() {
                    app.runbook_demo.move_down();
                }
                if ui.button("Mark Active").clicked() {
                    app.runbook_demo.mark_selected_active();
                    app.push_log("Runbook demo marked the selected step active.");
                }
                if ui.button("Mark Completed").clicked() {
                    app.runbook_demo.mark_selected_completed();
                    app.push_log("Runbook demo marked the selected step completed.");
                }
                if ui.button("Mark Blocked").clicked() {
                    app.runbook_demo.mark_selected_blocked();
                    app.push_log("Runbook demo marked the selected step blocked.");
                }
                if ui.button("Reset Statuses").clicked() {
                    app.runbook_demo.reset_statuses();
                    app.push_log("Runbook demo reset all statuses.");
                }
            });
        },
    );
    ui.add_space(8.0);

    let counts = app.runbook_demo.counts();
    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("pending {}", counts.pending),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!("active {}", counts.active),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("completed {}", counts.completed),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            format!("blocked {}", counts.blocked),
            egui::Color32::from_rgb(255, 133, 133),
        );
    });
    ui.add_space(8.0);

    let selected_id = app.runbook_demo.selected_step().map(|step| step.id.clone());
    let steps_snapshot = app.runbook_demo.steps.clone();

    section_card(
        ui,
        "Runbook steps",
        "Repo-relevant feature-wave runbook with one selected step at a time.",
        |ui| {
            for (index, step) in steps_snapshot.iter().enumerate() {
                let selected = selected_id.as_deref() == Some(step.id.as_str());
                result_card(
                    ui,
                    selected,
                    runbook_status_color(step.status.clone()),
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("Select").clicked() {
                                app.runbook_demo.set_selected_index(index);
                            }
                            if selected {
                                stat_chip(ui, "selected", egui::Color32::from_rgb(126, 188, 255));
                            }
                            stat_chip(
                                ui,
                                runbook_status_label(step.status.clone()),
                                runbook_status_color(step.status.clone()),
                            );
                            ui.strong(&step.title);
                        });
                        ui.add_space(4.0);
                        ui.small(&step.id);
                    },
                );
                ui.add_space(6.0);
            }
        },
    );

    if let Some(step) = app.runbook_demo.selected_step() {
        ui.add_space(12.0);
        section_card(
            ui,
            "Selected step",
            "Current runbook step detail and status.",
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    stat_chip(ui, &step.id, egui::Color32::from_rgb(188, 199, 220));
                    stat_chip(
                        ui,
                        runbook_status_label(step.status.clone()),
                        runbook_status_color(step.status.clone()),
                    );
                });
                ui.add_space(6.0);
                ui.strong(&step.title);
                ui.label(&step.detail);
            },
        );
    }
}
