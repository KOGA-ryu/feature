use std::time::Instant;

use eframe::egui;
use scroll_reader_capsule_host_v0::{
    CapsuleHostSession, HOST_WINDOW_TITLE, HostCommand, HostWindowPosition, HostWindowPreferences,
};
use scroll_reader_capsule_v1::{CapsulePalette, CapsuleRenderPlan, OverlayStatus};

fn main() -> eframe::Result<()> {
    let preferences_path = HostWindowPreferences::default_path();
    let preferences = HostWindowPreferences::load_or_default(&preferences_path);
    let mut session = CapsuleHostSession::from_args(std::env::args());
    session.apply_preferences(&preferences);
    let plan = session.render_plan();
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([
            plan.geometry.width as f32 + 32.0,
            plan.geometry.height as f32 + 32.0,
        ])
        .with_min_inner_size([
            plan.geometry.width as f32 + 32.0,
            plan.geometry.height as f32 + 32.0,
        ])
        .with_decorations(false)
        .with_transparent(true)
        .with_movable_by_background(true)
        .with_title(HOST_WINDOW_TITLE);
    if let Some(position) = preferences.window_position {
        viewport = viewport.with_position([position.x as f32, position.y as f32]);
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        HOST_WINDOW_TITLE,
        options,
        Box::new(|creation_context| {
            creation_context.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(ScrollReaderHostApp::new(
                session,
                preferences_path,
                preferences,
            )))
        }),
    )
}

struct ScrollReaderHostApp {
    session: CapsuleHostSession,
    palette: CapsulePalette,
    last_tick: Instant,
    preferences_path: std::path::PathBuf,
    last_saved_preferences: HostWindowPreferences,
}

impl ScrollReaderHostApp {
    fn new(
        session: CapsuleHostSession,
        preferences_path: std::path::PathBuf,
        last_saved_preferences: HostWindowPreferences,
    ) -> Self {
        Self {
            session,
            palette: CapsulePalette::default(),
            last_tick: Instant::now(),
            preferences_path,
            last_saved_preferences,
        }
    }

    fn handle_input(&mut self, ctx: &egui::Context) {
        let now = Instant::now();
        let elapsed_ms = now.duration_since(self.last_tick).as_millis() as u64;
        self.last_tick = now;
        if elapsed_ms > 0 {
            self.session.apply_command(HostCommand::Tick(elapsed_ms));
        }

        let (load_clipboard, toggle_mode, space, escape, up, down, scroll_y) = ctx.input(|input| {
            (
                input.key_pressed(egui::Key::V)
                    && (input.modifiers.command || input.modifiers.ctrl),
                input.key_pressed(egui::Key::M),
                input.key_pressed(egui::Key::Space),
                input.key_pressed(egui::Key::Escape),
                input.key_pressed(egui::Key::ArrowUp),
                input.key_pressed(egui::Key::ArrowDown),
                input.raw_scroll_delta.y + input.smooth_scroll_delta.y,
            )
        });

        if load_clipboard {
            if let Some(text) = read_clipboard_text() {
                self.session.load_clipboard_text(text);
            }
        }
        if toggle_mode {
            self.session.apply_command(HostCommand::ToggleMode);
            self.resize_window_to_current_mode(ctx);
        }
        if space {
            self.session.apply_command(HostCommand::TogglePlay);
        }
        if up {
            self.session.apply_command(HostCommand::SpeedUp);
        }
        if down {
            self.session.apply_command(HostCommand::SlowDown);
        }
        if scroll_y.abs() >= 1.0 {
            self.session.apply_command(HostCommand::Wheel {
                angle_delta_y: 0,
                pixel_delta_y: scroll_y.round() as i32,
            });
        }
        if escape {
            self.session.apply_command(HostCommand::Close);
        }
        if self.session.is_closed() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn resize_window_to_current_mode(&self, ctx: &egui::Context) {
        let plan = self.session.render_plan();
        let size = egui::vec2(
            plan.geometry.width as f32 + 32.0,
            plan.geometry.height as f32 + 32.0,
        );
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
        ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(size));
    }

    fn persist_preferences_if_changed(&mut self, ctx: &egui::Context) {
        let window_position = ctx.input(|input| {
            input.viewport().outer_rect.map(|rect| HostWindowPosition {
                x: rect.min.x.round() as i32,
                y: rect.min.y.round() as i32,
            })
        });
        let preferences = self.session.preferences_with_position(window_position);
        if preferences == self.last_saved_preferences {
            return;
        }

        if preferences.save_to_path(&self.preferences_path).is_ok() {
            self.last_saved_preferences = preferences;
        }
    }
}

