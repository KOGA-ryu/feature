# Paste Cleanup

## Purpose

Turn damaged terminal/chat/browser text into clean editor text.

Paste cleanup is explicit transformation policy, not automatic moral authority
over user text.

Default rule:

```text
Do not alter pasted text unless the user chooses a named cleanup action.
Every cleanup returns a receipt.
Preview before insertion when the host can support it.
```

## Why This Exists

Copied text from terminals, chats, browsers, logs, and AI tools often carries
damage:

- CRLF or old Mac line endings
- trailing spaces and tabs
- ANSI escape codes
- terminal box borders
- shell prompts
- fake newlines from soft wrapping

Some cleanup is safe. Some cleanup is risky. The difference must be encoded in
named actions and tests, not hidden inside a generic paste path.

## Actions

- paste plain
- paste and normalize line endings
- clean terminal paste
- strip ANSI escape codes
- strip box borders
- unwrap soft-wrapped lines
- preserve intentional blank lines
- remove shell prompts

V1 safe cleanup:

```text
normalize_line_endings
trim_trailing_whitespace
strip_ansi_escape_codes
```

Deferred or explicit-only cleanup:

```text
strip_shell_prompts
strip_box_borders
unwrap_soft_wrapped_lines
secret_detection_warning
redact_secrets
```

Policy:

```text
Do not unwrap soft wraps in V1.
Do not auto-redact secrets in V1.
Do not strip prompts or box borders unless the action name explicitly says so.
```

## Cleanup Result Shape

Use the same receipt pattern as clipboard export:

```text
ClipboardTransformResult:
  text
  changes[]
  warnings[]
```

Examples:

```text
change: normalized_line_endings, count: 3
change: trimmed_trailing_whitespace, count: 4
change: stripped_ansi_escape_codes, count: 8
warning: possible_secret_detected, severity: warn
```

V1 should not emit `possible_secret_detected` unless warning-only detection is
implemented. Do not fake security coverage.

## User Access Pattern

- toolbar: Clean Paste
- command palette: all cleanup transforms
- paste menu: Paste, Paste Plain, Paste Clean

## State Rules

- Cleanup can produce preview text before insertion.
- Insertion is one undo step.
- Destructive cleanup should be visible in tests and preview examples.
- Headless cleanup helpers do not mutate editor state.
- Host insertion owns undo behavior when cleaned text is pasted.
- Cleanup reports must be available before insertion.

## Security Policy

Secrets are credentials or credential-like values such as API keys, access
tokens, private keys, passwords, session cookies, and database URLs with embedded
credentials.

V1 policy:

```text
No auto-redaction.
No fake secret scanner.
No silent deletion.
```

Future policy:

```text
warning-only detection first
explicit redact action later
```

## Tests

- normalize line endings without changing visible words
- trim trailing spaces and tabs without collapsing blank lines
- strip ANSI sequences when explicitly requested
- exact paste path leaves text unchanged
- cleanup report lists each applied transform
- cleanup report is empty when no transform changes text
- box-drawing status output strips borders only in an explicit later action
- ANSI sequences are removed
- soft wraps do not join in V1
- blank lines remain intentional
- shell prompts are removed only when configured

## Acceptance

Paste cleanup V1 is accepted when it proves:

- exact paste is untouched
- safe cleanup transforms are explicit
- cleanup returns a text result and receipt
- editor state is not mutated by headless cleanup
- aggressive terminal cleanup remains deferred or explicitly named
