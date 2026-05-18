use ai_interaction_grader::AIInteractionGrader;
use eframe::egui;
use layer_escalation::LayerEscalationDecision;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_layer_escalation_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut reset_sample = false;
    let mut apply_poor_tail = false;

    sandbox_toolbar(
        ui,
        "Layer escalation demo",
        "Simulation zoom decision law driven by the shared AI grade card.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Load Sample Events").clicked() {
                    reset_sample = true;
                }
                if ui.button("Apply Poor Tail").clicked() {
                    apply_poor_tail = true;
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = ai_interaction_event::sample_events() {
            app.ai_interaction_event_demo = sample;
            app.push_log("Layer escalation demo reloaded the sample event stream.");
        }
    }
    if apply_poor_tail {
        let base_tick = app.ai_interaction_event_demo.len() as u32 + 1;
        app.ai_interaction_event_demo.extend([
            ai_interaction_event::AIInteractionEvent::new(
                ai_interaction_event::InteractionOutcome::LateDodge,
                Some(ai_intent_model::IntentKind::QuickStrike),
                base_tick,
            ),
            ai_interaction_event::AIInteractionEvent::new(
                ai_interaction_event::InteractionOutcome::MissedTelegraph,
                Some(ai_intent_model::IntentKind::GuardBreakWindup),
                base_tick + 1,
            ),
        ]);
        app.push_log("Layer escalation demo appended a poor-read tail.");
    }

    sync_ai_grade_stack(app);

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("stay {}", app.layer_escalation_demo.stay_coarse),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!("offer {}", app.layer_escalation_demo.offer_zoom),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("force {}", app.layer_escalation_demo.force_zoom),
            egui::Color32::from_rgb(255, 133, 133),
        );
        stat_chip(
            ui,
            format!("reward {}", app.layer_escalation_demo.reward_modifier),
            egui::Color32::from_rgb(126, 217, 140),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Escalation snapshot",
        "This is the deterministic handoff law between coarse encounters and the duel zoom layer.",
        |ui| {
            ui.monospace(format!(
                "reason={:?}\nnext_layer={:?}",
                app.layer_escalation_demo.reason, app.layer_escalation_demo.next_layer
            ));
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
