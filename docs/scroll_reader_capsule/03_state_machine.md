# State Machine

The core state model is intentionally small.

```text
Paused <-> Playing -> Finished
   |          |           |
   +----------+-----------+-> Closed
```

## Status Values

```rust
enum OverlayStatus {
    Paused,
    Playing,
    Finished,
    Closed,
}
```

The empty display is not a separate engine status in V1. It is a view condition:
`tokens.is_empty()` while not closed.

## Required Behavior

- Default state is `Paused` when selected text produces at least one token.
- Empty text must not enter `Playing`.
- `toggle_play` changes `Paused` to `Playing`.
- `toggle_play` changes `Playing` to `Paused`.
- `Finished` does not keep advancing.
- `Closed` ignores play, tick, and scrub.
- `Esc` always closes.

## Transition Rules

| Event | From | To | Notes |
| --- | --- | --- | --- |
| load non-empty text | any non-closed | `Paused` | reset `current_index` to 0 |
| load empty text | any non-closed | `Paused` | view renders empty message |
| `toggle_play` | `Paused` | `Playing` | only if tokens are non-empty |
| `toggle_play` | `Playing` | `Paused` | keeps current index |
| `toggle_play` | `Finished` | `Playing` or `Paused` | host may restart at index 0; document chosen behavior in implementation |
| tick reaches final token | `Playing` | `Finished` | no further automatic advance |
| wheel/trackpad | `Paused` | `Paused` | scrub current index |
| wheel/trackpad | `Playing` | `Playing` | scrub current index; host may show temporary manual state |
| wheel/trackpad | `Finished` | `Paused` | rewinding from done returns to paused |
| `Esc` | any | `Closed` | always wins |
| play/tick/scrub | `Closed` | `Closed` | ignored |

## Core Data Model

```rust
struct ScrollReaderOverlay {
    tokens: Vec<ReaderToken>,
    current_index: usize,
    words_per_minute: u32,
    status: OverlayStatus,
    wheel_scrubber: WheelScrubber,
}

struct ReaderToken {
    text: String,
    start: usize,
    end: usize,
}

struct ReaderViewport {
    before_fade: Vec<String>,
    lens: Option<String>,
    after_fade: Vec<String>,
    current_index: usize,
}
```

## Safety Rules

- `current_index` must always clamp to a valid token index when tokens exist.
- Empty token lists must not panic.
- `viewport(radius)` must be valid at start, middle, end, and empty input.
- Closed state must be terminal for that overlay instance.

