# Hybrid Brawler-Racer Sprite Integration Contract

## Purpose

This document is the canonical sprite-integration contract for the
`hybrid_brawler_racer` gameplay vertical in `/Users/kogaryu/dev/features`.

It bridges:

- the reusable gameplay systems already implemented in the repo
- the v1 texture, atlas, and frame-sheet layout for the prototype slice
- a future runtime or importer that must consume exact clip ids, playback
  rules, and event markers without reinterpretation

This is a hybrid-specific v1 contract. It is not a generic cross-genre sprite
schema.

## Asset Registry

| asset_id | filename | asset_kind | notes |
| --- | --- | --- | --- |
| `char_player_brawler_atlas` | `char_player_brawler_atlas.png` | `character_atlas` | Side-view brawler player clips |
| `char_player_runner_atlas` | `char_player_runner_atlas.png` | `character_atlas` | Over-the-shoulder runner player clips |
| `char_enemy_grunt_atlas` | `char_enemy_grunt_atlas.png` | `character_atlas` | First brawler enemy set |
| `fx_hit_spark_atlas` | `fx_hit_spark_atlas.png` | `fx_atlas` | Brawler hit feedback |
| `fx_dust_atlas` | `fx_dust_atlas.png` | `fx_atlas` | Movement and stop dust |
| `fx_crash_burst_atlas` | `fx_crash_burst_atlas.png` | `fx_atlas` | Runner crash feedback |
| `ui_health_bar_segments` | `ui_health_bar_segments.png` | `ui_texture` | Segmented health readout |
| `ui_life_icon` | `ui_life_icon.png` | `ui_texture` | Lives display |
| `env_checkpoint_gate` | `env_checkpoint_gate.png` | `environment_texture` | Checkpoint visual anchor |
| `ui_transition_brawler_runner` | `ui_transition_brawler_runner.png` | `transition_texture` | Brawler-to-runner mode transition splash |

## Atlas Contract

The atlas contract is documentation-first, but its field names are locked for
future importer or runtime use.

### Atlas definition

| field | meaning |
| --- | --- |
| `atlas_id` | Stable atlas identifier |
| `image_filename` | Source texture filename |
| `cell_width` | Width of one frame cell in pixels |
| `cell_height` | Height of one frame cell in pixels |
| `grid_columns` | Number of columns in the atlas grid |
| `grid_rows` | Number of rows in the atlas grid |
| `pivot_x` | Horizontal pivot in cell-local pixels |
| `pivot_y` | Vertical pivot in cell-local pixels |
| `frame_origin` | Grid origin for frame addressing |
| `notes` | Human-facing usage note |

### Locked values

- `frame_origin = top_left`
- character atlases use `256x256` cells on an `8x8` grid
- FX atlases use `128x128` cells on an `8x8` grid
- brawler and runner character pivots are `pivot_x = 128`, `pivot_y = 236`
- FX pivots are `pivot_x = 64`, `pivot_y = 64` unless a later contract says
  otherwise

### Atlas records

| atlas_id | image_filename | cell_width | cell_height | grid_columns | grid_rows | pivot_x | pivot_y | frame_origin | notes |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `char_player_brawler_atlas` | `char_player_brawler_atlas.png` | 256 | 256 | 8 | 8 | 128 | 236 | `top_left` | Prototype brawler player sheet |
| `char_player_runner_atlas` | `char_player_runner_atlas.png` | 256 | 256 | 8 | 8 | 128 | 236 | `top_left` | Prototype runner player sheet |
| `char_enemy_grunt_atlas` | `char_enemy_grunt_atlas.png` | 256 | 256 | 8 | 8 | 128 | 236 | `top_left` | Prototype grunt enemy sheet |
| `fx_hit_spark_atlas` | `fx_hit_spark_atlas.png` | 128 | 128 | 8 | 8 | 64 | 64 | `top_left` | Hit spark families A-C |
| `fx_dust_atlas` | `fx_dust_atlas.png` | 128 | 128 | 8 | 8 | 64 | 64 | `top_left` | Step and hard-stop dust |
| `fx_crash_burst_atlas` | `fx_crash_burst_atlas.png` | 128 | 128 | 8 | 8 | 64 | 64 | `top_left` | Runner crash burst effects |

