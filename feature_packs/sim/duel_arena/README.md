# sim.duel_arena

`sim.duel_arena` is the first deep simulation-zoom host for the hybrid game
lane.

It is still headless-first inside `/Users/kogaryu/dev/features`, but it is
allowed to model richer duel response than `sim.beat_em_up_plane`.

V1 owns:

- duel actors
- push impulses
- contact-range exchanges
- knockback-like response
- deterministic duel outcome reporting

It does not own:

- rendering
- Rapier integration
- camera
- animation playback
