# Clipboard And Export

## Purpose

Produce copy-safe text for humans, terminals, markdown docs, and AI prompts.

Clipboard/export policy is intentionally conservative:

```text
Exact by default.
Cleanup only by named action.
Report every cleanup.
Never mutate editor text from clipboard helpers.
Never touch the system clipboard from headless crates.
Keep prompt block format compatible with current contract tests.
```

The clipboard layer produces strings and transform reports. Host adapters own
the real OS clipboard.

## Why This Exists

Copy/export is where text often gets damaged silently:

- code indentation gets changed
- terminal escape codes leak into prompts
- soft-wrapped terminal output gains fake newlines
- prompt packets drift between formats
- a UI button formats text differently than a command-palette action

This document prevents that drift. Exact copy stays exact. Cleanup is explicit
and leaves a receipt.

## Ownership Boundary

```text
text_editor_plain
  owns raw text, selection, line helpers, and the current simple prompt block

text_editor_actions
  owns action identity and routing

text_editor_clipboard
  owns explicit export/cleanup policy variants and transform reports

host_qt / host_egui / host_terminal / host_web
  own real system clipboard access
```

Important rule:

```text
text_editor_clipboard must not create a second default meaning for
text.copy_prompt_block.
```

It may wrap the existing prompt block behavior with options or add explicitly
named variants, but it must not produce a competing default.

## Actions

- copy plain
- copy selection plain
- copy full document plain
- copy markdown block
- copy prompt block
- copy with line numbers
- copy without line numbers
- export selected text
- export full document

V1 action intent:

```text
copy_exact
copy_markdown_block
copy_prompt_block
copy_code_fence
clean_basic
```

Avoid vague names such as `clean_text`. Every cleanup action should say what it
does or what policy it follows.

## Prompt Block Format Policy

Current default prompt block format is Markdown-native:

````text
Source: notes/session.md

```text
selected or full document text
```
````

Missing or blank source becomes:

````text
Source: unknown

```text
selected or full document text
```
````

Decision:

```text
Keep this as the V1 default because it is already tested, human-readable, and
standard Markdown.
```

Do not replace this default with a lowercase `source/content` packet in V1.
That can be added later only as an explicit style, for example:

```text
PromptBlockStyle::MarkdownHuman
PromptBlockStyle::Contract
PromptBlockStyle::YamlPacket
```

If content contains triple backticks, V1 may keep current behavior. Fence-length
escalation is a later hardening slice.

## Transform Result Shape

Any cleanup or policy transform should return a receipt:

```text
ClipboardTransformResult:
  text
  changes[]
  warnings[]
```

Suggested change shape:

```text
ClipboardChange:
  change_id
  description
  count
```

Suggested warning shape:

```text
ClipboardWarning:
  warning_id
  message
  severity
```

Why this exists:

```text
If a helper changes text, the caller should be able to explain what changed.
```

## User Access Pattern

- toolbar: Copy Plain, Copy Markdown, Copy Prompt
- context menu: selection-sensitive copy actions
- command palette: every export action

## Default Hotkeys

- Copy Plain: standard host copy shortcut.
- Copy Markdown Block: command palette first; optional `Ctrl+Alt+C` / `Cmd+Option+C`.
- Copy Prompt Block: command palette first to avoid submit conflicts.

## Headless API

First useful helpers:

```text
copy_exact(input)
copy_markdown_block(input, language)
copy_prompt_block(input, source)
copy_code_fence(input, language)
clean_basic(input, policy)
```

V1 helpers return strings or `ClipboardTransformResult`. They do not mutate the
editor and do not call the OS clipboard.

## What Not To Do

- Do not auto-clean text during exact copy.
- Do not auto-redact secrets in V1.
- Do not unwrap soft wraps in V1.
- Do not strip shell prompts unless the action explicitly says so.
- Do not duplicate `text_editor_plain` prompt block behavior under the same
  action meaning.
- Do not call the system clipboard from `text_editor_clipboard`.

## Tests

- no extra spaces
- newlines preserved
- markdown fences correct
- empty language works
- selected text and full document output are distinct
- prompt block matches current `text_editor_plain` contract tests
- cleanup transforms return change receipts
- exact copy returns no cleanup changes

## Acceptance

This document is accepted when `text_editor_clipboard V1` can be implemented
without deciding:

- whether exact copy silently cleans text
- whether cleanup mutates editor state
- whether prompt block format changes
- whether transform receipts are required
- whether the headless crate may touch the OS clipboard
