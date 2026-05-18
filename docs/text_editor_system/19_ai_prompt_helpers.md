# AI Prompt Helpers

## Purpose

Prepare text for Codex, Spark, ChatGPT, and binder work orders.

## Actions

- copy prompt block
- make work order from selection
- make receipt block from selection
- split prompt into chunks
- estimate token/context cost
- add source metadata
- strip accidental terminal prompts
- preserve code indentation

## User Access Pattern

- command palette first
- toolbar buttons for Copy Prompt and Clean Paste
- context menu for selected text

## Tests

- prompt blocks preserve exact content
- metadata is explicit
- chunks do not split inside fenced code blocks when avoidable
- no hidden facts are invented
