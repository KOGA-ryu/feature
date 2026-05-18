# scroll_reader_capsule_host_v0

`ui.scroll_reader_capsule_host_v0` is the first runnable host slice for
`ui.scroll_reader_capsule_v1`.

It opens a small local desktop window and draws the capsule from the existing
headless `CapsuleRenderPlan`. This slice exists so the capsule can be run and
felt before selected-text capture, global shortcuts, persistence, or an options
window are built.

## Run

From `/Users/kogaryu/dev/features`:

```bash
cargo run --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/scroll_reader_capsule_host_v0/Cargo.toml
```

With custom text:

```bash
cargo run --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/scroll_reader_capsule_host_v0/Cargo.toml -- "Paste the text you want to read here"
```

## Controls

| Input | Behavior |
| --- | --- |
| click + drag capsule | reposition the window |
| `Cmd+V` / `Ctrl+V` | load current clipboard text once |
| `M` | toggle large/compact mode |
| `Space` | play/pause |
| `Esc` | close the window |
| mouse wheel / trackpad | advance or rewind |
| `Up` | speed up |
| `Down` | slow down |

Clipboard paste is on demand. Empty clipboard text does not wipe the current
reader. The host does not keep clipboard history and does not watch the
clipboard in the background.

The host remembers only object preferences:

- last window position
- large/compact mode

It does not persist reader text or clipboard text.

## Boundaries

- no clipboard history
- no background clipboard watch
- no selected-text capture
- no global shortcut registration
- no options window
- no app shell
- no document or clipboard persistence

The documented future desktop shortcut remains `Ctrl+Option+R`, but this host
does not register it yet. Global desktop registration belongs to the next host
slice because it needs platform conflict handling and permissions.

## Proof

```bash
cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/scroll_reader_capsule_host_v0/Cargo.toml
```
