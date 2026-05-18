# ui.calculator_basic

`ui.calculator_basic` is a reusable basic calculator reducer for the Rust
feature lab.

## Purpose

Provide a small state machine for calculator UI surfaces that need predictable,
left-to-right arithmetic without pulling in a full expression parser.

## V1 Behavior

- digits and decimal input
- `+`, `-`, `×`, `÷`
- direct percent transform on the current display value
- equals and clear
- bounded completed-calculation history
- divide-by-zero enters a safe error state

## Non-goals

- scientific functions
- operator precedence
- parentheses
- symbolic expression parsing

## Intended Host / Use Cases

- utility drawers in app shells
- thin live demo surfaces in `feature_lab_ui`
- quick numeric helpers inside reusable operator panels
