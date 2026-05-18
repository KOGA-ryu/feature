# Text Editor Surface Grammar Sheet

## Purpose

Define the reusable grammar sheet for every visible surface in the text editor
workbench.

This doc converts the reference screenshot into repeatable rules. Future hosts
should be able to render the same UI grammar in Qt/QSS, CSS, egui, or another
toolkit without inventing a new structure.

## Grammar Template

Every surface entry uses this structure:

```text
Surface
Purpose
User job
ui_path root
Layout role
Sizing
Spacing
Typography
Color tokens
Border/radius
Icon rules
Interaction states
Overflow rules
Accessibility
Proof expectations
What not to do
```

## Global Workbench Grammar

Purpose:

- make scope, work, commands, options, proof, and status visually distinct

User job:

- understand where they are
- write or inspect text
- run actions
- inspect results
- trust proof

Layout role:

```text
left rail | center work area | right inspector
top chrome/tabs/toolbar
bottom fixture/status stack
floating command palette
```

Core rule:

```text
Scope is dark.
Work is light.
Command search is dark floating.
Options live right.
Proof lives bottom.
Status is a dark bottom strip.
```

Workspace ownership rule:

```text
[text editor sidecar] Text Editor [text editor context]
[agent sidecar] Agent pages [agent context]
[repo sidecar] Repo pages [repo context]
[settings sidecar] Settings form [settings context]
```

The physical sidecar slots may be reused, but their content is owned by the
active workspace. A Text Editor center surface must never be paired with repo
project rows, agent worker rows, repo scan facts, or agent context facts.

Default Text Editor blank shell:

- left rail shows text-editor source buckets, not projects or workers
- center shows action strip, document surface, and fixture/proof shelf slots
- detail rail shows Text Editor lenses only
- right context shows inspector/receipt empty state only

What not to do:

- do not hide sidecars to disguise the wrong content
- do not let global repo or agent state leak into Text Editor sidecars
- do not make the center route independently from sidecar routes

Proof expectations:

- no overlapping regions at supported desktop sizes
- no text clipping in toolbar, rail rows, or status facts
- major surfaces are visually distinguishable without relying on color alone
- sidecars match the active workspace in screenshots

## Layout Grid Contract

This grid is the starting contract for the Text Editor workspace. Hosts may
adjust exact values for native toolkit constraints, but any adjustment must keep
the same hierarchy and must be proven with screenshots.

| Region | Target | Minimum | Maximum | Notes |
| --- | ---: | ---: | ---: | --- |
| app chrome | 36-42px tall | 34px | 48px | title, command palette affordance, native window controls |
| subject tabs | 44-52px tall | 40px | 56px | workspace switcher, selected tab visible |
| action toolbar | 52-64px tall | 48px | 68px | frequent actions only, no wrapping in desktop proof |
| left rail | 200-240px wide | 190px | 260px | scoped buckets, repo rows, or settings nav depending workspace |
| detail rail | 48-72px wide | 44px | 84px | compact lens switcher only |
| right inspector | 300-360px wide | 280px | 380px | fields, options, receipts, context |
| center work area | flexible | 460px | unbounded | owns editor/document/proof surfaces |
| bottom fixture shelf | 240-320px tall | 200px | 360px | can be hidden later, but not replaced by fake summary proof |
| bottom status bar | 34-42px tall | 30px | 44px | global facts, no primary controls |
| region divider | 1px | 1px | 2px | stronger between regions, quieter inside lists |

Grid rhythm:

- primary spacing step: `8px`
- dense row spacing: `4px`
- section spacing: `16px`
- panel padding: `12px` compact, `16px` comfortable
- major dividers align to region edges
- buttons in one strip share height
- exact text surfaces reserve stable dimensions before content loads

Why this default:

The reference image succeeds because it is dense without becoming a pile. The
grid gives every future feature a slot before it exists. Copy/export actions,
terminal helpers, AI helpers, fixture proof, and receipts can grow without
stealing space from each other.

What not to do:

- do not make the editor center depend on hidden sidecars to fit
- do not let bottom shelf height push the editor below usable size
- do not let inspector width consume the center below the minimum editor width
- do not add one-off margins that break alignment with sibling regions

## Surface Hierarchy Contract

The visible stack is ordered from global to local:

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

Ownership rule:

