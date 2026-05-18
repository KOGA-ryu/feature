use std::fmt;

use feature_core::{FeatureLabResult, FeatureManifest, parse_feature_manifest};
use serde::{Deserialize, Serialize};

pub const FEATURE_ID: &str = "ui.calculator_basic";
pub const DEFAULT_HISTORY_LIMIT: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculatorOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl CalculatorOperator {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "×",
            Self::Divide => "÷",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculatorHistoryEntry {
    pub expression: String,
    pub result: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculatorEvaluationError {
    DivideByZero,
}

impl fmt::Display for CalculatorEvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivideByZero => formatter.write_str("Cannot divide by zero."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculatorState {
    display: String,
    accumulator: Option<f64>,
    accumulator_text: Option<String>,
    pending_operator: Option<CalculatorOperator>,
    waiting_for_new_input: bool,
    error: Option<CalculatorEvaluationError>,
    history: Vec<CalculatorHistoryEntry>,
    history_limit: usize,
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            display: "0".into(),
            accumulator: None,
            accumulator_text: None,
            pending_operator: None,
            waiting_for_new_input: false,
            error: None,
            history: Vec::new(),
            history_limit: DEFAULT_HISTORY_LIMIT,
        }
    }
}

impl CalculatorState {
    pub fn new(history_limit: usize) -> Self {
        Self {
            history_limit: history_limit.max(1),
            ..Self::default()
        }
    }

    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn history(&self) -> &[CalculatorHistoryEntry] {
        &self.history
    }

    pub fn error(&self) -> Option<&CalculatorEvaluationError> {
        self.error.as_ref()
    }

    pub fn input_digit(&mut self, digit: char) {
        if !digit.is_ascii_digit() {
            return;
        }
        self.reset_after_error_if_needed();
        if self.waiting_for_new_input || self.display == "0" {
            self.display = digit.to_string();
            self.waiting_for_new_input = false;
            return;
        }
        self.display.push(digit);
    }

    pub fn input_decimal(&mut self) {
        self.reset_after_error_if_needed();
        if self.waiting_for_new_input {
            self.display = "0.".into();
            self.waiting_for_new_input = false;
            return;
        }
        if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    pub fn apply_percent(&mut self) {
        self.reset_after_error_if_needed();
        let current_value = self.current_value();
        self.display = format_number(current_value / 100.0);
        self.waiting_for_new_input = false;
    }

    pub fn apply_operator(&mut self, operator: CalculatorOperator) {
        self.reset_after_error_if_needed();
        let current_display = self.display.clone();
        let current_value = self.current_value();

        if let (Some(accumulator), Some(pending_operator)) =
            (self.accumulator, self.pending_operator)
        {
            if !self.waiting_for_new_input {
                match apply_operation(accumulator, current_value, pending_operator) {
                    Ok(result) => {
                        self.display = format_number(result);
                        self.accumulator = Some(result);
                        self.accumulator_text = Some(self.display.clone());
                    }
                    Err(error) => {
                        self.set_error(error);
                        return;
                    }
                }
            }
        } else {
            self.accumulator = Some(current_value);
            self.accumulator_text = Some(current_display);
        }

        self.pending_operator = Some(operator);
        self.waiting_for_new_input = true;
    }

    pub fn evaluate(&mut self) {
        self.reset_after_error_if_needed();
        let (Some(accumulator), Some(operator), Some(left_text)) = (
            self.accumulator,
            self.pending_operator,
            self.accumulator_text.clone(),
        ) else {
            return;
        };

        let right_text = self.display.clone();
        let right_value = self.current_value();
        match apply_operation(accumulator, right_value, operator) {
            Ok(result) => {
                let result_text = format_number(result);
                self.push_history(CalculatorHistoryEntry {
                    expression: format!("{} {} {}", left_text, operator.symbol(), right_text),
                    result: result_text.clone(),
                });
                self.display = result_text;
                self.accumulator = None;
                self.accumulator_text = None;
                self.pending_operator = None;
                self.waiting_for_new_input = true;
            }
            Err(error) => self.set_error(error),
        }
    }

    pub fn clear(&mut self) {
        self.display = "0".into();
        self.accumulator = None;
        self.accumulator_text = None;
        self.pending_operator = None;
        self.waiting_for_new_input = false;
        self.error = None;
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    fn current_value(&self) -> f64 {
        self.display.parse::<f64>().unwrap_or(0.0)
    }

    fn push_history(&mut self, entry: CalculatorHistoryEntry) {
        self.history.push(entry);
        if self.history.len() > self.history_limit {
            let overflow = self.history.len() - self.history_limit;
            self.history.drain(0..overflow);
        }
    }

    fn set_error(&mut self, error: CalculatorEvaluationError) {
        self.display = "Error".into();
        self.accumulator = None;
        self.accumulator_text = None;
        self.pending_operator = None;
        self.waiting_for_new_input = true;
        self.error = Some(error);
    }

    fn reset_after_error_if_needed(&mut self) {
        if self.error.is_some() {
            self.clear();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculatorFixture {
    pub steps: Vec<String>,
}

pub fn manifest() -> FeatureLabResult<FeatureManifest> {
    parse_feature_manifest(include_str!("../feature.toml"))
}

pub fn documentation_preview() -> &'static str {
    include_str!("../README.md")
}

pub fn sample_fixture() -> &'static str {
    include_str!("../fixtures/sample_input.json")
}

pub fn sample_steps() -> Result<Vec<String>, String> {
    let fixture: CalculatorFixture =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    Ok(fixture.steps)
}

pub fn sample_fixture_pretty() -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(sample_fixture()).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|error| error.to_string())
}

fn apply_operation(
    left: f64,
    right: f64,
    operator: CalculatorOperator,
) -> Result<f64, CalculatorEvaluationError> {
    match operator {
        CalculatorOperator::Add => Ok(left + right),
        CalculatorOperator::Subtract => Ok(left - right),
        CalculatorOperator::Multiply => Ok(left * right),
        CalculatorOperator::Divide => {
            if right == 0.0 {
                Err(CalculatorEvaluationError::DivideByZero)
            } else {
                Ok(left / right)
            }
        }
    }
}

fn format_number(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        let mut formatted = format!("{value:.8}");
        while formatted.contains('.') && formatted.ends_with('0') {
            formatted.pop();
        }
        if formatted.ends_with('.') {
            formatted.pop();
        }
        formatted
    }
}
