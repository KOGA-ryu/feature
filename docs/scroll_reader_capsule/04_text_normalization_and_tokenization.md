# Text Normalization And Tokenization

V1 reads a host-provided text snapshot as a stream of word tokens.

## Input Payload Contract

The headless capsule accepts text through a narrow payload:

```rust
enum ReaderInputSource {
    SelectedText,
    ClipboardSnapshot,
    ClipboardWatch,
    TextEditorClipboardPipeline,
}

struct ReaderInputPayload {
    source: ReaderInputSource,
    text: String,
}
```

`ClipboardSnapshot` means the host or parallel text-editor pipeline already read
the system clipboard. The capsule does not call clipboard APIs and does not own
clipboard policy.

`ClipboardWatch` means the host is in an explicit watch mode and observed a new,
non-empty copied text snapshot. The model treats it like any other text payload,
but preserves the source so the UI can explain where the current reading session
came from.

Clipboard watch ingest state is intentionally small:

```rust
struct ClipboardWatchIngestState {
    enabled: bool,
    last_seen_text: String,
}
```

It may emit:

```text
accepted
ignored_disabled
ignored_unchanged
ignored_empty
```

The state does not read the clipboard. It only decides whether a host-provided
snapshot should become a reader payload.

## Normalization Rules

Input text must be normalized before tokenization:

- trim leading and trailing whitespace
- collapse repeated spaces and tabs into one space
- preserve paragraph and line breaks as `\n`
- normalize CRLF and CR to LF
- empty or whitespace-only input becomes empty text

Example:

```text
"  First\t\tline.\r\nSecond   line.  "
```

normalizes to:

```text
First line.
Second line.
```

## Tokenization Rules

V1 tokenization is word-based:

- split on Unicode whitespace
- preserve byte spans for lens highlighting
- spans must remain valid UTF-8 boundaries
- punctuation may remain attached to the word in V1
- token order must be deterministic

V1 does not need sentence chunking, language detection, or hyphenation repair.
The model stays word-token based. The UI highlights exactly one focus anchor
token at a time while neighboring words remain visible and readable on the same
continuous sentence tape.

## Corruption Visibility

Copied PDF/OCR artifacts should remain visible as text. The reader may perform
only minimal safe normalization for tape layout:

- normalize CRLF/CR to LF
- trim outer whitespace
- collapse repeated spaces and tabs within a line
- preserve line breaks as `\n`

The reader must not:

- repair hyphenation
- remove page headers or footers
- dedupe repeated lines
- rewrite punctuation
- hide OCR glyph junk
- show cleanup controls

Dirty copied text should look dirty in the tape. Cleanup belongs to the full
text-editor path.

## Formatting Preservation

- Preserve word order exactly.
- Preserve paragraph and line-break rhythm through normalized `\n` boundaries.
- Do not rewrite punctuation.
- Do not alter capitalization.
- Do not insert decorative separators.
- Slightly wider word spacing is allowed near the fixed focus point.
- Slightly larger or heavier focus typography is allowed for the focus anchor
  word.
- Color, opacity, and contrast should do most of the magnification work.
- Formatting changes should be subtle enough that the text still feels intact.

## ReaderToken Contract

```rust
struct ReaderToken {
    text: String,
    start: usize,
    end: usize,
}
```

`start` and `end` are byte offsets into the normalized text.

## Viewport Rules

`viewport(radius)` returns:

- tokens before the lens
- the current focus anchor word
- tokens after the lens
- current index

Required behavior:

- clamps at start and end
- no panic on empty token list
- lens is `None` when there are no tokens
- before/after lists are empty when the lens is at an edge
- current focus anchor word remains centered by the host UI even when the
  returned context is asymmetric

`viewport_for_mode(mode)` returns a continuous-tape viewport with a character
budget:

- large mode target: roughly 90 characters of surrounding context
- compact mode target: reduced context while preserving left and right sentence
  continuity
- the focus anchor remains one token even when the surrounding context is long
- surrounding tokens are functional context for peripheral reading, not
  decoration

## Non-Goals

- no document model
- no edit model
- no clipboard manager
- no hidden clipboard watcher
- no global/background clipboard watcher in V1
- no cleanup panel
- no persistence
- no history storage
- no source attribution
