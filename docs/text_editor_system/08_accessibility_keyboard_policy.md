# Accessibility Keyboard Policy

## Purpose

Make text editor features usable without a mouse and renderable by host UIs with
clear labels, focus behavior, and keyboard access.

Accessibility is part of the action contract, not polish added later.

## Core Rules

- Every user-visible action needs a label.
- Icon-only buttons need tooltips or accessible names.
- Keyboard-only use must be possible for core editing actions.
- Disabled actions should explain why when the host supports it.
- Focus movement must be predictable.
- Text should remain readable at host-supported zoom/font sizes.

## Keyboard Profiles

Hotkeys are defined by host profile:

- Linux terminal
- Linux desktop editor
- macOS desktop editor
- AI prompt box
- web app
- game overlay

If a hotkey conflicts with the host, the host profile owns the override.

## Focus Rules

- Editor text surface owns typing focus during editing.
- Toolbars and panels must return focus to the editor after action execution
  when appropriate.
- Modal or popover actions must define escape/cancel behavior.
- Search boxes must define how focus returns to the editor.

## Screen Reader And Labels

Action metadata should provide:

- label
- short label
- tooltip
- accessible description when tooltip is not enough
- disabled reason where practical

## Button And Menu Placement

- Core actions belong in menus and command palette.
- Frequently used actions may appear as toolbar buttons.
- Advanced actions should default to command palette until proven common.
- No host should invent a label that differs from the action registry.

## Tests

- action metadata includes label and tooltip
- host adapter maps labels to buttons/actions
- disabled action exposes disabled state
- keyboard shortcut collisions are represented by host profile
- search focus return behavior is specified before implementation

## Acceptance

Accessibility is accepted when a keyboard-only user can reach the core feature
set and every visible command has a stable label, tooltip, and host profile
binding.
