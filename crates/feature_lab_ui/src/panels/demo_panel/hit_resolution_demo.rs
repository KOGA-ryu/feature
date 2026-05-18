use eframe::egui;
use hit_resolution::{HitClass, HitResolution};

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_hit_resolution_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut apply_hit = false;
    let mut tick_target = false;
    let mut reset_sample = false;

    sandbox_toolbar(
        ui,
        "Hit resolution demo",
        "Apply one deterministic hit request to one actor and inspect the resulting state transition.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                apply_hit = ui.button("Apply Hit").clicked();
                tick_target = ui.button("Tick Target 1").clicked();
                reset_sample = ui.button("Reset Sample").clicked();
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label("class");
                egui::ComboBox::from_id_salt("hit_resolution_class")
                    .selected_text(hit_class_label(app.hit_resolution_demo_request.class))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.hit_resolution_demo_request.class,
                            HitClass::Light,
                            hit_class_label(HitClass::Light),
                        );
                        ui.selectable_value(
                            &mut app.hit_resolution_demo_request.class,
                            HitClass::Heavy,
                            hit_class_label(HitClass::Heavy),
                        );
                        ui.selectable_value(
                            &mut app.hit_resolution_demo_request.class,
                            HitClass::Knockdown,
                            hit_class_label(HitClass::Knockdown),
                        );
                        ui.selectable_value(
                            &mut app.hit_resolution_demo_request.class,
                            HitClass::Crash,
                            hit_class_label(HitClass::Crash),
                        );
                    });
                ui.label("damage");
                ui.add(
                    egui::DragValue::new(&mut app.hit_resolution_demo_request.damage).range(0..=9),
                );
                ui.label("invuln");
                ui.add(
                    egui::DragValue::new(
                        &mut app.hit_resolution_demo_request.invulnerability_ticks,
                    )
                    .range(0..=9),
                );
                ui.label("recovery");
                ui.add(
                    egui::DragValue::new(&mut app.hit_resolution_demo_request.recovery_ticks)
                        .range(0..=9),
                );
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = hit_resolution::sample_scenario() {
            app.hit_resolution_demo_target = sample.target;
            app.hit_resolution_demo_request = sample.request;
            app.hit_resolution_demo_last_result = None;
            app.push_log("Hit resolution demo reset to the sample scenario.");
        }
    }
    if tick_target {
        app.hit_resolution_demo_target.tick_state(1);
        app.push_log("Hit resolution demo ticked the target actor by 1.");
    }
    if apply_hit {
        let result = HitResolution::apply(
            &mut app.hit_resolution_demo_target,
            &app.hit_resolution_demo_request,
        );
        app.hit_resolution_demo_last_result = Some(result.clone());
        app.push_log(format!(
            "Hit resolution demo applied a {} hit.",
            hit_class_label(app.hit_resolution_demo_request.class)
        ));
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!(
                "health {}/{}",
                app.hit_resolution_demo_target.health, app.hit_resolution_demo_target.max_health
            ),
            egui::Color32::from_rgb(255, 133, 133),
        );
        stat_chip(
            ui,
            format!("lives {}", app.hit_resolution_demo_target.lives),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            format!("state {:?}", app.hit_resolution_demo_target.action_state)
                .to_lowercase()
                .replace("actoractionstate::", ""),
            egui::Color32::from_rgb(110, 214, 188),
        );
        stat_chip(
            ui,
            format!(
                "invuln {}",
                app.hit_resolution_demo_target.invulnerable_ticks
            ),
            egui::Color32::from_rgb(177, 150, 255),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Last hit result",
        "Latest deterministic outcome over the selected target actor.",
        |ui| {
            if let Some(result) = &app.hit_resolution_demo_last_result {
                ui.monospace(format!(
                    "damage_applied={}\nlife_lost={}\ndefeated={}\nignored={}\nentered_state={:?}",
                    result.damage_applied,
                    result.life_lost,
                    result.defeated,
                    result.ignored,
                    result.entered_state
                ));
            } else {
                ui.label("Apply a hit to inspect the result payload.");
            }
        },
    );
}

fn hit_class_label(hit_class: HitClass) -> &'static str {
    match hit_class {
        HitClass::Light => "light",
        HitClass::Heavy => "heavy",
        HitClass::Knockdown => "knockdown",
        HitClass::Crash => "crash",
    }
}
