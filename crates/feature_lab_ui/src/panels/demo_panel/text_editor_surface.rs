mod input;
mod metrics;
mod paint;

use eframe::egui;

use crate::app::FeatureLabApp;

use self::input::handle_focused_input;
use self::metrics::{desired_surface_size, line_height_for_font};
use self::paint::paint_text_editor_demo_surface;

pub(super) fn render_text_editor_demo_surface(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let editor_id = ui.make_persistent_id("text_editor_plain_demo_surface");
    let font = egui::FontId::monospace(14.0);
    let line_height = line_height_for_font(ui, &font);
    let desired_size = desired_surface_size(
        ui.available_width(),
        app.text_editor_plain_demo.line_count(),
        line_height,
    );
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        ui.memory_mut(|memory| memory.request_focus(editor_id));
    }
    let has_focus = ui.memory(|memory| memory.has_focus(editor_id));

    handle_focused_input(ui, &mut app.text_editor_plain_demo, has_focus);
    paint_text_editor_demo_surface(ui, rect, &app.text_editor_plain_demo, has_focus);
}
