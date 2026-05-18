# Editor Anatomy Reference

## Purpose

This document explains the full anatomy of a text editor so future builders
understand what exists, what matters now, and what is intentionally postponed.

For this feature library, the immediate target is reusable editor actions for
apps. The eventual target may include a graphical desktop interface, but V1 is
not a full editor engine.

## Major Organs

A text editor is made of five large systems:

- text storage
- cursor and selection logic
- editing commands
- rendering and view logic
- intelligence layer

Most first useful app features do not need all five. Copy/export helpers,
prompt blocks, terminal cleanup, and action metadata can be built before a
custom rendering engine exists.

## Text Storage

Text storage is the hidden document structure.

### Single String

A single string is the simplest buffer.

Good for:

- small text fields
- prompt boxes
- settings files
- short notes

Weak for:

- large files
- heavy middle-of-file editing
- multi-cursor edits
- large search/replace

For current feature-library helpers, a single string is acceptable because the
first build focuses on extraction and formatting, not huge-file editing.

### Array Of Lines

An array of lines stores text as line records.

Good for:

- current line helpers
- line range helpers
- trimming trailing whitespace
- moving lines
- rendering line by line

Weak for:

- very large insert/delete operations
- preserving exact newline metadata without care

This is a useful mental model for Dex text actions because many useful actions
are line-oriented.

### Gap Buffer

A gap buffer keeps empty space near the cursor so local typing is fast.

Good for:

- simple editors
- prose editing
- localized typing
- terminal-style editors

Weak for:

- large random edits
- many cursors
- huge automated transformations

This is a real editor-engine structure and is not needed for the first Track B
feature-library slice.

### Rope

A rope stores text as a tree of chunks.

Good for:

- huge files
- large logs
- random edits
- collaboration
- background parsing
- heavy search

Weak for:

- complexity
- cursor math
- memory overhead
- implementation risk

Rope belongs to the later Track A engine discussion, not the first app-action
slice.

### Piece Table

A piece table keeps the original file immutable, appends inserted text to an add
buffer, and represents the visible document as ordered pieces.

Good for:

- undo/redo
- stable original text
- edit records
- large file workflows
- evidence-like command history

Weak for:

- indexing complexity
- piece fragmentation
- harder implementation

For a future serious Dex editor, piece table is worth studying because it pairs
well with command records and audit trails.

## Editing Commands

Most editor actions reduce to:

```text
insert(position, text)
delete(position, length)
replace(start, end, text)
```

Examples:

- typing a character is insert at cursor
- backspace is delete before cursor
- paste is insert clipboard text
- replace selection is delete selection then insert text

A mutating command must update:

- text buffer
- cursor position
- selection range
- undo stack
- dirty state
- render invalidation where a renderer exists
- search or syntax metadata where those systems exist

Track B starts with non-rendering helpers and pure transforms so these deeper
mutation systems can stay small.

## File I/O

File I/O means loading and saving documents.

Simple apps can read a file into memory and write it back. Serious editors also
handle:

- encoding
- line endings
- large files
- file locks
- external modifications
- atomic save
- backup files
- crash recovery

V1 copy/export helpers do not own file I/O. File save behavior belongs to host
apps or later file-state features.

## Cursor And Coordinates

Editors juggle several coordinate systems:

- byte offset
- character offset
- grapheme cluster
- line and column
- screen x/y

They are not equivalent. Unicode, tabs, wide characters, emoji, combining
marks, ligatures, proportional fonts, and soft wrap all complicate movement and
hit testing.

V1 policy:

- keep helpers line/selection oriented
- preserve text exactly
- avoid deep cursor movement until the text model policy is expanded
- use monospace in editor-like hosts where practical

## Navigation

Navigation includes:

- left/right
- up/down
- word movement
- line start/end
- document start/end
- page movement
- paragraph movement
- go to line
- matching bracket
- previous edit location
- scroll without moving cursor

Professional vertical movement usually tracks a preferred visual column. Word
movement requires a clear word-definition policy. These are real editor-engine
concerns and should not block Track B copy/export actions.

## Selection

Selection usually has:

- anchor
- active point

This preserves intent for shift-selection. Selection types include:

- linear selection
- word selection
- line selection
- block or column selection
- multi-selection

Track B needs basic selection range behavior. Column selection and multi-cursor
selection are later Track A or advanced-editing work.

## Clipboard And Export

