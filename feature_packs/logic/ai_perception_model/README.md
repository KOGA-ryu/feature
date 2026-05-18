# logic.ai_perception_model

`logic.ai_perception_model` is the reusable perception snapshot for authored,
readable AI targets.

It owns:

- what the target noticed
- what the target missed
- what the target misread
- current certainty

V1 is deterministic:

- no live learning
- no vision cones or geometry
- no hidden probabilistic runtime

Use it beside target state, intent selection, and interaction grading when a
host needs inspectable AI readability.
