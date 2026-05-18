# Action Registry Contract

## Purpose

Define the shared command/action contract for every text editor feature.

Every editor command exposed to a UI, hotkey, command palette, test, or future
agent call must have an action record. The action record is the stable shape
that lets one behavior render in Qt, egui, terminal, web, and future hosts
without each host inventing its own command.

## Why This Exists

Without an action registry, editor features become duplicated UI code:

- a toolbar button calls one function
- a context menu calls another
- a hotkey calls a third
- a test bypasses all of them

That is how behavior drifts. The registry makes the action the source of truth.
Hosts render the action; they do not define the action.

## Regular Application

Use an action record for:

- copy/export actions
- paste cleanup actions
- selection actions
- formatting actions
- search/replace actions
- validation actions
- host-only actions that still need labels, icons, hotkeys, and disabled state

The first implementation slice uses this contract as a naming and behavior
guide even before `text_editor_actions` exists as a crate.

## Reference Behavior

Reference behavior is the action's promise in plain language.

For example:

```text
text.copy_prompt_block copies selected text, or the full document when there is
no selection, as a prompt-safe text block. It does not mutate editor text.
```

The reference behavior must be stable enough that a host can call the action
without knowing how the output is built internally.

## Chosen Default

Defaults for this package:

- action IDs use the `text.*` namespace
- Linux desktop editor is the default hotkey profile
- Qt is the first graphical proof host
- terminal and web proof come later
- host-specific behavior not proven in a running host is marked
  `needs verification`

## Why This Default Was Chosen

The `text.*` namespace is short, user-readable, and not tied to a crate name.
That matters because `text.copy_prompt_block` may start with a
`text_editor_plain` helper, later route through `text_editor_actions`, and then
be rendered by Qt or web. The action name should survive those moves.

Linux desktop is the default hotkey profile because this project is being built
for local app work first and avoids terminal keybinding conflicts by default.

Qt is the first graphical proof host because Dex/Home-style app work is native
desktop oriented. Terminal and web behavior stay important, but they are more
conflict-heavy and should follow the action contract.

## Golden Action Template

Every user-visible action should be documented with this shape:

```text
action_id:
label:
short_label:
category:
purpose:
regular_application:
reference_behavior:
hotkeys_by_host:
icon:
tooltip:
ui_placement:
api_shape:
input:
output:
undo_behavior:
enabled_rule:
disabled_reason:
tests:
acceptance:
```

## Runtime Metadata Shape

The eventual `text_editor_actions` crate should expose a compact runtime record
equivalent to:

```text
id
label
short_label
category
icon
tooltip
default_hotkeys
enabled_when
disabled_reason
input_shape
output_shape
undo_behavior
host_placements
tests
```

Do not treat this as final Rust syntax yet. It is the required information
contract.

## Object Sheet Pattern

Every reusable editor object should be documented before it is implemented.

Use this shape:

```text
object:
kind:
purpose:
why_this_exists:
owned_by:
used_by:
functions:
inputs:
outputs:
state_changed:
undo_behavior:
host_access:
tests:
acceptance:
decision_notes:
```

This keeps the feature library from becoming a pile of functions with unclear
ownership. The object sheet says what the thing is, what it owns, which actions
it exposes, and which hosts are allowed to do with it.

Example object:

```text
object: TextActionRegistry
kind: action metadata and dispatch boundary
purpose: expose editor commands through stable action records
owned_by: text_editor_actions
used_by: Qt host, egui host, command palette, tests, future AI packets
state_changed: none except when an action explicitly routes to a mutating editor command
```

## Required Fields

### `action_id`

Stable command identifier.

Rules:

- use `text.*`
- use lower snake case after the namespace
- name behavior, not implementation
- examples: `text.copy_plain`, `text.copy_prompt_block`

### `label` And `short_label`

`label` is the human-facing command name.

`short_label` is for tight UI surfaces such as toolbar buttons.

Example:

```text
label: Copy Prompt Block
short_label: Prompt
```

### `category`

Used for command palette grouping and menu placement.

Initial categories:

- `selection`
- `clipboard`
- `cleanup`
- `format`
- `search`
- `navigation`
- `view`
- `validation`
- `host`

### `icon`

Stable icon contract name. The host adapter maps it to a real icon library.

Example:

```text
icon: clipboard-copy
```

### `tooltip`

Short explanation for hover text and accessibility.

Good tooltip:

```text
Copy selected text, or the full document, as a prompt-safe block.
```

Bad tooltip:

```text
Does copy stuff.
```

### `hotkeys_by_host`

Host-aware shortcut map.

Example:

```text
linux_desktop: Ctrl+Shift+C
linux_terminal: needs verification
macos_text: Cmd+Shift+C
web_app: needs verification
ai_prompt_box: needs verification
```

### `enabled_rule`

Named rule that says when the action can run.

Examples:

- `always`
- `document_has_text`
- `selection_has_text`
- `document_is_editable`
- `search_query_has_text`
- `host_supports_clipboard`

### `undo_behavior`

Required for every action.

Allowed values:

- `none`
- `creates_undo_step`
- `joins_typing_group`
- `selection_only`
- `host_only`

Clipboard/export actions use `none`.

### `input` And `output`

Input and output describe the action boundary.

Examples:

```text
input: editor_state
output: exact plain text
```

```text
input: selected_text_or_all + format_options
output: formatted prompt block
```

## First V1 Action Object

The first implementation object is:

```text
object: TextActionRegistry
kind: headless action catalog and execution boundary
owned_by: text_editor_actions
depends_on: text_editor_plain
host_dependency: none
```

