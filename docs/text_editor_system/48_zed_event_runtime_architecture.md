# Zed Event And Runtime Architecture Notes

Status: reference teardown for our Text Editor workspace.

This document records how Zed appears to structure its event/runtime model, based
on Zed's public docs and source files inspected on 2026-05-18. It is not a
copy target. It is a pattern mine: we keep the useful architecture lessons and
translate them into our Qt/headless feature-library shape.

## Purpose

The Text Editor workspace is growing from a blank shell into a real workbench.
Before we add more commands, fixtures, sidecars, and AI-assisted helpers, we
need a clean mental model for:

- where state lives,
- how commands are named and dispatched,
- how UI surfaces subscribe to change,
- how async work returns to the UI,
- how sidecars belong to a workspace,
- how a rendering host stays thin.

Zed is useful because it is a mature Rust editor with explicit runtime terms:
`App`, `Entity`, `Context`, `Action`, `Task`, `Window`, `Workspace`, `Pane`,
`Panel`, and `Dock`.

## Source Scope

Primary Zed sources inspected:

- `CONTRIBUTING.md`
- `docs/src/development/glossary.md`
- `docs/src/development/feature-process.md`
- `crates/gpui/src/app.rs`
- `crates/gpui/src/app/context.rs`
- `crates/gpui/src/app/async_context.rs`
- `crates/gpui/src/app/entity_map.rs`
- `crates/gpui/src/subscription.rs`
- `crates/gpui/src/action.rs`
- `crates/gpui/src/key_dispatch.rs`
- `crates/gpui/src/executor.rs`
- `crates/gpui/src/element.rs`
- `crates/gpui/src/view.rs`
- `crates/gpui/src/window.rs`
- `crates/gpui/src/platform.rs`
- `crates/workspace/src/workspace.rs`
- `crates/workspace/src/dock.rs`

Important caveat:

This is a teardown from public source, not an upstream design review from the
Zed team. Anything under "Interpretation" is our reading of the shape.

## High-Level Runtime Shape

Observed architecture:

```text
Platform
  owns OS integration, windows, menus, clipboard-ish platform seams, executors

Application
  configures and starts the app

App
  singleton application state, entity map, globals, effects, windows

Window
  focus, dispatch tree, dirty views, frame/render state, hitboxes

Workspace
  top-level product shell inside a window

Pane / Dock / Panel
  center items and side/bottom dockable panels

Entity<T>
  typed state object managed by App

Context<T>
  App access specialized for one Entity<T>

Render / Element tree
  current state -> layout/prepaint/paint
```

Interpretation:

Zed treats the runtime as a state graph owned by `App`, not as loose widgets
mutating each other directly. UI is rebuilt from state through `Render`, while
commands and events modify entities through typed contexts.

## Core Runtime Objects

### `App`

Observed:

- Zed's glossary describes `App` as a singleton that holds full application
  state, including entities.
- `App` is not `Send`; it belongs to the main/UI thread.
- `App` owns effect flushing, windows, action registry, entity map, globals,
  observers, event listeners, and executors.

Why this matters:

There is one central runtime truth. Views and panels do not each own private
runtime systems.

Our equivalent:

```text
DexHomeV2Window / app state bundle
```

Our rule:

Keep one `WorkspaceKind` and one selected-state model. Do not let each rail,
page, or context panel invent separate active-workspace truth.

### `Entity<T>`

Observed:

- `Entity<T>` is a typed handle to state managed by the app.
- `WeakEntity<T>` lets callbacks or async work refer to state without keeping
  it alive forever.
- `EntityMap` reserves/inserts/leases state, tracks entity IDs, and prevents
  invalid/double mutable access.

Why this matters:

State is addressable, typed, and lifecycle-aware.

Our equivalent:

```text
Plain C++ state models today:
  CockpitState
  selected project/worker/tab/lens
  text editor workspace state later
```

Our rule:

When Text Editor state grows, create explicit state structs rather than
sprinkling state fields across widgets.

### `Context<T>`

Observed:

`Context<T>` wraps `&mut App` plus the weak handle to the entity being updated.
It provides:

- `notify()` for state change notification,
- `observe(...)` for state observation,
- `subscribe(...)` for typed event subscription,
- `emit(...)` for typed event emission,
- `spawn(...)` for async work tied to the entity,
- `listener(...)` for event callbacks that update entity state safely.

