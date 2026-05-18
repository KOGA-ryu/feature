# Qt UI Contract

This document defines host UI responsibilities for a later Qt prototype. It is
not an instruction to build the widget in this slice.

## Host Responsibility

The Qt host owns:

- selected-text capture
- explicit clipboard watch toggle
- OS clipboard read/listen behavior while watch is enabled
- desktop shortcut registration
- launch hotkey or button routing
- widget creation and placement
- key event routing
- wheel event routing
- timer ticks
- close/destroy lifecycle

The reader model owns:

- tokens
- current index
- words per minute as an internal pace value
- status
- wheel scrub accumulation
- viewport data
- clipboard watch ingest decision for host-provided text snapshots

Runtime rule: the Qt reading widget displays only the pill. Any options or
layout editor surface must be a separate window that is opened explicitly.

## Widget Shape

The widget should be frameless, floating, and dismissible.

Required visual layers:

1. base capsule layer
2. continuous sentence tape layer
3. center lens layer
4. fade zones
5. optional function-mark layer

## Geometry Constants

The Qt host should expose the same V1 design tokens:

```text
overlay_large_width = 400
overlay_large_height = 100
overlay_large_radius = 28
overlay_compact_width = 200
overlay_compact_height = 50
overlay_compact_radius = 14
lens_width_large = 184
lens_height_large = 54
lens_width_compact = 92
lens_height_compact = 30
fade_width = 56
control_row_height = 20
progress_marker_height = 3
overlay_padding = 10
focus_context_target_chars_large = 90
focus_context_target_chars_compact = 44
large_context_font_size = 18
large_focus_font_size = 20
compact_context_font_size = 11
compact_focus_font_size = 12
max_focus_type_scale_percent = 112
focus_letter_spacing_tenths_px = 2
continuous_tape_word_gap_px = 6
context_text_opacity_percent = 72
focus_text_opacity_percent = 100
lens_bg_opacity_percent = 96
```

## Color Constants

The Qt host should expose only these user-facing theme controls:

```text
accent
background
foreground
contrast
```

The options window should offer only two built-in theme choices in V1:

```text
warm_lens
eye_comfort
```

Custom themes should be made through a small color-wheel path:

```text
accent_color_wheel
background_color_wheel
foreground_color_wheel
contrast_slider
```

The reader crate derives internal colors from that seed. The Qt host should name
derived colors by role:

```text
base_bg
capsule_bg
lens_bg
lens_edge
text_context
text_focus
text_muted
accent_speed
accent_progress
control_idle
control_hover
```

Color law:

- only the base/capsule background and lens background are allowed as
  background treatments
- accents are tiny functional details only
- no 11-color user-facing editor
- no long preset gallery
- hardcoded derived colors are allowed when they preserve the seed relationship
- `ReaderThemeSeed` stays local but is shaped so a later `features_binder`
  theme seed can map into it directly
- do not make the reader depend on `features_binder`
- no decorative multicolor treatment
- no literal glow, bloom, blur halo, or decorative light effect

## Event Mapping

| Qt input | Model action |
| --- | --- |
| `Ctrl+Option+R` | capture selected or copied text, then open capsule |
| `Space` | `toggle_play` |
| `Esc` | `close` |
| wheel up | scrub `-1` |
| wheel down | scrub `+1` |
| trackpad pixel delta | accumulate and emit scrub steps |
| `Up` | speed up |
| `Down` | slow down |
| Watch Clipboard on | host begins listening to clipboard changes |
| Watch Clipboard off | host stops listening to clipboard changes |
| clipboard changed | host calls clipboard-watch ingest, then loads accepted payload |

## Desktop Shortcut

The default desktop shortcut is:

```text
Ctrl+Option+R
```

Contract fields:

```text
id = summon_scroll_reader_capsule
scope = desktop_global
modifiers = ctrl, option
key = r
action = capture_selected_or_copied_text_and_open_capsule
enabled_by_default = true
host_owned = true
```

Host rules:

- register the shortcut in the desktop host, not in the reader model
- detect conflicts before silently taking the key
- expose the binding in the future light options window
- if the active app has selected text, prefer that selected text path
- otherwise, use the most recent explicit copied-text snapshot if available
- if no readable text is available, open the capsule in the empty state

## Clipboard Watch UI

Clipboard watch is a host control, not a reader-core clipboard API.

Rules:

- The default state is off.
- The control label should be `Watch Clipboard`.
- While active, the host should visibly indicate that clipboard watch is on.
- Session watch is allowed without a macOS menu bar item when the capsule or
  options window is visible.
- Global/background watch requires a persistent macOS menu bar indicator with a
  stop action.
- V1 should not implement global/background watch.
- Copying new text while active should update the reader payload immediately.
- Copying the same text again should not reload or reset the reader.
- Empty copied text should not start the reader.
- The host should not keep a clipboard history.
- The host should not write back to the clipboard.
- The host should not hide the fact that clipboard watching is active.

## Layout Rules

- Host drawing should consume the headless `CapsuleRenderPlan`.
- The render plan owns capsule rect, lens rect, text clip rect, fade zones,
  progress marker geometry, text baseline, and tape placements.
- The capsule frame stays fixed while text moves.
- The lens remains centered.
- Exactly one focus anchor word is centered at the fixed focus point.
- The focus anchor is an eye-position guide, not the whole reading unit.
- Readable context before and after the focus anchor is core functionality.
- Neighboring words remain visible as one continuous sentence line.
- Context words fade near edges.
- Controls do not resize the capsule.
- Compact mode hides any nonessential labels first.
- Close control, if present, must be quiet and low chrome.
- Progress marker, if present, is tiny and non-dominant.
- Numeric WPM display is not required.
- Play/pause indicators are not required.
- Persistent play/pause indicators should be absent by default.
- The runtime surface should not include a settings panel, title bar, legend, or
  surrounding app chrome.
- Focus typography may be slightly larger and more legible under the lens.
- Word spacing may be slightly wider under the lens.
- Color, opacity, and contrast do most of the magnification work.
- Active-word type scale must remain subtle and should not exceed
  `max_focus_type_scale_percent`.
- The original text rhythm should still feel intact.

## Options Window

A light options window is allowed for saved user parameters and color tuning.

Rules:

- it is opened only when the user requests options
- it is not part of the capsule body
- it should not become a dashboard or settings workspace
- it may expose shape, color, optional function placement, pace, mode, and
  subtle typography preferences

## Accessibility Notes

- Host should expose the current focus anchor word and status as accessible text.
- `Esc` must remain reliable.
- Focus should not trap the user after the overlay closes.
- Empty state should expose `Select text to read`.
