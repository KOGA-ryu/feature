# UI Style And Pathing Contract

## Purpose

Define how text-editor UI surfaces are named, styled, and routed before more
buttons, menus, inspectors, or fixture views are added.

This contract gives CSS, QSS, action placement, screenshots, and host adapters
one shared language.

Reference docs:

- `46_personal_text_editor_reference_dissection.md` dissects the reference
  screenshot into visible pieces.
- `47_text_editor_surface_grammar_sheet.md` defines the reusable grammar for
  each surface.

## Why This Exists

Action metadata already says what a command is and where it may appear. The
style/pathing contract says what that rendered location is called and how it is
allowed to look.

Without this layer, every host starts inventing local selector names:

```text
copyButton
PromptBtn
qtCleanAction
webToolbarMagic
```

Those names do not compose. They also make screenshots, tests, and future host
adapters harder to compare.

## Core Hierarchy

Style decisions must flow in this order:

```text
foundation tokens
-> semantic tokens
-> layout regions
-> surfaces
-> components
-> action slots
-> states
-> exceptions
```

Definitions:

- `foundation token`: raw reusable value such as spacing, radius, or font size.
- `semantic token`: meaning-based value such as editor background or danger
  border.
- `layout region`: major workbench area such as toolbar, editor, inspector, or
  fixture bench.
- `surface`: visible container inside a region.
- `component`: reusable control such as action button, menu row, field, or
  result block.
- `action slot`: a specific action rendered in a specific surface.
- `state`: visual condition such as active, disabled, warning, or focused.
- `exception`: documented one-off override with a reason.

## Shared Terms

`ui_path`

: Stable location path for a rendered action or surface.

`style_token`

: Reusable design value consumed by hosts instead of hardcoded widget values.

`component_state`

: One of `default`, `hover`, `pressed`, `focused`, `active`, `disabled`,
  `warning`, `danger`, or `success`.

## UI Path Format

UI paths use lowercase snake case and dot-separated ownership:

```text
workbench.<region>.<surface_or_group>.<component_or_action>
```

Required examples:

```text
workbench.toolbar.primary.copy_plain
workbench.menu.edit.copy_plain
workbench.context.selection.clean_basic
workbench.inspector.action.clean_basic
workbench.fixture_bench.results.expected_actual
workbench.rail.text_editor.documents
workbench.editor.surface.document
workbench.inspector.text_editor.options
workbench.status.editor.cursor
```

Action paths use the action ID without the `text.` prefix:

```text
text.copy_prompt_block -> workbench.toolbar.primary.copy_prompt_block
text.trim_trailing_whitespace -> workbench.menu.tools.trim_trailing_whitespace
```

## Surface Path Rules

Workspace sidecars:

```text
workbench.rail.text_editor.<bucket>
workbench.rail.repo.<section>
workbench.rail.agent.<section>
workbench.rail.settings.<section>
workbench.inspector.text_editor.<section>
workbench.inspector.repo.<section>
workbench.inspector.agent.<section>
workbench.inspector.settings.<section>
```

The active workspace owns left rail, center content, detail rail, and right
context together. A host must not render `workbench.rail.repo.*` beside Text
Editor center content.

Command palette:

```text
workbench.palette.action.<action_id_without_text_prefix>
```

Menus:

```text
workbench.menu.file.<action>
workbench.menu.edit.<action>
workbench.menu.view.<action>
workbench.menu.tools.<action>
workbench.menu.help.<action>
```

Toolbar:

```text
workbench.toolbar.primary.<action>
workbench.toolbar.secondary.<action>
```

Context menu:

```text
workbench.context.selection.<action>
workbench.context.cursor.<action>
```

Inspector:

```text
workbench.inspector.action.<action>
workbench.inspector.fields.<field>
workbench.inspector.receipts.<receipt>
```

Fixture bench:

```text
workbench.fixture_bench.list.<fixture>
workbench.fixture_bench.controls.<action>
workbench.fixture_bench.results.expected_actual
workbench.fixture_bench.results.receipts
```

Status/chrome:

```text
workbench.chrome.title
workbench.chrome.command_palette_button
workbench.tabs.subject.text_editor
workbench.status.editor.cursor
workbench.status.editor.encoding
workbench.status.system.ready
```

Editor surface:

```text
workbench.editor.tabs.active_file
workbench.editor.surface.document
workbench.editor.gutter.line_numbers
workbench.editor.selection.active
workbench.editor.status.cursor_position
```

## Workspace Routing Rules

The active workspace selects every major region as one unit:

```text
active_workspace = TextEditor
  -> workbench.rail.text_editor.*
  -> workbench.editor.*
  -> workbench.detail.text_editor.*
  -> workbench.inspector.text_editor.*

active_workspace = Repo
  -> workbench.rail.repo.*
  -> repo center pages
  -> workbench.detail.repo.*
  -> workbench.inspector.repo.*

active_workspace = Agent
  -> workbench.rail.agent.*
  -> agent center pages
  -> workbench.detail.agent.*
  -> workbench.inspector.agent.*

active_workspace = Settings
  -> workbench.rail.settings.*
  -> settings spec pages
  -> workbench.detail.settings.*
  -> workbench.inspector.settings.*
```

Sidecars are not global. The left rail and right context are scoped render
targets. A screenshot with `workbench.editor.*` in the center and
`workbench.rail.repo.*` on the left is a failing screenshot, even if no widgets
overlap.

## Layout Region Contract

Hosts should implement these regions before adding more actions:

| Region | `ui_path` Root | Sizing Contract | Owns |
| --- | --- | --- | --- |
| app chrome | `workbench.chrome` | 36-42px tall | app title, command palette affordance |
| subject tabs | `workbench.tabs.subject` | 44-52px tall | workspace/subject navigation |
| action toolbar | `workbench.toolbar.primary` | 52-64px tall | frequent atomic actions |
| left rail | `workbench.rail.<workspace>` | 190-260px wide | workspace scope/source objects |
| detail rail | `workbench.detail.<workspace>` | 44-84px wide | active workspace lenses |
| center editor | `workbench.editor` | flexible, minimum 460px | document/workbench surface |
| inspector | `workbench.inspector.<workspace>` | 280-380px wide | options, receipts, context |
| fixture bench | `workbench.fixture_bench` | 200-360px tall | proof and expected/actual review |
| status bar | `workbench.status` | 30-44px tall | persistent mode/status facts |
| palette | `workbench.palette` | 440-520px wide overlay | all actions through search |

The layout contract is intentionally boring. It prevents clever shortcuts such
as using the inspector for fixture output, using the rail for action buttons, or
using the status bar for primary controls.

## Token Rules

Foundation tokens should cover:

- spacing
- density
- typography
- color
- border width
- radius
- shadow/elevation if a host supports it
- grid step
- divider width
- scrollbar width
- focus ring width

Semantic tokens should cover:

- `surface.editor.background`
- `surface.chrome.background`
- `surface.rail.background`
- `surface.toolbar.background`
- `surface.inspector.background`
- `surface.fixture_bench.background`
- `surface.palette.background`
- `surface.status.background`
- `text.primary`
- `text.muted`
- `border.default`
- `border.focused`
- `divider.default`
- `selection.editor.background`
- `status.warning`
- `status.danger`
- `status.success`

Do not hardcode final colors, spacing, or radii inside action renderers. Hosts
may choose concrete values, but the values must map back to semantic meaning.

## Component Rules

Alignment and density:

- use a consistent alignment grid
- keep dense work surfaces dense, but not cramped
- section padding must be consistent inside each region
- row spacing should communicate grouping before decoration does

Divider rules:

- dividers separate regions, not every small detail
- left rail, inspector, fixture bench, and status bar may use strong region
  dividers
- internal list dividers should be quieter than region dividers

Elevation rules:

- command palette is the primary elevated overlay
- selected tabs may be slightly raised
- left rail and status bar stay flat
- do not add decorative shadows to every surface

Scrollbar rules:

- scrollbars are allowed where exact content must remain unelided
- editor, command palette, inspector, fixture bench, and receipt log may scroll
- scrollbars should be visible enough to communicate overflow

Toolbar buttons:

- compact fixed-height controls
- no text overflow
- icon plus short label when space allows
- icon-only only when icon policy allows it
- tooltip and accessible name required
- disabled state must be visually distinct

Menu rows:

- complete action labels
- shortcut display where assigned
- disabled reason where practical
- no hidden behavior that differs from the action registry

Context menu rows:

- selection/current-location actions only
- cleanup actions require selected text
- no whole-document cleanup from context menu

Inspector fields:

- contextual to the selected action or fixture
- one field label per input
- receipts and expected-vs-actual output stay readable in monospace where useful
- irrelevant fields are hidden, not disabled clutter

Fixture result surfaces:

- expected text and actual text are visually paired
- receipts are adjacent to the result they explain
- failures use warning/danger state tokens
- exact text output must be copyable without visual decoration

## State Rules

Every host should support these component states:

- `default`
- `empty`
- `hover`
- `pressed`
- `focused`
- `active`
- `selected`
- `disabled`
- `warning`
- `danger`
- `success`
- `running`
- `pass`
- `fail`