Why this matters:

Mutation happens through a context that knows which entity is active and how to
notify the rest of the system.

Our equivalent:

```text
MainWindow routing callbacks:
  on project selected
  on worker selected
  on settings selected
  on shelf/text editor selected
```

Our rule:

Callbacks should update state first, then call a single refresh/render route.
Do not mutate child widgets directly from unrelated child widgets.

## Event Flow: State Change

Observed path:

```text
event handler / action
  updates Entity<T> through Context<T>
  calls cx.notify()
    App marks entity changed
    window invalidators mark dependent views dirty
    effects flush after update cycle
    next draw refreshes affected views
```

Source signals:

- `Context::notify()` calls `App::notify(entity_id)`.
- `WindowInvalidator` tracks dirty views.
- `App::finish_update()` flushes pending effects when updates complete.
- `App::refresh_windows()` schedules redraw work once per update cycle.

Interpretation:

Zed avoids immediate ad hoc repainting. It records "this state changed" and lets
the runtime invalidate/re-render the right surfaces.

Our rule:

For Text Editor:

```text
user action -> update TextEditorWorkspaceState -> refreshViews()
```

Do not let `TextEditorActionStrip` directly rewrite the right context or fixture
shelf. The center, rail, detail rail, and context should render from the same
state.

## Event Flow: Typed Events

Observed path:

```text
Entity A emits Event
  Context::emit(event)
    App event listener set finds subscribers
    subscribed entities receive typed event
    dead weak entities are ignored/removed
```

Source signals:

- `Context::subscribe(...)` and `subscribe_self(...)` subscribe to typed events.
- `Context::emit(...)` emits events for subscribers.
- `Subscription` unregisters callbacks on drop.
- `SubscriberSet` retains active subscribers and removes dropped ones.

Interpretation:

Zed distinguishes state observation from typed events. A state change can mean
"rerender me"; a typed event means "something happened."

Our rule:

Use this split in our docs and later code:

```text
State:
  current document, selection, selected action, fixture result

Events:
  action executed, fixture completed, receipt added, import failed
```

Do not store every one-time event as permanent UI state unless the receipt log
needs it.

## Event Flow: Actions And Key Dispatch

Observed path:

```text
Action type declared
  keymap maps keystroke -> action
  render tree registers key context + action listeners
  focused dispatch node receives input
  dispatch bubbles from focused node up through parent views
  matching listener handles action
```

Source signals:

- Zed/GPUI actions are typed Rust structs implementing `Action`.
- `actions!` macro creates simple namespaced actions.
- More complex actions can derive `Action` and carry JSON-shaped parameters.
- `key_dispatch.rs` documents a focus-based dispatch tree.
- Keybinding conflicts resolve by focus depth: editor before pane before
  workspace.
- Sequences such as `cmd-k left` are supported.

Interpretation:

Zed makes commands first-class. Buttons, menus, keymaps, and command palette
entries route to actions, not to duplicated widget-local functions.

Our rule:

For Text Editor:

```text
text.copy_plain
text.copy_prompt_block
text.clean_paste_basic
fixtures.run_all
```

must be action IDs first. Toolbar buttons, menu rows, hotkeys, command palette,
and tests call the same command identity.

## Event Flow: Mouse And Propagation

Observed:

- `Window` holds hitboxes and mouse listeners for the rendered frame.
- Dispatch has phases and propagation controls.
- `App::stop_propagation()` and `App::propagate()` control whether lower or
  higher handlers should continue receiving the event.

Interpretation:

Mouse behavior is scoped to the rendered frame and can be stopped or allowed to
bubble intentionally.

Our rule:

For Qt:

- Context menu actions should be local to selected text/current location.
- Whole-document cleanup must not trigger from accidental local context clicks.
- Sidecar row clicks should only select within the active workspace.

## Render Flow

Observed path:

```text
Window draw
  root View renders
  Render::render builds Element tree
  Element::request_layout asks Taffy for layout
  Element::prepaint commits bounds/hitboxes/frame state
  Element::paint draws
  transient callbacks/elements are dropped before next frame
```

Source signals:

- `Element` module says elements form a tree, are laid out by Taffy, and paint
  the window contents.
