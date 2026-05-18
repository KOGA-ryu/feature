use ai_interaction_grader::AIInteractionGrader;
use eframe::egui;
use layer_escalation::LayerEscalationDecision;

use crate::app::FeatureLabApp;

use super::cards::{sandbox_toolbar, section_card};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_ai_interaction_grader_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut reset_sample = false;

    sandbox_toolbar(
        ui,
        "AI interaction grader demo",
        "Turns the shared event stream into stable read, timing, control, adaptation, efficiency, style, band, and confidence output.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Load Sample Events").clicked() {
                    reset_sample = true;
                }
            });
        },
    );
    ui.add_space(8.0);

    if reset_sample {
        if let Ok(sample) = ai_interaction_event::sample_events() {
            app.ai_interaction_event_demo = sample;
            app.push_log("AI interaction grader demo reloaded the sample event stream.");
        }
    }

    sync_ai_grade_stack(app);

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("band {:?}", app.ai_interaction_grade_demo.overall_band),
            egui::Color32::from_rgb(126, 217, 140),
        );
        stat_chip(
            ui,
            format!("confidence {:?}", app.ai_interaction_grade_demo.confidence),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!(
                "poor tail {}",
                app.ai_interaction_grade_demo.poor_read_streak
            ),
            egui::Color32::from_rgb(255, 133, 133),
        );
    });
    ui.add_space(8.0);

    section_card(
        ui,
        "Grade card",
        "The same deterministic event stream should always produce the same card.",
        |ui| {
            ui.monospace(format!(
                "read_quality={}\ntiming={}\ncontrol={}\nadaptation={}\nefficiency={}\nstyle={}\nevents={}",
                app.ai_interaction_grade_demo.read_quality,
                app.ai_interaction_grade_demo.timing,
                app.ai_interaction_grade_demo.control,
                app.ai_interaction_grade_demo.adaptation,
                app.ai_interaction_grade_demo.efficiency,
                app.ai_interaction_grade_demo.style,
                app.ai_interaction_grade_demo.event_count,
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
