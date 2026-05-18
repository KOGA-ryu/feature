# Text Editor Detail Pass TODO

This checklist captures what needs a focused detail pass before the text editor
feature system grows wider. It is intentionally a **what to detail** list, not a
how-to-build plan.

The goal is to slow down at the right layer: name the objects, surfaces,
contracts, and review gates before adding more behavior.

## Discussion Order

Use this order when filling the docs out together:

1. UI organization model
2. Action surfaces and placement
3. Fixture/review bench
4. Copy/export and paste cleanup
5. Host adapter boundaries
6. Future AI/local librarian layer

## Detail TODO

- [x] Define the overall UI organization model.
- [x] Detail command surfaces: menu, toolbar, command palette, context menu, inspector.
- [ ] Detail the action category taxonomy.
- [x] Detail action placement rules.
- [ ] Detail hotkey profile rules.
- [ ] Detail icon naming and button rules.
- [x] Detail CSS/QSS style and UI pathing rules.
- [x] Detail personal text editor reference image dissection.
- [x] Detail text editor surface grammar sheet.
- [x] Detail full UI layout spec sheet.
- [ ] Detail fixture runner behavior.
- [ ] Detail fixture catalog structure.
- [ ] Detail expected vs actual review display.
- [ ] Detail result receipts and cleanup reports.
- [ ] Detail copy/export actions.
- [ ] Detail paste cleanup actions.
- [ ] Detail terminal text helper actions.
- [ ] Detail prompt/AI helper actions.
- [ ] Detail selection helper actions.
- [ ] Detail line helper actions.
- [ ] Detail navigation helper actions.
- [ ] Detail search/replace actions.
- [ ] Detail markdown formatting actions.
- [ ] Detail draft/file state actions.
- [ ] Detail validation and cleanup actions.
- [ ] Detail undo/redo policy.
- [ ] Detail text model and Unicode policy.
- [ ] Detail empty/error/disabled action behavior.
- [ ] Detail security, secrets, and redaction policy.
- [ ] Detail accessibility and keyboard-only behavior.
- [ ] Detail Qt host adapter expectations.
- [ ] Detail future terminal host expectations.
- [ ] Detail future web host expectations.
- [ ] Detail command palette behavior.
- [x] Detail settings/workbench layout behavior.
- [ ] Detail reusable component ownership.
- [ ] Detail feature crate boundaries.
- [ ] Detail review rubric for Spark/code work.
- [ ] Detail golden test requirements.
- [ ] Detail release gate checklist.
- [ ] Detail AI/local librarian symbol language.
- [ ] Detail build order after the current fixture bench.
- [ ] Detail what stays postponed: renderer, rope, piece table, LSP, syntax engine, multi-cursor.

## First Detail Cluster

Start with UI organization before adding more actions:

- [x] What belongs in the menu.
- [x] What belongs in the toolbar.
- [x] What belongs in the command palette.
- [x] What belongs in the context menu.
- [x] What belongs in the inspector/options panel.
- [x] What belongs in the fixture bench.
- [x] What should never be duplicated per host.

## Completed Detail: UI Organization Model

The locked model is the mature editor model.

Why this exists:

Every text action needs a predictable home before the action list gets large.
The command palette can hold everything, but the visible UI cannot. Menus,
toolbars, context menus, the inspector, and the fixture bench each solve a
different user problem. If those roles are not fixed early, later features will
sprawl into whatever widget happened to be open when they were built.

Surface ownership:

- Command palette contains every action.
- Menus provide complete discovery through classic editor categories.
- Toolbar contains frequent atomic actions only.
- Context menu contains selection/current-location actions only.
- Inspector owns parameters, options, receipts, and expected/actual output.
- Fixture bench owns proof and regression review.

Why these defaults were chosen:

- Mature editors already trained users to look in `File`, `Edit`, `View`,
  `Tools`, and `Help`.
- The toolbar should support muscle-memory work, not become a catalog.
- Context menus should feel local to the selected text or cursor location.
- Cleanup actions can damage intent, so whole-document cleanup must require an
  explicit non-context path.
- The inspector is the right place for parameters because it can change without
  crowding the editor.
- The fixture bench belongs outside normal editing because it is proof and
  regression review, not day-to-day text manipulation.

Classic menu map:

- `File`: document, draft, and file-state actions later.
- `Edit`: copy, paste, select, line helpers, and undo later.
- `View`: layout and readability controls later.
- `Tools`: cleanup, terminal helpers, fixtures, and validation.
- `Help`: commands/hotkeys, docs, and policy references.

Toolbar V1:

- `Copy`
- `Prompt`
- `Markdown`
- `Fence`
- `Clean`
- `Run All`
- `Commands`

Context-menu safety:

- Copy/export may use selected-or-all behavior.
- Cleanup from context menu must require selected text.
- Whole-document cleanup must be explicit from menu, palette, or inspector.

Inspector behavior:

- Inspector fields are contextual to the selected action or fixture.
- Receipts and expected-vs-actual output belong in the inspector or adjacent
  result surface.

What not to do:

