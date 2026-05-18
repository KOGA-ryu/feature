# text_editor_clipboard

Headless clipboard/export policy helpers for the text editor system.

This crate produces strings and transform receipts. It does not call the system
clipboard, mutate editor state, open files, redact secrets, unwrap terminal soft
wraps, strip shell prompts, or remove box borders in V1.

## V1 policy

- exact copy stays exact
- cleanup only happens through named helpers
- every cleanup returns a receipt
- prompt block output delegates to `text_editor_plain`
- no OS clipboard access
- no editor mutation

## Main helpers

- `copy_exact`
- `copy_markdown_block`
- `copy_prompt_block`
- `copy_code_fence`
- `clean_basic`
- `normalize_line_endings_with_report`
- `trim_trailing_whitespace_with_report`
- `strip_ansi_escape_codes_with_report`
