# text_editor_actions

Headless action registry for the plain text editor feature system.

This crate owns action metadata and deterministic dispatch. It does not own UI,
system clipboard calls, filesystem writes, cloud AI calls, or host-specific
shortcut behavior.

Host placements are typed as `TextHostPlacement`, not loose strings. That keeps
Qt, egui, terminal, and web adapters on the same placement vocabulary.

## V1 actions

- `text.copy_plain`
- `text.copy_markdown_block`
- `text.copy_prompt_block`
- `text.copy_code_fence`
- `text.select_all`
- `text.current_line_text`
- `text.line_range_text`
- `text.trim_trailing_whitespace`
- `text.clean_basic`
- `text.normalize_line_endings`
- `text.strip_ansi_escape_codes`

## Design rule

`text_editor_plain` owns text behavior.

`text_editor_actions` owns the action records that hosts render and call.

Qt, egui, terminal, and web hosts should render these action records instead of
inventing local labels, icons, tooltips, shortcuts, or behavior.

`text.copy_plain` returns `TextActionOutput::ClipboardPayload`. The payload text
is still exact text by default, but the action output also carries the selection
export policy and metadata that explains whether selected text or the full
document was used.

Cleanup policy actions return `ClipboardTransformResult` through
`TextActionOutput::ClipboardTransform`, preserving cleanup receipts for hosts and
future agent calls.

## Cleanup policy

`text.trim_trailing_whitespace` is output-only in V1. It returns cleaned text and
does not mutate editor state. Undoable cleanup can be added later after action
mutation and undo contracts are stable.
