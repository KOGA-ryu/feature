# Zed UI-Mounted Learning Notes

Status: teaching companion to `48_zed_event_runtime_architecture.md`.

This document keeps the Zed architecture lessons mounted to visible UI design.
The point is not to admire runtime architecture in the abstract. The point is
to explain what the user sees, clicks, edits, waits on, and trusts inside the
Text Editor workbench.

## Purpose

Use Zed as a mature reference for how an editor keeps runtime structure clean,
then translate each idea into our UI grammar:

```text
Zed concept
  -> plain-English meaning
  -> visible UI surface
  -> ui_path
  -> interaction rule
  -> state rule
  -> styling/proof requirement
```

This creates a learning bridge between:

- `48_zed_event_runtime_architecture.md`
- `45_ui_style_and_pathing_contract.md`
- `47_text_editor_surface_grammar_sheet.md`
- the future Qt Text Editor workspace

## Why This Exists

Runtime ideas are easy to overbuild when they are not tied to a user-facing
surface. A command system, event bus, or async job model is only useful when it
keeps the UI predictable.

The teaching rule:

```text
Architecture is accepted only when it explains a visible interaction.
```

If an idea cannot be tied to a surface, action, receipt, focus rule, proof
state, or screenshot requirement, it stays postponed.

## What Not To Do

- Do not copy GPUI into Qt.
- Do not build a full entity runtime before the Text Editor needs it.
- Do not let Zed vocabulary hide unclear UI ownership.
- Do not add invisible architecture that has no action, surface, receipt, or
  proof requirement.
- Do not treat the Text Editor workbench as a place to show repo or agent
  state.

## Lesson Template

Each lesson follows this structure:

```text
Concept
Plain-English meaning
Visible UI surface
UI path examples
Interaction rule
State rule
Styling/proof requirement
What we borrow
What we do not copy yet
Question to ask during review
```

## Lesson 1: App State Means One Workspace Truth

Concept:

```text
Zed App
```

Plain-English meaning:

The app has one central runtime truth. Panels and widgets do not each decide
what the active workspace is.

Visible UI surface:

- subject tabs
- left rail
- center editor/workbench
- detail rail
- right inspector/context
- bottom status bar

UI path examples:

```text
workbench.tabs.subject.text_editor
workbench.rail.text_editor.documents
workbench.editor.surface.document
workbench.inspector.text_editor.options
workbench.status.editor.mode
```

Interaction rule:

When the active workspace is Text Editor, every sidecar must become Text
Editor-scoped. The center cannot switch alone.

State rule:

Use one active workspace value:

```text
active_workspace = TextEditor | Repo | Agent | Settings
```

Everything visible derives from that value.

Styling/proof requirement:

Screenshots must show sidecars open. A passing Text Editor proof shows no repo
projects, no repo workers, no agent rows, no scan facts, and no fake document.

What we borrow:

The central runtime truth pattern.

What we do not copy yet:

Zed's full `App` implementation or entity storage.

Question to ask during review:

```text
If I toggle the rail open, does it still match the active workspace?
```

## Lesson 2: Entities Mean Named State Objects

Concept:

```text
Zed Entity<T>
```

Plain-English meaning:

A serious editor names its pieces of state instead of scattering fields across
widgets.

Visible UI surface:

- document tab strip
- editor text surface
- inspector fields
- fixture bench
- receipt log

UI path examples:

```text
workbench.editor.tabs.active_file
workbench.editor.surface.document
workbench.inspector.action.copy_prompt_block
workbench.fixture_bench.results.expected_actual
workbench.receipts.action_log
```

Interaction rule:

Selecting a document, action, fixture, or receipt changes a named state object.
It should not directly rewrite unrelated widgets.

State rule:

Future Text Editor state should be split into explicit objects:

```text
TextEditorWorkspaceState
TextDocumentState
TextSelectionState
TextActionSelectionState
TextFixtureBenchState
TextReceiptLogState
```

