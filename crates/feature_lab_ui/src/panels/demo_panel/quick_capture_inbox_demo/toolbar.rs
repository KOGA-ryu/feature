use eframe::egui;

use crate::app::FeatureLabApp;

use super::super::cards::sandbox_toolbar;

pub(super) fn show_toolbar(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut capture_clicked = false;
    let mut should_submit = false;

    sandbox_toolbar(
        ui,
        "Quick capture inbox demo",
        "Headless intake queue with draft capture, filtered visibility, stable status transitions, and selection-safe triage.",
        |ui| {
            ui.label("Capture");
            ui.horizontal(|ui| {
                let response = ui.add_sized(
                    [ui.available_width() - 168.0, 28.0],
                    egui::TextEdit::singleline(&mut app.quick_capture_inbox_demo.draft_text)
                        .hint_text("Capture an idea, follow-up, or rough note"),
                );
                should_submit =
                    response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
                ui.add_sized(
                    [148.0, 28.0],
                    egui::TextEdit::singleline(&mut app.quick_capture_inbox_demo_captured_at)
                        .hint_text("captured_at"),
                );
                capture_clicked = ui.button("Capture").clicked();
            });
            ui.add_space(6.0);
            ui.label("Filter");
            let mut query = app.quick_capture_inbox_demo.query.clone();
            if ui
                .add_sized(
                    [ui.available_width(), 28.0],
                    egui::TextEdit::singleline(&mut query)
                        .hint_text("Filter by text, timestamp, or status"),
                )
                .changed()
            {
                app.quick_capture_inbox_demo.set_query(query);
            }
        },
    );
    ui.add_space(8.0);

    if (should_submit || capture_clicked)
        && app
            .quick_capture_inbox_demo
            .capture_draft(app.quick_capture_inbox_demo_captured_at.clone())
    {
        app.push_log("Quick capture inbox demo captured a draft item.");
    }
}
