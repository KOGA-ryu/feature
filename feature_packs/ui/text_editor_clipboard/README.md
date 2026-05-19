# text_editor_clipboard

Headless clipboard/export policy helpers for the text editor system.

This crate produces strings and transform receipts. It does not call the system
clipboard, mutate editor state, open files, redact secrets, unwrap terminal soft
wraps, strip shell prompts, or remove box borders in V1.

## V1 policy

- exact copy stays exact
- cleanup only happens through named helpers
- every cleanup returns a receipt
- payload export is metadata-bearing, but the host-visible clipboard text is
  still plain text by default
- selected-or-full-document export is the default policy
- selection-only and full-document export policies are explicit
- prompt block output delegates to `text_editor_plain`
- no OS clipboard access
- no editor mutation

## Clipboard Payload V1

`ClipboardPayload` contains:

- `text`: the string a host may place on the system clipboard
- `metadata`: payload kind, export policy, selection use, line range, optional
  source/language hints, counts, and first-line indentation
- `receipt`: the exact transform result, including cleanup changes and warnings

Policy names:

- `SelectedOrFullDocument`: use selected text when present, otherwise full
  document text
- `SelectionOnly`: export selected text only; no selection returns empty text
  without falling back
- `FullDocument`: export the full document even when a selection exists

Headless payload helpers still do not call the OS clipboard. Hosts decide
whether to write `payload.text` to a real clipboard.

## Main helpers

- `copy_exact`
- `copy_exact_payload`
- `copy_exact_payload_with_policy`
- `copy_clean_payload_with_policy`
- `copy_markdown_block`
- `copy_prompt_block`
- `copy_code_fence`
- `clean_basic`
- `normalize_line_endings_with_report`
- `trim_trailing_whitespace_with_report`
- `strip_ansi_escape_codes_with_report`
