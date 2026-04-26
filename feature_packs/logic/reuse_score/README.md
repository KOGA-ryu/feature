# logic.reuse_score

`logic.reuse_score` ranks feature candidates for reuse using a deterministic local scoring model.

The v1 scorer is intentionally simple:

- explicit lifecycle weights
- capped usage bonuses
- evidence bonuses for tests, docs, examples, fixtures, and recent review
- capped failure penalties
- deterministic score bands

It does not look at external history, telemetry, or semantic similarity. The output is designed to be explainable enough for Dex to prefer stable, proven features without hiding the reasons.
