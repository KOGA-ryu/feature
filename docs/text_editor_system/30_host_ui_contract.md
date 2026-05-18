# Host UI Contract

## Purpose

Define how hosts present editor actions without inventing behavior locally.

A host is any UI or shell that exposes editor actions:

- Qt desktop app
- egui demo app
- terminal helper
- web prompt box
- future Dex/Home binder surface

## Why This Exists

Hosts are tempting places to add behavior because buttons and menus are visible.
That is dangerous. If a Qt button implements copy prompt block directly, the
terminal host, web host, command palette, and tests will not share the same
behavior.

The host contract keeps hosts thin. Hosts render actions, bind inputs, call
action execution, and display results.

## Regular Application

Use this contract when building:

- toolbar buttons
- context menus
- command palette entries
- keyboard shortcuts
- right-context action buttons
- status or warning displays
- clipboard bridges
- file dialogs

## Reference Behavior

A host receives action metadata and editor state, then renders the command.

Example:

```text
action: text.copy_prompt_block
host: Qt
render: QAction + optional toolbar button + command palette row
execute: call action/helper and place output on clipboard if host owns clipboard
```

The host does not rebuild the prompt block formatting itself.

## Chosen Default

First graphical proof host:

```text
Qt
```

Default host behavior:

- render from action metadata
- use Linux desktop hotkey profile first
- show unsupported actions as hidden or disabled
- mark unproven host-specific behavior as `needs verification`

## Why This Default Was Chosen

Qt is the first native desktop proof target for this project family. It gives a
real graphical UI surface without forcing the feature library to become a full
editor engine.

Linux desktop hotkeys fit Qt proof better than terminal shortcuts because they
avoid shell process-control conflicts.

## Host Responsibilities

Hosts own:

- rendering toolbar buttons
- rendering command palette entries
- rendering menu and context menu items
- binding hotkeys from the selected profile
- mapping icon names to real icon assets
- showing disabled states
- showing disabled reasons where practical
- calling action execution or helper functions
- handling system clipboard APIs
- handling filesystem dialogs
- handling UI focus
- showing host-specific errors

## Engine And Action Responsibilities

Headless crates own:

- document state
- text helper behavior
- output strings
- validation results
- undo/redo behavior when implemented
- action metadata
- enabled rule names
- input/output contracts

The host should not know the internal formatting rules for a helper.

## How Hosts Render Actions

Every host adapter should render from action records:

- `label` becomes menu and command palette text
- `short_label` becomes compact button text where needed
- `icon` maps to host icon library
- `tooltip` becomes hover/help text
- `default_hotkeys` maps to host shortcuts
- `enabled_rule` controls enabled state
- `disabled_reason` explains disabled state when possible
- `ui_placement` controls toolbar/menu/context placement

## Disabled Actions

Disabled behavior must be honest.

Allowed disabled states:

- disabled because document is empty
- disabled because no selection exists
- disabled because document is read-only
- disabled because host clipboard is unavailable
- disabled because host behavior is not implemented
- hidden because action is unsupported in this host

Do not show a working-looking button that silently does nothing.

## Clipboard Boundary

Headless helpers may produce output strings.

Hosts own system clipboard calls.

Example:

```text
text_editor_plain: produce exact prompt block string
text_editor_host_qt: put that string onto QClipboard
```

This keeps tests deterministic and avoids system side effects in headless crates.

## File Boundary

Headless helpers do not open file dialogs.

Hosts own:

- open dialogs
- save dialogs
- path picking
- filesystem permissions
- platform-specific file errors

Headless crates may operate on text provided by the host.

## Placement Rules

- Common actions may appear in toolbar.
- All actions should appear in command palette.
- Selection actions may appear in context menu.
- Host-specific status belongs in footer/status bar.
- Workflow-specific actions may appear in a right context panel.

## What Not To Do

- Do not hardcode action labels in host code.
- Do not hardcode icons in host code.
- Do not implement formatting behavior in host widgets.
- Do not assign host shortcuts outside hotkey profiles.
- Do not call system clipboard APIs from headless crates.
- Do not make unsupported actions look enabled.
- Do not let a host silently reinterpret an action.

## Common Failure Mode

The common failure is building a useful UI button first and treating the action
contract as paperwork later. That reverses the architecture. The action
metadata comes first; the host renders it.

## Examples

Good Qt flow:

```text
read action record
create QAction(label)
assign shortcut from linux_desktop profile
assign icon from icon name
connect triggered signal to action execution
copy output to QClipboard only in host layer
```

Bad Qt flow:

```text
create QPushButton("Prompt")
build prompt block formatting inside clicked handler
invent Ctrl+Alt+P shortcut locally
skip disabled state
```

## Proof A Host Adapter Must Provide

A host adapter is not accepted until it proves:

- actions render from metadata
- labels/tooltips/icons come from action records
- hotkeys come from selected profile
- disabled actions are disabled or hidden honestly
- headless tests prove behavior
- host screenshots prove layout only
- no host-local duplicate command semantics exist

## Tests

Host contract tests should verify:

- action record maps to host action object
- disabled rule maps to disabled state
- missing icon mapping falls back to text label
- unsupported action is hidden or disabled
- host calls action execution rather than local formatting logic

## Questions Intentionally Deferred

- Exact Qt class wrappers.
- egui adapter implementation shape.
- Terminal UI proof format.
- Web component framework.
- User-customizable toolbar layout.

## Acceptance

A host adapter is valid when it can render editor actions from metadata instead
of hardcoding labels, icons, tooltips, shortcuts, and behavior locally.
