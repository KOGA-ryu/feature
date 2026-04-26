# ui.theme_editor

`ui.theme_editor` is the visual-state half of the theme customization system.

It owns:

- preset switching
- editable theme config state
- live preview model
- import/export payload handling
- validation surface

It does not own color derivation. All token math stays in
`logic.theme_token_generator`.

## Why It Exists

The editor lets a user change a small, meaningful theme config without exposing
every component to raw color overrides. It is built to preview the same token
contract that production UI components will consume.

## Current Scope

- preset application for dark, light, low-contrast, and high-accent themes
- config setters for mode, accent, background, foreground, contrast, sidebar
  translucency, and fonts
- live preview model generated from semantic tokens
- import/export of JSON theme configs
- validation findings surfaced from the token generator

## Non-Goals

- no real settings persistence yet
- no clipboard integration yet
- no egui widget rendering inside the feature crate itself
