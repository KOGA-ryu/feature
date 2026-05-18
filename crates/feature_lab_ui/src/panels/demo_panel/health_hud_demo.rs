use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_health_hud_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut damage_amount = None;
    let mut tick_flash = false;
    let mut reset_sample = false;

    sandbox_toolbar(
        ui,
        "Health HUD demo",
        "Segmented health and lives display synced from actor state with brief damage flash timing.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Damage 1").clicked() {
                    damage_amount = Some(1);
                }
                if ui.button("Damage 2").clicked() {
                    damage_amount = Some(2);
                }
                tick_flash = ui.button("Tick Flash 1").clicked();
                reset_sample = ui.button("Reset Sample").clicked();
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample_actor) = actor_state::sample_state() {
            app.health_hud_demo_actor = sample_actor.clone();
            app.health_hud_demo = health_hud::HealthHudState::from_actor(&sample_actor);
            app.push_log("Health HUD demo reset to the sample actor.");
        }
    }
    if let Some(amount) = damage_amount {
        let applied = app.health_hud_demo_actor.apply_damage(amount);
        app.health_hud_demo
            .sync_from_actor(&app.health_hud_demo_actor);
        app.push_log(format!(
            "Health HUD demo synced {applied} damage from the actor."
        ));
    }
    if tick_flash {
        app.health_hud_demo.tick_flash(1);
        app.push_log("Health HUD demo ticked flash timing by 1.");
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!(
                "health {}/{}",
                app.health_hud_demo.current_health, app.health_hud_demo.max_health
            ),
            egui::Color32::from_rgb(255, 133, 133),
        );
        stat_chip(
            ui,
            format!("lives {}", app.health_hud_demo.lives),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            if app.health_hud_demo.is_flash_active() {
                "flash active"
            } else {
                "flash idle"
            },
            egui::Color32::from_rgb(255, 201, 110),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Segments",
        "Current HUD segment fill derived from the synced actor state.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                for segment in &app.health_hud_demo.segments {
                    let color = if segment.filled {
                        egui::Color32::from_rgb(255, 133, 133)
                    } else {
                        egui::Color32::from_rgb(70, 78, 92)
                    };
                    ui.colored_label(color, if segment.filled { "■" } else { "□" });
                }
            });
            ui.add_space(6.0);
            ui.small(format!(
                "source actor -> health {} / {}, lives {}",
                app.health_hud_demo_actor.health,
                app.health_hud_demo_actor.max_health,
                app.health_hud_demo_actor.lives
            ));
        },
    );
}
