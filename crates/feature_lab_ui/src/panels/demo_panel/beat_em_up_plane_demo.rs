use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_beat_em_up_plane_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut movement = (0, 0);
    let mut reset_sample = false;

    sandbox_toolbar(
        ui,
        "Beat-em-up plane demo",
        "Move actors on a bounded x/depth plane and inspect deterministic engagement candidates.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Player Left").clicked() {
                    movement.0 -= 2;
                }
                if ui.button("Player Right").clicked() {
                    movement.0 += 2;
                }
                if ui.button("Depth -1").clicked() {
                    movement.1 -= 1;
                }
                if ui.button("Depth +1").clicked() {
                    movement.1 += 1;
                }
                reset_sample = ui.button("Reset Sample").clicked();
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = beat_em_up_plane::sample_state() {
            app.beat_em_up_plane_demo = sample;
            app.push_log("Beat-em-up plane demo reset to the sample encounter.");
        }
    }
    if movement != (0, 0)
        && app
            .beat_em_up_plane_demo
            .move_actor(1, movement.0, movement.1)
    {
        app.push_log("Beat-em-up plane demo moved actor 1.");
    }

    let near_candidates = app.beat_em_up_plane_demo.engagement_candidates(1, 1);
    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!(
                "bounds x {}..{}",
                app.beat_em_up_plane_demo.bounds.min_x, app.beat_em_up_plane_demo.bounds.max_x
            ),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!(
                "depth {}..{}",
                app.beat_em_up_plane_demo.bounds.min_depth,
                app.beat_em_up_plane_demo.bounds.max_depth
            ),
            egui::Color32::from_rgb(110, 214, 188),
        );
        stat_chip(
            ui,
            format!("nearby {}", near_candidates.len()),
            egui::Color32::from_rgb(255, 201, 110),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Plane actors",
        "Current actor ordering and clamped positions inside the encounter space.",
        |ui| {
            for actor in &app.beat_em_up_plane_demo.actors {
                ui.monospace(format!(
                    "#{} {} -> x={}, depth={}",
                    actor.id, actor.label, actor.state.position_x, actor.state.position_depth
                ));
            }
            ui.add_space(6.0);
            ui.small(format!(
                "Engagement candidates for actor 1 at depth delta 1: {:?}",
                near_candidates
            ));
        },
    );
}
