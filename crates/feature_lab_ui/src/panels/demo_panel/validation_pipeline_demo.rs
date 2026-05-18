use eframe::egui;
use validation_pipeline::{
    ValidationPipeline, sample_invalid_input, sample_valid_input, validate_input,
};

use crate::app::FeatureLabApp;

use super::cards::{empty_state_card, result_card, sandbox_toolbar};
use super::chips::{stat_chip, stats_row};

pub(super) fn show_validation_pipeline_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    let mut use_valid = false;
    let mut use_invalid = false;

    sandbox_toolbar(
        ui,
        "Validation pipeline demo",
        "Pure Rust validation rules, findings, severities, and aggregated output.",
        |ui| {
            ui.horizontal(|ui| {
                use_valid = ui.button("Valid Fixture").clicked();
                use_invalid = ui.button("Invalid Fixture").clicked();
            });
        },
    );
    ui.add_space(8.0);

    if use_valid {
        app.validation_pipeline_demo_use_invalid = false;
        app.push_log("Validation pipeline switched to valid fixture.");
    }
    if use_invalid {
        app.validation_pipeline_demo_use_invalid = true;
        app.push_log("Validation pipeline switched to invalid fixture.");
    }

    let input = if app.validation_pipeline_demo_use_invalid {
        match sample_invalid_input() {
            Ok(input) => input,
            Err(error) => {
                ui.label(error);
                return;
            }
        }
    } else {
        match sample_valid_input() {
            Ok(input) => input,
            Err(error) => {
                ui.label(error);
                return;
            }
        }
    };

    let pipeline = ValidationPipeline::default();
    let result = validate_input(&input);

    stats_row(ui, |ui| {
        stat_chip(
            ui,
            if app.validation_pipeline_demo_use_invalid {
                "fixture invalid"
            } else {
                "fixture valid"
            },
            if app.validation_pipeline_demo_use_invalid {
                egui::Color32::from_rgb(255, 133, 133)
            } else {
                egui::Color32::from_rgb(126, 217, 140)
            },
        );
        stat_chip(
            ui,
            format!("rules {}", pipeline.rules().len()),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("outcome {}", result.summary.outcome),
            if result.summary.error_count > 0 {
                egui::Color32::from_rgb(255, 133, 133)
            } else if result.summary.warning_count > 0 {
                egui::Color32::from_rgb(255, 201, 110)
            } else {
                egui::Color32::from_rgb(126, 217, 140)
            },
        );
        stat_chip(
            ui,
            format!(
                "e:{} w:{} total:{}",
                result.summary.error_count,
                result.summary.warning_count,
                result.summary.total_findings
            ),
            egui::Color32::from_rgb(188, 199, 220),
        );
    });
    ui.add_space(8.0);
    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("required {}", input.required_fields.len()),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!("collections {}", input.collections.len()),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!("timestamps {}", input.timestamps.len()),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!("dependencies {}", input.dependencies.len()),
            egui::Color32::from_rgb(188, 199, 220),
        );
        stat_chip(
            ui,
            format!("compatible {}", input.compatible_features.len()),
            egui::Color32::from_rgb(188, 199, 220),
        );
    });
    ui.add_space(8.0);

    if result.findings.is_empty() {
        empty_state_card(ui, "No findings.");
    } else {
        for finding in result.findings {
            let severity_color = match finding.severity.to_string().as_str() {
                "error" => egui::Color32::from_rgb(255, 133, 133),
                "warning" => egui::Color32::from_rgb(255, 201, 110),
                _ => egui::Color32::from_rgb(188, 199, 220),
            };
            result_card(ui, false, severity_color, |ui| {
                ui.horizontal_wrapped(|ui| {
                    stat_chip(ui, finding.severity.to_string(), severity_color);
                    stat_chip(
                        ui,
                        finding.code.clone(),
                        egui::Color32::from_rgb(188, 199, 220),
                    );
                });
                ui.add_space(4.0);
                ui.small(
                    egui::RichText::new(finding.path.clone())
                        .monospace()
                        .color(egui::Color32::from_gray(185)),
                );
                ui.label(finding.message);
            });
            ui.add_space(6.0);
        }
    }
}
