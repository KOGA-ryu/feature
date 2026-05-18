# logic.ai_intent_model

`logic.ai_intent_model` is the reusable authored-intent state for readable AI
targets.

It owns:

- current intent kind
- telegraph phase
- committed phase
- recovery phase

V1 is intentionally narrow:

- exact authored behaviors only
- no planner
- no goal graph
- no ML policy

Use this crate when a host needs deterministic intent timing that the player
can learn, bait, and punish.
