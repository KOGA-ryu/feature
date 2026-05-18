use ai_intent_model::IntentKind;
use ai_interaction_event::{AIInteractionEvent, InteractionOutcome, poor_read_streak};
use ai_interaction_grader::AIInteractionGrader;
use eframe::egui;
use layer_escalation::LayerEscalationDecision;

use crate::app::FeatureLabApp;

use super::cards::{empty_state_card, sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_ai_interaction_event_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut reset_sample = false;
    let mut next_event = None;

    sandbox_toolbar(
        ui,
        "AI interaction event demo",
        "Canonical event stream for what the player actually did with a readable AI target.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Baited Commit").clicked() {
                    next_event = Some((
                        InteractionOutcome::BaitedCommit,
                        Some(IntentKind::OvercommitLunge),
                    ));
                }
                if ui.button("Correct Dodge").clicked() {
                    next_event = Some((
                        InteractionOutcome::CorrectDodge,
                        Some(IntentKind::QuickStrike),
                    ));
                }
                if ui.button("Punish Hit").clicked() {
                    next_event = Some((
                        InteractionOutcome::PunishWindowHit,
                        Some(IntentKind::OvercommitLunge),
                    ));
                }
                if ui.button("Missed Telegraph").clicked() {
                    next_event = Some((
                        InteractionOutcome::MissedTelegraph,
                        Some(IntentKind::GuardBreakWindup),
                    ));
                }
                if ui.button("Bad Trade").clicked() {
                    next_event =
                        Some((InteractionOutcome::BadTrade, Some(IntentKind::QuickStrike)));
                }
                if ui.button("Reset Sample").clicked() {
                    reset_sample = true;
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = ai_interaction_event::sample_events() {
            app.ai_interaction_event_demo = sample;
            app.push_log("AI interaction event demo reset to the sample event stream.");
        }
    }
    if let Some((outcome, intent)) = next_event {
        let tick = app.ai_interaction_event_demo.len() as u32 + 1;
        app.ai_interaction_event_demo
            .push(AIInteractionEvent::new(outcome, intent, tick));
        app.push_log(format!("AI interaction event demo appended {:?}.", outcome));
    }

    sync_ai_grade_stack(app);

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("events {}", app.ai_interaction_event_demo.len()),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!(
                "poor tail {}",
                poor_read_streak(&app.ai_interaction_event_demo)
            ),
            egui::Color32::from_rgb(255, 133, 133),
        );
    });
    ui.add_space(8.0);

    if app.ai_interaction_event_demo.is_empty() {
        empty_state_card(
            ui,
            "No interaction events. Append events to see grading and simulation zoom react to the same deterministic event stream.",
        );
        return;
    }

    section_card(
        ui,
        "Event stream",
        "Stable interaction vocabulary consumed by grading, escalation, and later replay systems.",
        |ui| {
            for event in &app.ai_interaction_event_demo {
                ui.monospace(format!(
                    "t={} outcome={:?} intent={:?}",
                    event.tick, event.outcome, event.intent
                ));
            }
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