```text
Text Editor owns Text Editor rail, center, detail rail, and context.
Repo owns repo rail, center, detail rail, and context.
Agent owns agent rail, center, detail rail, and context.
Settings owns settings rail, center, detail rail, and context.
```

The command palette is the only cross-workspace overlay, but it still renders
workspace-aware actions. If the active workspace is Text Editor, palette rows
for repo scan or agent records are either absent or disabled with a reason.

## Text Editor Blank Workspace Target

The blank Text Editor workspace is not an empty app. It is an empty workbench
with named places where future behavior will land.

Left rail buckets:

```text
workbench.rail.text_editor.documents
workbench.rail.text_editor.clipboard
workbench.rail.text_editor.drafts
workbench.rail.text_editor.fixtures
```

Center regions:

```text
workbench.toolbar.primary
workbench.editor.surface.document
workbench.fixture_bench
```

Right context regions:

```text
workbench.inspector.text_editor.options
workbench.inspector.text_editor.receipts
workbench.inspector.text_editor.context
```

Allowed blank content:

- short region labels such as `Documents`, `Clipboard`, `Drafts`, `Fixtures`
- honest empty states such as `No document open`
- disabled controls with reasons
- structural slots for future editor, fixture, receipt, and inspector behavior

Forbidden blank content:

- sample documents
- fake projects
- repo workers
- agent workers
- scan facts
- contract facts
- proof results that have not run
- placeholder prose claiming unbuilt behavior exists

Proof expectation:

Default Text Editor screenshots must show sidecars open and scoped. A passing
blank screenshot is not a collapsed layout; it is a visible Text Editor rail,
visible Text Editor center, visible Text Editor detail rail, and visible Text
Editor context with no repo or agent content.

## Allowed Content Matrix

| Surface | Allowed Content | Forbidden Content |
| --- | --- | --- |
| app chrome | app identity, command palette shortcut, native controls | text action execution, fixture controls |
| subject tabs | workspace/subject navigation | action parameters, receipts, document text |
| action toolbar | frequent atomic text actions, command entry | full action catalog, proof logs, project rows |
| Text Editor rail | documents, clipboard bucket, drafts, fixtures | repo projects, repo workers, agent workers |
| Repo rail | saved projects, repo-scoped workers | text documents, clipboard receipts, agent-only rows |
| Agent rail | agent records/workers when backed by data | repo scan facts, Text Editor document buckets |
| Settings rail | spec-sheet navigation and save/edit status | editor documents, fake project shortcuts |
| center editor | document surface, blank workspace slots, selected lens content | sidecar facts, global settings forms unless Settings active |
| detail rail | active workspace lenses | mixed workspace lenses |
| right inspector | selected workspace context, options, receipts | unrelated repo/agent/text facts |
| fixture bench | fixtures, expected/actual output, run controls, pass/fail | normal editing context menu, fake proof |
| status bar | mode, cursor, encoding, readiness | primary editing actions |
| command palette | searchable registry actions with workspace-aware enablement | host-local behavior not in action registry |

## State Grammar Matrix

All visible components must map to these state names before host-specific CSS or
QSS selectors are invented.

| State | Meaning | Required Visual Proof |
| --- | --- | --- |
| `empty` | slot has no saved/open/running content | empty-state text, stable space, no fake data |
| `hover` | pointer is over interactive control | subtle background or border change |
| `selected` | item/lens/tab is current | persistent selected surface, not color-only |
| `focused` | keyboard focus is here | visible focus ring or native focus indicator |
| `disabled` | action is unavailable | lower emphasis plus reason through tooltip/status/inspector |
| `warning` | action may be lossy or incomplete | warning token plus text/icon |
| `danger` | action failed or destructive action pending | danger token plus text/icon |
| `success` | action passed/completed | success token plus text/icon |
| `running` | async/proof action in progress | stable progress state, no layout jump |
| `pass` | fixture matched expected output | pass badge with label/icon |
| `fail` | fixture differed from expected output | fail badge with label/icon and inspectable diff |

Exact text surfaces have a stricter state policy:

- never elide exact content
- never apply decorative formatting that changes perceived text
- show empty content as an empty text block plus label, not as invented sample
- show failed output with adjacent expected/actual text

## App Chrome

Purpose:

- identify app and expose global entry points

User job:

- confirm app identity
- open command palette
- use native window controls

`ui_path` root:

```text
workbench.chrome
```

Layout role:

- topmost global band
- not part of editing command flow

Sizing:

- height: 36-42px
- command palette button: compact pill

Spacing:

- left title padding: 12-16px
- right controls gap: 8px

Typography:

- title: 13-14px, medium weight
- shortcut chip: 12px

Color tokens:

- `surface.chrome.background`
- `text.chrome.primary`
- `border.chrome.bottom`

Border/radius:

- bottom divider only
- command palette pill radius: 5-7px

Icon rules:

- app icon only
- command palette button may show shortcut text

Interaction states:

- command palette button supports hover, focused, pressed

Overflow rules:

- title may elide if window is narrow

Accessibility:

- command palette button requires accessible name and shortcut hint

Proof expectations:

- title and palette affordance visible at desktop proof size

What not to do:

- do not add document actions to chrome

## Left Rail

Purpose:

- define the active workspace scope

User job:

- see what scope the center workspace is operating inside
- select workspace-specific source objects
- avoid mixing repo, agent, settings, and text-editor navigation

`ui_path` root:

```text
workbench.rail
```

Workspace path roots:

```text
workbench.rail.text_editor
workbench.rail.repo
workbench.rail.agent
workbench.rail.settings
```

Layout role:

- fixed left scope column

Sizing:

- width: 190-240px depending host
- row height: 38-58px depending row type

Spacing:

- section padding: 12-16px
- row gap: 6-8px
- section divider spacing: 18-24px

Typography:

- section labels: uppercase, 11-12px
- selected project: 13-14px, semibold
- worker name: 13px, semibold
- metadata: 11-12px
- text-editor bucket labels: 12-13px, semibold

Color tokens:

- `surface.rail.background`
- `surface.rail.card.selected`
- `text.rail.primary`
- `text.rail.muted`
- `border.rail.divider`
- `status.success`

Border/radius:

- selected card radius: 6px
- internal dividers: 1px alpha line

Icon rules:

- project/folder icon allowed
- worker status uses icon/shape plus label, not color alone
- text-editor buckets may use document, clipboard, draft, and fixture icons

Interaction states:

- hover
- selected
- focused
- disabled if project/worker unavailable

Overflow rules:

- project and worker names elide
- full value available through tooltip or inspector/context

Accessibility:

- selected project and worker state must be announced by label/state

Proof expectations:

- long project names do not clip
- selected state is visible without color-only meaning
- Text Editor rail shows no repo project rows or agent worker rows

What not to do:

- do not place text action execution controls here
- do not expose worker session ids in compact rows
- do not reuse repo worker rows as Text Editor source buckets
- do not solve wrong rail content by collapsing the rail

## Subject Tabs

Purpose:

- switch major workbench subject

User job:

- move between profile/inventory/contracts/text actions/diff/settings

`ui_path` root:

```text
workbench.tabs.subject
```

Layout role:

- top navigation below app chrome

Sizing:

- height: 44-52px
- tab minimum width based on label and icon

Spacing:

- horizontal gap: 4-8px
- tab padding: 10-16px

Typography:

- label: 13-14px
- selected label: semibold/bold

Color tokens:

- `surface.tabs.background`
- `surface.tabs.selected`
- `text.tabs.primary`
- `text.tabs.muted`
- `border.tabs.selected`

Border/radius:

- selected tab has rounded top corners or raised pill edge
- inactive tabs stay flatter

Icon rules:

- icons allowed for scan/navigation recognition
- icons do not replace labels in V1

Interaction states:

- hover
- selected
- focused
- disabled if subject unavailable

Overflow rules:

- tab labels should not wrap
- narrow hosts may collapse later, but not in V1 proof

Accessibility:

- tabs expose role and selected state

Workspace routing:

- selecting a repo subject routes rail, center, detail rail, and context to repo
  content
- selecting an agent subject routes all four surfaces to agent content
- selecting Text Editor routes all four surfaces to Text Editor content
- selecting Settings routes all four surfaces to Settings content
- sidecars are not global; they are workspace-owned surfaces

Proof expectations:

- active subject is unmistakable
- switching subjects updates sidecar content as well as center content

What not to do:

- do not style tabs like primary action buttons
- do not update only the center page when a subject changes

## Action Toolbar

Purpose:

- expose frequent atomic actions

User job:

- run common text actions without opening the palette

`ui_path` root:

```text
workbench.toolbar.primary
```

Layout role:

- horizontal action strip above editor

Sizing:

- height: 52-64px
- button height: 34-38px
- icon button width: 34-38px