Styling/proof requirement:

Each named state object should have a visible empty state before it has content:

- no document open
- no action selected
- no fixture result
- no receipts yet

What we borrow:

Typed state ownership.

What we do not copy yet:

Entity IDs, weak handles, leasing, or borrow-control machinery.

Question to ask during review:

```text
Can I point to the one state object that owns this surface?
```

## Lesson 3: Context Notify Means Update State, Then Refresh

Concept:

```text
Zed Context::notify()
```

Plain-English meaning:

After state changes, the app announces that the UI needs to update. It does not
randomly repaint pieces from inside unrelated widgets.

Visible UI surface:

- toolbar enabled states
- inspector options
- status cursor facts
- receipt log
- fixture result surface

UI path examples:

```text
workbench.toolbar.primary.copy_plain
workbench.inspector.text_editor.options
workbench.status.editor.cursor
workbench.receipts.action_log
```

Interaction rule:

Every command follows this order:

```text
user input
  -> dispatch action
  -> update state/result
  -> refresh surfaces from state
```

State rule:

Do not let the toolbar directly mutate the right inspector. The toolbar changes
state; the inspector renders from state.

Styling/proof requirement:

When state changes, all dependent surfaces must agree:

- selected action appears active in toolbar/palette if applicable
- inspector shows that action's options
- status/receipt reflects the result

What we borrow:

The single update-and-refresh route.

What we do not copy yet:

Zed's invalidator and effect flushing internals.

Question to ask during review:

```text
Did this feature update state first, or did it reach across to a widget?
```

## Lesson 4: Typed Events Become Receipts

Concept:

```text
Zed EventEmitter / Context::emit()
```

Plain-English meaning:

Some things are lasting state. Other things are moments that happened. A command
result, warning, or fixture completion is often an event before it becomes a
receipt.

Visible UI surface:

- receipt log
- fixture bench status
- command palette recent commands
- bottom status bar

UI path examples:

```text
workbench.receipts.action_log
workbench.fixture_bench.list.copy_plain
workbench.palette.recent.copy_prompt_block
workbench.status.system.ready
```

Interaction rule:

Action execution emits a result event. If the user needs to audit it later, the
event is recorded as a receipt.

State rule:

Separate current state from event history:

```text
Current state:
  selected action, current document, current fixture selection

Event/receipt history:
  action ran, cleanup changed text, fixture passed, fixture failed
```

Styling/proof requirement:

Receipts need:

- timestamp or sequence
- action ID
- input scope
- output summary
- warnings
- pass/fail where relevant

What we borrow:

The distinction between state and one-time events.

What we do not copy yet:

A general event bus before we have enough event types.

Question to ask during review:

```text
Is this a current fact, or is it a thing that happened?
```

## Lesson 5: Actions Own Commands, Not Buttons

Concept:

```text
Zed Action + key dispatch
```

Plain-English meaning:

A command is one identity used by many surfaces. The button, menu item, hotkey,
and command palette row are just different doors into the same command.

Visible UI surface:

- action toolbar
- classic menu
- context menu
- command palette
- keyboard shortcut layer
- fixture runner

UI path examples:

```text
workbench.toolbar.primary.copy_prompt_block
workbench.menu.edit.copy_prompt_block
workbench.context.selection.copy_prompt_block
workbench.palette.action.copy_prompt_block
workbench.fixture_bench.controls.run_all
```

Interaction rule:

The same action ID routes everywhere:

```text
text.copy_prompt_block
```

No host may create a separate "toolbar prompt copy" behavior.

State rule:

Action metadata owns:

- ID
- label
- icon
- tooltip
- enabled rule
- undo behavior
- hotkeys
- host placements

The handler owns behavior.

Styling/proof requirement:

Buttons, menu rows, palette rows, and context-menu rows should use the same
label/icon language unless a documented short label is provided.

What we borrow:

