use ai_target_state::{TargetAwareness, TargetPosture, TargetPressure};
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_ai_target_state_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut reset_sample = false;
    let mut tick_state = false;
    let mut commitment_ticks = None;
    let mut vulnerability_ticks = None;
    let mut next_posture = None;
    let mut next_awareness = None;
    let mut next_pressure = None;

    sandbox_toolbar(
        ui,
        "AI target state demo",
        "Readable target posture, awareness, pressure, and explicit commitment or vulnerability windows.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Commit +2").clicked() {
                    commitment_ticks = Some(2);
                }
                if ui.button("Vulnerable +2").clicked() {
                    vulnerability_ticks = Some(2);
                }
                if ui.button("Tick 1").clicked() {
                    tick_state = true;
                }
                if ui.button("Reset Sample").clicked() {
                    reset_sample = true;
                }
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button("Guarding").clicked() {
                    next_posture = Some(TargetPosture::Guarding);
                }
                if ui.button("Pressing").clicked() {
                    next_posture = Some(TargetPosture::Pressing);
                }
                if ui.button("Overcommitted").clicked() {
                    next_posture = Some(TargetPosture::Overcommitted);
                }
                if ui.button("Recovering").clicked() {
                    next_posture = Some(TargetPosture::Recovering);
                }
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button("Relaxed").clicked() {
                    next_awareness = Some(TargetAwareness::Relaxed);
                }
                if ui.button("Tracking").clicked() {
                    next_awareness = Some(TargetAwareness::Tracking);
                }
                if ui.button("Locked On").clicked() {
                    next_awareness = Some(TargetAwareness::LockedOn);
                }
                if ui.button("Panicked").clicked() {
                    next_pressure = Some(TargetPressure::Panicked);
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = ai_target_state::sample_state() {
            app.ai_target_state_demo = sample;
            app.push_log("AI target state demo reset to the sample target.");
        }
    }
    if let Some(ticks) = commitment_ticks {
        app.ai_target_state_demo.start_commitment(ticks);
    }
    if let Some(ticks) = vulnerability_ticks {
        app.ai_target_state_demo.expose_vulnerability(ticks);
    }
    if tick_state {
        app.ai_target_state_demo.tick(1);
        app.push_log("AI target state demo ticked windows by 1.");
    }
    if let Some(posture) = next_posture {
        app.ai_target_state_demo.set_posture(posture);
    }
    if let Some(awareness) = next_awareness {
        app.ai_target_state_demo.set_awareness(awareness);
    }
    if let Some(pressure) = next_pressure {
        app.ai_target_state_demo.set_pressure(pressure);
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            posture_label(app.ai_target_state_demo.posture),
            egui::Color32::from_rgb(110, 214, 188),
        );
        stat_chip(
            ui,
            awareness_label(app.ai_target_state_demo.awareness),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            pressure_label(app.ai_target_state_demo.pressure),
            egui::Color32::from_rgb(255, 201, 110),
        );
        stat_chip(
            ui,
            format!(
                "commit {}",
                app.ai_target_state_demo.commitment_ticks_remaining
            ),
            egui::Color32::from_rgb(255, 133, 133),
        );
        stat_chip(
            ui,
            format!(
                "vulnerable {}",
                app.ai_target_state_demo.vulnerability_ticks_remaining
            ),
            egui::Color32::from_rgb(177, 150, 255),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Readable target snapshot",
        "This is the authored target surface that later intent, grading, and escalation layers consume.",
        |ui| {
            ui.monospace(format!(
                "label={}\ncommitted={}\nvulnerable={}",
                app.ai_target_state_demo.label,
                app.ai_target_state_demo.is_committed(),
                app.ai_target_state_demo.is_vulnerable()
            ));
        },
    );
}

fn posture_label(posture: TargetPosture) -> &'static str {
    match posture {
        TargetPosture::Guarding => "guarding",
        TargetPosture::Pressing => "pressing",
        TargetPosture::Overcommitted => "overcommitted",
        TargetPosture::Recovering => "recovering",
    }
}

fn awareness_label(awareness: TargetAwareness) -> &'static str {
    match awareness {
        TargetAwareness::Relaxed => "relaxed",
        TargetAwareness::Tracking => "tracking",
        TargetAwareness::Alerted => "alerted",
        TargetAwareness::LockedOn => "locked_on",
    }
}

fn pressure_label(pressure: TargetPressure) -> &'static str {
    match pressure {
        TargetPressure::Stable => "stable",
        TargetPressure::Strained => "strained",
        TargetPressure::Panicked => "panicked",
    }
}
