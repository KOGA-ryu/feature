# logic.validation_pipeline

`logic.validation_pipeline` is a reusable rule-based validation feature crate for the Rust feature lab.

It validates structured inputs using a fixed first-pass rule set:

- required field presence
- non-empty collections
- simple UTC timestamp format
- known dependency references
- known compatible-feature references

The crate returns explicit findings with severities, plus an aggregated summary and outcome. It does not do async work, plugin loading, or schema inference.

## Contract

- Feature id: `logic.validation_pipeline`
- Kind: `logic_pattern`
- Status: `experimental`
- Inputs: required fields, collections, timestamps, dependencies, compatible features
- Outputs: validation findings, validation summary, validation outcome

## Behavior

- Empty required strings fail.
- Empty collections fail.
- Timestamps use a simple `YYYY-MM-DDTHH:MM:SSZ` UTC rule.
- Missing dependencies fail.
- Unknown compatible features warn.
- Findings are sorted with errors before warnings.

## Harness

The feature lab UI uses this crate for a small center-panel demo that flips between a valid and invalid fixture and renders the resulting findings packet.