Action-first command dispatch and focus-aware routing.

What we do not copy yet:

Zed's exact keymap and dispatch tree implementation.

Question to ask during review:

```text
Can I trigger this through toolbar, palette, menu, hotkey, and test without
duplicating behavior?
```

## Lesson 6: Focus Decides Which Surface Handles Input

Concept:

```text
Zed focused dispatch node
```

Plain-English meaning:

Keyboard input belongs to the focused surface first. If the editor has focus,
editor commands should win over workspace-level commands.

Visible UI surface:

- editor surface
- command palette
- inspector fields
- fixture bench
- rail selection

UI path examples:

```text
workbench.editor.surface.document
workbench.palette.search
workbench.inspector.fields.source
workbench.fixture_bench.list.current
workbench.rail.text_editor.documents
```

Interaction rule:

Focus precedence:

```text
command palette input
  beats editor input while palette is open

inspector text field
  owns typing while focused

editor surface
  owns editing/navigation shortcuts while focused

workspace
  handles global actions only after local surfaces decline
```

State rule:

Track the focused surface when behavior depends on it:

```text
focused_surface = Editor | Palette | Inspector | FixtureBench | Rail | None
```

Styling/proof requirement:

Every keyboard-active surface needs a visible focus state. Focus must not be
color-only.

What we borrow:

Focus-sensitive routing.

What we do not copy yet:

Multi-stroke key sequence handling unless a real action needs it.

Question to ask during review:

```text
If this shortcut fires, which visible surface had the right to handle it?
```

## Lesson 7: Subscriptions Need Owners

Concept:

```text
Zed Subscription
```

Plain-English meaning:

If a surface listens to state changes, that listening relationship has a
lifetime. When the surface goes away, the subscription must go away too.

Visible UI surface:

- right inspector
- receipt log
- fixture bench
- command palette recent commands
- sidecars

UI path examples:

```text
workbench.inspector.text_editor.receipts
workbench.fixture_bench.results.expected_actual
workbench.palette.recent
workbench.rail.text_editor.fixtures
```

Interaction rule:

Panels may listen to state, but they may not keep dead callbacks alive after
workspace switches or panel destruction.

State rule:

Qt V1 can often avoid explicit subscriptions by rendering from state during
`refreshViews()`. If we add a listener later, its owner must be named.

Styling/proof requirement:

Workspace switching must not leave stale content in old sidecars.

What we borrow:

Subscription ownership as a lifecycle rule.

What we do not copy yet:

General-purpose subscription infrastructure.

Question to ask during review:

```text
Who owns this listener, and what removes it?
```

## Lesson 8: Async Work Returns Through State

Concept:

```text
Zed AsyncApp / Task
```

Plain-English meaning:

Slow work runs outside direct widget mutation. It returns a result, then the
main state updates and the UI refreshes.

Visible UI surface:

- running fixture rows
- AI helper output later
- token/context estimate later
- cleanup batch receipts
- status bar

UI path examples:

```text
workbench.fixture_bench.list.running
workbench.inspector.text_editor.context_cost
workbench.receipts.cleanup_report
workbench.status.system.running
```

Interaction rule:

Async flow:

```text
start action
  -> show running state
  -> background/worker result returns
  -> update result state
  -> add receipt
  -> clear running state
```

State rule:

Running state is explicit:

```text
running_action_id
running_fixture_id
pending_job_id
```

Styling/proof requirement:

Running states must reserve stable space. The UI should not jump when a job
starts or finishes.

What we borrow:

Async results return through app state.

What we do not copy yet:

Thread pools or async runtime scaffolding before the first real async feature.

Question to ask during review:

```text
Where does the result land before the UI shows it?
```

## Lesson 9: Render Flow Means State-To-Surface

Concept:

```text
Zed Render / Element
```

Plain-English meaning:

The visible UI is rebuilt from state. Rendering is not where behavior is
invented.

Visible UI surface:

