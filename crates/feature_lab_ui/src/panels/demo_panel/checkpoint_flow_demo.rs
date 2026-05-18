use checkpoint_flow::StageMode;
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_checkpoint_flow_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut complete_current = false;
    let mut finish_transition = false;
    let mut restart = false;
    let mut reset_sample = false;

    sandbox_toolbar(
        ui,
        "Checkpoint flow demo",
        "Linear hybrid-stage progression with pending transitions between brawler and runner checkpoints.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                complete_current = ui.button("Complete Current").clicked();
                finish_transition = ui.button("Finish Transition").clicked();
                restart = ui.button("Restart Checkpoint").clicked();
                reset_sample = ui.button("Reset Sample").clicked();
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = checkpoint_flow::sample_state() {
            app.checkpoint_flow_demo = sample;
            app.push_log("Checkpoint flow demo reset to the sample route.");
        }
    }
    if complete_current && app.checkpoint_flow_demo.complete_current_checkpoint() {
        app.push_log("Checkpoint flow demo completed the current checkpoint.");
    }
    if finish_transition && app.checkpoint_flow_demo.finish_transition() {
        app.push_log("Checkpoint flow demo finished the pending transition.");
    }
    if restart && app.checkpoint_flow_demo.restart_from_checkpoint() {
        app.push_log("Checkpoint flow demo restarted from the current checkpoint.");
    }

    let current_id = app
        .checkpoint_flow_demo
        .current_checkpoint()
        .map(|checkpoint| checkpoint.id)
        .unwrap_or_default();
    stats_row(ui, |ui| {
        stat_chip(
            ui,
            mode_label(app.checkpoint_flow_demo.current_mode),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("checkpoint {}", current_id),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!(
                "completed {}",
                app.checkpoint_flow_demo.completed_checkpoint_ids.len()
            ),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            if app.checkpoint_flow_demo.pending_transition.is_some() {
                "transition pending"
            } else {
                "transition clear"
            },
            egui::Color32::from_rgb(255, 201, 110),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Stage route",
        "Current ordered route with completion state and pending mode transitions.",
        |ui| {
            for checkpoint in &app.checkpoint_flow_demo.checkpoints {
                let current = app
                    .checkpoint_flow_demo
                    .current_checkpoint()
                    .is_some_and(|active| active.id == checkpoint.id);
                let completed = app
                    .checkpoint_flow_demo
                    .completed_checkpoint_ids
                    .contains(&checkpoint.id);
                ui.monospace(format!(
                    "#{} {} -> {} current={} completed={}",
                    checkpoint.id,
                    checkpoint.label,
                    mode_label(checkpoint.mode),
                    current,
                    completed
                ));
            }
        },
    );
}

fn mode_label(mode: StageMode) -> &'static str {
    match mode {
        StageMode::Brawler => "brawler",
        StageMode::Racing => "racing",
    }
}
