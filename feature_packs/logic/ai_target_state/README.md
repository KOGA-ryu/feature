# logic.ai_target_state

`logic.ai_target_state` is the reusable readable-target state for the
simulation-zoom gameplay lane.

It owns:

- target posture
- target awareness
- target pressure
- commitment windows
- vulnerability windows

V1 keeps this deterministic and authored:

- no machine learning
- no pathfinding
- no combat geometry solver
- no animation ownership

Use this crate beside intent, perception, grading, and duel hosts when a game
needs AI targets that the player can read, bait, pressure, and punish.