## Clip Contract

The clip contract is also locked for future importer or runtime use.

### Clip definition

| field | meaning |
| --- | --- |
| `clip_id` | Stable animation clip identifier |
| `atlas_id` | Owning atlas identifier |
| `row` | Zero-based atlas row |
| `col_start` | Zero-based starting column |
| `frame_count` | Number of frames in the clip |
| `fps` | Default playback rate |
| `playback` | One of `loop`, `one_shot`, or `hold_last` |
| `markers` | Ordered marker list for runtime events |
| `notes` | Human-facing note |

### Playback law

Playback values are locked to exactly:

- `loop`
- `one_shot`
- `hold_last`

Default fps law:

- idle and cruise loops: `8`
- walk and run loops: `10`
- attack clips: `12`
- hurt clips: `10`
- knockdown and crash clips: `12`
- recovery clips: `10`
- FX clips: `14`

### Character clip tables

#### `char_player_brawler_atlas`

| clip_id | atlas_id | row | col_start | frame_count | fps | playback | markers | notes |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| `player_brawler_idle` | `char_player_brawler_atlas` | 0 | 0 | 6 | 8 | `loop` | `[]` | Neutral brawler stance |
| `player_brawler_walk` | `char_player_brawler_atlas` | 1 | 0 | 8 | 10 | `loop` | `[]` | Standard movement loop |
| `player_brawler_run` | `char_player_brawler_atlas` | 2 | 0 | 8 | 10 | `loop` | `[]` | Faster committed movement |
| `player_brawler_light_attack_1` | `char_player_brawler_atlas` | 3 | 0 | 6 | 12 | `one_shot` | `[hit_frame@3]` | First combo hit |
| `player_brawler_light_attack_2` | `char_player_brawler_atlas` | 4 | 0 | 6 | 12 | `one_shot` | `[hit_frame@3]` | Second combo hit |
| `player_brawler_light_attack_3` | `char_player_brawler_atlas` | 5 | 0 | 8 | 12 | `one_shot` | `[hit_frame@5]` | Combo finisher |
| `player_brawler_heavy_attack` | `char_player_brawler_atlas` | 6 | 0 | 8 | 12 | `one_shot` | `[hit_frame@5]` | Slow heavy strike |
| `player_brawler_hurt` | `char_player_brawler_atlas` | 7 | 0 | 4 | 10 | `one_shot` | `[state_commit_frame@1]` | Hitstun entry |
| `player_brawler_knockdown` | `char_player_brawler_atlas` | 7 | 4 | 4 | 12 | `hold_last` | `[state_commit_frame@2]` | Prototype downed state; no get-up clip in v1 |

#### `char_player_runner_atlas`

| clip_id | atlas_id | row | col_start | frame_count | fps | playback | markers | notes |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| `player_runner_cruise_center` | `char_player_runner_atlas` | 0 | 0 | 6 | 8 | `loop` | `[]` | Stable center-lane hold |
| `player_runner_lean_left` | `char_player_runner_atlas` | 1 | 0 | 4 | 8 | `loop` | `[]` | Left-lane hold read |
| `player_runner_lean_right` | `char_player_runner_atlas` | 1 | 4 | 4 | 8 | `loop` | `[]` | Right-lane hold read |
| `player_runner_shift_left` | `char_player_runner_atlas` | 2 | 0 | 6 | 10 | `one_shot` | `[lane_commit_frame@3]` | Center-to-left lane shift |
| `player_runner_shift_right` | `char_player_runner_atlas` | 3 | 0 | 6 | 10 | `one_shot` | `[lane_commit_frame@3]` | Center-to-right lane shift |
| `player_runner_hit_reaction` | `char_player_runner_atlas` | 4 | 0 | 4 | 10 | `one_shot` | `[state_commit_frame@1]` | Minor runner hit reaction |
| `player_runner_crash` | `char_player_runner_atlas` | 5 | 0 | 8 | 12 | `one_shot` | `[state_commit_frame@1, crash_lock_frame@2]` | Full crash entry |
| `player_runner_recovery` | `char_player_runner_atlas` | 6 | 0 | 6 | 10 | `one_shot` | `[recovery_end_frame@6]` | Return to stable runner control |

