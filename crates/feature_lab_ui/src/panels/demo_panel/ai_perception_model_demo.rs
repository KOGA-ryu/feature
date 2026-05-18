use ai_perception_model::PerceptionEvent;
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_ai_perception_model_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut reset_sample = false;
    let mut event = None;

    sandbox_toolbar(
        ui,
        "AI perception model demo",
        "Deterministic record of what the target noticed, missed, or misread.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Notice Advance").clicked() {
                    event = Some(PerceptionEvent::NoticedAdvance);
                }
                if ui.button("Notice Bait").clicked() {
                    event = Some(PerceptionEvent::NoticedBait);
                }
                if ui.button("Miss Telegraph").clicked() {
                    event = Some(PerceptionEvent::MissedTelegraph);
                }
                if ui.button("Misread Feint").clicked() {
                    event = Some(PerceptionEvent::MisreadFeint);
                }
                if ui.button("Reset Sample").clicked() {
                    reset_sample = true;
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = ai_perception_model::sample_state() {
            app.ai_perception_demo = sample;
            app.push_log("AI perception demo reset to the sample snapshot.");
        }
    }
    if let Some(event) = event {
        app.ai_perception_demo.apply(event);
        app.push_log(format!("AI perception demo applied {:?}.", event));
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("noticed {}", app.ai_perception_demo.noticed_count),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            format!("missed {}", app.ai_perception_demo.missed_count),
            egui::Color32::from_rgb(255, 201, 110),
        );
        stat_chip(
            ui,
            format!("misread {}", app.ai_perception_demo.misread_count),
            egui::Color32::from_rgb(255, 133, 133),
        );
        stat_chip(
            ui,
            format!("certainty {}", app.ai_perception_demo.certainty),
            egui::Color32::from_rgb(126, 188, 255),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Perception snapshot",
        "Hosts can use this as a readable explanation layer before they choose intent or grade interactions.",
        |ui| {
            ui.monospace(format!(
                "last_event={:?}\nmisread_pressure={}",
                app.ai_perception_demo.last_event,
                app.ai_perception_demo.has_recent_misread_pressure()
            ));
        },
    );
}
