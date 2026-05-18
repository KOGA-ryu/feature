use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{empty_state_card, result_card, sandbox_toolbar};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_checklist_single_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut add_clicked = false;
    let mut should_submit = false;

    sandbox_toolbar(
        ui,
        "Checklist demo",
        "Single durable checklist state with stable append order and clear-completed.",
        |ui| {
            ui.horizontal(|ui| {
                let response = ui.add_sized(
                    [ui.available_width() - 96.0, 28.0],
                    egui::TextEdit::singleline(&mut app.checklist_demo_draft)
                        .hint_text("Add checklist item"),
                );
                should_submit =
                    response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
                add_clicked = ui.button("Add").clicked();
            });
            ui.add_space(6.0);
            if ui.button("Clear Completed").clicked() {
                app.checklist_demo.clear_completed();
                app.push_log("Checklist demo cleared completed items.");
            }
        },
    );
    ui.add_space(8.0);

    if should_submit || add_clicked {
        if app
            .checklist_demo
            .add_item(app.checklist_demo_draft.clone())
        {
            app.checklist_demo_draft.clear();
            app.push_log("Checklist demo added an item.");
        }
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("items {}", app.checklist_demo.items.len()),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            if app.checklist_demo.has_completed_items() {
                "completed present"
            } else {
                "no completed"
            },
            if app.checklist_demo.has_completed_items() {
                egui::Color32::from_rgb(126, 217, 140)
            } else {
                egui::Color32::from_rgb(188, 199, 220)
            },
        );
    });
    ui.add_space(8.0);

    if app.checklist_demo.items.is_empty() {
        empty_state_card(ui, "No checklist items yet.");
        return;
    }

    let snapshot = app.checklist_demo.items.clone();
    for item in snapshot {
        result_card(
            ui,
            item.is_completed,
            if item.is_completed {
                egui::Color32::from_rgb(126, 217, 140)
            } else {
                egui::Color32::from_rgb(188, 199, 220)
            },
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    let toggle_label = if item.is_completed { "☑" } else { "☐" };
                    if ui.button(toggle_label).clicked() {
                        app.checklist_demo.toggle_item(item.id);
                    }
                    let title = if item.is_completed {
                        egui::RichText::new(&item.title)
                            .strikethrough()
                            .color(egui::Color32::from_gray(170))
                    } else {
                        egui::RichText::new(&item.title)
                    };
                    ui.label(title);
                    stat_chip(
                        ui,
                        format!("order {}", item.created_at_order),
                        egui::Color32::from_rgb(188, 199, 220),
                    );
                    if ui.button("Delete").clicked() {
                        app.checklist_demo.delete_item(item.id);
                    }
                });
            },
        );
        ui.add_space(6.0);
    }
}
