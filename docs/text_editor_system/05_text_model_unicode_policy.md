# Text Model Unicode Policy

## Purpose

Define how the text editor talks about text positions, newlines, tabs, and
Unicode before implementation spreads across crates.

This policy prevents cursor bugs, broken selections, and copy/export corruption.

## Core Terms

- document text: the exact stored string
- line ending: newline representation inside document text
- byte offset: UTF-8 byte position
- char offset: Unicode scalar position
- grapheme cluster: user-perceived character such as an emoji or accented glyph
- visual column: rendered column after tabs and wide glyphs
- logical line: text between real newline characters
- visual line: host-rendered wrap line

## V1 Defaults

- Store text as UTF-8.
- Preserve document text exactly unless an explicit mutating command changes it.
- Treat `\n` as the normalized internal newline for new documents.
- Preserve loaded newline style only when the host/file layer provides that
  metadata.
- Do not insert visual-wrap newlines during copy/export.
- Keep byte offsets internal if needed, but public behavior should be described
  in line/column and selection terms.

## Cursor And Selection Rules

- Cursor movement must never split a UTF-8 sequence.
- Cursor movement should not split a grapheme cluster when a grapheme-aware
  implementation is available.
- V1 may document grapheme-perfect movement as a required future upgrade if the
  current crate cannot support it safely.
- Selection ranges must be valid, ordered, and clamped or rejected according to
  `06_error_empty_state_policy.md`.
- Empty selection means cursor-only state, not failure.

## Tabs And Columns

- A tab is stored as `\t` unless an explicit indent conversion command runs.
- Visual tab width is host display state.
- Copy/export preserves tabs by default.
- Convert tabs/spaces only through explicit formatting or cleanup commands.

## Line Endings

- New documents use `\n`.
- Paste cleanup may normalize CRLF only when the command contract says so.
- Copy/export must not silently change line endings unless the chosen export
  action names that behavior.
- Tests must include newline preservation cases.

## Tests

- insert Unicode text and preserve exact output
- cursor movement around multi-byte characters
- selection around multi-byte characters
- newline preservation for LF and CRLF input
- copy/export does not add visual-wrap newlines
- tab preservation during plain copy

## Acceptance

Text model behavior is acceptable when every public API states whether it uses
line/column, byte offsets, character offsets, or selection records, and tests
prove exact text preservation.