- Elements are constructed by calling `Render::render()` on the root view.
- The element tree and callbacks are dropped before the next frame.
- `AnyView::cached(...)` can recycle previous layout/paint if the view was not
  dirtied.

Interpretation:

Zed's UI is declarative at the view/component level but allows low-level custom
elements for editor rendering when needed.

Our rule:

For now:

```text
Qt widgets render from state.
QPlainTextEdit is the first real text surface.
Custom editor rendering is postponed.
```

Do not start custom rendering until the action system, fixture bench, and host
adapter contracts are stable.

## Async Runtime Flow

Observed path:

```text
Context::spawn(...)
  captures WeakEntity<T>
  provides AsyncApp across await points
  foreground executor runs main-thread async work
  background executor runs Send work off-thread
  async result updates entity/window through App context
```

Source signals:

- `AsyncApp` is an async-friendly version of `App` with static lifetime.
- It holds a weak app reference plus foreground/background executors.
- `ForegroundExecutor` is for main-thread tasks.
- `BackgroundExecutor` is for Send work and can spawn with priority.
- Tasks must be held or detached.
- `TaskExt` adds helpers for logging detached task errors.

Interpretation:

Zed separates UI-thread state mutation from background work. Async work does not
borrow `&mut App` across awaits; it comes back through `AsyncApp`.

Our rule:

For future Text Editor automation:

```text
UI action starts job
job runs outside direct widget mutation
job returns result/receipt
main state updates
workspace rerenders
```

Examples:

- terminal cleanup batch,
- token estimate,
- local librarian lookup,
- AI suggestion request,
- fixture run.

## Workspace, Pane, Panel, Dock

Observed:

Zed's glossary gives this vocabulary:

- `Workspace`: root of the window.
- `Center`: split into panes.
- `Pane`: area in center where items like editor, multi-buffer, or terminal
  live.
- `Panel`: dockable view implementing the `Panel` trait.
- `Dock`: left/right/bottom containers for panels.

The `Panel` trait includes:

- persistent name,
- panel key,
- position,
- valid positions,
- default/min size,
- flexible sizing support,
- icon and tooltip,
- toggle action,
- activation priority,
- enabled state.

Interpretation:

Zed treats side/bottom UI as owned dock panels with identity, sizing, position,
and toggles. They are not arbitrary global sidebars.

Our rule:

This confirms our workspace-scoped sidecar decision:

```text
Text Editor owns text editor rail/context.
Repo owns repo rail/context.
Agent owns agent rail/context.
Settings owns settings rail/context.
```

Do not show repo project rows around the Text Editor. That is a workspace
ownership failure.

## Runtime Checklist Worth Borrowing

For every non-trivial Text Editor feature, ask:

- What action ID does it expose?
- Which state object owns its data?
- Is this state or a one-time event?
- Does it need a receipt?
- Does it need async work?
- Can it run with no document?
- Can it run with no selection?
- What is the focused dispatch scope?
- Where does the command appear: palette, menu, toolbar, context, inspector?
- What sidecar owns the visible state?
- What proof screenshot or fixture verifies it?

This is the useful part of Zed's architecture for us.

## Bench Extraction: `crates/editor/benches`

Source scope:

- `crates/editor/benches/display_map.rs`
- `crates/editor/benches/editor_render.rs`

These files are small, but they are useful because benchmarks reveal what a
serious editor treats as hot-path behavior. Zed is not benchmarking button
clicks. It is benchmarking coordinate transforms, render pipeline cost,
long-line editor construction, and multi-cursor edits.

### `display_map.rs`: coordinate transforms are core editor machinery

Observed benchmark shape:

```text
random text
  -> MultiBuffer
  -> buffer snapshot
  -> InlayMap
  -> FoldMap
  -> TabMap
  -> convert point between map coordinate systems
```

It benchmarks:

- fold point -> tab point
- tab point -> fold point

Why this exists:

The text in the buffer is not the same as the text the user sees.

Real editors maintain several coordinate layers:

```text
buffer text
  raw document coordinates

inlay map
  virtual inline text, hints, decorations, completions

fold map
  hidden/collapsed regions

tab map
  tab characters expanded into display columns

screen/layout position
  what the user can point at with cursor or mouse
```

