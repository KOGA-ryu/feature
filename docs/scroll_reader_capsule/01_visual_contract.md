# Visual Contract

The generated mockup is the primary visual target.

The capsule should read as:

- dark/dull base background
- floating rounded capsule
- central magnifier or focus lens
- continuous sentence tape
- left and right fade context
- minimal controls
- summoned utility capsule, not app chrome

The runtime surface shows only the pill. The options surface is separate and
may expose shape, color, and function-placement controls, but those controls do
not travel with the normal reading overlay.

The lens must not rely on literal glow, bloom, blur halo, or decorative light
effects. It should read as focus through typography and color composition:
slightly larger focus type, refined spacing, stronger weight, crisp lens edge,
and beautiful contrast between `text_focus`, `lens_bg`, and `capsule_bg`.

## Geometry Tokens

All dimensions are initial V1 design tokens in logical pixels.

| Token | Value | Notes |
| --- | ---: | --- |
| `overlay_large_width` | 400 | Large capsule width |
| `overlay_large_height` | 100 | Large capsule height |
| `overlay_large_radius` | 28 | Large capsule corner radius |
| `overlay_compact_width` | 200 | Compact capsule width |
| `overlay_compact_height` | 50 | Compact capsule height |
| `overlay_compact_radius` | 14 | Compact capsule corner radius |
| `lens_width_large` | 184 | Central lens width in large mode |
| `lens_height_large` | 54 | Central lens height in large mode |
| `lens_width_compact` | 92 | Central lens width in compact mode |
| `lens_height_compact` | 30 | Central lens height in compact mode |
| `fade_width` | 56 | Per-side fade zone width in large mode; host may scale to 28 in compact |
| `control_row_height` | 20 | Minimal controls row height |
| `progress_marker_height` | 3 | Tiny progress marker height |
| `overlay_padding` | 10 | Capsule inner padding in large mode; host may scale to 6 in compact |
| `focus_context_target_chars_large` | 90 | Rough large-mode surrounding context target |
| `focus_context_target_chars_compact` | 44 | Compact-mode surrounding context target |

## Typography Tokens

These values keep magnification subtle. Color, opacity, and contrast carry the
effect; type scale only clarifies the focus anchor word.

| Token | Value | Notes |
| --- | ---: | --- |
| `large_context_font_size` | 18 | Normal sentence tape size in large mode |
| `large_focus_font_size` | 20 | Focus anchor word size in large mode |
| `compact_context_font_size` | 11 | Normal sentence tape size in compact mode |
| `compact_focus_font_size` | 12 | Focus anchor word size in compact mode |
| `max_focus_type_scale_percent` | 112 | Hard cap for subtle magnification |
| `focus_letter_spacing_tenths_px` | 2 | `0.2px`; slight glyph clarity only |
| `continuous_tape_word_gap_px` | 6 | Gap around the centered focus anchor |
| `context_text_opacity_percent` | 72 | Readable peripheral context words |
| `focus_text_opacity_percent` | 100 | Focus anchor clarity |
| `lens_bg_opacity_percent` | 96 | Lens contrast without glow |

## Color Tokens

These names define roles, not final palette ownership.

The options UI should expose only four theme controls:

| Control | Meaning |
| --- | --- |
| `accent` | Tiny functional marks such as progress, speed, and active glyph clarity |
| `background` | Dull base/capsule color family |
| `foreground` | Readable text and lens contrast color family |
| `contrast` | Numeric strength used to derive internal role colors |

All other colors are derived internal roles. Do not expose an 11-color editor in
the capsule options. The small control set lets sensitive eyes rotate color
families without turning the reader into a color cockpit.

V1 should ship with only two built-in theme options:

- `warm_lens`
- `eye_comfort`

That is enough when custom theme creation is easy. The color wheel does most of
the work: users should be able to tune accent, background, and foreground
directly, then use one contrast slider to strengthen or soften the derived
reader roles.

The seed shape is intentionally compatible with a later `features_binder` theme
route:

```json
{
  "accent": "#39E83C",
  "background": "#6B6F78",
  "foreground": "#33085D",
  "contrast": 76
}
```

Keep the reader and binder decoupled until a shared theme primitive exists. The
future binder path should map its theme seed into `ReaderThemeSeed`, then let
the reader derive its internal palette.

| Token | Role |
| --- | --- |
| `base_bg` | dull surrounding overlay base or dimmed underlay |
| `capsule_bg` | main capsule body background |
| `lens_bg` | small contrast/focus lens background |
| `lens_edge` | lens outline or rim |
| `text_context` | previous/future context words |
| `text_focus` | focus anchor word inside the lens |
| `text_muted` | secondary labels and quiet state text |
| `accent_speed` | optional tiny pace feedback or options-page speed control |
| `accent_progress` | tiny progress marker |
| `control_idle` | low-chrome control glyphs |
| `control_hover` | hover/active clarity for controls |

## Color Law

Only two background treatments are allowed:

1. dull base or capsule background
2. small contrast/focus lens background

Accent colors may be used only for tiny functional details:

- speed
- progress
- glyph clarity
- play/pause state

No rainbow UI. No decorative color clutter. No gradient-orb decoration. No
extra visual panels.

No literal glow is required or preferred. The focus anchor should glow
typographically, not graphically.

## Display Layers

Required layers, back to front:

1. base capsule layer
2. continuous sentence tape layer
3. center lens layer
4. fade zones
5. optional function-mark layer

## Text Behavior

- Exactly one focus anchor word is centered at the fixed focus point.
- The focus anchor is the eye-position guide, not the whole reading unit.
- The readable context band around the focus anchor is the reading unit.
- Previous and future words remain readable and functional.
- Words fade before and after the lens.
- The frame stays still.
- The text moves through the frame.
- Original word order and formatting rhythm remain intact.
- The visual model is a normal sentence line moving through a fixed clarity
  point, not phrase-card RSVP.
- Neighboring words can share the broader focus band when the focus anchor is
  short, but they are not highlighted as focus anchors.
- V1 highlights exactly one focus anchor at a time.
- Large mode must preserve readable words ahead of and behind the focus anchor
  when enough tokens are available. This supports peripheral reading of roughly
  3-6 words, not binary one-word RSVP.
- For left-to-right English, the context budget is slightly ahead-weighted:
  60% ahead of the anchor and 40% behind it.
- Large mode should expose roughly 90 characters of readable surrounding
  sentence context. Compact mode may expose less, but still preserves left and
  right sentence context.
- The lens may use slightly larger type and slightly wider word spacing for ease
  of reading, but changes must stay subtle.
- Color, opacity, and contrast are the primary magnification tools.
- Subtle elegance is the visual quality bar.
- No persistent numeric WPM readout is required.
- No persistent play/pause indicator is required.
