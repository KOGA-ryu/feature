use ai_intent_model::IntentKind;
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_ai_intent_model_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut reset_sample = false;
    let mut tick_state = false;
    let mut next_intent = None;
    let mut cancel = false;

    sandbox_toolbar(
        ui,
        "AI intent model demo",
        "Authored telegraph, committed, and recovery phases for one readable target behavior.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Quick Strike").clicked() {
                    next_intent = Some(IntentKind::QuickStrike);
                }
                if ui.button("Overcommit Lunge").clicked() {
                    next_intent = Some(IntentKind::OvercommitLunge);
                }
                if ui.button("Guard Break").clicked() {
                    next_intent = Some(IntentKind::GuardBreakWindup);
                }
                if ui.button("Tick 1").clicked() {
                    tick_state = true;
                }
                if ui.button("Cancel").clicked() {
                    cancel = true;
                }
                if ui.button("Reset Sample").clicked() {
                    reset_sample = true;
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = ai_intent_model::sample_state() {
            app.ai_intent_demo = sample;
            app.push_log("AI intent demo reset to the sample intent.");
        }
    }
    if let Some(intent) = next_intent {
        app.ai_intent_demo.choose_intent(intent, 2, 2, 1);
        app.push_log(format!("AI intent demo chose {:?}.", intent));
    }
    if tick_state {
        app.ai_intent_demo.tick(1);
        app.push_log("AI intent demo ticked by 1.");
    }
    if cancel {
        app.ai_intent_demo.cancel();
    }

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            intent_label(app.ai_intent_demo.current_intent),
            egui::Color32::from_rgb(110, 214, 188),
        );
        stat_chip(
            ui,
            commitment_label(app.ai_intent_demo.commitment),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("telegraph {}", app.ai_intent_demo.telegraph_ticks_remaining),
            egui::Color32::from_rgb(255, 201, 110),
        );
        stat_chip(
            ui,
            format!("commit {}", app.ai_intent_demo.commit_ticks_remaining),
            egui::Color32::from_rgb(255, 133, 133),
        );
        stat_chip(
            ui,
            format!("recovery {}", app.ai_intent_demo.recovery_ticks_remaining),
            egui::Color32::from_rgb(177, 150, 255),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Intent phase snapshot",
        "This is the deterministic authored behavior timeline that the player is meant to read and react to.",
        |ui| {
            ui.monospace(format!("committed={}", app.ai_intent_demo.is_committed()));
        },
    );
}

fn intent_label(intent: IntentKind) -> &'static str {
    match intent {
        IntentKind::QuickStrike => "quick_strike",
        IntentKind::OvercommitLunge => "overcommit_lunge",
        IntentKind::GuardBreakWindup => "guard_break_windup",
    }
}

fn commitment_label(commitment: ai_intent_model::IntentCommitment) -> &'static str {
    match commitment {
        ai_intent_model::IntentCommitment::Telegraphing => "telegraphing",
        ai_intent_model::IntentCommitment::Committed => "committed",
        ai_intent_model::IntentCommitment::Recovering => "recovering",
        ai_intent_model::IntentCommitment::Cancelled => "cancelled",
    }
}
