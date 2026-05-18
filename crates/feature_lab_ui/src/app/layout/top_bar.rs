use eframe::egui;

use crate::app::{FeatureCounts, FeatureLabApp, KindFilter};

use super::super::style::{glass_panel_frame, pill, toolbar_card};

pub(super) fn show_top_bar(
    ctx: &egui::Context,
    app: &mut FeatureLabApp,
    style: &egui::Style,
    selected_summary: &str,
    counts: FeatureCounts,
) {
    egui::TopBottomPanel::top("top_bar")
        .frame(glass_panel_frame(
            style,
            app.top_bar_fill(),
            app.chrome_stroke(),
        ))
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                #[cfg(target_os = "macos")]
                ui.add_space(84.0);
                ui.horizontal_wrapped(|ui| {
                    ui.heading("Feature Lab");
                    ui.small("Rust feature bench for isolated, testable, reusable crates.");
                    ui.separator();
                    ui.small(
                        egui::RichText::new(selected_summary)
                            .monospace()
                            .color(egui::Color32::from_gray(185)),
                    );
                });
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    show_search_card(ui, app);
                    show_counts_card(ui, counts, app.chrome_stroke());
                    show_actions_card(ui, app);
                });
            });
        });
}

fn show_search_card(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    toolbar_card(ui, |ui| {
        let mut search_query = app.browser.query().to_owned();
        ui.label("Search");
        let search_response =
            ui.add_sized([250.0, 28.0], egui::TextEdit::singleline(&mut search_query));
        if search_response.changed() {
            app.browser.set_query(search_query);
        }
        let current_kind_filter = app.kind_filter();
        let mut kind_filter = current_kind_filter;
        egui::ComboBox::from_label("Filter")
            .selected_text(current_kind_filter.label())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut kind_filter, KindFilter::All, KindFilter::All.label());
                ui.selectable_value(&mut kind_filter, KindFilter::Ui, KindFilter::Ui.label());
                ui.selectable_value(
                    &mut kind_filter,
                    KindFilter::Logic,
                    KindFilter::Logic.label(),
                );
                ui.selectable_value(&mut kind_filter, KindFilter::Sim, KindFilter::Sim.label());
                ui.selectable_value(
                    &mut kind_filter,
                    KindFilter::Workflow,
                    KindFilter::Workflow.label(),
                );
            });
        if kind_filter != current_kind_filter {
            app.set_kind_filter(kind_filter);
        }
        pill(
            ui,
            format!("{} visible", app.visible_feature_count()),
            egui::Color32::from_rgb(188, 199, 220),
        );
    });
}

fn show_counts_card(ui: &mut egui::Ui, counts: FeatureCounts, chrome_stroke: egui::Color32) {
    toolbar_card(ui, |ui| {
        pill(ui, format!("{} total", counts.total), chrome_stroke);
        pill(
            ui,
            format!("{} ui", counts.ui),
            egui::Color32::from_rgb(126, 188, 255),
        );
        pill(
            ui,
            format!("{} logic", counts.logic),
            egui::Color32::from_rgb(255, 201, 110),
        );
        pill(
            ui,
            format!("{} sim", counts.sim),
            egui::Color32::from_rgb(110, 214, 188),
        );
        pill(
            ui,
            format!("{} workflows", counts.workflow),
            egui::Color32::from_rgb(160, 150, 255),
        );
    });
}

fn show_actions_card(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    toolbar_card(ui, |ui| {
        #[cfg(target_os = "macos")]
        ui.checkbox(&mut app.glass_enabled, "Glass");
        if ui.button("Run Tests").clicked() {
            app.run_selected_tests();
        }
        if ui.button("Open Feature Folder").clicked() {
            app.open_selected_feature_folder();
        }
    });
}