- every region of the workbench

UI path examples:

```text
workbench.rail.text_editor.documents
workbench.toolbar.primary.copy_plain
workbench.editor.surface.document
workbench.inspector.text_editor.options
workbench.fixture_bench.results.expected_actual
```

Interaction rule:

The renderer decides what to show for current state. It does not decide what a
command means.

State rule:

Blank states are first-class render states:

```text
no document open
no selected action
no fixture results
no receipts
```

Styling/proof requirement:

Every rendered control should expose a stable `ui_path` or host-equivalent test
ID.

What we borrow:

State-to-view discipline.

What we do not copy yet:

Custom element rendering or layout engine work.

Question to ask during review:

```text
Is this visual surface rendering state, or secretly owning behavior?
```

## Lesson 10: Workspace Panels Are Owned Surfaces

Concept:

```text
Zed Workspace / Pane / Panel / Dock
```

Plain-English meaning:

Side and bottom regions are not generic containers. They are owned surfaces with
identity, allowed content, sizing, toggles, and activation rules.

Visible UI surface:

- Text Editor rail
- Text Editor right inspector
- fixture/proof shelf
- detail rail
- status bar

UI path examples:

```text
workbench.rail.text_editor.documents
workbench.detail.text_editor.dashboard
workbench.inspector.text_editor.options
workbench.fixture_bench
workbench.status.editor.mode
```

Interaction rule:

Each workspace owns its sidecars:

```text
[text editor rail] Text Editor [text editor context]
[repo rail] Repo [repo context]
[agent rail] Agent [agent context]
[settings rail] Settings [settings context]
```

State rule:

Panel visibility and panel content are separate:

- the user may toggle a rail open or closed
- when open, it must show the active workspace's content

Styling/proof requirement:

Sidecar proof must show open sidecars. A collapsed sidecar does not prove
scoped content.

What we borrow:

Panel identity and workspace ownership.

What we do not copy yet:

Docking/repositioning mechanics.

Question to ask during review:

```text
Does this sidecar have an owner, or is it acting global?
```

## Lesson 11: Coordinate Maps Explain Editor Surface Discipline

Concept:

```text
Zed display-map benchmarks
```

Plain-English meaning:

The text in memory is not always the text the user sees. Real editors track
buffer text, folded text, inlays, tabs, visual rows, selections, and screen
coordinates.

Visible UI surface:

- editor text surface
- gutter line numbers
- selection highlight
- status cursor facts
- expected/actual result panes

UI path examples:

```text
workbench.editor.gutter.line_numbers
workbench.editor.selection.active
workbench.editor.status.cursor_position
workbench.fixture_bench.results.expected_actual
```

Interaction rule:

For Track B, exact copy/export helpers operate on document text, not visual
wrapped text, folded text, or inlay text.

State rule:

Do not create visual-coordinate features until the state model can name:

```text
buffer_position
line_column
visual_position
selection_range
```

Styling/proof requirement:

Exact output panes must not elide or decorate text in a way that changes what
the user thinks was copied.

What we borrow:

Respect for coordinate layers.

What we do not copy yet:

Fold maps, inlay maps, tab maps, or multi-buffer maps.

Question to ask during review:

```text
Is this action using buffer text, visible text, selected text, or rendered text?
```

## Lesson 12: Long Lines Are A Proof Requirement

Concept:

```text
Zed long-line editor benchmark
```

Plain-English meaning:

Editor UIs often fail on long lines before they fail on normal paragraphs.
Long lines stress scrolling, selection, wrapping, copy, and proof output.

Visible UI surface:

- editor surface
- expected/actual panes
- receipt snippets
- status bar line/column

UI path examples:

```text
workbench.editor.surface.document
workbench.fixture_bench.results.expected_actual
workbench.receipts.cleanup_report
workbench.status.editor.cursor
```

Interaction rule:

Long-line fixtures belong in the fixture bench before we claim editor actions
are stable.

