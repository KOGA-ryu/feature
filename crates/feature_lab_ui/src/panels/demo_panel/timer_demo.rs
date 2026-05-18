use eframe::egui;
use timer_basic::TimerStatus;

use crate::app::FeatureLabApp;

use super::cards::{result_card, sandbox_toolbar};
use super::chips::{stat_chip, stats_row};
use super::status_colors::{timer_status_color, timer_status_label};

pub(super) fn show_timer_basic_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    sandbox_toolbar(
        ui,
        "Timer demo",
        "Deterministic countdown state driven only by explicit ticks and command buttons.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("5m").clicked() {
                    app.timer_demo.set_duration(300);
                    app.push_log("Timer demo duration set to 5 minutes.");
                }
                if ui.button("15m").clicked() {
                    app.timer_demo.set_duration(900);
                    app.push_log("Timer demo duration set to 15 minutes.");
                }
                if ui.button("25m").clicked() {
                    app.timer_demo.set_duration(1500);
                    app.push_log("Timer demo duration set to 25 minutes.");
                }
                if ui.button("Start").clicked() {
                    app.timer_demo.start();
                    app.push_log("Timer demo started.");
                }
                if ui.button("Pause").clicked() {
                    app.timer_demo.pause();
                    app.push_log("Timer demo paused.");
                }
                if ui.button("Resume").clicked() {
                    app.timer_demo.resume();
                    app.push_log("Timer demo resumed.");
                }
                if ui.button("Reset").clicked() {
                    app.timer_demo.reset();
                    app.push_log("Timer demo reset.");
                }
                if ui.button("+10s").clicked() {
                    app.timer_demo.tick(10);
                    app.push_log("Timer demo advanced by 10 seconds.");
                }
                if ui.button("+60s").clicked() {
                    app.timer_demo.tick(60);
                    app.push_log("Timer demo advanced by 60 seconds.");
                }
            });
        },
    );
    ui.add_space(8.0);

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("remaining {}", app.timer_demo.formatted_remaining()),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("status {}", timer_status_label(app.timer_demo.status)),
            timer_status_color(app.timer_demo.status),
        );
        stat_chip(
            ui,
            format!(
                "progress {:>3.0}%",
                (app.timer_demo.progress_ratio() * 100.0).clamp(0.0, 100.0)
            ),
            egui::Color32::from_rgb(126, 217, 140),
        );
    });
    ui.add_space(8.0);

    result_card(
        ui,
        app.timer_demo.status == TimerStatus::Completed,
        timer_status_color(app.timer_demo.status),
        |ui| {
            ui.small("Countdown");
            ui.heading(app.timer_demo.formatted_remaining());
            ui.add_space(6.0);
            ui.add(
                egui::ProgressBar::new(app.timer_demo.progress_ratio())
                    .desired_width(ui.available_width())
                    .show_percentage(),
            );
        },
    );
}