Clipboard behavior is more than copy and paste. Useful formats include:

- plain text
- markdown block
- code fence
- prompt block
- terminal-safe text
- JSON-safe escaped text
- path or source reference block

The first real feature group should be copy/export:

- `text.select_all`
- `text.selected_text_or_all`
- `text.copy_plain`
- `text.copy_markdown_block`
- `text.copy_prompt_block`
- `text.copy_code_fence`
- `text.copy_terminal_safe`
- `text.current_line_text`
- `text.line_range_text`
- `text.clean_paste_basic`

These do not require a full editor engine.

## Undo And Redo

Weak undo copies the whole document before every edit. Strong undo records
commands such as insert, delete, replace, paste, and format.

Professional undo needs:

- command records
- inverse operation
- redo behavior
- grouping rules
- undo boundaries
- dirty-state integration

Examples:

- paste is one undo step
- format document is one undo step
- replace all is one undo step
- sequential typing may be grouped

Undo/redo matters soon, but deep history design should be bounded by exact
tests before it grows.

## Rendering And View Logic

Rendering turns text into pixels.

Renderer concerns include:

- viewport
- visible line range
- dirty line redraw
- scroll offsets
- cursor drawing
- selection drawing
- monospace metrics
- soft wrap
- hit testing
- folded regions

This is Track A or host-adapter work. Track B action helpers must not depend on
a renderer.

## Search And Replace

Search/replace includes:

- find
- find next/previous
- case sensitivity
- whole word
- regex
- search in selection
- replace current
- replace all
- preview replacements

For small documents, simple scanning is fine. Large-file algorithms can wait.
Replace all must be undoable once mutation behavior exists.

## Code Editing Intelligence

Code editing features include:

- syntax highlighting
- bracket matching
- auto-indent
- snippets
- autocomplete
- diagnostics
- code actions
- go to definition
- find references
- rename symbol
- format document
- LSP integration

These are valuable, but not first. They belong behind explicit code-editing,
host-adapter, or future Track A contracts.

## Syntax And Theme

Syntax highlighting can be regex/TextMate-style or parser-based.

Tree-sitter is the stronger later direction for serious code structure, but it
requires grammar integration and should not enter the first app-action slice.

Theme engines should map semantic tokens to styles, not hard-code colors in the
parser or editor core.

## Async Work

Serious editors do work in the background:

- file save
- project search
- syntax parse
- diagnostics
- symbol indexing
- preview rendering

Async systems need cancellation, debouncing, stale-result checks, and message
queues. None of this is required for the first copy/export helpers.

## Command And Action Registry

The action registry is the most important current layer.

An action record makes every feature discoverable and callable:

```text
id
label
description
category
icon
default_hotkeys
enabled_when
input_shape
output_shape
undo_behavior
execute_handler
```

Why this matters:

- button calls action
- menu calls action
- hotkey calls action
- command palette calls action
- agent can call action
- tests can call action

Without this layer, each UI host reinvents behavior.

## Host Adapters

A host adapter maps editor actions into a UI framework.

Examples:

- Qt adapter
- egui adapter
- terminal adapter
- web adapter
- Codex prompt adapter

Qt adapter maps:

- action id to `QAction`
- hotkey to shortcut
- icon to toolbar button
- enabled rule to enabled state
- action result to host behavior

The UI should not know how copy prompt block works. It should call the action.

## What We Build First

Build now:

- action registry contract
- plain text helpers
- clipboard/export formats
- hotkey profiles
- host adapter contract
- golden tests

Do not build first:

- piece table
- rope
- rendering engine
- syntax highlighting
- LSP
- multi-cursor
- full global search engine

## Mental Stack

```text
Text Buffer
  stores text
Cursor + Selection
  describes where the user is working
Commands
  modify or extract text
Undo Stack
  reverses mutations
Renderer
  draws visible text
Action Registry
  exposes commands to UI, hotkeys, menus, and agents
Host Adapter
  connects actions to Qt, terminal, web, or egui
Advanced Intelligence
  syntax, search, LSP, diagnostics, autocomplete
```

For the current path:

```text
plain text helpers
-> action registry
-> clipboard/export
-> host adapter
-> UI buttons and hotkeys
-> later real editor engine
```

## Acceptance

This anatomy is accepted when future builders can tell whether a proposed
feature belongs in Track B app actions now or Track A editor-engine work later.
