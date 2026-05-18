# Personal Text Editor Reference Dissection

## Purpose

Dissect `personaltexteditor.png` into reusable UI pieces for the text editor
workbench.

This is a reference specimen, not an implementation mandate. The goal is to
preserve the structure and style grammar so future Qt/QSS, CSS, screenshots,
and host adapters can copy the useful shape without guessing.

## Source

Image:

```text
/Users/kogaryu/Downloads/personaltexteditor.png
```

Observed size:

```text
1536 x 1024
```

Sampled colors:

| Sample | Hex | Use |
| --- | --- | --- |
| titlebar background | `#f7f8f8` | app chrome |
| left rail background | `#25282b` | scope rail |
| left rail selected card | `#3e3f42` | selected project |
| top navigation background | `#ffffff` | subject tabs |
| toolbar background | `#fdfdfd` | action toolbar |
| right inspector background | `#ffffff` | option surface |
| bottom status background | `#26282b` | global status |
| command palette background | `#2b2e30` | floating overlay |
| editor selection | `#e5f1f7` | selected line/text |
| pass badge background | `#e5f5ec` | success badge |
| input field background | `#efeeef` | form fields |

## High-Level Grammar

The screenshot works because it separates scope, work, command, proof, and
status into different visual zones.

```text
dark scope rail
light working canvas
floating dark command palette
right-side inspector
bottom proof shelf
dark global status bar
```

Do not flatten these into generic panels. Each zone has a different job.

## What We Copy From The Reference

Copy the structure:

- dark left scope rail
- light center work area
- top subject tabs
- action-first toolbar
- right inspector/options column
- bottom fixture/proof shelf
- bottom global status strip
- floating dark command palette

Copy the behavior grammar:

- command palette owns search across every action
- toolbar owns frequent atomic actions only
- inspector owns parameters and receipts
- fixture bench owns expected/actual proof
- exact text surfaces use monospace and scrolling
- pass/fail status uses text plus icon/color

Copy the density:

- compact rows
- tight but readable controls
- visible dividers between major regions
- few decorative surfaces
- high information density without hiding primary labels

## What We Do Not Copy From The Reference

Do not copy sample content as product truth:

- the sample file text is reference content only
- the project named `features` is reference content only
- the worker rows are reference content only
- pass badges are reference proof content only

Do not copy cross-workspace leakage:

- Text Editor blank state must not show repo projects or workers
- Repo workspace must not show Text Editor clipboard buckets
- Agent workspace must not inherit repo scan context
- Settings workspace must not inherit editor fixtures

Do not copy unbuilt claims:

- blank Text Editor pages may name slots, but they must not claim actions,
  fixtures, clipboard ingest, or cleanup behavior are ready until the code is
  mounted there
- fixture pass/fail visuals appear only when fixtures have actually run

## Layout Measurement Map

The source image is `1536x1024`. Approximate visual proportions:

| Region | Approximate Size | Role |
| --- | --- | --- |
| app chrome | full width x 40px | global app identity |
| subject tabs | full width x 50px | main subject navigation |
| action toolbar | full width x 60px | frequent actions |
| left rail | 200px wide | scope and context |
| right inspector | 340px wide | options and receipts |
| bottom fixture shelf | 270px tall | proof and expected/actual review |
| bottom status | full width x 40px | persistent operational facts |
| command palette | 470px wide overlay | all commands through search |

Implementation target ranges are defined in
`47_text_editor_surface_grammar_sheet.md`. This dissection records what the eye
sees; the grammar sheet records the contract a host must satisfy.

## Workspace Mapping

The screenshot shows a repo-flavored specimen around a text-action workbench.
The implementation target is stricter:

```text
Text Editor active
  left rail: Documents, Clipboard, Drafts, Fixtures
  center: action strip, document surface, fixture/proof shelf
  detail rail: Text Editor lenses
  right context: inspector, options, receipts

Repo active
  left rail: projects and repo-scoped workers
  center: repo binder pages
  detail rail: repo lenses
  right context: repo facts, scan/contract state

Agent active
  left rail: agent navigation only
  center: agent pages
  detail rail: agent lenses
  right context: agent records/empty states

Settings active
  left rail: settings/spec navigation
  center: spec sheet
  detail rail: settings sections if needed
  right context: save/edit/back status
```