#### `char_enemy_grunt_atlas`

| clip_id | atlas_id | row | col_start | frame_count | fps | playback | markers | notes |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| `enemy_grunt_idle` | `char_enemy_grunt_atlas` | 0 | 0 | 4 | 8 | `loop` | `[]` | Neutral grunt stance |
| `enemy_grunt_walk` | `char_enemy_grunt_atlas` | 1 | 0 | 8 | 10 | `loop` | `[]` | Encounter advance loop |
| `enemy_grunt_attack` | `char_enemy_grunt_atlas` | 2 | 0 | 8 | 12 | `one_shot` | `[hit_frame@4]` | Basic enemy melee strike |
| `enemy_grunt_hurt` | `char_enemy_grunt_atlas` | 3 | 0 | 4 | 10 | `one_shot` | `[state_commit_frame@1]` | Enemy hitstun entry |
| `enemy_grunt_knockdown` | `char_enemy_grunt_atlas` | 4 | 0 | 8 | 12 | `one_shot` | `[state_commit_frame@3]` | Enemy knockdown entry |
| `enemy_grunt_get_up` | `char_enemy_grunt_atlas` | 5 | 0 | 8 | 10 | `one_shot` | `[recovery_end_frame@8]` | Enemy return to standing state |
| `enemy_grunt_defeated_ground` | `char_enemy_grunt_atlas` | 6 | 0 | 4 | 8 | `hold_last` | `[]` | Defeated ground pose |

### FX clip tables

#### `fx_hit_spark_atlas`

| clip_id | atlas_id | row | col_start | frame_count | fps | playback | markers | notes |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| `fx_hit_spark_a` | `fx_hit_spark_atlas` | 0 | 0 | 4 | 14 | `one_shot` | `[]` | Light impact variant |
| `fx_hit_spark_b` | `fx_hit_spark_atlas` | 0 | 4 | 4 | 14 | `one_shot` | `[]` | Medium impact variant |
| `fx_hit_spark_c` | `fx_hit_spark_atlas` | 1 | 0 | 4 | 14 | `one_shot` | `[]` | Heavy impact variant |

#### `fx_dust_atlas`

| clip_id | atlas_id | row | col_start | frame_count | fps | playback | markers | notes |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| `fx_dust_step` | `fx_dust_atlas` | 0 | 0 | 6 | 14 | `one_shot` | `[]` | Footfall dust |
| `fx_dust_hard_stop` | `fx_dust_atlas` | 1 | 0 | 6 | 14 | `one_shot` | `[]` | Hard stop or skid dust |

#### `fx_crash_burst_atlas`

| clip_id | atlas_id | row | col_start | frame_count | fps | playback | markers | notes |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| `fx_crash_front_impact` | `fx_crash_burst_atlas` | 0 | 0 | 6 | 14 | `one_shot` | `[]` | Front-facing runner collision burst |
| `fx_crash_side_scrape` | `fx_crash_burst_atlas` | 1 | 0 | 6 | 14 | `one_shot` | `[]` | Side scrape collision burst |
| `fx_crash_debris_pop` | `fx_crash_burst_atlas` | 2 | 0 | 6 | 14 | `one_shot` | `[]` | Debris pop collision burst |

## Event Marker Contract

