# Interaction Model

The capsule is optimized for a short selected-text reading session.

## Input

The capsule accepts a plain text snapshot from a host-owned input path:

- selected or highlighted text from a host application
- explicit system clipboard snapshot after the user copies text
- clipboard watch snapshot after the user intentionally enables clipboard
  watching and copies new text
- cleaned text emitted by the parallel text-editor clipboard pipeline

The capsule does not own the OS clipboard. It does not poll, mutate, restore,
or maintain clipboard history. Clipboard access belongs to the host app or the
text-editor pipeline. This feature receives only a text payload plus a source
label.

### Clipboard Watch Ingest

The user-facing behavior is:

```text
turn on Watch Clipboard
highlight text anywhere
copy
reader updates from the copied text
```

The watch is host-owned and must be explicit. The runtime should show a visible
watching state while it is active. The reader model only receives changed text
snapshots from the host.

Clipboard watch has two lifecycle modes:

- `Session Watch`: active only while the capsule or options window is visible.
  It needs a visible in-app state, but no macOS menu bar item.
- `Global Watch`: active while the capsule is closed or the app is otherwise in
  the background. It requires a persistent macOS menu bar indicator and a clear
  stop action from that menu.

V1 should not build `Global Watch`. It is a later host feature because it has
privacy, permission, and lifecycle implications. If watch behavior is added
before then, it should be session-only.

Required host policy:

- default watch state is off
- user must explicitly enable `Watch Clipboard`
- host reads the OS clipboard
- host sends only changed text snapshots to the reader model
- unchanged clipboard text is ignored
- empty or whitespace-only clipboard text is ignored
- host must expose a clear way to stop watching
- no clipboard history is stored by the reader
- no copied text is uploaded by this feature
- global/background watch requires a visible OS-level status affordance

This keeps the one-action workflow fast without making clipboard access hidden.

If the host provides no readable text, the capsule enters the `empty` display
state and shows:

```text
Select text to read
```

Extraction corruption should remain visible as copied text. The reader does not
hide PDF/OCR artifacts, repair hyphenation, dedupe page headers, or show a
cleanup panel. If the text is dirty, the tape should make that obvious and the
user can clean it in the full text-editor path.

## Launch

Launch is host-owned:

- desktop shortcut key
- command button
- command palette action

Default desktop shortcut:

```text
Ctrl+Option+R
```

The action is:

```text
capture selected or copied text -> open capsule
```

The capsule defines this as a host contract only. The desktop host registers the
shortcut, checks conflicts, requests any required platform permission, and
routes the launch payload. The reader crate does not install a system keyboard
hook or define a global shortcut registry in V1.

## Primary Display

The capsule displays one continuous horizontal sentence tape.

- Runtime display is pill-only. The user should not see a dashboard, board,
  legend, toolbar, or options panel during normal reading.
- The lens is fixed at center.
- Exactly one focus anchor word is centered inside the lens.
- The focus anchor trains eye position; it is not the whole reading unit.
- Context words sit on the tape before and after the lens and are part of the
  reading experience.
- In left-to-right English, future-word context receives a slight budget bias
  because useful preview is stronger ahead of fixation.
- Fade zones reduce context contrast near the capsule edges.
- Progress, pace, and play state indicators are optional. When present, they are
  tiny details, never dominant UI.
- Words remain in their original order and retain their original rhythm.
- The magnifier may make slight typographic adjustments for readability:
  slightly larger focus type, slightly wider word spacing, and refined contrast.
- Color, opacity, and contrast are the primary magnification tools.
- Neighboring words remain visible in the same sentence line. Small words may
  share space inside the broader focus band, but V1 highlights only one focus
  anchor word.
- The user is expected to read the visible context band with peripheral vision,
  commonly several words at once.
- Punctuation and paragraph breaks affect timing and pauses, not the one-line
  visual model.

## Beauty Rule

Beauty is functional for this feature. The capsule must look beautiful because
visual comfort is part of reading comfort. Adding controls, labels, and panels
cannot compensate for a poorly composed lens, tape, or color relationship.

## Controls

Required controls:

| Input | Behavior |
| --- | --- |
| `Space` | play/pause |
| `Esc` | close |
| mouse wheel or trackpad | manual advance/rewind |
| `Up` | speed up |
| `Down` | slow down |

Optional controls:

- quiet close button
- tiny progress marker
- hover or focus-only play/pause affordance
- light options entry point for colors, shape, saved user parameters, and
  function placement

Controls should be visible but restrained. The UI should still work primarily
through hotkeys and wheel/trackpad input.

## Options Window

The options window is separate from the runtime pill. It may behave like a small
wireframe layout tool where the user can shape the capsule, tune color
composition, and place optional function marks. It should save user preferences
only when that later slice explicitly adds persistence.

For V1, the runtime keeps the hardcoded geometry tokens and the options window
is documented only as a boundary, not implemented as a full settings surface.

## Reader States

### `idle_paused`

- selected text is loaded
- clipboard-watch text may be loaded if Watch Clipboard is enabled and the host
  sees new copied text
- first or current focus anchor word is visible
- pause state may be represented by a subtle control affordance, but no visible
  pause indicator is required
- `Space` starts playback
- wheel/trackpad can scrub immediately

### `playing`

- focus anchor advances by words-per-minute timing
- focus lens remains centered
- numeric WPM display is not required
- play/pause indicators are not required
- `Space` pauses
- `Esc` closes

### `manual_scrub`

- wheel/trackpad moves the focus anchor index forward or backward
- playback is controlled by user motion
- navigation clamps at first and last token
- the state may return to `idle_paused` or `playing` depending on host policy

### `finished`

- final focus anchor word is reached
- quiet done state is shown
- close/restart is available
- automatic tick does not advance further

### `empty`

- no selected text
- show minimal message: `Select text to read`
- `Space` does not enter playback
- `Esc` closes

### `closed`

- overlay is dismissed
- no tick, scrub, or play action changes state
- host may destroy the widget
