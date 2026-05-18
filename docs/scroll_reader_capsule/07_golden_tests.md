# Golden Tests

These tests define acceptance for a later implementation slice. They are listed
here so the design contract remains testable.

## Documentation Golden Checks

- all required docs exist
- all geometry tokens are present
- all color tokens are present
- color law states exactly two background treatments
- runtime surface is documented as pill-only
- options window is documented as separate from the runtime pill
- hard boundaries exclude app shell, persistence, document library, dashboard,
  editor, AI summary, MIDI, and repo scanner integration
- beauty is documented as a functional requirement
- literal glow is excluded in favor of typographic focus

## Text Normalization Tests

- accepts selected-text, clipboard-snapshot, and text-editor-pipeline payloads
- accepts clipboard-watch payloads from explicit host watch mode
- records the input source without owning clipboard behavior
- trims leading and trailing whitespace
- collapses repeated spaces and tabs
- preserves line breaks as `\n`
- normalizes CRLF and CR to LF
- whitespace-only input produces empty text
- copied PDF/OCR corruption remains visible as text
- no cleanup warnings, cleanup panel, or clipboard history is created

## Clipboard Watch Tests

- watch defaults to disabled
- disabled watch ignores copied text
- enabling watch allows new copied text to emit a reader payload
- accepted watch payload uses source `ClipboardWatch`
- unchanged copied text is ignored
- whitespace-only copied text is ignored
- accepted watch payload loads the reader like selected text
- watch state stores only last seen text, not history
- session watch exposes visible in-app state while active
- global/background watch requires a macOS menu bar indicator and stop action
- V1 excludes global/background watch

## Tokenization Tests

- splits into word tokens
- preserves punctuation attached to words in V1
- stores valid UTF-8 byte spans
- produces deterministic token order
- handles empty text without panic
- preserves word order, capitalization, and punctuation
- allows subtle lens-only typography and spacing changes
- highlights exactly one focus anchor word
- treats the context band as functional reading surface

## Viewport Tests

- middle viewport returns before, lens, and after tokens
- lens contains exactly one focus anchor word
- mode viewport uses roughly 90 characters of surrounding context in large mode
- large mode preserves at least three readable context words before and after
  the focus anchor when enough tokens exist
- context budget is slightly ahead-weighted for left-to-right English
- start viewport clamps before tokens to empty
- end viewport clamps after tokens to empty
- empty viewport returns no lens and no context
- current index is reported

## State Machine Tests

- non-empty load defaults to `Paused`
- empty text cannot enter `Playing`
- `toggle_play` changes `Paused` to `Playing`
- `toggle_play` changes `Playing` to `Paused`
- `Finished` does not keep advancing
- `Esc` closes from every state
- `Closed` ignores play, tick, and scrub

## Wheel Tests

- wheel up rewinds
- wheel down advances
- trackpad pixel deltas accumulate
- angle-wheel input resets pixel accumulation
- scrub clamps at first token
- scrub clamps at last token

## Pace Tests

- default WPM is defined as an internal timing value
- speed clamps to min and max
- speed step is constant
- milliseconds per token derives from WPM

## Visual Smoke Checks

For a later mock UI, capture large and compact modes:

- render plan exposes capsule, lens, text clip, fade zones, progress marker,
  and tape placements as data
- large capsule is 400 by 100 with radius 28
- compact capsule is 200 by 50 with radius 14
- large lens is 184 by 54
- compact lens is 92 by 30
- lens is centered
- text clip rect stays inside the capsule
- fade zones do not cover the fixed focus anchor
- current focus anchor word is highest contrast
- sentence line remains continuous through the focus point
- context words are readable, not decorative
- focus font is only slightly larger than context font
- focus type scale does not exceed 112%
- focus letter spacing is slight, not decorative
- lens contrast uses color and opacity instead of glow
- fade zones are visible
- controls remain restrained
- numeric WPM display is absent
- play/pause state does not require a persistent indicator and is absent by
  default in the runtime pill
- no dashboard, legend, toolbar, or options panel surrounds the runtime pill
- focus lens reads through type scale, contrast, and spacing instead of glow
- SVG preview consumes the render plan instead of recomputing geometry

## SVG Golden Snapshot Checks

Checked-in render-plan SVG fixtures must cover:

- `large_mid.svg`: large mode, sample text, focus on `fox`
- `compact_mid.svg`: compact mode, sample text, focus on `fox`
- `large_start.svg`: focus at first token
- `large_end.svg`: focus at final token
- `compact_empty.svg`: empty text with `Select text to read`
- `large_long_focus_word.svg`: long focused word clipped by the render plan
- `large_transition_50.svg`: transition plan from token index `3` to `4` at
  `500` permille

Snapshot tests compare exact SVG text after LF and trailing-newline
normalization. Snapshots are updated manually and intentionally; tests must not
auto-rewrite fixtures.