Marker names are locked. Clips without markers must not invent implicit runtime
events.

### Marker definition

| field | meaning |
| --- | --- |
| `name` | Stable marker name |
| `frame` | One-based frame index within the clip |
| `meaning` | Runtime meaning of the marker |

### Marker records

| name | frame | meaning |
| --- | ---: | --- |
| `hit_frame` | clip-specific | The frame where hit resolution and contact FX become eligible |
| `state_commit_frame` | clip-specific | The frame where the actor is considered committed to the transient state |
| `lane_commit_frame` | clip-specific | The frame where a runner lane shift becomes the authoritative lane |
| `crash_lock_frame` | clip-specific | The frame where crash lockout becomes authoritative |
| `recovery_end_frame` | clip-specific | The frame where recovery completes and stable control may resume |

### Allowed-marker law

- `hit_frame`
  - allowed only on brawler attack clips
- `state_commit_frame`
  - allowed on hurt, knockdown, and crash entry clips
- `lane_commit_frame`
  - allowed on `player_runner_shift_left` and `player_runner_shift_right`
- `crash_lock_frame`
  - required on `player_runner_crash`
- `recovery_end_frame`
  - required on recovery and get-up clips

## Runtime Mapping

This section defines how the sprite contract maps onto the current gameplay
feature lane.

### `logic.actor_state`

- `Idle` -> `player_brawler_idle`
- `Moving` -> `player_brawler_walk` or `player_brawler_run`
- `Attacking` -> one of:
  - `player_brawler_light_attack_1`
  - `player_brawler_light_attack_2`
  - `player_brawler_light_attack_3`
  - `player_brawler_heavy_attack`
- `Hitstun` -> `player_brawler_hurt`
- `KnockedDown` -> `player_brawler_knockdown`
- `Crashed` -> `player_runner_crash`
- `Defeated` -> hold the last frame of a defeat-appropriate clip

### `sim.runner_track`

- center hold -> `player_runner_cruise_center`
- left-lane hold -> `player_runner_lean_left`
- right-lane hold -> `player_runner_lean_right`
- shift-to-left -> `player_runner_shift_left`
- shift-to-right -> `player_runner_shift_right`

### `sim.crash_recovery`

- `Stable` -> normal runner clip selection is allowed
- crash entry -> `player_runner_crash`
- recovery window -> `player_runner_recovery`

### `logic.hit_resolution`

- attack resolution consumes `hit_frame`
- impact FX use:
  - `fx_hit_spark_a`
  - `fx_hit_spark_b`
  - `fx_hit_spark_c`

### `sim.checkpoint_flow`

- checkpoint visual anchor -> `env_checkpoint_gate.png`
- brawler-to-runner transition visual -> `ui_transition_brawler_runner.png`

### `ui.health_hud`

- segmented health visual -> `ui_health_bar_segments.png`
- lives display visual -> `ui_life_icon.png`

## Prototype Delivery Rules

- This contract is the single source of truth for atlas ids, clip ids,
  playback modes, frame placement, and event markers for the
  `hybrid_brawler_racer` v1 slice.
- Character frame addressing is zero-based by `row` and `col_start`.
- Every v1 clip is a single-row contiguous run inside its atlas.
- Marker `frame` values are one-based within each clip.
- Held poses loop only when `playback = loop`.
- Attack, hurt, crash, recovery, and FX clips are one-shot unless explicitly
  marked otherwise.
- `player_brawler_knockdown` is allowed to end in a held final frame during the
  prototype because `player_brawler_get_up` is deferred.
- No runtime or importer may infer hidden events from clip names alone. Only
  explicit markers are authoritative.

## Open Deferred Work

- `player_brawler_get_up` remains deferred from the prototype atlas
- future combo-chain timing may add more `hit_frame` granularity
- obstacle-specific runner reactions are not yet split into separate clips
- no generic sprite schema is defined in this cut
- no importer or playback code is introduced in this cut
