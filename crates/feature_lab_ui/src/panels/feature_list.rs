use eframe::egui;

use crate::app::FeatureLabApp;

pub fn show(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    ui.heading("Feature Packs");
    ui.label("Browse isolated features by pack.");
    ui.separator();

    let rows = app.feature_rows();
    let mut current_category = String::new();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for row in rows {
            if row.category != current_category {
                current_category = row.category.clone();
                ui.separator();
                ui.strong(current_category.to_uppercase());
            }

            let selected = app.selected_feature_id.as_deref() == Some(row.id.as_str());
            if ui
                .selectable_label(selected, format!("{} [{}]", row.name, row.status))
                .clicked()
            {
                app.select_feature(row.id.clone());
                app.push_log(format!("Selected feature {}", row.id));
            }
            ui.label(egui::RichText::new(row.summary).small());
            ui.add_space(8.0);
        }
        if current_category.is_empty() {
            ui.label("No features matched the current search/filter.");
        }
    });
}
