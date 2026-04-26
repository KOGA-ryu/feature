use std::collections::BTreeMap;

use eframe::egui;

use crate::app::FeatureLabApp;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.heading("Feature Packs");
    ui.small("Browse isolated features by pack and maturity.");
    ui.add_space(6.0);

    let rows = app.feature_rows();
    let mut category_counts = BTreeMap::new();
    let mut ui_count = 0usize;
    let mut logic_count = 0usize;
    let mut workflow_count = 0usize;
    for row in &rows {
        *category_counts
            .entry(row.category.clone())
            .or_insert(0usize) += 1;
        if row.id.starts_with("ui.") {
            ui_count += 1;
        } else if row.id.starts_with("logic.") {
            logic_count += 1;
        } else if row.id.starts_with("workflow.") {
            workflow_count += 1;
        }
    }
    let mut current_category = String::new();

    egui::ScrollArea::vertical().show(ui, |ui| {
        summary_card(
            ui,
            rows.len(),
            ui_count,
            logic_count,
            workflow_count,
            app.kind_filter.label(),
            app.search_query.trim(),
        );
        ui.add_space(10.0);

        for row in rows {
            if row.category != current_category {
                current_category = row.category.clone();
                let count = category_counts
                    .get(&current_category)
                    .copied()
                    .unwrap_or_default();
                category_header(ui, &current_category, count);
                ui.add_space(6.0);
            }

            let selected = app.selected_feature_id.as_deref() == Some(row.id.as_str());
            let fill = if selected {
                egui::Color32::from_rgba_unmultiplied(140, 176, 255, 40)
            } else {
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 10)
            };
            let stroke = if selected {
                egui::Color32::from_rgba_unmultiplied(160, 190, 255, 96)
            } else {
                app.chrome_stroke()
            };

            let response = egui::Frame::group(ui.style())
                .fill(fill)
                .stroke(egui::Stroke::new(1.0, stroke))
                .corner_radius(10.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.strong(&row.name);
                        let category_color = category_color(&row.id);
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgba_unmultiplied(
                                category_color.r(),
                                category_color.g(),
                                category_color.b(),
                                22,
                            ))
                            .stroke(egui::Stroke::new(1.0, category_color))
                            .corner_radius(999.0)
                            .inner_margin(egui::Margin::symmetric(8, 4))
                            .show(ui, |ui| {
                                ui.small(
                                    egui::RichText::new(kind_label(&row.id))
                                        .color(category_color)
                                        .strong(),
                                );
                            });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let badge = egui::RichText::new(row.status.to_uppercase())
                                .small()
                                .color(status_color(&row.status));
                            ui.label(badge);
                        });
                    });
                    ui.small(egui::RichText::new(&row.id).monospace());
                    ui.add_space(2.0);
                    ui.label(egui::RichText::new(&row.summary).small());
                })
                .response;

            if response.clicked() {
                app.select_feature(row.id.clone());
                app.push_log(format!("Selected feature {}", row.id));
            }

            ui.add_space(8.0);
        }

        if current_category.is_empty() {
            egui::Frame::group(ui.style())
                .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
                .stroke(egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18),
                ))
                .corner_radius(14.0)
                .inner_margin(egui::Margin::symmetric(12, 10))
                .show(ui, |ui| {
                    ui.strong("No matching features");
                    ui.small("Try widening the search query or resetting the filter.");
                });
        }
    });
}

fn summary_card(
    ui: &mut egui::Ui,
    visible_count: usize,
    ui_count: usize,
    logic_count: usize,
    workflow_count: usize,
    filter_label: &str,
    search_query: &str,
) {
    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18),
        ))
        .corner_radius(16.0)
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.strong("Visible slice");
            ui.small("What the current search and filter settings are exposing.");
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                stat_pill(
                    ui,
                    format!("{visible_count} visible"),
                    egui::Color32::from_rgb(188, 199, 220),
                );
                stat_pill(
                    ui,
                    format!("{ui_count} ui"),
                    egui::Color32::from_rgb(126, 188, 255),
                );
                stat_pill(
                    ui,
                    format!("{logic_count} logic"),
                    egui::Color32::from_rgb(255, 201, 110),
                );
                stat_pill(
                    ui,
                    format!("{workflow_count} workflows"),
                    egui::Color32::from_rgb(177, 150, 255),
                );
            });
            ui.add_space(8.0);
            ui.small(format!("Filter: {filter_label}"));
            if search_query.is_empty() {
                ui.small("Search: none");
            } else {
                ui.small(egui::RichText::new(format!("Search: {search_query}")).monospace());
            }
        });
}

fn category_header(ui: &mut egui::Ui, category: &str, count: usize) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(category.to_uppercase())
                .strong()
                .extra_letter_spacing(0.5),
        );
        ui.small(format!("{count} features"));
    });
}

fn stat_pill(ui: &mut egui::Ui, text: impl Into<String>, color: egui::Color32) {
    let text = text.into();
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            20,
        ))
        .stroke(egui::Stroke::new(1.0, color))
        .corner_radius(999.0)
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.small(egui::RichText::new(text).color(color).strong());
        });
}

fn status_color(status: &str) -> egui::Color32 {
    match status {
        "stable" => egui::Color32::from_rgb(126, 217, 140),
        "tested" => egui::Color32::from_rgb(126, 188, 255),
        "experimental" => egui::Color32::from_rgb(255, 201, 110),
        "deprecated" => egui::Color32::from_rgb(255, 133, 133),
        _ => egui::Color32::LIGHT_GRAY,
    }
}

fn category_color(feature_id: &str) -> egui::Color32 {
    if feature_id.starts_with("ui.") {
        egui::Color32::from_rgb(126, 188, 255)
    } else if feature_id.starts_with("logic.") {
        egui::Color32::from_rgb(255, 201, 110)
    } else if feature_id.starts_with("workflow.") {
        egui::Color32::from_rgb(177, 150, 255)
    } else {
        egui::Color32::LIGHT_GRAY
    }
}

fn kind_label(feature_id: &str) -> &'static str {
    if feature_id.starts_with("ui.") {
        "ui"
    } else if feature_id.starts_with("logic.") {
        "logic"
    } else if feature_id.starts_with("workflow.") {
        "workflow"
    } else {
        "other"
    }
}