Focus rings are required for keyboard-visible controls. Do not rely on hover as
the only discoverability signal.

Disabled actions must not look clickable. When the host supports it, the
disabled reason should be available through tooltip, status text, or inspector
detail.

Empty, loading, error, and success states:

- empty state explains what is missing and how to proceed
- loading state should preserve layout dimensions where possible
- error state uses danger token plus text
- success state uses success token plus text
- proof/result surfaces must not rely on color alone for pass/fail

State selector policy:

```text
component_state=default
component_state=empty
component_state=hover
component_state=pressed
component_state=focused
component_state=active
component_state=selected
component_state=disabled
component_state=warning
component_state=danger
component_state=success
component_state=running
component_state=pass
component_state=fail
```

Hosts may translate those values to CSS classes, QSS dynamic properties, test
IDs, or accessibility states. Hosts must not create a separate state language
such as `hot`, `chosen`, `bad`, or `green_ok` for shared editor surfaces.

## Overflow And Elision

Text may elide only when the full value remains available through tooltip,
inspector detail, or adjacent expanded view.

Rules:

- toolbar short labels may elide
- menu labels should not elide in normal desktop widths
- inspector field values may wrap or elide depending on field type
- exact fixture output must not elide inside the result body
- command palette rows may elide secondary descriptions, not labels

Receipt and result typography:

- exact output should use monospace
- receipts can use proportional text for metadata and monospace for exact
  snippets
- expected/actual comparisons should align line numbers and content
- warnings should remain scannable even in dense layouts

Responsive collapse:

- first collapse optional descriptions
- then collapse secondary toolbar actions into `Commands`
- then allow inspector or fixture bench to hide behind a toggle
- never collapse proof/result exact text into an unreadable summary-only state
- never collapse sidecars to mask that they contain the wrong workspace content

## Host Mapping Rules

Hosts may translate `ui_path` into:

- CSS class names
- QSS object names
- QSS dynamic properties
- test IDs
- accessibility identifiers
- screenshot manifest keys

Hosts may not invent local selector names for shared editor actions.

Good:

```text
ui_path: workbench.toolbar.primary.copy_prompt_block
qss object/property: textActionButton[uiPath="workbench.toolbar.primary.copy_prompt_block"]
```

Bad:

```text
objectName: promptMagicButton
```

## Qt/QSS Mapping

Qt maps `ui_path` into object/property metadata for QSS and proof tests.

Recommended shape:

```text
objectName: textActionButton
property uiPath: workbench.toolbar.primary.copy_prompt_block
property componentState: active
```

QSS may style by region, component, and state. It must not define behavior.
Action behavior still comes from action metadata and host adapter execution.

## Screenshot Proof Naming

Screenshot and manifest names should use the same path vocabulary where
practical.

Examples:

```text
workbench_text_actions_1280x800.png
workbench_palette_open_1280x800.png
workbench_fixture_bench_fail_state_1280x800.png
```

Proof screenshots should cover:

- Text Editor blank workbench layout with both sidecars open
- workspace-scoped sidecar proof for Text Editor, Repo, Agent, and Settings
- command palette open
- inspector with selected action
- fixture bench pass/fail states
- narrow supported desktop width

Proof screenshot rules:

- sidecars must be open for scoped-sidecar proof
- Text Editor proof must show no repo projects, no repo workers, no agent
  workers, no scan facts, and no fake document
- exact text regions must not elide
- proof filenames should identify workspace and state, for example
  `workbench_text_editor_blank_sidecars_1280x800.png`
- proof manifests should record the active workspace and selected lens

## What Not To Do

- Do not style individual actions with ad hoc widget names.
- Do not create separate CSS/QSS vocabularies per host.
- Do not make visual placement disagree with `TextHostPlacement`.
- Do not hide action meaning behind decorative styling.
- Do not encode behavior in selector names.
- Do not let screenshots use names that differ from host paths.
- Do not create proof screenshots that cannot be mapped back to a surface path.

## Tests And Proof

Future host tests should prove:

- visible action controls expose stable `ui_path` or equivalent test IDs
- toolbar/menu/context/inspector surfaces use the mature editor placement rules
- disabled, focused, active, warning, danger, and success states are visible
- exact fixture output is not elided or visually corrupted
- Qt/QSS object names and properties derive from the same path vocabulary

## Acceptance

This contract is accepted when future CSS, QSS, screenshots, and host adapters
can name the same UI surface with the same path and no host needs to invent a
parallel selector vocabulary.
