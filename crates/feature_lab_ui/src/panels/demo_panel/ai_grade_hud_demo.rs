use ai_interaction_grader::AIInteractionGrader;
use eframe::egui;
use layer_escalation::LayerEscalationDecision;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_ai_grade_hud_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut reset_sample = false;
    let mut add_adapted = false;
    let mut add_missed = false;

    sandbox_toolbar(
        ui,
        "AI grade HUD demo",
        "Visible-but-not-overwhelming feedback layer over the deterministic grade card.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Load Sample Events").clicked() {
                    reset_sample = true;
                }
                if ui.button("Add Adapted").clicked() {
                    add_adapted = true;
                }
                if ui.button("Add Missed").clicked() {
                    add_missed = true;
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = ai_interaction_event::sample_events() {
            app.ai_interaction_event_demo = sample;
            app.push_log("AI grade HUD demo reloaded the sample event stream.");
        }
    }
    if add_adapted {
        let tick = app.ai_interaction_event_demo.len() as u32 + 1;
        app.ai_interaction_event_demo
            .push(ai_interaction_event::AIInteractionEvent::new(
                ai_interaction_event::InteractionOutcome::PatternAdapted,
                Some(ai_intent_model::IntentKind::GuardBreakWindup),
                tick,
            ));
    }
    if add_missed {
        let tick = app.ai_interaction_event_demo.len() as u32 + 1;
        app.ai_interaction_event_demo
            .push(ai_interaction_event::AIInteractionEvent::new(
                ai_interaction_event::InteractionOutcome::MissedTelegraph,
                Some(ai_intent_model::IntentKind::QuickStrike),
                tick,
            ));
    }

    sync_ai_grade_stack(app);

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("band {:?}", app.ai_grade_hud_demo.visible_band),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            format!("confidence {:?}", app.ai_grade_hud_demo.confidence),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("zoom_ready {}", app.ai_grade_hud_demo.zoom_ready),
            egui::Color32::from_rgb(255, 201, 110),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Visible feedback",
        "The game can teach the player what it is reading without dumping raw internals on every frame.",
        |ui| {
            ui.heading(app.ai_grade_hud_demo.summary_label.as_str());
            ui.label(app.ai_grade_hud_demo.emphasis_label.as_str());
        },
    );
}

fn sync_ai_grade_stack(app: &mut FeatureLabApp) {
    app.ai_interaction_grade_demo = AIInteractionGrader::grade(&app.ai_interaction_event_demo);
    app.layer_escalation_demo =
        LayerEscalationDecision::from_grade_card(&app.ai_interaction_grade_demo);
    app.ai_grade_hud_demo
        .sync_from_grade(&app.ai_interaction_grade_demo);
}
