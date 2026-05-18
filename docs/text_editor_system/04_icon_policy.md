# Icon Policy

## Purpose

Define stable icon names and placement rules for text editor actions.

Icons are contracts. Host adapters map icon names to actual libraries.

## Why This Exists

Without an icon policy, UI work turns into sticker soup:

- one host uses a hand-drawn copy icon
- another uses a text button
- another uses a different name
- accessibility labels are forgotten
- command palettes and toolbars stop matching

The icon policy keeps visual commands consistent without making the headless
editor depend on any UI toolkit.

## Regular Application

Use icon names for:

- toolbar buttons
- compact context actions
- command palette rows
- right-context action buttons
- status indicators
- future host adapters

## Reference Behavior

Action metadata declares:

```text
icon: clipboard-copy
```

Qt, egui, web, or another host maps `clipboard-copy` to its own icon library.
The action contract does not embed SVG, pixmaps, or toolkit-specific resources.

## Chosen Default

Default icon source:

```text
stable semantic icon names
```

Preferred icon style for future hosts:

- use host/library icons where possible
- prefer common names compatible with Lucide-style naming
- avoid custom drawings unless a host has no suitable icon

## Why This Default Was Chosen

Stable semantic names let actions move across hosts. The action should say
`copy`, not "use this exact Qt resource path." The host is responsible for
mapping that name to a real visual asset.

This keeps the action registry reusable.

## Core Icon Names

| Action family | Icon name |
| --- | --- |
| copy plain | `copy` |
| copy prompt block | `clipboard-copy` |
| copy markdown block | `braces` |
| copy code fence | `code` |
| cut | `scissors` |
| paste | `clipboard-paste` |
| clean paste | `wand-sparkles` |
| select all | `scan-text` |
| undo | `undo-2` |
| redo | `redo-2` |
| search | `search` |
| replace | `replace` |
| terminal cleanup | `terminal` |
| redaction | `key-round` |
| warning | `triangle-alert` |
| validation | `shield-check` |
| save draft | `save` |
| outline | `list-tree` |
| command palette | `command` |

## Placement Rules

### Toolbar

Use toolbar buttons for frequent actions:

- copy plain
- copy prompt block
- clean paste
- search
- undo/redo when editing is active

Toolbar icons must have tooltips and accessible names.

### Menu

Menus can carry more actions than toolbars.

Use menus for:

- all common editor actions
- actions with shortcuts
- grouped clipboard/export variants
- validation and cleanup commands

### Command Palette

Every action should be available in the command palette.

The command palette should show:

- label
- short description or tooltip
- shortcut when assigned
- category
- disabled state when relevant

### Context Menu

Context menus should be selection-aware.

Use context menus for:

- copy selection
- copy prompt block
- format selection
- clean selected pasted text
- search selected text

### Right Context Panel

Use right-context action buttons for workflow-specific actions such as:

- copy as prompt block
- make work order from selection
- attach selected text as receipt

These actions still come from action metadata.

## Icon-Only Buttons

Icon-only buttons are allowed only when:

- the action is common or clearly represented
- the button has a tooltip
- the button has an accessible label
- the same action is available through command palette or menu

If any of those are missing, use text or icon plus text.

## Accessibility Requirements

Every visible icon action needs:

- action label
- tooltip
- accessible name
- disabled reason where possible

The icon is not the label.

## What Not To Do

- Do not hand-draw icons per host.
- Do not bake icon paths into headless crates.
- Do not invent new icon names in host code.
- Do not use icon-only controls for obscure commands.
- Do not rely on color alone to explain risk or disabled state.
- Do not hide command meaning behind decorative art.

## Common Failure Mode

The common failure is treating icons as decoration instead of command metadata.
That makes UI pretty but brittle. A button should render from the same action
record as the menu and hotkey.

## Examples

Good action metadata:

```text
action_id: text.copy_prompt_block
label: Copy Prompt Block
short_label: Prompt
icon: clipboard-copy
tooltip: Copy text as a prompt-safe block.
ui_placement: command_palette, clipboard_menu, optional_toolbar
```

Bad action metadata:

```text
action_id: text.copy_prompt_block
icon: shiny_prompt_button
tooltip: does it
```

## Tests

Icon policy tests should verify:

- toolbar actions have icon names
- icon-only actions have tooltips
- command palette can render every action without icon dependency
- host adapters use icon names from action metadata
- missing icon mapping degrades to text label instead of breaking behavior

## Questions Intentionally Deferred

- Exact icon library per host.
- Theme-specific icon variants.
- Custom product icon design.
- Animated icons.
- User-configurable toolbar layout.

## Acceptance

Icon policy is accepted when hosts can render buttons, menus, command palette
rows, and context actions from stable action metadata without inventing local
labels or icon names.