Regular application:

- cursor movement
- selection painting
- mouse hit testing
- line wrapping later
- bracket matching and diagnostics later
- inlay hints and inline AI suggestions later
- folded sections later

Teaching point:

An editor is not a string plus a cursor. It is a set of translation tables
between what exists, what is visible, and what is interactable.

Our current Track B rule:

Do not build Zed's full display-map stack now. We are still using
`QPlainTextEdit` for the visual host and headless helpers for actions.

What to borrow now:

- Name coordinate systems explicitly.
- Keep line/column helpers in `text_editor_plain`.
- Treat inlays, folds, tabs, wrapping, and visual hit testing as future
  coordinate layers, not as random UI patches.
- Do not let clipboard/export helpers depend on visual coordinates.

What to postpone:

- Inlay map
- Fold map
- Tab expansion map
- soft-wrap visual map
- mouse hit-test map
- custom renderer coordinate conversion

### `editor_render.rs`: render, startup, and multi-cursor edits are different costs

Observed benchmark groups:

```text
Time to render
Build buffer with one long line
multi cursor edits
```

Observed render path:

```text
Editor entity
  -> into element
  -> request_layout
  -> prepaint
  -> paint
```

Why this exists:

Rendering is not "draw the text." It is a pipeline:

1. Build an element/view representation.
2. Compute layout.
3. Prepare paint state, hitboxes, and bounds.
4. Paint the frame.

Regular application:

- typing latency
- scrolling latency
- large-file opening
- long-line handling
- visible selection and cursor rendering
- sidecar or overlay invalidation

Our current rule:

Use `QPlainTextEdit` while our useful behavior is action/export oriented.
Custom rendering belongs later, after the action registry, fixture bench, and
host adapter are stable.

### Long-line benchmark: one line can be worse than many lines

Zed has a benchmark specifically for opening an editor with one long line.

Teaching point:

A document with 4,000 characters on one line can stress different systems than
a document with 4,000 characters across many lines.

Why:

- soft wrap becomes expensive,
- horizontal measurement grows,
- cursor column math grows,
- layout cannot rely on cheap line chunks,
- some algorithms accidentally scan the whole line per movement.

Our golden test implication:

Add long-line fixtures before claiming editor behavior is mature.

Examples:

```text
one_line_4k_chars
one_line_with_tabs
one_line_with_markdown_fence
one_line_terminal_output
```

### Multi-cursor benchmark: repeated edits need offset discipline

Zed benchmarks an editor containing 1000 cursors, then repeatedly:

```text
handle_input("hello world")
delete_to_previous_word_start(...)
delete_to_previous_word_start(...)
```

Teaching point:

Multi-cursor editing is not "do the same edit many times." Every edit changes
the offsets of later edits unless the engine has a disciplined transaction
model.

Regular application:

- select matching occurrences,
- edit many lines at once,
- split selection into lines,
- delete previous word at every cursor,
- AI-assisted repeated transforms later.

Our current rule:

Do not build multi-cursor editing in V1. But design mutating actions so they
can later become batch transactions.

What to borrow now:

- Keep `text.select_all` and selection mutation distinct from clipboard
  output.
- Keep cleanup actions output-only until we define undo/transaction policy.
- Add future tests that prove multiple ranges are applied in stable order.

### Bench lessons translated to our package

For `text_editor_plain`:

- Own document text, selection, line helpers, and undo later.
- Keep coordinate terminology clear.
- Add long-line and selection-bound tests.
- Do not know about rendering or system clipboard.

For `text_editor_actions`:

- Every operation needs a stable action ID.
- Selection-only actions and output-producing actions must be distinct.
- Disabled results are normal output, not panics.

For `text_editor_clipboard`:

- Use buffer/selection text, not visual text.
- Never depend on folded/inlay/wrapped display state.
- Return receipts for transforms.

For `text_editor_host_adapter`:

- Hosts translate action metadata and results.
- Hosts may write to the OS clipboard.
- Hosts should not implement cleanup logic.

For future custom editor rendering:

- Define coordinate maps before drawing.
- Benchmark line mapping, tab expansion, long lines, render pipeline, and
  multi-cursor edits separately.

### What this changes in our TODO list

Add detail passes later for:

