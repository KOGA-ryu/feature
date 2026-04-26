# logic.spec_selector

`logic.spec_selector` is the assembly-aware intake step between raw idea input
and prompt generation.

It owns:

- matching selected features to reuse-score evidence
- enforcing a minimum reuse threshold
- running explicit compatibility evaluation
- resolving incompatible pairs deterministically
- passing the final ordered feature list into `logic.spec_generator`

It does not own:

- free-form feature search
- semantic compatibility inference
- prompt text generation

The output is a prompt-ready `GeneratedSpec` plus the reuse and compatibility
evidence that produced it.
