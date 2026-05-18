# egui Host Adapter

## Purpose

Connect the editor feature system to `feature_lab_ui` and other egui hosts.

## Responsibilities

- render a plain text editing surface
- render toolbar action buttons
- bind egui keyboard input to action ids
- show action tooltips
- show test/proof output where useful

## Boundaries

- Do not make egui the document source of truth.
- Do not duplicate action labels or shortcut policy in UI code.
- Keep proof terminal-first unless the task specifically depends on UI behavior.

## Tests

- host can load sample editor state
- toolbar actions call registry ids
- keyboard profile maps expected shortcuts
- no host action mutates text outside the engine command path
