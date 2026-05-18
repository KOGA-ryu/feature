use eframe::egui;
use text_editor_plain::{EditorCommand, sample_document_text};

use crate::app::FeatureLabApp;

use super::cards::sandbox_toolbar;
use super::chips::{stat_chip, stats_row};
use super::text_editor_surface::render_text_editor_demo_surface;

pub(super) fn show_text_editor_plain_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    sandbox_toolbar(
        ui,
        "Text editor demo",
        "Keyboard-driven plain-text editor core with selection, navigation, undo/redo, and dirty tracking.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Reset Sample").clicked() {
                    if let Ok(sample) = sample_document_text() {
                        app.text_editor_plain_demo.load_text(sample);
                        app.push_log("Text editor demo reset to sample.");
                    }
                }
                if ui.button("Clear").clicked() {
                    app.text_editor_plain_demo.load_text(String::new());
                    app.push_log("Text editor demo cleared.");
                }
                if ui.button("Undo").clicked() {
                    app.text_editor_plain_demo.apply(EditorCommand::Undo);
                    app.push_log("Text editor demo performed undo.");
                }
                if ui.button("Redo").clicked() {
                    app.text_editor_plain_demo.apply(EditorCommand::Redo);
                    app.push_log("Text editor demo performed redo.");
                }
            });
        },
    );
    ui.add_space(8.0);

    let selected_length = app
        .text_editor_plain_demo
        .selected_text()
        .map(|text| text.chars().count())
        .unwrap_or(0);
    let cursor = app.text_editor_plain_demo.cursor();
    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("lines {}", app.text_editor_plain_demo.line_count()),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("chars {}", app.text_editor_plain_demo.character_count()),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            format!("cursor {}:{}", cursor.line + 1, cursor.column + 1),
            egui::Color32::from_rgb(255, 201, 110),
        );
        stat_chip(
            ui,
            format!("selection {}", selected_length),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            if app.text_editor_plain_demo.is_dirty() {
                "dirty"
            } else {
                "clean"
            },
            if app.text_editor_plain_demo.is_dirty() {
                egui::Color32::from_rgb(255, 133, 133)
            } else {
                egui::Color32::from_rgb(126, 217, 140)
            },
        );
        stat_chip(
            ui,
            format!(
                "undo {} / redo {}",
                app.text_editor_plain_demo.undo_depth(),
                app.text_editor_plain_demo.redo_depth()
            ),
            egui::Color32::from_rgb(176, 197, 255),
        );
    });
    ui.add_space(8.0);
    ui.small("Keyboard-only demo: click once to focus, then type. Mouse editing, drag selection, clipboard, and rich text are intentionally out of scope.");
    ui.add_space(8.0);

    egui::ScrollArea::vertical()
        .id_salt("text_editor_plain_demo_scroll")
        .max_height(320.0)
        .show(ui, |ui| {
            render_text_editor_demo_surface(ui, app);
        });
}
