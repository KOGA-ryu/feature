# Hotkey Profiles

## Purpose

Define how text editor actions map to keyboard shortcuts across different
hosts.

The action ID stays stable. The shortcut changes by environment.

## Why This Exists

Keyboard shortcuts are one of the fastest places for editor behavior to drift.
`Ctrl+C` means copy in a desktop editor, but in a terminal it may interrupt a
running process. `Cmd+A` is normal on macOS but not on Linux. Browser apps have
their own reserved shortcuts.

Hotkey profiles keep the action stable while allowing each host to use safe
shortcuts.

## Regular Application

Use profiles when rendering:

- menu accelerators
- toolbar shortcut hints
- command palette shortcut labels
- context menu shortcuts
- tests for action metadata

## Reference Behavior

Reference behavior:

```text
action_id: text.copy_plain
linux_desktop: Ctrl+C
macos_text: Cmd+C
linux_terminal: Ctrl+Shift+C or needs verification
web_app: browser-safe mapping or needs verification
```

The same action runs. Only the shortcut changes.

## Chosen Default

Default profile:

```text
linux_desktop
```

## Why This Default Was Chosen

Linux desktop editor behavior is the safest default for this project because it
matches KDE/Kate-like editing expectations without terminal process-control
conflicts. It also keeps Qt desktop proof straightforward.

Terminal, web, macOS, AI prompt, and game-overlay profiles are overrides, not
the base truth.

## Profile Names

| Profile | Use |
| --- | --- |
| `linux_desktop` | KDE/Kate-like desktop text editing and first Qt proof |
| `linux_terminal` | terminal-hosted editor or Konsole-like shell |
| `macos_text` | native macOS text fields and desktop apps |
| `ai_prompt_box` | ChatGPT/Codex-like prompt input |
| `web_app` | browser-hosted app |
| `game_overlay` | game or creative-tool overlay where shortcuts may conflict |

## Profile Rules

### Linux Desktop

Default bias:

- use `Ctrl`
- use familiar editor shortcuts first
- reserve `Alt` and function keys for host-specific behavior

Examples:

- select all: `Ctrl+A`
- copy: `Ctrl+C`
- paste: `Ctrl+V`
- undo: `Ctrl+Z`
- redo: `Ctrl+Shift+Z`
- find: `Ctrl+F`

### Linux Terminal

Terminal is conflict-heavy.

Avoid assuming:

- `Ctrl+C`
- `Ctrl+Z`
- `Ctrl+D`
- `Ctrl+L`
- `Ctrl+A`
- `Ctrl+E`

These may mean interrupt, suspend, EOF, clear screen, line start, or line end.

Terminal shortcuts should be `needs verification` until tested in the target
terminal host. Konsole-style copy/paste often uses `Ctrl+Shift+C` and
`Ctrl+Shift+V`, but this still needs host verification before becoming a
contract.

### macOS Text

Default bias:

- use `Cmd`
- preserve common native text expectations

Examples:

- select all: `Cmd+A`
- copy: `Cmd+C`
- paste: `Cmd+V`
- undo: `Cmd+Z`
- redo: `Cmd+Shift+Z`
- find: `Cmd+F`

### AI Prompt Box

AI prompt boxes should prioritize not stealing host shortcuts.

Rules:

- match observed OpenAI/Codex behavior only when verified
- prefer command palette or visible buttons for special export actions
- do not assign risky submit-related shortcuts casually
- mark unverified behavior as `needs verification`

### Web App

Web apps must respect browser conflicts.

Common conflicts:

- `Ctrl+L` browser address bar
- `Ctrl+R` reload
- `Ctrl+W` close tab
- `Ctrl+T` new tab
- `Ctrl+F` browser find unless captured intentionally

Browser-specific capture must be explicit.

### Game Overlay

Game overlays should prefer visible buttons and command palette first.

Reason:

- movement keys may already be used
- modifier keys may conflict with gameplay
- editing may be secondary to the main app mode

## Example Hotkey Table

| Action | Linux desktop | Linux terminal | macOS | AI prompt | Web |
| --- | --- | --- | --- | --- | --- |
| `text.select_all` | `Ctrl+A` | needs verification | `Cmd+A` | host focused default | browser focused default |
| `text.copy_plain` | `Ctrl+C` | `Ctrl+Shift+C` needs verification | `Cmd+C` | host focused default | browser focused default |
| `text.paste_plain` | `Ctrl+V` | `Ctrl+Shift+V` needs verification | `Cmd+V` | host focused default | browser focused default |
| `text.undo` | `Ctrl+Z` | needs verification | `Cmd+Z` | host focused default | browser focused default |
| `text.redo` | `Ctrl+Shift+Z` | needs verification | `Cmd+Shift+Z` | host focused default | browser focused default |
| `text.find` | `Ctrl+F` | needs verification | `Cmd+F` | needs verification | browser conflict |
| `text.copy_prompt_block` | unassigned | unassigned | unassigned | visible action preferred | visible action preferred |

## What Not To Do

- Do not use one shortcut table for every host.
- Do not assume terminal `Ctrl+C` means copy.
- Do not override browser/system shortcuts without documenting the reason.
- Do not assign a hotkey to every action just because it exists.
- Do not make special AI prompt actions invisible behind obscure shortcuts.
- Do not let host code silently change shortcuts without updating metadata.

## Common Failure Mode

The common failure is treating hotkeys as UI decoration. They are not. Hotkeys
are an action access layer. If they are wrong, the action is dangerous or
unreachable.

Terminal conflicts are the highest-risk example.

## Examples

Action metadata should look like:

```text
action_id: text.copy_prompt_block
default_profile: linux_desktop
hotkeys:
  linux_desktop: unassigned
  linux_terminal: unassigned
  macos_text: unassigned
  ai_prompt_box: visible action preferred
  web_app: visible action preferred
```

The action can still appear in menus, toolbar, and command palette without a
default shortcut.

## Tests

Hotkey tests should verify:

- every action has a default profile entry
- terminal conflicts are not assigned casually
- macOS mappings use `Cmd` where expected
- unverified host behavior is marked `needs verification`
- host adapters render shortcut labels from metadata

## Questions Intentionally Deferred

- Final keybinding customization UI.
- User-defined shortcut storage.
- Conflict resolution UI.
- Full Codex/OpenAI prompt box shortcut parity.
- Exact Konsole shortcut proof.

## Acceptance

Hotkey profiles are accepted when each action can expose safe shortcuts per
host, terminal conflicts are explicit, and host adapters do not invent shortcut
behavior locally.
