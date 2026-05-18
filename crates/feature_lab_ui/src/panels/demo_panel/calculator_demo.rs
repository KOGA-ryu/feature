use calculator_basic::CalculatorOperator;
use eframe::egui;

use crate::app::FeatureLabApp;

use super::cards::{empty_state_card, result_card, sandbox_toolbar, section_card};
use super::chips::{activation_chip, stat_chip, stats_row};

pub(super) fn show_calculator_basic_demo(ui: &mut egui::Ui, app: &mut FeatureLabApp) {
    sandbox_toolbar(
        ui,
        "Calculator demo",
        "Left-to-right reducer with direct percent handling and bounded history.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Clear").clicked() {
                    app.calculator_demo.clear();
                    app.push_log("Calculator display cleared.");
                }
                if ui.button("Clear History").clicked() {
                    app.calculator_demo.clear_history();
                    app.push_log("Calculator history cleared.");
                }
            });
        },
    );
    ui.add_space(8.0);

    let error_label = app
        .calculator_demo
        .error()
        .map(ToString::to_string)
        .unwrap_or_else(|| "No error".into());
    stats_row(ui, |ui| {
        stat_chip(
            ui,
            format!("display {}", app.calculator_demo.display()),
            egui::Color32::from_rgb(126, 188, 255),
        );
        stat_chip(
            ui,
            format!("history {}", app.calculator_demo.history().len()),
            egui::Color32::from_rgb(126, 217, 140),
        );
        activation_chip(ui, &error_label);
    });
    ui.add_space(8.0);

    result_card(ui, false, egui::Color32::from_rgb(126, 188, 255), |ui| {
        ui.small("Display");
        ui.heading(app.calculator_demo.display());
    });
    ui.add_space(8.0);

    egui::Grid::new("calculator_basic_demo_grid")
        .num_columns(4)
        .spacing([8.0, 8.0])
        .show(ui, |ui| {
            calculator_button(ui, "7", || app.calculator_demo.input_digit('7'));
            calculator_button(ui, "8", || app.calculator_demo.input_digit('8'));
            calculator_button(ui, "9", || app.calculator_demo.input_digit('9'));
            calculator_button(ui, "÷", || {
                app.calculator_demo
                    .apply_operator(CalculatorOperator::Divide)
            });
            ui.end_row();

            calculator_button(ui, "4", || app.calculator_demo.input_digit('4'));
            calculator_button(ui, "5", || app.calculator_demo.input_digit('5'));
            calculator_button(ui, "6", || app.calculator_demo.input_digit('6'));
            calculator_button(ui, "×", || {
                app.calculator_demo
                    .apply_operator(CalculatorOperator::Multiply)
            });
            ui.end_row();

            calculator_button(ui, "1", || app.calculator_demo.input_digit('1'));
            calculator_button(ui, "2", || app.calculator_demo.input_digit('2'));
            calculator_button(ui, "3", || app.calculator_demo.input_digit('3'));
            calculator_button(ui, "-", || {
                app.calculator_demo
                    .apply_operator(CalculatorOperator::Subtract)
            });
            ui.end_row();

            calculator_button(ui, "0", || app.calculator_demo.input_digit('0'));
            calculator_button(ui, ".", || app.calculator_demo.input_decimal());
            calculator_button(ui, "%", || app.calculator_demo.apply_percent());
            calculator_button(ui, "+", || {
                app.calculator_demo.apply_operator(CalculatorOperator::Add)
            });
            ui.end_row();

            calculator_button(ui, "=", || app.calculator_demo.evaluate());
            ui.end_row();
        });

    ui.add_space(12.0);
    section_card(
        ui,
        "History",
        "Completed calculations kept inside the bounded reducer history.",
        |ui| {
            if app.calculator_demo.history().is_empty() {
                empty_state_card(ui, "No completed calculations yet.");
            } else {
                for entry in app.calculator_demo.history().iter().rev() {
                    result_card(ui, false, egui::Color32::from_rgb(126, 217, 140), |ui| {
                        ui.horizontal_wrapped(|ui| {
                            stat_chip(ui, &entry.result, egui::Color32::from_rgb(126, 217, 140));
                            ui.label(&entry.expression);
                        });
                    });
                    ui.add_space(6.0);
                }
            }
        },
    );
}

fn calculator_button(ui: &mut egui::Ui, label: &str, on_click: impl FnOnce()) {
    if ui
        .add_sized([52.0, 34.0], egui::Button::new(label))
        .clicked()
    {
        on_click();
    }
}
