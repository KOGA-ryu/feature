# Zed Clipboard, Selection, Movement Extraction

Status: reference teardown for learning and future Text Editor planning.

Source scope:

- Zed `crates/editor/src/clipboard.rs`
- Zed `crates/editor/src/selections_collection.rs`
- Zed `crates/editor/src/movement.rs`
- Zed `crates/editor/src/input.rs`

This document extracts editor mechanics from Zed's public source. It is not a
porting target. The goal is to learn what mature editors protect, then translate
that into our feature-library shape.

## Purpose

The previous Zed pass taught runtime and benchmark lessons. This pass focuses
on the editor organs closest to our current work:

- clipboard/export,
- selection state,
- movement,
- text input.

These are the mechanics behind the simple-looking actions we are building now.

## Clipboard Lessons

Zed does not treat clipboard text as just a string. It stores extra metadata
with copied text.

Observed metadata concepts:

- selection length,
- whether the selection was an entire line,
- first-line indentation,
- optional file path,
- optional line range.

Why this exists:

Pasting is easier and smarter when the editor knows how the text was copied.
For example, a copied whole line should paste like a line, not like inline text.
Multiple copied selections can map back onto multiple active selections.
Indentation can be restored or adapted when pasting into a new location.

Regular applications:

- copy current line with no selection,
- paste full-line copy before current line,
- paste multiple copied chunks into multiple cursors,
- preserve or adapt indentation,
- attach source metadata for review/prompt/export workflows.

Our current state:

`text_editor_clipboard` already has the correct first principle:

```text
exact by default
cleanup only by named action
host owns OS clipboard
transforms return receipts
```

What we should borrow next:

Add a future clipboard payload model separate from raw text:

```text
ClipboardPayload:
  text
  selections[]
  source_path?
  line_range?
  copied_as_entire_line?
  first_line_indent?
  receipt?
```

Do not add this to V1 copy buttons yet unless the next task needs it. The
current Rust runner and Qt host clipboard write are still allowed to write plain
text.

What not to copy now:

- line-mode paste behavior,
- multi-selection paste distribution,
- markdown URL paste transformation,
- editor-owned OS clipboard calls in headless crates.

## Selection Lessons

Zed's selection collection is more than one anchor and one caret.

Observed concepts:

- disjoint committed selections,
- one pending selection while dragging or extending,
- line mode,
- select mode,
- newest/oldest/first/last selection,
- anchor-based selections,
- display-point selections,
- columnar selections,
- selection goals for horizontal/visual movement.

Why this exists:

Selections are both data and interaction state. A mouse drag is not the same as
a finished selection. A column selection is not the same as a byte range. A
soft-wrapped visual line is not the same as a buffer line.

Regular applications:

- select all,
- extend selection with shift,
- drag selection,
- select full lines,
- column selection,
- multi-cursor editing,
- split selection into lines,
- merge overlapping selections,
- preserve visual column while moving up/down.

Our current state:

`text_editor_plain` has a simple `EditorSelection` with:

```text
anchor
caret
```

That is correct for Track B and for current copy/export helpers.

What we should borrow next:

Before adding multi-cursor or column selection, create a separate detail doc for
`SelectionCollection` with these concepts:

```text
committed selections
pending selection
line mode
selection mode
newest selection
visual goal
merge/disjoint rule
```

What not to do:

- Do not stuff multi-cursor state into the existing single `EditorSelection`.
- Do not make column selection depend on byte columns.
- Do not mix visual display positions with buffer positions without naming the
  conversion.

## Movement Lessons

Zed movement functions operate over a display snapshot, not just raw text.

Observed movement families:

- left/right with line wrapping behavior,
- up/down by displayed rows,
- line beginning with soft-boundary and indentation policy,
- line end with soft-boundary policy,
- previous/next word boundary,
- previous/next subword boundary,
- paragraph boundaries,
- excerpt boundaries,
- deletion-adjusted movement.

Why this exists:

Users do not always move through raw buffer lines. With soft wrap, folds, tabs,
and display maps, "up" and "down" are visual actions. Word deletion also has
ergonomic behavior that differs from simple word movement.

Regular applications:

- arrow keys,
- home/end,
- ctrl/alt word movement,
- camelCase/snake_case subword movement,
- delete previous word,
- delete next word,
- move through wrapped prose,
- move through excerpts or multi-buffer views.

Our current state:

`text_editor_plain` has basic cursor movement and preferred-column behavior.
That is enough for the current reusable action path.

What we should borrow next:

Split future movement into explicit policies:

```text
buffer movement
visual movement
word movement
subword movement
line-edge movement
deletion movement
paragraph movement
```

The first practical next movement helpers should be:

- move to document start/end,
- move by word,
- delete word backward/forward,
- select line.

Do not build visual soft-wrap movement until we own a custom visual map or a
host adapter can expose it cleanly.

## Input Lessons

Zed's input path handles much more than inserting typed text.

Observed input concerns:

- input can be disabled,
- read-only selections are skipped,
- typed text applies to every active selection,
- bracket auto-close,
- bracket skip-over,
- auto-surround selected text,
- linked edit ranges,
- emoji shortcode replacement,
- auto-indent,
- on-type formatting,
- completion triggering,
- edit prediction refresh.

Why this exists:

Text input is the busiest editor path. A single keypress may modify text,
selection, linked ranges, bracket regions, formatting state, completion state,
and prediction state.

Regular applications:

- typing,
- replacing selected text,
- bracket pair insertion,
- quote wrapping,
- list continuation,
- comment continuation,
- autocomplete,
- local or cloud AI prediction later.

Our current rule:

Keep Track B helpers output-oriented until we deliberately enter mutating editor
engine work.

What we should borrow next:

When we add mutating actions, require a transaction result shape:

```text
TextEditTransaction:
  before_selection
  edits[]
  after_selection
  undo_group
  receipts[]
```

This keeps future AI and formatting helpers from directly rewriting text with
no audit trail.

## How This Changes Our Build Roadmap

Immediate current path stays:

```text
text_editor_plain
text_editor_actions
text_editor_clipboard
text_editor_host_adapter
Qt workbench proof
```

Next practical builds should be:

1. Clipboard payload metadata V1, but still plain-text OS clipboard output by
   default.
2. Selection detail doc and tests for select line / select word.
3. Movement helper docs for word and subword behavior.
4. Transaction result policy before mutating cleanup or formatting actions.

Postpone:

- multi-cursor editing,
- column selection,
- visual soft-wrap movement,
- bracket auto-close,
- linked edits,
- LSP/completion,
- edit prediction.

## Golden Test Implications

Add future fixtures for:

- empty selection means current-line copy only when the action says so,
- full-line copy records line-mode metadata,
- selected text copy does not add a trailing newline,
- whole-line copy at EOF can add a newline intentionally,
- multiple selections preserve order,
- paste distribution is disabled until explicitly implemented,
- word movement handles punctuation,
- subword movement handles camelCase and snake_case,
- delete-word movement is allowed to differ from move-word behavior but must be
  named separately.

## Acceptance

This extraction is useful when:

- clipboard metadata is recognized as a future payload, not mixed into plain
  copy V1,
- selection collection is treated as its own future object,
- movement policies are named before they are implemented,
- mutating text input waits for transaction policy,
- Zed is used as a teaching reference rather than a code template.

## Source Links

- Zed clipboard source: <https://github.com/zed-industries/zed/blob/main/crates/editor/src/clipboard.rs>
- Zed selections collection source: <https://github.com/zed-industries/zed/blob/main/crates/editor/src/selections_collection.rs>
- Zed movement source: <https://github.com/zed-industries/zed/blob/main/crates/editor/src/movement.rs>
- Zed input source: <https://github.com/zed-industries/zed/blob/main/crates/editor/src/input.rs>
