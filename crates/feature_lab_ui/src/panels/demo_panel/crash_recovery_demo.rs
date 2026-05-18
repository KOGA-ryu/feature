use crash_recovery::CrashState;
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_crash_recovery_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut trigger_crash = false;
    let mut tick_amount = None;
    let mut reset_sample = false;

    sandbox_toolbar(
        ui,
        "Crash recovery demo",
        "Deterministic crash lockout and recovery timing for the runner lane.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                trigger_crash = ui.button("Trigger Crash").clicked();
                if ui.button("Tick 1").clicked() {
                    tick_amount = Some(1);
                }
                if ui.button("Tick 2").clicked() {
                    tick_amount = Some(2);
                }
                reset_sample = ui.button("Reset Sample").clicked();
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = crash_recovery::sample_state() {
            app.crash_recovery_demo = sample;
            app.push_log("Crash recovery demo reset to the sample state.");
        }
    }
    if trigger_crash && app.crash_recovery_demo.trigger_crash(2, 3) {
        app.push_log("Crash recovery demo entered crash state.");
    }
    if let Some(amount) = tick_amount {
        app.crash_recovery_demo.tick(amount);
        app.push_log(format!("Crash recovery demo ticked by {amount}."));
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            crash_state_label(app.crash_recovery_demo.state),
            egui::Color32::from_rgb(255, 133, 133),
        );
        stat_chip(
            ui,
            format!("locked {}", app.crash_recovery_demo.locked_ticks_remaining),
            egui::Color32::from_rgb(255, 201, 110),
        );
        stat_chip(
            ui,
            format!(
                "recovery {}",
                app.crash_recovery_demo.recovery_ticks_remaining
            ),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            if app.crash_recovery_demo.can_steer() {
                "steer yes"
            } else {
                "steer no"
            },
            egui::Color32::from_rgb(110, 214, 188),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Eligibility",
        "Current control and collision eligibility during the crash cycle.",
        |ui| {
            ui.monospace(format!(
                "can_steer={}\ncan_collide={}\nis_stable={}",
                app.crash_recovery_demo.can_steer(),
                app.crash_recovery_demo.can_collide(),
                app.crash_recovery_demo.is_stable()
            ));
        },
    );
}

fn crash_state_label(state: CrashState) -> &'static str {
    match state {
        CrashState::Stable => "stable",
        CrashState::Crashed => "crashed",
        CrashState::Recovering => "recovering",
    }
}