It starts with these actions only:

```text
text.copy_plain
text.copy_markdown_block
text.copy_prompt_block
text.copy_code_fence
text.select_all
text.current_line_text
text.line_range_text
text.trim_trailing_whitespace
text.clean_basic
text.normalize_line_endings
text.strip_ansi_escape_codes
```

Why these actions first:

- they are useful immediately for prompt drafting, specs, notes, and receipts
- they exercise metadata, hotkeys, icons, enabled rules, and output contracts
- most are pure export actions, so they are easy to test exactly
- `text.select_all` proves the registry can route a controlled mutating command
  without becoming a UI toolkit

What this object must not do:

- call the system clipboard
- create Qt, egui, terminal, or web widgets
- own file I/O
- invent new text-buffer behavior already owned by `text_editor_plain`
- trigger cloud AI, local AI, or background automation

Future AI automation may call these action IDs, but V1 action execution remains
local, explicit, and deterministic.

## First V1 Runtime Shape

The first Rust shape should stay boring and inspectable:

```text
TextActionId
TextActionRecord
TextActionInput
TextActionOutput
TextEnabledRule
TextUndoBehavior
TextHostPlacement
ClipboardTransformResult
all_text_actions()
parse_text_action_id()
execute_text_action()
```

The registry should prefer deterministic enum dispatch over dynamic plugins in
V1. Plugin registration, user-defined actions, macros, and AI-composed actions
come later.

Recommended V1 output rule:

```text
simple copy/export/line/cleanup actions return Text(String)
receipt-aware clipboard policy actions return ClipboardTransformResult
select_all returns None and changes selection only
disabled actions return Disabled with an action id and reason
```

The reason to return disabled output instead of panicking is simple: hosts and
future agent calls need honest feedback when an action cannot run.

## Examples

These examples show the expected level of detail for concrete actions.

## Example Actions

### Copy Plain

```text
action_id: text.copy_plain
label: Copy Plain
short_label: Copy
category: clipboard
purpose: produce exact plain text from the selection or full document
regular_application: move text into another app without formatting damage
reference_behavior: selected text is copied when present; otherwise full document text is copied
hotkeys_by_host: linux_desktop=Ctrl+C, macos_text=Cmd+C, linux_terminal=needs verification
icon: copy
tooltip: Copy selected text, or the full document when nothing is selected.
ui_placement: clipboard menu, command palette, context menu, optional toolbar
api_shape: pure output action
input: editor_state
output: exact plain text
undo_behavior: none
enabled_rule: document_has_text
disabled_reason: document is empty
tests: selected output, full-doc fallback, empty document, newline preservation
acceptance: output exactly matches expected text and editor state is unchanged
```

### Copy Prompt Block

```text
action_id: text.copy_prompt_block
label: Copy Prompt Block
short_label: Prompt
category: clipboard
purpose: copy selected text or full document as a prompt-safe block
regular_application: prepare clean Codex/Spark prompts from notes, code, or terminal output
reference_behavior: selected text is preferred; full document is used when no selection exists
hotkeys_by_host: linux_desktop=unassigned, macos_text=unassigned, terminal=needs verification
icon: clipboard-copy
tooltip: Copy text as a prompt-safe block.
ui_placement: command palette, clipboard menu, optional toolbar
api_shape: pure helper called through action execution
input: selected_text_or_all
output: formatted clipboard text
undo_behavior: none
enabled_rule: document_has_text
disabled_reason: document is empty
tests: exact output, empty selection fallback, newline preservation
acceptance: UI can call the action without knowing formatting internals
```

## Host Placement Metadata

Every action should declare where it may appear.

V1 uses a typed placement vocabulary instead of raw strings:

```text
command_palette
clipboard_menu
context_menu
toolbar
edit_menu
cleanup_menu
```

Why this is typed:

- host adapters should not drift between names like `toolbar`, `tool_bar`, and
  `top_toolbar`
- tests can prove exact placement without string spelling mistakes
- future hosts can map one stable enum to their own UI primitives

Default rule:

- every action appears in command palette
- common actions may appear in toolbar
- selection-sensitive actions may appear in context menu
- status-only actions do not mutate text

## What Not To Do

- Do not use crate names as action IDs.
- Do not let Qt define one action name and egui define another.
- Do not hardcode labels, icons, or tooltips in host code.
- Do not omit undo behavior.
- Do not say an action is enabled without a named enabled rule.
- Do not hide host-specific conflicts by pretending one shortcut works
  everywhere.
- Do not make clipboard/export actions mutate editor text.

## Common Failure Mode

The most common failure is building a useful helper and then wiring it directly
to one UI button. That makes the helper work once, but it prevents command
palette use, tests, hotkeys, alternate hosts, and future agent calls from using
the same behavior.

The action must sit between helper and host.

## Test Requirements

Action tests should prove:

- action ID is stable
- label and tooltip are present
- icon name is present when toolbar placement exists
- enabled rule works for empty and non-empty state
- undo behavior is declared
- output shape matches the helper contract
- host placement metadata is present

For V1, tests may be simple record/fixture tests. Runtime dispatch comes later.

## Questions Intentionally Deferred

- Final Rust struct names.
- Serialization format for action records.
- Whether action dispatch uses traits, enums, or function tables.
- How agent-callable actions are exposed.
- How host plugins register extra actions.

## Acceptance

This contract is accepted when another builder can define an action without
deciding naming, metadata, hotkey profile shape, UI placement, enabled rules,
undo behavior, or test expectations.
