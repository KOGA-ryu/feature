# Security Privacy Redaction

## Purpose

Protect users when editor text contains terminal output, prompts, credentials,
private paths, account names, or other sensitive material.

The editor will often handle Codex prompts and terminal transcripts, so
copy/export helpers must make redaction possible without hiding the fact that
redaction happened.

## Sensitive Data Classes

- API keys
- tokens
- private keys
- passwords
- credentials
- account emails
- private local paths
- personal names or identifiers
- secret-looking environment variables
- proprietary source snippets
- terminal command output with secrets

## V1 Rules

- Do not store secrets in fixtures.
- Redaction must be explicit, not silent.
- Redacted output should preserve useful structure.
- A redaction marker should explain the reason at a high level.
- Pure editor helpers do not call external secret scanners unless a later
  validation crate owns that dependency.
- Host apps decide whether full raw logs are allowed.

## Redaction Markers

Use simple markers:

```text
[redacted: possible api key]
[redacted: credential-like string]
[redacted: private path]
[redacted: account identifier]
```

## Copy And Export Rules

- Plain copy preserves text unless the chosen action explicitly says redacted
  copy.
- Prompt block export may include optional metadata, but must avoid leaking
  secrets in metadata.
- Terminal cleanup should strip control characters before redaction checks where
  relevant.
- Redaction should not corrupt indentation in code blocks.

## Fixture Rules

- Use fake tokens only.
- Name fake credentials clearly.
- Never paste real `/status` account strings into committed fixtures.
- Local paths in fixtures should be synthetic unless the test is explicitly
  about path formatting.

## Tests

- fake API key redaction
- fake private key marker redaction
- account-like string redaction
- redaction preserves line count when required
- redaction does not mutate source document unless command is explicitly
  mutating

## Acceptance

Security behavior is acceptable when sensitive text can be detected or redacted
by explicit actions, tests use fake secrets only, and raw-vs-redacted output is
never ambiguous.
