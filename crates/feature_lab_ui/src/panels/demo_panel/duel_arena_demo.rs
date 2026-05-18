use ai_interaction_event::InteractionOutcome;
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_duel_arena_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut reset_sample = false;
    let mut tick_state = false;
    let mut push_player = false;
    let mut push_target = false;
    let mut light_exchange = false;
    let mut finisher_exchange = false;

    sandbox_toolbar(
        ui,
        "Duel arena demo",
        "First deep zoom simulation with richer positional response and deterministic duel outcomes.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Push Player").clicked() {
                    push_player = true;
                }
                if ui.button("Push Target").clicked() {
                    push_target = true;
                }
                if ui.button("Tick 1").clicked() {
                    tick_state = true;
                }
                if ui.button("Exchange 1").clicked() {
                    light_exchange = true;
                }
                if ui.button("Finisher").clicked() {
                    finisher_exchange = true;
                }
                if ui.button("Reset Sample").clicked() {
                    reset_sample = true;
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = duel_arena::sample_state() {
            app.duel_arena_demo = sample;
            app.push_log("Duel arena demo reset to the sample duel.");
        }
    }
    if push_player {
        app.duel_arena_demo.apply_push(1, -2);
    }
    if push_target {
        app.duel_arena_demo.apply_push(2, 2);
    }
    if tick_state {
        app.duel_arena_demo.tick(1);
    }
    if light_exchange {
        let _ = app
            .duel_arena_demo
            .resolve_exchange(1, 1, 2, InteractionOutcome::PunishWindowHit);
    }
    if finisher_exchange {
        let _ = app
            .duel_arena_demo
            .resolve_exchange(1, 5, 3, InteractionOutcome::PunishWindowHit);
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("player hp {}", app.duel_arena_demo.player.actor.health),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("target hp {}", app.duel_arena_demo.target.actor.health),
            egui::Color32::from_rgb(255, 133, 133),
        );
        stat_chip(
            ui,
            format!("contact {}", app.duel_arena_demo.in_contact_range()),
            egui::Color32::from_rgb(110, 214, 188),
        );
        stat_chip(
            ui,
            format!("tick {}", app.duel_arena_demo.elapsed_ticks),
            egui::Color32::from_rgb(255, 201, 110),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Arena snapshot",
        "This headless duel sim is the first deeper layer the escalation system can hand off to.",
        |ui| {
            ui.monospace(format!(
                "player=({}, v={})\ntarget=({}, v={})\noutcome={:?}",
                app.duel_arena_demo.player.position_x,
                app.duel_arena_demo.player.velocity_x,
                app.duel_arena_demo.target.position_x,
                app.duel_arena_demo.target.velocity_x,
                app.duel_arena_demo.last_outcome
            ));
        },
    );
}
