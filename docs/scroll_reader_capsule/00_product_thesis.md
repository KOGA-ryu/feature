# Product Thesis

`scroll_reader_capsule_v1` is a lightweight text-snapshot reading overlay.

It exists for one temporary job:

```text
text snapshot -> hotkey -> floating capsule reader -> user controls pace -> Esc closes
```

The capsule should feel like a summoned utility object: a dome magnifier, a
reading ruler, a paper tape, and a speed reader compressed into one small
floating aid.

It should not feel like a workspace, app shell, document system, dashboard, or
editor.

In use, the runtime surface is only the pill. No board, inspector, toolbar,
sidebar, or dashboard should appear around the reader.

Beauty is part of the function. If the capsule is not beautiful, it is not
finished enough to be functional for this use case. The reader must reduce
friction through elegance, restraint, and visual composition rather than through
more controls.

## Product Rules

- The overlay is temporary and dismissible.
- The text snapshot is loaded into the capsule, then the capsule owns only the
  active reading session.
- Text may arrive from direct selection, an explicit clipboard snapshot, or the
  parallel text-editor clipboard pipeline.
- The frame stays physically still while the continuous sentence tape moves
  through it.
- The fixed center point highlights exactly one focus anchor word.
- The focus anchor trains eye position; the readable context band carries
  meaning through peripheral vision.
- The central lens is a clarity zone, not a phrase card or one-word RSVP unit.
- Context exists only to preserve flow, not to compete with the lens.
- Controls are behavior-first; hotkeys and wheel input are primary. Persistent
  control indicators are optional and should disappear unless they clarify a
  direct action.
- The original word order and formatting rhythm should remain intact.
- The lens may make subtle typographic changes for readability.
- Subtlety is the design standard; heavy effects are a failure mode.

## Hard Boundaries

Do not build:

- document library
- persistence
- history storage
- AI summary tools
- MIDI controls
- dashboard
- sidebar
- full text editor
- large settings pages
- account or auth features
- repo scanner integration
- large app shell

Allowed exception:

- a light options window may exist for user-controlled parameters, saved
  defaults, color tuning, shape, and function placement, but it is only opened
  when the user explicitly needs it
- the options window is a small layout tool, not a persistent workspace

## V1 Deliverable

This slice is documentation and UI contract only.

The first implementation should be limited to mock UI constants or a tiny
prototype only if the repo already has a clear local place for that. Otherwise,
engine work and host integration wait for a later slice.
