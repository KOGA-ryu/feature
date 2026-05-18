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

## Host Choice Decision

The chosen shape is:

```text
Text editor core: Rust/headless feature crates
Neutral host model: text_editor_host_adapter
First desktop host: Qt
First Qt text surface: QPlainTextEdit
Future hosts: egui, terminal, web, and native platform adapters as needed
```

Why this exists:

- Qt already fits the Dex Home native desktop lane.
- `QPlainTextEdit` gives us practical editing behavior without custom rendering.
- Rust/headless crates keep behavior reusable outside Qt.
- The neutral host adapter proves action rendering and result summaries before
  any framework-specific adapter owns widgets.

Common failure mode:

```text
Qt button works -> behavior gets buried in clicked handler -> no other host can
reuse it.
```

The host contract exists to prevent that failure. Qt should render and call
`text.*` actions; it should not define the meaning of those actions.

## Host Options And Current Policy

| Host | Use | Current policy |
| --- | --- | --- |
| Qt + `QPlainTextEdit` | first native proof surface | first graphical host |
| Qt + `QTextEdit` | rich text | defer unless rich text becomes the target |
| Custom Qt widget | full editor rendering | defer until Track A engine work |
| egui | Rust-native demos/tools | later adapter |
| terminal TUI | keyboard-heavy helper | later adapter after conflict review |
| web/CodeMirror/Monaco | browser or webview editor | later adapter |
| native macOS/AppKit | Mac-only polish | not first because portability matters |

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
- typed host placement metadata controls toolbar/menu/context placement

The first implemented host layer is `ui.text_editor_host_adapter`. It is not a
Qt, egui, terminal, or web implementation. It produces framework-neutral action
items and action results:

```text
HostActionItem
HostActionResult
ClipboardReceiptSummary
```

Framework-specific hosts should consume those shapes instead of reinterpreting
raw action records independently.

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
- Do not invent local placement names outside the action registry vocabulary.
- Do not turn the neutral host adapter into a widget toolkit.
- Do not put text cleanup, OS clipboard calls, or file dialogs in the neutral
  host adapter.
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
