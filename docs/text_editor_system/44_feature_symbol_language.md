# Feature Symbol Language And Local Librarian

## Purpose

Define the future compact language that lets a human, local AI librarian, and
cloud AI talk about reusable feature-library work without dumping the whole
repo into every prompt.

This is a north-star design note, not an implementation order. The current text
editor work still stays on Track B basics: headless helpers, action records,
clipboard/export behavior, and host adapters.

## Why This Exists

Feature libraries become valuable when the right feature can be found, shaped,
and proven quickly.

Plain English is flexible, but it can be expensive in cloud AI context. Dense
custom shorthand saves tokens, but it can become unreadable. The goal is a
middle language: compact enough to reduce context cost, readable enough for
humans to inspect, and stable enough for a local librarian to expand into real
repo paths, docs, tests, and feature contracts.

## Mental Model

The system has four roles:

```text
Feature Library
  owns crates, docs, contracts, examples, fixtures, and tests

Feature Symbol Language
  names app intent, selected features, host, proof, limits, and rough shape

Local Librarian
  resolves symbols into concrete feature packs, docs, commands, and warnings

Cloud AI
  receives a compact curated packet and asks for more context only when needed
```

The text editor is the workbench where these packets are authored, cleaned,
copied, reviewed, and eventually transformed into action calls.

## Human Request Vs Symbol Request

Human request:

```text
Build me a note editor with a left rail, session notes, markdown copy, and
prompt export. Use tested feature packs and keep it desktop friendly.
```

Symbol request:

```text
APP note_editor
USE ui.left_rail + logic.session_notes + ui.session_notes_panel + ui.text_editor_plain
ADD text.copy_prompt_block + text.copy_markdown_block
HOST qt_desktop
PROOF cargo_test + screenshot
LIMIT no_cloud_write
```

The symbol request is not meant to replace human language everywhere. It is a
compact index card that a local librarian can expand.

## Local Librarian Role

The local librarian runs close to the repo and should be able to:

- resolve `USE` feature IDs into `feature_packs/**` paths
- read `feature.toml`, `README.md`, fixtures, and contract tests
- identify compatible features and missing dependencies
- choose the smallest useful doc set for a cloud AI packet
- report unknown feature IDs honestly
- include exact commands for proof
- avoid sending unnecessary repo text to cloud AI

The librarian should not silently mutate source files. It curates context and
hands off packets.

## Cloud AI Role

Cloud AI should receive compact, curated context:

```text
selected_features:
- ui.left_rail
- logic.session_notes
- ui.session_notes_panel
- ui.text_editor_plain

constraints:
- headless state remains separate from host UI
- host adapter renders actions from metadata
- no system clipboard in headless crates

proof:
- cargo test -p text_editor_plain
- cargo test -p session_notes_panel
```

Cloud AI can reason, draft, review, or build from that packet. If it needs more
context, it should request more by feature ID or source reference, not by asking
for the whole repo.

## Text Editor Role

The text editor makes the symbol language practical by providing:

- exact copy as plain text
- Markdown fenced copy
- prompt block copy with source
- selection and line extraction
- paste cleanup
- future action records for `text.make_work_order`, `text.extract_todos`, and
  `text.split_prompt_chunks`

The editor is not just a place to type. It is the packet preparation surface.

## First Symbol Set

Use readable uppercase verbs first:

```text
APP     app or feature target name
USE     feature packs required by the work
ADD     actions or helper behaviors to include
HOST    target host or adapter
PROOF   proof commands or proof types
LIMIT   constraints and hard boundaries
STYLE   rough UX or implementation shape
DOCS    required docs or source references
OUT     expected artifact or result
```

Examples:

```text
APP repo_binder
USE ui.left_rail + ui.right_inspector + ui.text_editor_plain
ADD text.copy_prompt_block
HOST qt_desktop
PROOF cargo_test + screenshot
LIMIT no_jsonl_write + no_cloud_write
```

```text
APP prompt_cleaner
USE ui.text_editor_plain
ADD text.copy_markdown_block + text.trim_trailing_whitespace
HOST terminal_first
PROOF cargo_test
LIMIT no_system_clipboard
```

## Packet Expansion Example

Input:

```text
APP note_editor
USE logic.session_notes + ui.session_notes_panel + ui.text_editor_plain
ADD text.copy_prompt_block
HOST qt_desktop
PROOF cargo_test
```

Local librarian expands to:

```text
app_target: note_editor
host: qt_desktop

feature_paths:
- feature_packs/logic/session_notes
- feature_packs/ui/session_notes_panel
- feature_packs/ui/text_editor_plain

required_docs:
- feature.toml
- README.md
- tests/contract_tests.rs

action_contracts:
- text.copy_prompt_block

proof_commands:
- cargo test -p session_notes
- cargo test -p session_notes_panel
- cargo test -p text_editor_plain

warnings:
- qt host adapter is not implemented yet
```

## Rules For Compression

- Keep symbols readable before making them dense.
- Prefer stable feature IDs over prose.
- Prefer action IDs over vague behavior names.
- Keep source refs exact.
- Mark unknowns instead of guessing.
- Compress repeated structure, not important meaning.
- Keep proof and mutation limits visible.

Good:

```text
USE ui.text_editor_plain
ADD text.copy_prompt_block
LIMIT no_system_clipboard
```

Too dense for V1:

```text
U:utep A:tcpb L:nsc
```

The second form saves tokens but costs too much human and model clarity until
the language is proven.

## What Not To Compress

Do not compress:

- secrets or redaction warnings
- destructive edit permission
- proof failure output
- unresolved questions
- source paths when exact paths matter
- legal/license constraints
- user-authored requirements that have not been normalized yet

Compression should reduce context cost, not hide risk.

## Future Actions

Future text editor actions can use this language:

```text
text.make_feature_symbol_request
text.expand_feature_symbol_request
text.extract_feature_ids
text.make_work_order_from_selection
text.make_cloud_packet_from_selection
text.estimate_context_cost
text.split_prompt_chunks
```

AI-backed actions should remain proposal-first:

```text
AI suggests.
Editor owns state.
User approves mutation.
Actions record the boundary.
```

## Acceptance

This concept is ready to build on when:

- humans can read and write the basic symbols without a decoder ring
- the local librarian can expand symbols into exact repo paths and commands
- cloud packets become smaller without losing proof or constraints
- text editor actions can create and copy these packets cleanly
- unknown symbols produce honest missing-context reports