- text model and coordinate systems,
- long-line fixtures,
- visual-vs-buffer coordinate policy,
- selection transaction policy,
- future multi-cursor transaction model,
- future render pipeline proof.

Do not start those before the current Track B actions/clipboard/host path is
stable.

## Patterns To Borrow Directly

### 1. Typed command identity before UI

Borrow:

```text
action id -> metadata -> host placement -> handler
```

Do not let a button define the command.

### 2. Focus/context-sensitive action dispatch

Borrow:

```text
Editor action beats workspace action when editor has focus.
Context menu selection action requires selection.
Global command palette can still list everything.
```

### 3. Subscriptions are lifecycle objects

Borrow:

If a future Qt helper subscribes to model changes, subscription ownership must
be explicit. Dropping a panel should not leave callbacks alive.

### 4. Async returns through state

Borrow:

Background work produces result objects, then the main workspace state changes.
Widgets do not mutate each other directly from async callbacks.

### 5. Workspace owns docks/sidecars

Borrow:

Sidecars are active-workspace surfaces. They are not global content slots.

### 6. Feature proposal checklist

Borrow Zed's feature-process categories for our docs:

- actions/keybindings,
- settings,
- themes/styling,
- host/platform differences,
- persistence,
- accessibility,
- performance,
- security,
- tests/proof.

## What Not To Copy

Do not copy:

- GPUI internals into Qt.
- Zed's full entity system before we need it.
- Zed's crate count.
- custom editor rendering before QPlainTextEdit proves insufficient.
- `rope`, `multi_buffer`, `lsp`, or syntax-engine runtime work in Track B.
- background task architecture before we have actual async tasks.

The goal is not to become Zed. The goal is to borrow the separation laws.

## Mapping To Our Text Editor Package

Recommended local mapping:

```text
Zed Action
  -> TextActionId + TextActionRecord

Zed key dispatch
  -> host hotkey profile + focused surface policy

Zed App/Entity state
  -> TextEditorWorkspaceState and focused model structs

Zed Context notify
  -> state mutation followed by one refresh/render route

Zed EventEmitter
  -> receipts and one-time action/fixture events

Zed Workspace/Panel/Dock
  -> WorkspaceKind + scoped rail/detail/context renderers

Zed Render/Element
  -> Qt widgets rendered from state; QSS via ui_path

Zed AsyncApp/Task
  -> future local worker/async job result -> state update -> receipt
```

## Concrete Rules For Our Next Builds

1. Add no new Text Editor button without an action ID.
2. Add no new sidecar content without naming the owning workspace.
3. Add no async helper that writes directly into widgets.
4. Add no cleanup action without receipt shape.
5. Add no host-specific hotkey unless it maps to action metadata.
6. Add no fixture runner behavior without expected/actual/receipt proof.
7. Add no UI route that hides rails to cover wrong scoped content.

## Acceptance

This reference is useful when:

- it keeps Text Editor command work action-first,
- it keeps sidecars workspace-scoped,
- it helps us explain state vs event vs receipt,
- it prevents Qt widgets from becoming command owners,
- it gives Spark enough runtime vocabulary to avoid inventing a second system.

## Source Links

- Zed repository: <https://github.com/zed-industries/zed>
- Zed contributing guide: <https://github.com/zed-industries/zed/blob/main/CONTRIBUTING.md>
- Zed feature process: <https://github.com/zed-industries/zed/blob/main/docs/src/development/feature-process.md>
- Zed glossary: <https://github.com/zed-industries/zed/blob/main/docs/src/development/glossary.md>
- GPUI app runtime: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app.rs>
- GPUI context: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/context.rs>
- GPUI async context: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/async_context.rs>
- GPUI entity map: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/app/entity_map.rs>
- GPUI subscriptions: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/subscription.rs>
- GPUI actions: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/action.rs>
- GPUI key dispatch: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/key_dispatch.rs>
- GPUI executor: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/executor.rs>
- GPUI render elements: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/element.rs>
- GPUI views: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/view.rs>
- GPUI platform trait: <https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs>
- Zed workspace shell: <https://github.com/zed-industries/zed/blob/main/crates/workspace/src/workspace.rs>
- Zed dock/panel trait: <https://github.com/zed-industries/zed/blob/main/crates/workspace/src/dock.rs>
