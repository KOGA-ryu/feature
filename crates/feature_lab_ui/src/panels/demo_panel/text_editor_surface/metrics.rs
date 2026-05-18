use eframe::egui;

pub(super) fn line_height_for_font(ui: &mut egui::Ui, font: &egui::FontId) -> f32 {
    ui.fonts_mut(|fonts| fonts.row_height(font)).max(18.0)
}

pub(super) fn desired_surface_size(
    available_width: f32,
    line_count: usize,
    line_height: f32,
) -> egui::Vec2 {
    let desired_height = (line_count as f32 * line_height + 24.0).max(220.0);
    egui::vec2(available_width, desired_height)
}

pub(super) fn measure_text_width(ui: &egui::Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
}

pub(super) fn prefix_for_column(line: &str, column: usize) -> &str {
    let byte_index = line
        .char_indices()
        .nth(column)
        .map(|(index, _)| index)
        .unwrap_or(line.len());
    &line[..byte_index]
}
