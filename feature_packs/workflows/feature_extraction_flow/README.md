# workflow.feature_extraction_flow

`workflow.feature_extraction_flow` converts completed feature work into reusable
library extraction packets.

It owns:

- review-packet acceptance as source truth
- extraction validation
- proposed library item generation
- reusable summary and note normalization

It does not own:

- writing library files
- review packet generation
- prompt generation

## Current Scope

- accept a source spec and source review packet
- reject incomplete or invalid review packet input
- produce deterministic blueprint, contract, test, and failure-note outputs
- emit proposed library paths without mutating the library
