# Scroll And Pace Controls

The wheel/trackpad model follows the existing chess-scroll convention:

- wheel up rewinds
- wheel down advances
- pixel deltas accumulate for trackpads
- navigation clamps at first and last token

## Wheel Direction

| Input | Step |
| --- | ---: |
| wheel angle delta Y greater than 0 | `-1` |
| wheel angle delta Y less than 0 | `+1` |
| accumulated pixel delta Y reaches positive threshold | `-1` |
| accumulated pixel delta Y reaches negative threshold | `+1` |

The sign convention means positive steps advance through text and negative
steps rewind.

## Trackpad Accumulation

Trackpad pixel deltas should accumulate until they cross a threshold.

Initial V1 token:

```text
wheel_pixel_step = 18
```

Behavior:

- small pixel deltas below threshold do not move the token index
- accumulated positive delta emits rewind steps
- accumulated negative delta emits advance steps
- angle-wheel input resets pixel accumulation
- zero wheel input changes nothing

## Navigation Clamp

Manual scrubbing must never move outside the token list.

- rewinding at the first token stays on the first token
- advancing past the final token stays on the final token
- if advancing would pass the final token during playback, status becomes `Finished`
- closed overlays ignore scrub input

## Pace Tokens

Initial V1 pace constants:

| Token | Value | Notes |
| --- | ---: | --- |
| `default_words_per_minute` | 420 | internal default reading pace |
| `min_words_per_minute` | 120 | lower clamp |
| `max_words_per_minute` | 900 | upper clamp |
| `speed_step_words_per_minute` | 30 | per Up/Down adjustment |

## Pace Behavior

- `Up` increases speed by `speed_step_words_per_minute`.
- `Down` decreases speed by `speed_step_words_per_minute`.
- speed clamps between min and max
- milliseconds per token derives from WPM
- numeric words-per-minute does not need to be displayed in the capsule
- if pace feedback is shown, keep it temporary, tiny, and functional

Formula:

```text
milliseconds_per_token = max(1, 60000 / words_per_minute)
```

The capsule is not a dashboard, chart, or settings panel.

## Options Window

If parameters need user control, use a light options window outside the runtime
capsule.

Allowed saved options:

- colors
- default pace
- compact or large preference
- subtle typography scale
- lens contrast preference
- capsule shape
- optional function placement

The options window is called only when the user needs to change something. It is
not part of the normal reading loop, and it must not add chrome to the runtime
pill.
