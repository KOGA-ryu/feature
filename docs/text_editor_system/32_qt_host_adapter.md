# Qt Host Adapter

## Purpose

Prepare for native Qt apps that need the same editor system.

Qt is the first graphical proof host. It should make editor actions visible and
pleasant without becoming the owner of text-editor behavior.

## Chosen Surface

Use `QPlainTextEdit` first.

Why:

- it is a real native plain-text editing widget
- it handles typing, selection, scrolling, focus, and platform input behavior
- it fits exact text, Markdown, prompt blocks, logs, and code-ish text better
  than rich-text widgets
- it lets us prove action metadata, shortcuts, menus, and clipboard bridges
  before building a custom editor engine

Do not start with a custom Qt text widget. A custom widget would require us to
own rendering, cursor math, hit testing, IME behavior, selection painting,
accessibility, scroll behavior, and font metrics before the action system has
earned that cost.

Do not start with `QTextEdit` unless the feature goal becomes rich text. This
package is currently optimized for exact plain text and predictable export.

## Responsibilities

- map action ids to Qt actions/buttons
- map icon names to Qt icon resources
- map shortcut profiles to `QShortcut` or `QAction`
- keep clipboard/filesystem logic in the host layer
- map `text.*` records into `QAction` objects
- map toolbar/context-menu/command-palette placement from metadata
- place export strings onto `QClipboard` only in the Qt layer
- show disabled actions honestly

## Boundaries

- Do not port the editor engine into Qt-specific logic.
- Do not hardcode labels or hotkeys in every widget.
- Do not add Qt host work until the headless action registry is stable.
- Do not format prompt blocks in a Qt click handler.
- Do not call Rust/headless helpers through multiple local button-specific
  wrappers.
- Do not add cloud AI, prediction, or background automation to the Qt adapter in
  V1.

## V1 Mapping Target

The first Qt adapter should be able to render these actions from metadata:

```text
text.copy_plain
text.copy_markdown_block
text.copy_prompt_block
text.select_all
text.current_line_text
text.line_range_text
text.trim_trailing_whitespace
```

Expected mapping:

```text
TextActionRecord.label -> QAction text
TextActionRecord.tooltip -> QAction tooltip/status tip
TextActionRecord.icon -> host icon lookup
TextHostPlacement -> menu/toolbar/context placement
hotkey profile -> QAction shortcut
enabled rule -> QAction enabled state
execute_text_action -> action trigger behavior
Text(String) output -> QClipboard when the action is a copy/export command
```

If an icon is missing, the host may render a text-only action. It must not
invent a different icon contract name.

## Acceptance

The Qt adapter is valid when a screenshot can prove the actions are visible and
host tests can prove the `QAction` labels, tooltips, shortcuts, and disabled
states are derived from action records.

## Tests

- action metadata round-trips into Qt action properties
- disabled state is visible
- copy/export returns exact strings from the engine/clipboard layer
- no prompt-block formatting exists in Qt code