- Do not add every action to the toolbar.
- Do not put fixture runner commands in the text context menu.
- Do not make cleanup context actions silently operate on the whole document.
- Do not duplicate action behavior in host widgets.
- Do not create a second placement vocabulary outside the action registry and
  host adapter contracts.

Proof later:

- Action metadata can render command palette, menu, toolbar, and context-menu
  entries without host-local labels.
- Context cleanup actions are disabled when there is no selection.
- Whole-document cleanup appears only in menu, palette, or inspector flows.
- Fixture outputs show expected text, actual text, and receipts outside the
  normal context menu.

## Completed Detail: UI Style And Pathing

The detailed contract lives in `45_ui_style_and_pathing_contract.md`.
The reference dissection lives in `46_personal_text_editor_reference_dissection.md`.
The reusable surface grammar lives in `47_text_editor_surface_grammar_sheet.md`.

Locked rule:

```text
foundation tokens -> semantic tokens -> layout regions -> surfaces -> components -> action slots -> states -> exceptions
```

`ui_path` is the shared path vocabulary for CSS, QSS, screenshots, tests, and
host adapters. Hosts may translate paths into class names, object names,
dynamic properties, or test IDs, but they may not invent separate selector names
for shared editor actions.

Reference grammar:

```text
dark scope rail
light working canvas
floating dark command palette
right-side inspector
bottom proof shelf
dark global status bar
```

## Completed Detail: Workspace-Scoped Sidecars

The locked model is one active workspace owns every visible sidecar.

Correct pairings:

```text
[text editor sidecar] Text Editor [text editor context]
[agent sidecar] Agent pages [agent context]
[repo sidecar] Repo pages [repo context]
[settings sidecar] Settings form [settings context]
```

Why this exists:

The center page is not the whole app. If the center is Text Editor but the left
rail still shows projects/workers and the right context still shows repo facts,
the user is inside a mixed workspace. That makes the app feel broken even when
the center widget is technically correct.

Surface ownership:

- Text Editor owns text-editor source buckets, editor surface, editor lenses,
  inspector empty states, receipts, and fixture/proof shelves.
- Repo owns projects, repo-scoped workers, repo facts, scan facts, contracts,
  and repo context.
- Agent owns agent workers, agent records, agent context, and agent lenses.
- Settings owns spec editing navigation, registry status, save/back guidance,
  and settings context.

What not to do:

- Do not hide sidecars to cover wrong content.
- Do not treat left rail and right context as global surfaces.
- Do not route only the center content when the subject changes.
- Do not show repo/agent sidecar content around Text Editor.

Proof later:

- Toggle sidecars open while Text Editor is active and verify only Text
  Editor-specific blank sidecars appear.
- Switch to repo content and verify sidecars become repo-specific.
- Switch to Settings and verify sidecars become settings-specific.
- Screenshots should prove sidecars are scoped, not collapsed.

## Completed Detail: Text Editor UI Layout Spec Sheet

The deep layout contract is split across:

- `45_ui_style_and_pathing_contract.md`
- `46_personal_text_editor_reference_dissection.md`
- `47_text_editor_surface_grammar_sheet.md`

Why this exists:

The Text Editor is becoming a main workspace. It needs a precise anatomy before
more actions, proof tools, clipboard helpers, or AI helper surfaces are added.
Without this pass, every new feature would compete for toolbar, inspector, rail,
or proof space based on convenience instead of a shared grammar.

Locked surface hierarchy:

```text
app chrome
subject tabs
action toolbar
scoped left rail
center editor surface
scoped detail rail
scoped right inspector
fixture/proof shelf
bottom status bar
command palette overlay
```

Locked workspace ownership:

```text
Text Editor owns Text Editor rail, center, detail rail, and context.
Repo owns repo rail, center, detail rail, and context.
Agent owns agent rail, center, detail rail, and context.
Settings owns settings rail, center, detail rail, and context.
```

Text Editor blank target:

- left rail: `Documents`, `Clipboard`, `Drafts`, `Fixtures`
- center: action strip, document surface, fixture/proof shelf slots
- right context: inspector, options, receipts
- no repo rows
- no workers
- no agent facts
- no fake document
- no proof result unless a fixture actually ran

Why this default was chosen:

The screenshot is a strong visual grammar, but its sample project and worker
content are not the blank Text Editor contract. We keep the mature editor
layout and the dense proof-oriented shape while refusing to carry over seeded
content.

What not to do:

- Do not hide sidecars as a substitute for scoped content.
- Do not put repo or agent content around Text Editor center content.
- Do not place every future command in the toolbar.
- Do not elide exact text regions.
- Do not create sample documents to make the blank workspace look active.

Proof later:

- Text Editor proof screenshots show sidecars open.
- Sidecar proof screenshots prove scoped content, not collapsed rails.
- Exact text output regions scroll instead of eliding.
- The command palette, inspector, fixture bench, and receipts use shared
  `ui_path` names.

## Acceptance For This Detail Pass

- Each completed topic names the object or surface being detailed.
- Each completed topic states why it exists.
- Each completed topic states where the behavior belongs.
- Each completed topic states what not to do.
- Each completed topic names the tests or proof that should verify it later.
- No new implementation slice starts from a topic still marked unchecked when it
  is directly relevant to that implementation.
