# Terminal Host Adapter

## Purpose

Define how terminal-like hosts use the text editor system without confusing
terminal control behavior with document editing behavior.

Konsole/KDE is the first reference style where terminal behavior matters.

## Responsibilities

Terminal host adapter may own:

- terminal keybinding profile
- copy/paste integration with terminal selection
- transcript import
- ANSI/control sequence stripping
- prompt prefix cleanup options
- soft-wrap cleanup preview
- terminal-safe export actions

The headless engine owns:

- text state
- cursor and selection records
- copy/export string helpers
- cleanup transforms when they are pure and testable

## Boundaries

- Do not execute terminal commands.
- Do not inspect shell history unless the host explicitly provides text.
- Do not assume visual line wraps are real newlines.
- Do not make `Ctrl+C` mean copy in a terminal profile unless the host profile
  says it is safe.

## User Access Pattern

- command palette: clean transcript, copy prompt block, strip ANSI
- context menu: copy selection, clean selection
- toolbar: compact copy/export actions only if terminal host has one
- status strip: cleanup warnings and detected transcript properties

## Hotkey Notes

Terminal hosts have conflicts:

- `Ctrl+C` may interrupt a process
- `Ctrl+Z` may suspend a process
- `Ctrl+D` may send EOF
- `Ctrl+L` may clear screen

Terminal profiles must document any override before implementation.

## Tests

- ANSI stripping exact output
- box border cleanup exact output
- prompt prefix stripping only when configured
- soft-wrap unwrap exact output
- intentional blank lines preserved
- terminal hotkey conflicts represented in action metadata

## Acceptance

The terminal adapter is accepted when terminal cleanup is useful without
pretending the terminal is a normal desktop text box.
