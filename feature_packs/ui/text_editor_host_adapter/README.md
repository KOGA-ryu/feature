# text_editor_host_adapter

Framework-neutral adapter models for the text editor action system.

This crate does not render Qt widgets, egui buttons, terminal menus, web
controls, or system clipboard calls. It turns `text_editor_actions` records and
outputs into stable host-facing data that those real hosts can render.

## What it owns

- host action rows/items
- enabled and disabled display state
- conservative hotkey labels by host profile
- execution result summaries
- clipboard transform receipt summaries

## What it does not own

- editor behavior
- cleanup behavior
- OS clipboard access
- host widget construction
- filesystem dialogs
- host-specific shortcut registration

## V1 policy

Only universal shortcuts are assigned by default:

- Linux desktop: `Ctrl+C`, `Ctrl+A`
- terminal: `Ctrl+Shift+C`
- macOS: `Cmd+C`, `Cmd+A`

Export and cleanup actions stay menu/palette driven until their shortcuts are
explicitly chosen. This avoids accidental collisions in terminal, browser, and
AI prompt hosts.