Spacing:

- toolbar padding: 10-12px
- button gap: 8px
- group divider gap: 12-16px

Typography:

- button label: 13px
- short labels only

Color tokens:

- `surface.toolbar.background`
- `surface.button.background`
- `text.button.primary`
- `border.button.default`
- `border.button.focused`

Border/radius:

- button radius: 5-6px
- border: 1px neutral

Icon rules:

- icon plus short label preferred
- icon-only allowed for common settings/utility controls with tooltip

Interaction states:

- hover
- pressed
- focused
- active when action context is open
- disabled

Overflow rules:

- labels may elide only if tooltip preserves full action label
- toolbar must not wrap in V1 desktop proof

Accessibility:

- every button needs accessible name and tooltip

Proof expectations:

- toolbar contains only frequent atomic actions
- no clipped button text

What not to do:

- do not add every action to the toolbar
- do not duplicate action semantics in click handlers

## Editor Surface

Purpose:

- show and edit exact plain text

User job:

- read, select, copy, inspect, and edit text

`ui_path` root:

```text
workbench.editor
```

Layout role:

- main flexible center surface

Sizing:

- fills remaining center width
- minimum comfortable editor width should preserve line-number gutter

Spacing:

- inner text padding: 8-12px
- gutter width based on line count

Typography:

- editor text: monospace, 13-14px
- line numbers: monospace, muted
- status strip: 12px

Color tokens:

- `surface.editor.background`
- `surface.editor.gutter`
- `text.editor.primary`
- `text.editor.muted`
- `selection.editor.background`
- `border.editor.default`

Border/radius:

- thin border around editor surface
- file tabs may have top radius

Icon rules:

- file tab close/add icons allowed

Interaction states:

- focused
- selection
- active line
- read-only
- disabled if no document loaded

Overflow rules:

- exact text must scroll, not elide
- line tabs may elide filenames

Accessibility:

- text surface owns typing focus
- focus return after toolbar actions must be predictable

Proof expectations:

- selection is visible
- line numbers remain aligned
- exact text is not visually corrupted

What not to do:

- do not decorate exact text output as rich text unless explicitly in a preview

## Command Palette

Purpose:

- expose every action through search and keyboard

User job:

- find and run actions without memorizing location

`ui_path` root:

```text
workbench.palette
```

Layout role:

- floating overlay centered over work area

Sizing:

- width: 440-520px
- maximum height: fit viewport with scrolling command list

Spacing:

- overlay padding: 10-14px
- row padding: 8-10px
- section gap: 12px

Typography:

- search input: 14px
- command label: 13px
- description: 12px
- shortcut chip: 11-12px

Color tokens:

- `surface.palette.background`
- `surface.palette.input`
- `text.palette.primary`
- `text.palette.muted`
- `border.palette.focused`
- `surface.shortcut_chip.background`

Border/radius:

- overlay radius: 10-12px
- search radius: 5-6px
- shortcut chip radius: 4-5px

Icon rules:

- command icons may appear left of labels
- icons come from action metadata

Interaction states:

- focused search
- hovered row
- selected row
- disabled action row

Overflow rules:

- command label should not elide before description
- descriptions may elide

Accessibility:

- keyboard navigation required
- escape closes
- enter runs selected command

Proof expectations:

- recent and all-command sections are distinguishable
- selected command has visible state

What not to do:

- do not let palette command behavior diverge from toolbar/menu behavior

## Right Inspector

Purpose:

- expose parameters, options, receipts, and selected-action context

User job:

- configure action scope
- see what will happen
- inspect what happened

`ui_path` root:

```text
workbench.inspector
```

Workspace path roots:

```text
workbench.inspector.text_editor
workbench.inspector.repo
workbench.inspector.agent
workbench.inspector.settings
```

Text Editor blank-state fields:

- active document: none
- clipboard: not touched
- fixtures: not running
- receipts: empty
- selected action: none

Layout role:

- fixed right option column

Sizing:

- width: 300-360px
- field height: 30-34px

Spacing:

- panel padding: 14-18px
- field gap: 8-12px
- section gap: 16-20px

Typography:

- section label: uppercase, 11-12px, bold
- field label: 12-13px
- field text: 13px
- helper text: 11-12px

Color tokens:

- `surface.inspector.background`
- `text.inspector.primary`
- `text.inspector.muted`
- `border.inspector.divider`
- `surface.field.background`
- `status.toggle.active`