This means the reference is a grammar target, not a content target. The Text
Editor blank workspace must use the same visual shape without carrying the
project/worker rows from the screenshot.

## App Chrome

Purpose:

- names the app
- exposes global command access
- stays out of the editing flow

Visible pieces:

- app icon
- title: `Dex Home - Developer Binder`
- command palette shortcut button
- native window controls

Path roots:

```text
workbench.chrome.title
workbench.chrome.command_palette_button
```

Styling:

- height: about `40px`
- background: `#f7f8f8`
- bottom border: light neutral divider
- title: 13-14px, medium weight
- command shortcut button: pill-like, compact, right aligned

What not to do:

- do not place editing actions in app chrome
- do not use app chrome as a second toolbar

## Left Rail

Purpose:

- defines active project and worker scope
- shows binder context facts
- communicates system readiness

Visible pieces:

- Binder header
- Projects section
- selected project card
- Workers section
- worker rows
- Binder Context facts
- system ready footer

Path roots:

```text
workbench.rail.projects.selected_project
workbench.rail.workers.worker_row
workbench.rail.context.fact_row
workbench.rail.status.system_ready
```

Styling:

- width: about `200px`
- background: `#25282b`
- selected card: `#3e3f42`
- text: white primary, gray muted
- separators: subtle white alpha dividers
- status dot: green success token
- cards: compact radius, dense padding

What not to do:

- do not put editor action controls in the rail
- do not overfill worker rows with long metadata
- do not rely on color alone for worker status

## Subject Tabs

Purpose:

- switch binder/workbench subject
- make `Text Actions` the active work area

Visible pieces:

- Profile
- Inventory
- Contracts
- Text Actions selected
- Diff Scan
- Terminal
- Settings

Path roots:

```text
workbench.tabs.subject.profile
workbench.tabs.subject.text_actions
workbench.tabs.subject.diff_scan
```

Styling:

- height: about `48px`
- background: white/light surface
- selected tab: raised rounded tab with border
- inactive tabs: flat text/icon actions
- spacing: roomy enough for scan, not banner-sized

What not to do:

- do not style subject tabs like toolbar buttons
- do not hide active subject state

## Action Toolbar

Purpose:

- exposes frequent atomic text actions
- keeps command execution close to the editor

Visible pieces:

- Commands dropdown
- Run All
- Reset Sample
- Copy
- Prompt
- Markdown
- Clean
- settings icon

Path roots:

```text
workbench.toolbar.primary.commands
workbench.toolbar.primary.run_all
workbench.toolbar.primary.copy_plain
workbench.toolbar.primary.copy_prompt_block
workbench.toolbar.primary.copy_markdown_block
workbench.toolbar.primary.clean_basic
```

Styling:

- height: about `60px`
- button height: 36-38px
- button radius: 5-6px
- button border: light neutral
- button fill: white
- icon left, label right
- gaps: 8-12px

What not to do:

- do not add every action to the toolbar
- do not create host-local labels for toolbar actions
- do not let toolbar labels overflow

## Editor Surface

Purpose:

- provides the main text work area
- shows exact text and selection state

Visible pieces:

- file tabs
- add tab affordance
- line-number gutter
- monospaced text body
- selected line/text highlight
- scroll bar
- editor-local status strip

Path roots:

```text
workbench.editor.tabs.active_file
workbench.editor.tabs.add_file
workbench.editor.gutter.line_numbers
workbench.editor.surface.text_body
workbench.editor.selection.active
workbench.editor.status.cursor_position
```

Styling:

- surface: white or near-white editor paper
- border: light neutral
- text: monospace, 13-14px
- gutter: muted gray line numbers
- selection: `#e5f1f7`
- active line: subtle blue/selection family
- scroll bar: visible but quiet

What not to do:

- do not use proportional font for exact result text
- do not decorate text in a way that changes perceived content
- do not hide line numbers when line actions are active

## Command Palette Overlay

Purpose:

- exposes every action through search
- keeps rare commands available without crowding the toolbar

Visible pieces:

- floating overlay
- search field
- recent commands section
- all commands section
- command rows
- shortcut chips
- footer help

Path roots:

```text
workbench.palette.overlay
workbench.palette.search.input
workbench.palette.section.recent
workbench.palette.section.all
workbench.palette.action.copy_prompt_block
workbench.palette.shortcut_chip
workbench.palette.footer.help
```

Styling:

- width: about `470px`
- background: `#2b2e30`
- radius: 10-12px
- shadow/elevation: strong overlay shadow
- search field: dark input with blue focus border
- command title: white
- description: muted gray
- shortcut chip: compact bordered token

What not to do:

- do not make palette actions different from registry actions
- do not make palette labels independent from action metadata
- do not flatten the palette into the toolbar

## Right Inspector

Purpose:

- shows selected action parameters
- shows cleanup options and receipts
- keeps options out of the editor surface

Visible pieces:

- Inspector header
- Action Options section
- source input
- language dropdown
- line range steppers
- selection mode dropdown
- toggles
- apply button
- receipt log surface below

Path roots:

```text
workbench.inspector.header
workbench.inspector.fields.source
workbench.inspector.fields.language
workbench.inspector.fields.line_range
workbench.inspector.fields.selection_mode
workbench.inspector.toggle.strip_ansi
workbench.inspector.toggle.normalize_line_endings
workbench.inspector.toggle.trim_trailing_whitespace
workbench.inspector.action.apply_selected_text
```

Styling:

- width: about `340px`
- background: `#ffffff`
- left border: light neutral divider
- section titles: uppercase, 12px, bold
- input height: about `32px`
- input radius: 5px
- toggles: blue when active
- apply button: full width

What not to do:

- do not show irrelevant fields for the selected action
- do not put proof fixture controls in the inspector unless selected
- do not hide receipt output behind transient toast-only UI

## Fixture Bench

Purpose:

- proves helper behavior
- provides expected vs actual regression review

Visible pieces:

- fixture runner list
- fixture filenames
- short fixture descriptions
- pass badges
- timings
- run all fixtures button
- expected panel
- actual panel
- result pass badge

Path roots:

```text
workbench.fixture_bench.list
workbench.fixture_bench.fixture.copy_plain
workbench.fixture_bench.fixture.copy_markdown
workbench.fixture_bench.controls.run_all
workbench.fixture_bench.results.expected
workbench.fixture_bench.results.actual
workbench.fixture_bench.results.status
```

Styling:

- bottom shelf height: about `270px`
- background: white/light surface
- top border: light neutral
- panel dividers: visible but quiet
- pass badge: `#e5f5ec`
- expected/actual text: monospace
- result status: badge aligned near result surface edge

What not to do:

- do not elide exact expected/actual output
- do not mix fixture setup with normal text context menu
- do not show pass/fail by color alone

## Receipt Log

Purpose:

- gives action execution receipts
- records source, selection, changes, warnings, and duration

Visible pieces:

- receipt log header
- timestamped rows
- green success/check icons
- action id and details
- clear log button

Path roots:

```text
workbench.receipts.log
workbench.receipts.row
workbench.receipts.row.status
workbench.receipts.action.clear_log
```

Styling:

- surface: white, aligned with inspector/bottom shelf
- text: compact, 12-13px
- success icon: green token plus shape
- details: indented under action row
- clear log button: full width, low-emphasis

What not to do:

- do not use receipts as the only proof surface
- do not hide warnings in tiny muted text
- do not store secrets in visible receipts unless explicitly allowed

## Bottom Status Bar

Purpose:

- communicates global workbench status
- shows editor/runtime facts that should always be visible

Visible pieces:

- editor type
- mode
- project
- cursor position
- indentation
- encoding
- line endings
- system status

Path roots:

```text
workbench.status.editor_type
workbench.status.mode
workbench.status.project
workbench.status.cursor
workbench.status.indentation
workbench.status.encoding
workbench.status.line_endings
workbench.status.system
```

Styling:

- height: about `40px`
- background: `#26282b`
- text: white primary, gray muted
- success: green dot/icon plus text
- layout: left facts, center editor facts, right system status

What not to do:

- do not put interactive editing controls in the status bar
- do not let status facts wrap vertically

## Acceptance

This reference dissection is accepted when every visible major piece from the
screenshot has a purpose, path root, styling direction, and explicit warning
about what not to do.
