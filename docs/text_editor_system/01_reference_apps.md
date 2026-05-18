# Reference Apps

## Priority

1. Konsole and KDE terminal behavior for terminal-first workflows.
2. Kate/KWrite for KDE-native editor conventions.
3. VS Code and Sublime Text for command palette, multi-selection, and action
   discoverability.
4. ChatGPT/Codex prompt boxes where behavior is directly observed.
5. macOS standard text behavior for Mac host fallback.

## What To Copy

- Familiar shortcut grammar.
- Discoverable menu and command-palette labels.
- Conservative default buttons.
- Selection and clipboard behavior users already trust.
- Host-specific shortcut profiles instead of one universal shortcut table.

## What Not To Copy Blindly

- Terminal shortcuts that conflict with shell control keys.
- AI prompt shortcuts that change by product version.
- UI-only behavior that cannot be proven in the headless engine.
- Rich-text features before plain-text contracts are stable.

## Reference Capture Rule

Every reference claim should eventually be backed by one of:

- app documentation
- local observed behavior note
- screenshot or receipt
- test fixture that recreates the behavior
