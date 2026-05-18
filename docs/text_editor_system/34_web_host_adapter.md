# Web Host Adapter

## Purpose

Define how web apps and AI prompt boxes consume the text editor system.

The web host should render actions from metadata and keep browser-specific
clipboard, focus, and accessibility behavior outside the headless engine.

## Responsibilities

Web host adapter may own:

- DOM focus management
- browser clipboard API calls
- web hotkey profile
- command palette rendering
- toolbar/menu rendering
- responsive layout
- ARIA labels and keyboard navigation
- paste event interception

The headless engine owns:

- document state
- cursor/selection state
- pure edit commands
- pure copy/export strings
- pure cleanup transforms

## Boundaries

- Do not make headless crates depend on browser APIs.
- Do not use screenshots as the only proof of text behavior.
- Do not silently change hotkeys from the action registry.
- Do not auto-send AI prompts from an editor action in V1.

## User Access Pattern

- toolbar: copy/export, formatting, search
- command palette: full action set
- context menu: selection actions
- keyboard shortcuts: web profile with browser conflicts handled explicitly

## AI Prompt Box Notes

AI prompt box hosts should prioritize:

- copy as prompt block
- clean paste
- preserve indentation
- split long text into chunks
- estimate context cost when a future token estimator exists
- attach source metadata only when the user chooses it

## Tests

- action metadata renders to web controls
- disabled actions are not clickable
- browser clipboard failure reports an honest error
- paste cleanup preview does not mutate source until accepted
- keyboard shortcut conflict has a documented host override

## Acceptance

The web adapter is accepted when browser behavior stays in the host, text
behavior stays headless, and AI prompt helpers require explicit user action.
