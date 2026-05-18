# Structure Outline

## Purpose

Help users understand and navigate long text.

Structure features read the document and produce navigation metadata. They do
not judge writing quality or rewrite content.

## Regular Applications

- navigate long specs
- jump between markdown headings
- inspect symbols supplied by a code host
- fold sections
- extract TODOs and bookmarks

## Actions

- show document outline
- list headings
- list symbols
- fold section
- unfold section
- fold all
- unfold all
- show breadcrumbs
- add bookmark
- remove bookmark
- next bookmark
- previous bookmark
- extract TODOs
- go to outline item

## V1 Boundary

Markdown heading outline can be a headless helper once specified.

Future or host-owned:

- language symbols
- code folding from parsers
- breadcrumbs from project paths
- persistent bookmarks

## User Access Pattern

- side panel: outline tree
- command palette: jump to heading, next bookmark
- gutter: fold controls and bookmarks
- menu: `Navigate`

## Default Hotkeys

- go to symbol/heading: host-specific
- fold/unfold: host-specific
- bookmarks: host-specific

Mark all default structure hotkeys as `needs verification` before coding.

## Button And Icon

- outline: `list-tree`
- fold: `chevrons-up`
- unfold: `chevrons-down`
- bookmark: `bookmark`
- TODOs: `list-checks`

## Tests

- markdown headings produce deterministic outline entries
- outline item maps back to exact line/offset
- TODO extraction preserves source line
- folding metadata does not alter text

## Acceptance

Structure features are accepted when metadata is deterministic, navigable, and
separate from document text.
