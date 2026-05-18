use eframe::egui;
use runner_track::{RunnerLane, RunnerSpeedState};

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_runner_track_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut advance_distance = None;
    let mut clear_passed = false;
    let mut reset_sample = false;
    let mut shift_left = false;
    let mut shift_right = false;

    sandbox_toolbar(
        ui,
        "Runner track demo",
        "Lane-based runner state with forward distance, obstacle collisions, and passed-obstacle cleanup.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                shift_left = ui.button("Shift Left").clicked();
                shift_right = ui.button("Shift Right").clicked();
                if ui.button("+2").clicked() {
                    advance_distance = Some(2);
                }
                if ui.button("+5").clicked() {
                    advance_distance = Some(5);
                }
                clear_passed = ui.button("Clear Passed").clicked();
                reset_sample = ui.button("Reset Sample").clicked();
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button("Speed Slow").clicked() {
                    app.runner_track_demo
                        .set_speed_state(RunnerSpeedState::Slow);
                }
                if ui.button("Speed Cruising").clicked() {
                    app.runner_track_demo
                        .set_speed_state(RunnerSpeedState::Cruising);
                }
                if ui.button("Speed Fast").clicked() {
                    app.runner_track_demo
                        .set_speed_state(RunnerSpeedState::Fast);
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = runner_track::sample_state() {
            app.runner_track_demo = sample;
            app.push_log("Runner track demo reset to the sample lane state.");
        }
    }
    if shift_left && app.runner_track_demo.shift_left() {
        app.push_log("Runner track demo shifted left.");
    }
    if shift_right && app.runner_track_demo.shift_right() {
        app.push_log("Runner track demo shifted right.");
    }
    if let Some(distance) = advance_distance {
        app.runner_track_demo.advance(distance);
        let collisions = app.runner_track_demo.detect_collisions();
        if collisions.is_empty() {
            app.push_log(format!("Runner track demo advanced by {distance}."));
        } else {
            app.push_log(format!(
                "Runner track demo collided with {:?}.",
                collisions
                    .iter()
                    .map(|obstacle| obstacle.label.as_str())
                    .collect::<Vec<_>>()
            ));
        }
    }
    if clear_passed {
        let removed = app.runner_track_demo.clear_passed_obstacles();
        app.push_log(format!(
            "Runner track demo cleared {removed} passed obstacles."
        ));
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            lane_label(app.runner_track_demo.current_lane),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            speed_label(app.runner_track_demo.speed_state),
            egui::Color32::from_rgb(110, 214, 188),
        );
        stat_chip(
            ui,
            format!("distance {}", app.runner_track_demo.distance_travelled),
            egui::Color32::from_rgb(255, 201, 110),
        );
        stat_chip(
            ui,
            format!("obstacles {}", app.runner_track_demo.obstacles.len()),
            egui::Color32::from_rgb(188, 199, 220),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Obstacle stream",
        "Current obstacle stream state after lane changes, advances, and cleanup.",
        |ui| {
            for obstacle in &app.runner_track_demo.obstacles {
                ui.monospace(format!(
                    "#{} {} -> lane={}, distance={}, resolved={}",
                    obstacle.id,
                    obstacle.label,
                    lane_label(obstacle.lane),
                    obstacle.distance,
                    obstacle.resolved
                ));
            }
        },
    );
}

fn lane_label(lane: RunnerLane) -> &'static str {
    match lane {
        RunnerLane::Left => "left",
        RunnerLane::Center => "center",
        RunnerLane::Right => "right",
    }
}

fn speed_label(speed: RunnerSpeedState) -> &'static str {
    match speed {
        RunnerSpeedState::Slow => "slow",
        RunnerSpeedState::Cruising => "cruising",
        RunnerSpeedState::Fast => "fast",
    }
}
