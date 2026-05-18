use calculator_basic::{
    CalculatorEvaluationError, CalculatorOperator, CalculatorState, DEFAULT_HISTORY_LIMIT,
    FEATURE_ID, sample_fixture, sample_steps,
};
use feature_core::parse_feature_manifest;

#[test]
fn feature_manifest_matches_contract() {
    let manifest =
        parse_feature_manifest(include_str!("../feature.toml")).expect("manifest should parse");

    assert_eq!(manifest.id, FEATURE_ID);
    assert_eq!(manifest.name, "calculator_basic");
    assert_eq!(manifest.kind.to_string(), "ui_pattern");
    assert_eq!(manifest.status.to_string(), "tested");
    assert_eq!(manifest.inputs.items.len(), 6);
    assert_eq!(
        manifest.outputs.items,
        vec!["display", "history", "error_state"]
    );
}

#[test]
fn sample_fixture_is_valid_json() {
    let fixture: serde_json::Value =
        serde_json::from_str(sample_fixture()).expect("fixture should be valid json");
    assert_eq!(fixture["steps"].as_array().map(Vec::len), Some(4));
}

#[test]
fn simple_arithmetic_evaluates() {
    let mut state = CalculatorState::default();
    state.input_digit('2');
    state.apply_operator(CalculatorOperator::Add);
    state.input_digit('2');
    state.evaluate();

    assert_eq!(state.display(), "4");
    assert_eq!(state.history().len(), 1);
    assert_eq!(state.history()[0].expression, "2 + 2");
    assert_eq!(state.history()[0].result, "4");
}

#[test]
fn chained_operations_are_left_to_right() {
    let mut state = CalculatorState::default();
    state.input_digit('9');
    state.apply_operator(CalculatorOperator::Subtract);
    state.input_digit('3');
    state.apply_operator(CalculatorOperator::Multiply);
    state.input_digit('2');
    state.evaluate();

    assert_eq!(state.display(), "12");
}

#[test]
fn decimal_and_percent_input_work() {
    let mut state = CalculatorState::default();
    state.input_digit('5');
    state.input_digit('0');
    state.apply_percent();
    assert_eq!(state.display(), "0.5");

    state.apply_operator(CalculatorOperator::Add);
    state.input_digit('1');
    state.input_decimal();
    state.input_digit('2');
    state.evaluate();
    assert_eq!(state.display(), "1.7");
}

#[test]
fn clear_and_clear_history_reset_state() {
    let mut state = CalculatorState::default();
    state.input_digit('8');
    state.apply_operator(CalculatorOperator::Divide);
    state.input_digit('2');
    state.evaluate();
    assert_eq!(state.history().len(), 1);

    state.clear_history();
    assert!(state.history().is_empty());

    state.clear();
    assert_eq!(state.display(), "0");
    assert!(state.error().is_none());
}

#[test]
fn divide_by_zero_enters_safe_error_state() {
    let mut state = CalculatorState::default();
    state.input_digit('8');
    state.apply_operator(CalculatorOperator::Divide);
    state.input_digit('0');
    state.evaluate();

    assert_eq!(state.display(), "Error");
    assert_eq!(
        state.error(),
        Some(&CalculatorEvaluationError::DivideByZero)
    );

    state.input_digit('7');
    assert_eq!(state.display(), "7");
    assert!(state.error().is_none());
}

#[test]
fn history_is_bounded() {
    let mut state = CalculatorState::default();
    for index in 0..(DEFAULT_HISTORY_LIMIT + 2) {
        state.input_digit('1');
        state.apply_operator(CalculatorOperator::Add);
        state.input_digit(char::from_digit((index % 9 + 1) as u32, 10).unwrap());
        state.evaluate();
    }

    assert_eq!(state.history().len(), DEFAULT_HISTORY_LIMIT);
}

#[test]
fn sample_steps_load() {
    let steps = sample_steps().expect("sample steps should load");
    assert_eq!(steps, vec!["2", "+", "2", "="]);
}
