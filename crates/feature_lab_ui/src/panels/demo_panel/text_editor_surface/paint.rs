use eframe::egui;
use text_editor_plain::TextEditorPlain;

use super::metrics::{line_height_for_font, measure_text_width, prefix_for_column};

pub(super) fn paint_text_editor_demo_surface(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    editor: &TextEditorPlain,
    has_focus: bool,
) {
    let font = egui::FontId::monospace(14.0);
    let line_height = line_height_for_font(ui, &font);
    let painter = ui.painter();
    let visuals = ui.visuals();
    let text_color = visuals.strong_text_color();
    let muted_color = egui::Color32::from_gray(120);
    let selection_color = egui::Color32::from_rgba_unmultiplied(126, 188, 255, 64);
    let caret_color = egui::Color32::from_rgb(255, 201, 110);
    let background = if has_focus {
        egui::Color32::from_rgba_unmultiplied(12, 14, 18, 220)
    } else {
        egui::Color32::from_rgba_unmultiplied(12, 14, 18, 180)
    };
    let stroke = egui::Stroke::new(
        1.0,
        if has_focus {
            egui::Color32::from_rgb(126, 188, 255)
        } else {
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 28)
        },
    );

    painter.rect_filled(rect, 14.0, background);
    painter.rect_stroke(rect, 14.0, stroke, egui::StrokeKind::Outside);

    let content_rect = rect.shrink2(egui::vec2(12.0, 12.0));
    let lines: Vec<&str> = editor.text().split('\n').collect();
    let selection = editor.selection();
    let selection_start = selection.anchor.min(selection.caret);
    let selection_end = selection.anchor.max(selection.caret);

    for (line_index, line) in lines.iter().enumerate() {
        let top = content_rect.top() + line_index as f32 * line_height;
        let baseline = egui::pos2(content_rect.left(), top);

        paint_selection_if_needed(
            painter,
            ui,
            content_rect.left(),
            top,
            line_height,
            line_index,
            line,
            selection_start,
            selection_end,
            &font,
            selection_color,
        );

        if line.is_empty() {
            painter.text(
                baseline,
                egui::Align2::LEFT_TOP,
                " ",
                font.clone(),
                muted_color,
            );
        } else {
            painter.text(
                baseline,
                egui::Align2::LEFT_TOP,
                *line,
                font.clone(),
                text_color,
            );
        }
    }

    if has_focus {
        paint_caret(
            painter,
            ui,
            content_rect,
            line_height,
            &lines,
            editor,
            &font,
            caret_color,
        );
    }
}

fn paint_selection_if_needed(
    painter: &egui::Painter,
    ui: &egui::Ui,
    content_left: f32,
    top: f32,
    line_height: f32,
    line_index: usize,
    line: &str,
    selection_start: text_editor_plain::EditorPosition,
    selection_end: text_editor_plain::EditorPosition,
    font: &egui::FontId,
    selection_color: egui::Color32,
) {
    if selection_start == selection_end
        || line_index < selection_start.line
        || line_index > selection_end.line
    {
        return;
    }

    let line_char_count = line.chars().count();
    let start_column = if line_index == selection_start.line {
        selection_start.column
    } else {
        0
    };
    let end_column = if line_index == selection_end.line {
        selection_end.column
    } else {
        line_char_count
    };
    let start_x =
        content_left + measure_text_width(ui, prefix_for_column(line, start_column), font);
    let end_x = content_left + measure_text_width(ui, prefix_for_column(line, end_column), font);
    let selection_rect = egui::Rect::from_min_max(
        egui::pos2(start_x, top),
        egui::pos2(end_x.max(start_x + 2.0), top + line_height),
    );
    painter.rect_filled(selection_rect, 4.0, selection_color);
}

fn paint_caret(
    painter: &egui::Painter,
    ui: &egui::Ui,
    content_rect: egui::Rect,
    line_height: f32,
    lines: &[&str],
    editor: &TextEditorPlain,
    font: &egui::FontId,
    caret_color: egui::Color32,
) {
    let cursor = editor.cursor();
    let line = lines.get(cursor.line).copied().unwrap_or("");
    let caret_x =
        content_rect.left() + measure_text_width(ui, prefix_for_column(line, cursor.column), font);
    let caret_top = content_rect.top() + cursor.line as f32 * line_height;
    painter.line_segment(
        [
            egui::pos2(caret_x, caret_top),
            egui::pos2(caret_x, caret_top + line_height),
        ],
        egui::Stroke::new(1.5, caret_color),
    );
}
