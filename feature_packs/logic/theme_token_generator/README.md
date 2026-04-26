# logic.theme_token_generator

`logic.theme_token_generator` turns a compact `ThemeConfig` into a deterministic
set of semantic `ThemeTokens`.

It is the logic seam behind a future `ui.theme_editor` feature. The user edits a
small set of meaningful inputs:

- theme mode
- accent
- background
- foreground
- contrast
- translucent sidebar
- font names

The generator validates those inputs, derives surface/text/interaction tokens,
and emits findings when the configuration is malformed or low-contrast.

## Why It Exists

Custom color systems fail when components own their own hex values. This feature
keeps all derivation in one place so components consume semantic tokens instead
of inventing styles locally.

## Current Scope

- parses compact theme fixtures
- validates color strings
- derives semantic tokens for surfaces, borders, text, and interactions
- warns on low-contrast foreground/background pairs
- keeps sidebar translucency isolated to sidebar surfaces

## Non-Goals

- no UI editor yet
- no app-wide persistence layer
- no semantic theme search or preset marketplace
