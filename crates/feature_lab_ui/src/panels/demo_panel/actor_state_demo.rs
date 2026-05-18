use actor_state::{ActorActionState, ActorFacing};
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_actor_state_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut reset_sample = false;
    let mut tick_state = false;
    let mut damage_amount = None;
    let mut invulnerability_ticks = None;
    let mut next_action_state = None;
    let mut next_facing = None;
    let mut movement = (0, 0);

    sandbox_toolbar(
        ui,
        "Actor state demo",
        "Shared player or enemy actor record with health, lives, facing, positions, and recovery timers.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Damage 1").clicked() {
                    damage_amount = Some(1);
                }
                if ui.button("Damage 2").clicked() {
                    damage_amount = Some(2);
                }
                if ui.button("Invuln +2").clicked() {
                    invulnerability_ticks = Some(2);
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
                if ui.button("Face Left").clicked() {
                    next_facing = Some(ActorFacing::Left);
                }
                if ui.button("Face Right").clicked() {
                    next_facing = Some(ActorFacing::Right);
                }
                if ui.button("Hitstun").clicked() {
                    next_action_state = Some(ActorActionState::Hitstun);
                }
                if ui.button("Knocked Down").clicked() {
                    next_action_state = Some(ActorActionState::KnockedDown);
                }
                if ui.button("Crashed").clicked() {
                    next_action_state = Some(ActorActionState::Crashed);
                }
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button("X -2").clicked() {
                    movement.0 -= 2;
                }
                if ui.button("X +2").clicked() {
                    movement.0 += 2;
                }
                if ui.button("Depth -1").clicked() {
                    movement.1 -= 1;
                }
                if ui.button("Depth +1").clicked() {
                    movement.1 += 1;
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = actor_state::sample_state() {
            app.actor_state_demo = sample;
            app.push_log("Actor state demo reset to the sample actor.");
        }
    }
    if let Some(amount) = damage_amount {
        let applied = app.actor_state_demo.apply_damage(amount);
        app.push_log(format!("Actor state demo applied {applied} damage."));
    }
    if let Some(ticks) = invulnerability_ticks {
        app.actor_state_demo.grant_invulnerability(ticks);
        app.push_log(format!(
            "Actor state demo granted {ticks} invulnerability ticks."
        ));
    }
    if tick_state {
        app.actor_state_demo.tick_state(1);
        app.push_log("Actor state demo ticked timers by 1.");
    }
    if let Some(facing) = next_facing {
        app.actor_state_demo.set_facing(facing);
    }
    if let Some(action_state) = next_action_state {
        app.actor_state_demo.enter_action_state(action_state, 2);
    }
    if movement != (0, 0) {
        let actor = &mut app.actor_state_demo;
        actor.set_position(
            actor.position_x.saturating_add(movement.0),
            actor.position_depth.saturating_add(movement.1),
        );
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!(
                "health {}/{}",
                app.actor_state_demo.health, app.actor_state_demo.max_health
            ),
            egui::Color32::from_rgb(255, 133, 133),
        );
        stat_chip(
            ui,
            format!("lives {}", app.actor_state_demo.lives),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            facing_label(app.actor_state_demo.facing),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            action_label(app.actor_state_demo.action_state),
            egui::Color32::from_rgb(110, 214, 188),
        );
        stat_chip(
            ui,
            format!("invuln {}", app.actor_state_demo.invulnerable_ticks),
            egui::Color32::from_rgb(177, 150, 255),
        );
        stat_chip(
            ui,
            format!("recovery {}", app.actor_state_demo.recovery_ticks),
            egui::Color32::from_rgb(255, 201, 110),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Actor snapshot",
        "Current reusable actor record for combat and runner hosts.",
        |ui| {
            ui.monospace(format!(
                "position=({}, {})\ndefeated={}\nhealth={}\nlives={}",
                app.actor_state_demo.position_x,
                app.actor_state_demo.position_depth,
                app.actor_state_demo.is_defeated(),
                app.actor_state_demo.health,
                app.actor_state_demo.lives
            ));
        },
    );
}

fn facing_label(facing: ActorFacing) -> &'static str {
    match facing {
        ActorFacing::Left => "facing left",
        ActorFacing::Right => "facing right",
    }
}

fn action_label(action_state: ActorActionState) -> &'static str {
    match action_state {
        ActorActionState::Idle => "idle",
        ActorActionState::Moving => "moving",
        ActorActionState::Attacking => "attacking",
        ActorActionState::Hitstun => "hitstun",
        ActorActionState::KnockedDown => "knocked_down",
        ActorActionState::Crashed => "crashed",
        ActorActionState::Defeated => "defeated",
    }
}
