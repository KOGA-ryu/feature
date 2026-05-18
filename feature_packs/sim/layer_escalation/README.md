# sim.layer_escalation

`sim.layer_escalation` turns AI interaction grading into a deterministic
decision about whether play should stay coarse or zoom into a deeper
simulation layer.

V1 is intentionally narrow:

- only the brawler duel zoom is active
- advanced or better interaction can unlock zoom
- repeated poor reads suppress zoom

This crate does not own campaign routing or presentation. It only owns the
decision law.