State rule:

Line helpers must define whether ranges are:

- zero-based or one-based
- inclusive or exclusive
- byte-based, character-based, or line-based

Styling/proof requirement:

Long exact text should scroll. It should not be silently elided in proof panes.

What we borrow:

Benchmark hot paths reveal real product risk.

What we do not copy yet:

Performance benchmarking infrastructure beyond focused fixtures.

Question to ask during review:

```text
Have we tested the ugly case, or only the pleasant sample?
```

## Lesson 13: Feature Process Becomes A UI Checklist

Concept:

```text
Zed feature-process categories
```

Plain-English meaning:

A feature is not just code. It changes actions, settings, styling, platform
behavior, accessibility, performance, and proof.

Visible UI surface:

- toolbar
- menu
- command palette
- inspector
- fixture bench
- status bar
- settings/reference popout

UI path examples:

```text
workbench.toolbar.primary.clean_basic
workbench.menu.tools.clean_basic
workbench.palette.action.clean_basic
workbench.inspector.action.clean_basic
workbench.fixture_bench.list.clean_basic
workbench.status.system.ready
```

Interaction rule:

Every new Text Editor feature spec must answer:

- What action ID owns it?
- Where does it appear?
- What enables/disables it?
- What does it show in the inspector?
- What receipt does it create?
- What fixture proves it?
- What shortcut profiles apply?
- What accessibility label/focus rule applies?

State rule:

No new feature without a state/event/receipt decision.

Styling/proof requirement:

No new feature without a proof surface or fixture expectation.

What we borrow:

Feature proposals as complete product slices.

What we do not copy yet:

Zed's project-management process.

Question to ask during review:

```text
Can a user find it, run it, inspect it, and trust what happened?
```

## Surface-Mounted Summary

| Surface | Zed lesson | Our rule |
| --- | --- | --- |
| Chrome | App truth | global identity only, no hidden command behavior |
| Subject tabs | workspace state | switching subject switches sidecars too |
| Left rail | Panel ownership | content belongs to active workspace |
| Toolbar | Action dispatch | buttons render action metadata |
| Command palette | full action registry | every action appears or is disabled with reason |
| Editor surface | state/render split | edit/view surfaces render state, do not own command semantics |
| Inspector | focused context | parameters and receipts follow selected action/surface |
| Fixture bench | events/proof | expected/actual/receipt proof lives outside normal editing |
| Receipt log | typed events | actions create auditable event records |
| Status bar | runtime facts | compact state summary, not primary controls |

## Review Questions For Learning Sessions

Use these when reading Zed or reviewing our Text Editor work:

1. What visible surface is affected?
2. What is the user trying to do on that surface?
3. What action ID owns the command?
4. What state object owns the data?
5. Is the result persistent state or a receipt event?
6. Which surface has keyboard focus?
7. Which sidecar owns this information?
8. What is the `ui_path`?
9. What proof screenshot or fixture would catch a regression?
10. What are we postponing on purpose?

## Immediate Rules For Our Next Builds

Before adding more Text Editor behavior:

1. Name the visible surface.
2. Name the `ui_path`.
3. Name the action ID if it is callable.
4. Name the state object if it persists.
5. Name the receipt/event if it is something that happened.
6. Name the focused surface if keyboard input matters.
7. Name the fixture or screenshot proof.

If any of those names are missing, the implementation is not ready.

## Acceptance

This learning note is accepted when it helps us explain Zed-derived runtime
ideas through UI design:

- app state becomes workspace-scoped layout,
- entities become named Text Editor state objects,
- context notify becomes update-then-refresh,
- events become receipts,
- actions become toolbar/menu/palette/context/hotkey entries,
- focus becomes a visible keyboard-routing rule,
- panels become scoped sidecars,
- rendering becomes state-to-surface proof.

The next implementation slice should use this doc as a checklist before adding
new Text Editor controls or behavior.