impl eframe::App for ScrollReaderHostApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_input(ctx);

        if self.session.status() == OverlayStatus::Playing {
            ctx.request_repaint();
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        let plan = self.session.render_plan();
                        let size =
                            egui::vec2(plan.geometry.width as f32, plan.geometry.height as f32);
                        let (rect, response) =
                            ui.allocate_exact_size(size, egui::Sense::click_and_drag());
                        if response.hovered() {
                            ui.output_mut(|output| output.cursor_icon = egui::CursorIcon::Grab);
                        }
                        if response.drag_started_by(egui::PointerButton::Primary) {
                            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }
                        paint_capsule(ui, rect, &plan, &self.palette);
                    },
                );
            });
        self.persist_preferences_if_changed(ctx);
    }
}

fn paint_capsule(
    ui: &egui::Ui,
    origin_rect: egui::Rect,
    plan: &CapsuleRenderPlan,
    palette: &CapsulePalette,
) {
    let painter = ui.painter();
    let origin = origin_rect.min;
    let capsule_rect = offset_rect(origin, plan.capsule_rect);
    let lens_rect = offset_rect(origin, plan.lens_rect);
    let progress_track = offset_rect(origin, plan.progress_marker.track);
    let progress_fill = offset_rect(origin, plan.progress_marker.fill);

    painter.rect_filled(
        capsule_rect,
        plan.capsule_rect.radius as f32,
        color(&palette.capsule_bg, 255),
    );
    painter.rect_stroke(
        capsule_rect,
        plan.capsule_rect.radius as f32,
        egui::Stroke::new(1.0, color(&palette.lens_edge, 90)),
        egui::StrokeKind::Outside,
    );
    painter.rect_filled(progress_track, 1.0, color(&palette.base_bg, 115));
    painter.rect_filled(progress_fill, 1.0, color(&palette.accent_progress, 230));
    painter.rect_filled(lens_rect, 13.0, color(&palette.lens_bg, 245));
    painter.rect_stroke(
        lens_rect,
        13.0,
        egui::Stroke::new(1.0, color(&palette.lens_edge, 255)),
        egui::StrokeKind::Outside,
    );

    let clip_rect = offset_rect(origin, plan.text_clip_rect);
    let clipped = painter.with_clip_rect(clip_rect);
    if let Some(message) = &plan.empty_message {
        clipped.text(
            lens_rect.center(),
            egui::Align2::CENTER_CENTER,
            message,
            egui::FontId::proportional(13.0),
            color(&palette.text_muted, 220),
        );
    } else {
        for placement in &plan.motion_frame.placements {
            let font_size = placement.font_size as f32;
            let text_color = if placement.focus_anchor {
                color(&palette.text_focus, 255)
            } else {
                color(
                    &palette.text_context,
                    opacity_to_alpha(placement.opacity_percent),
                )
            };
            clipped.text(
                egui::pos2(
                    origin.x + placement.x as f32,
                    origin.y + plan.text_baseline_y as f32,
                ),
                egui::Align2::CENTER_CENTER,
                &placement.text,
                egui::FontId::proportional(font_size),
                text_color,
            );
        }
    }

    for zone in &plan.fade_zones {
        painter.rect_filled(
            offset_rect(origin, zone.rect),
            0.0,
            color(&palette.capsule_bg, 96),
        );
    }
}

fn offset_rect(origin: egui::Pos2, rect: scroll_reader_capsule_v1::RenderRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(origin.x + rect.x as f32, origin.y + rect.y as f32),
        egui::vec2(rect.width as f32, rect.height as f32),
    )
}

fn color(hex: &str, alpha: u8) -> egui::Color32 {
    let Some(rgb) = parse_hex_color(hex) else {
        return egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
    };
    egui::Color32::from_rgba_unmultiplied(rgb.0, rgb.1, rgb.2, alpha)
}

fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
    let value = hex.strip_prefix('#')?;
    if value.len() != 6 {
        return None;
    }
    let red = u8::from_str_radix(&value[0..2], 16).ok()?;
    let green = u8::from_str_radix(&value[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&value[4..6], 16).ok()?;
    Some((red, green, blue))
}

fn opacity_to_alpha(opacity_percent: u8) -> u8 {
    ((opacity_percent.min(100) as u16 * 255) / 100) as u8
}

fn read_clipboard_text() -> Option<String> {
    let mut clipboard = arboard::Clipboard::new().ok()?;
    clipboard.get_text().ok()
}
