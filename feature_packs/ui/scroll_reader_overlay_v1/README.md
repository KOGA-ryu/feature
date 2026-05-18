# scroll_reader_overlay_v1

`ui.scroll_reader_overlay_v1` is a small, headless contract for a selected-text
speed-reader overlay.

It intentionally stops at the reusable state and input model. It does not own a
document library, persistence, MIDI, AI summary, full editor, or settings panel.

## Source Examples Used

Vox speed-reader sources:

- `/Users/kogaryu/dev/vox/app/parser.py`
- `/Users/kogaryu/dev/vox/app/playback_session.py`
- `/Users/kogaryu/dev/vox/app/reading_window.py`
- `/Users/kogaryu/dev/vox/app/bubble_overlay_window.py`
- `/Users/kogaryu/dev/vox/app/main_window_ui_surfaces.py`
- `/Users/kogaryu/dev/vox/app/main_window_playback.py`

Parlawl chess-scroll sources:

- `/Users/kogaryu/dev/parlawl/apps/desktop/board_widget.cpp`
- `/Users/kogaryu/dev/parlawl/apps/desktop/puzzle_runner_window.cpp`
- `/Users/kogaryu/dev/parlawl/libs/puzzle_runner/game_state_store.cpp`
- `/Users/kogaryu/dev/parlawl/libs/puzzle_runner/game_state_store.h`

## Adapted Contracts

From Vox:

- normalize selected text by trimming, preserving newlines, and collapsing
  repeated spaces and tabs inside each line
- keep the reader as explicit session state with `idle`, `playing`, `paused`,
  `finished`, and `closed` statuses
- render a centered focus token with nearby before/after context, matching the
  existing focus-word reader shape
- treat overlay geometry and styling as downstream host work

From Parlawl:

- convert wheel angle deltas into signed one-step scrub commands
- accumulate pixel deltas with an 18-pixel step threshold for trackpads
- clamp navigation to valid state instead of allowing invalid positions
- ignore zero-step wheel input without changing reader state

## Intended Host UI

Input:

- selected or highlighted text

Launch:

- hotkey or button supplied by the host application

Display:

- dull background
- tiny contrast lens
- sliding word tape
- fade zones before and after the lens

Controls:

- `Space` toggles play and pause
- `Esc` closes the overlay
- wheel or trackpad advances and rewinds
- `Up` and `Down` adjust reading speed

## Worker Proof

```bash
cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/scroll_reader_overlay_v1/Cargo.toml
```
