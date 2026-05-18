# Clipboard And Export

## Purpose

Produce copy-safe text for humans, terminals, markdown docs, and AI prompts.

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
plain_text()
selected_or_all_text()
markdown_block(language)
prompt_block()
line_text(index)
current_line_text()
```

## Tests

- no extra spaces
- newlines preserved
- markdown fences correct
- empty language works
- selected text and full document output are distinct