Border/radius:

- left divider
- field radius: 5px
- button radius: 5-6px

Icon rules:

- section collapse icon allowed
- toggle state must not rely only on color

Interaction states:

- field focused
- toggle active
- action apply pressed
- disabled field
- warning/danger when options are invalid

Overflow rules:

- long source paths elide with tooltip
- receipt text can wrap

Accessibility:

- every field has label
- toggles expose on/off state

Proof expectations:

- fields visible and not crowded
- selected action options match current action
- Text Editor context shows no repo scan, contract, or agent facts

What not to do:

- do not show irrelevant fields for inactive actions
- do not pair Text Editor center content with repo or agent context
- do not hide context to avoid rendering the correct empty state

## Fixture Bench

Purpose:

- show proof and regression review

User job:

- run fixtures
- compare expected and actual
- identify failures quickly

`ui_path` root:

```text
workbench.fixture_bench
```

Layout role:

- bottom shelf above status bar

Sizing:

- height: 240-320px
- fixture list width: 280-340px
- result panels split remaining width evenly

Spacing:

- shelf padding: 10-12px
- row gap: 6px
- panel gap/divider: 1px divider plus 8-10px padding

Typography:

- section labels: uppercase, 11-12px, bold
- fixture name: 12-13px
- fixture description: 11-12px
- expected/actual text: monospace 12-13px

Color tokens:

- `surface.fixture_bench.background`
- `surface.result.background`
- `text.result.primary`
- `border.fixture.divider`
- `status.success`
- `status.warning`
- `status.danger`

Border/radius:

- top divider
- result boxes: light border, small radius
- status badge: compact pill

Icon rules:

- fixture file icon allowed
- pass/fail icon required with badge text

Interaction states:

- selected fixture
- running
- pass
- fail
- warning

Overflow rules:

- exact expected/actual output scrolls; it does not elide
- fixture descriptions may elide

Accessibility:

- pass/fail uses label plus icon/color

Proof expectations:

- expected and actual are side by side
- pass/fail is visible without reading tiny text

What not to do:

- do not hide exact output behind summary-only cards

## Receipt Log

Purpose:

- record action receipts and cleanup reports

User job:

- understand what action ran, what changed, and whether warnings occurred

`ui_path` root:

```text
workbench.receipts
```

Layout role:

- bottom/right receipt surface adjacent to inspector and proof

Sizing:

- width follows inspector or proof column
- rows compact but readable

Spacing:

- row padding: 6-8px
- detail indent: 18-24px

Typography:

- timestamp: 12px muted
- action label/id: 12-13px
- details: 11-12px

Color tokens:

- `surface.receipts.background`
- `text.receipts.primary`
- `text.receipts.muted`
- `status.success`
- `status.warning`
- `status.danger`

Border/radius:

- outer border matches inspector/proof surfaces
- clear button radius matches toolbar buttons

Icon rules:

- status icon plus text/row detail

Interaction states:

- row hover
- selected receipt
- warning
- danger

Overflow rules:

- long paths wrap or elide with tooltip
- action id should remain visible

Accessibility:

- receipt status announced by text

Proof expectations:

- changes, warnings, source, and duration are inspectable

What not to do:

- do not bury warnings in muted-only details

## Bottom Status Bar

Purpose:

- show global mode and operational facts

User job:

- confirm editor mode, project, cursor facts, and system readiness

`ui_path` root:

```text
workbench.status
```

Layout role:

- bottom global strip

Sizing:

- height: 34-42px

Spacing:

- left/right padding: 14-18px
- fact gap: 20-28px

Typography:

- status text: 12-13px
- operational label: semibold when needed

Color tokens:

- `surface.status.background`
- `text.status.primary`
- `text.status.muted`
- `status.success`
- `status.warning`
- `status.danger`

Border/radius:

- no card radius
- top divider optional

Icon rules:

- system state uses icon plus text

Interaction states:

- normal
- warning
- danger

Overflow rules:

- status facts elide before wrapping
- no vertical growth

Accessibility:

- operational state available as text

Proof expectations:

- all critical facts fit at supported proof widths

What not to do:

- do not put primary editing controls in the status bar

## Acceptance

The grammar sheet is accepted when each major workbench surface has a complete
styling and behavior grammar that a host implementer can map into CSS, QSS, or
another toolkit without inventing local surface names.
