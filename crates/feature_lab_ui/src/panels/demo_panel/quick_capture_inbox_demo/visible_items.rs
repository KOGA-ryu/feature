use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::{empty_state_card, result_card, section_card};
use super::super::chips::stat_chip;
use super::super::status_colors::{inbox_status_color, inbox_status_label};

pub(super) fn show_visible_items(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let selected_id = app
        .quick_capture_inbox_demo
        .selected_item()
        .map(|item| item.id);
    let visible_snapshot = app
        .quick_capture_inbox_demo
        .visible_items()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();

    if visible_snapshot.is_empty() {
        empty_state_card(ui, "No inbox items match the current query.");
        return;
    }

    section_card(
        ui,
        "Visible inbox items",
        "Select, process, archive, or delete captured items without reordering the queue.",
        |ui| {
            for (index, item) in visible_snapshot.iter().enumerate() {
                let selected = Some(item.id) == selected_id;
                result_card(ui, selected, inbox_status_color(item.status), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Select").clicked() {
                            app.quick_capture_inbox_demo.set_selected_index(index);
                        }
                        if ui.button("Processed").clicked() {
                            app.quick_capture_inbox_demo.set_selected_index(index);
                            if app.quick_capture_inbox_demo.mark_selected_processed() {
                                app.push_log(format!(
                                    "Quick capture inbox demo marked item {} processed.",
                                    item.id
                                ));
                            }
                        }
                        if ui.button("Archive").clicked() {
                            app.quick_capture_inbox_demo.set_selected_index(index);
                            if app.quick_capture_inbox_demo.mark_selected_archived() {
                                app.push_log(format!(
                                    "Quick capture inbox demo archived item {}.",
                                    item.id
                                ));
                            }
                        }
                        if ui.button("Delete").clicked() {
                            app.quick_capture_inbox_demo.set_selected_index(index);
                            if app.quick_capture_inbox_demo.delete_selected() {
                                app.push_log(format!(
                                    "Quick capture inbox demo deleted item {}.",
                                    item.id
                                ));
                            }
                        }
                        if selected {
                            stat_chip(ui, "selected", egui::Color32::from_rgb(126, 188, 255));
                        }
                        stat_chip(
                            ui,
                            inbox_status_label(item.status),
                            inbox_status_color(item.status),
                        );
                        stat_chip(
                            ui,
                            format!("id {}", item.id),
                            egui::Color32::from_rgb(188, 199, 220),
                        );
                        stat_chip(
                            ui,
                            &item.captured_at,
                            egui::Color32::from_rgb(188, 199, 220),
                        );
                    });
                    ui.add_space(4.0);
                    ui.label(&item.text);
                });
                ui.add_space(6.0);
            }
        },
    );
}
