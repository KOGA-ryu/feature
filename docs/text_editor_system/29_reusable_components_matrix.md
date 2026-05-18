# Reusable Components Matrix

## Purpose

Identify reusable pieces across the text editor package before implementation
starts.

This is the project-block reuse pass. It says which pieces should be shared,
which crate should own them first, when a concept graduates into a shared type,
and which pieces must not be duplicated locally by host adapters or later
feature crates.

## Why This Exists

The text editor package will touch many surfaces:

- headless helpers
- action registry
- clipboard/export
- Qt host
- terminal host
- web host
- future editor engine

If each surface invents its own range model, action result, hotkey profile,
fixture naming, or error vocabulary, the package becomes hard to reuse. The
matrix catches shared concepts before they become duplicated code.

## Core Rule

If two hosts or crates need the same concept, define it once as a contract.

Do not let Qt, egui, terminal, and web hosts each invent their own action IDs,
range model, hotkey names, icon names, result shape, or fixture style.

## Regular Application

Use this matrix before:

- adding a helper to `text_editor_plain`
- creating `text_editor_actions`
- creating `text_editor_clipboard`
- wiring a host adapter
- writing golden fixtures
- adding review or release gates

## Reference Behavior

Before code, ask:

```text
Does this feature create a reusable concept?
Does another crate or host need the same concept?
Where is the owner documented?
What tests prove it?
```

If the answers are unclear, write the contract before code.

## Chosen Default

Default reuse rule:

```text
one concept, one owner, many consumers
```

## Why This Default Was Chosen

The editor package is meant to be reused across apps. Reuse requires stable
interfaces. The cheapest time to prevent duplicate local types is before the
first implementation slice.

## Reuse Categories

| Category | Reusable Component | First Owner | Reused By | Timing |
| --- | --- | --- | --- | --- |
| text position | document range | `text_editor_plain` | actions, clipboard, search, hosts | early |
| text position | line/column position | `text_editor_plain` | navigation, selection, host status bars | early |
| selection | selected-or-full policy | `text_editor_plain` | copy/export, prompt helpers, host buttons | V1 |
| selection | anchor/active selection shape | `text_editor_plain` | navigation, formatting, future multi-select | early |
| lines | current line helper | `text_editor_plain` | copy line, status, command palette | V1 |
| lines | line range helper | `text_editor_plain` | copy ranges, diagnostics, outline | V1 |
| output | plain text output | `text_editor_plain` first, `text_editor_clipboard` later | copy, export, tests | V1 |
| output | markdown block output | `text_editor_plain` primitive, `text_editor_clipboard` policy | prompt tools, docs, AI helpers | V1/early |
| output | prompt block output | `text_editor_plain` primitive, `text_editor_clipboard` policy | Codex/Spark workflows | V1/early |
| cleanup | line ending normalization | owner depends on mutating vs pure behavior | paste cleanup, fixtures, file state | early |
| cleanup | trailing whitespace trim | `text_editor_plain` if pure, later action if mutating | cleanup, formatting, validation | early |
| actions | action ID namespace | `text_editor_actions` | every host and test | V1 |
| actions | action record shape | `text_editor_actions` | Qt, egui, terminal, web, command palette | V1 |
| actions | enabled rule names | `text_editor_actions` | buttons, menus, tests | V1 |
| actions | action result shape | `text_editor_actions` | hosts, tests, agent calls | V1 |
| UI metadata | icon names | `04_icon_policy.md` + `text_editor_actions` | all hosts | V1 |
| UI metadata | tooltip/accessibility text | `text_editor_actions` | all hosts | V1 |
| UI metadata | menu/toolbar placement hints | `text_editor_actions` | Qt, egui, web | early |
| hotkeys | host profile names | `03_hotkey_profiles.md` + `text_editor_actions` | all hosts | V1 |
| hotkeys | Linux desktop default profile | `text_editor_actions` | Qt first proof | V1 |
| host bridge | action-to-QAction mapping | `text_editor_host_qt` | Dex/Home desktop-style apps | early |
| host bridge | action-to-egui command mapping | `text_editor_host_egui` | feature lab demos | later |
| host bridge | terminal conflict profile | `33_terminal_host_adapter.md` | terminal prompt tools | later |
| host bridge | web action rendering profile | `34_web_host_adapter.md` | prompt boxes/web apps | later |
| tests | exact string fixture naming | `09_fixture_example_policy.md` | all crates | V1 |
| tests | golden action test shape | `40_golden_tests.md` | all crates | V1 |
| tests | code review result shape | `41_spark_code_review_rubric.md` | Spark review buckets | V1 |
| release | final gate checklist | `42_release_gate_checklist.md` | all feature slices | V1 |
| security | redaction marker style | `07_security_privacy_redaction.md` | prompt helpers, terminal cleanup | early |
| errors | empty document behavior | `06_error_empty_state_policy.md` | every action | V1 |
| errors | unsupported host action behavior | `06_error_empty_state_policy.md` | host adapters | early |

## When A Concept Graduates Into A Shared Type

A concept should graduate from prose into a shared type or shared enum when:

- more than one crate needs it
- a host and a headless crate both need it
- tests need to compare it directly
- stringly typed values are likely to drift
- a future action registry must expose it

Do not graduate a concept just because it sounds architectural. The first slice
may keep concepts as documented behavior until reuse pressure is real.

## Components To Reuse Immediately

The first implementation slice should reuse or define:

- `text.*` action ID namespace
- selected-or-full behavior
- exact line/column or range terms used by helpers
- plain output result
- markdown block output result
- prompt block output result
- empty document behavior
- empty selection behavior
- fixture naming pattern
- exact output assertions

Do not create a separate action ID style, fixture naming style, or output record
style inside the first helper slice.

## Components To Prepare In Pass 2

The action registry and host contract pass should lock:

- action record fields
- action result shape
- enabled rule vocabulary
- host profile names
- hotkey override rules
- icon name source
- tooltip and accessibility field requirements
- command palette/menu/toolbar placement fields

This is where reusable UI metadata becomes real.

## Components To Prepare In Pass 3

The clipboard/export pass should lock:

- copy format names
- output option names
- terminal cleanup option names
- redacted output markers
- clean paste result shape
- warning/result distinction
- exact before/after fixture pattern

This prevents terminal, AI prompt, and desktop clipboard helpers from diverging.

## Components To Prepare In Pass 4

The core editing pass should lock:

- cursor/selection terminology
- undo/redo result terms
- dirty state vocabulary
- search match shape
- replacement preview shape
- formatting action result shape

These should be shared before later code or UI layers build on them.

## Reusable API Candidates

These are candidate concepts, not final Rust APIs yet.

```text
TextRange
TextPosition
SelectionState
SelectedTextPolicy
TextOutput
CopyFormat
PasteCleanupOptions
PasteCleanupReport
ActionId
ActionRecord
ActionResult
EnabledRule
HotkeyProfile
HostPlacement
IconName
FixtureCase
ReviewVerdict
```

Do not implement all of these at once. Use the list as a duplication warning:
if a builder needs one of these concepts, check whether the docs already define
the owner and shape.

## What Not To Do

- Do not create duplicate action ID types per crate.
- Do not create one output result shape for clipboard and another for prompt
  helpers unless the difference is specified.
- Do not let host adapters define their own enabled rule names.
- Do not create fixture naming styles per crate.
- Do not graduate every candidate into code before it is needed.
- Do not sneak Track A engine types into V1 helper code.

## Common Failure Mode

The common failure is overcorrecting in one of two directions:

- no shared contracts, causing local drift
- too many shared abstractions too early, causing design weight

The right move is staged reuse: document the shared concept first, implement it
only when a slice actually needs it.

## Examples

Good reuse:

```text
text.copy_prompt_block uses the shared text.* action namespace, the shared
selected-or-full policy, and the shared fixture naming pattern.
```

Bad reuse:

```text
Qt creates qt.copyPrompt, web creates copy_prompt, and tests call
exportPromptBlock directly.
```

## First Proof Questions

Before a code slice starts, the reviewer should ask:

- Which reusable component does this feature consume?
- Which reusable component does this feature create?
- Which crate owns that component?
- Which docs define its behavior?
- Which tests prove it?
- Which later crates or hosts will reuse it?

If those answers are unclear, write the contract before code.

## Questions Intentionally Deferred

- Final Rust module layout.
- Whether shared concepts live in `text_editor_actions` or a smaller common
  crate later.
- Serialization format for action/result records.
- Exact host adapter trait boundaries.

## Acceptance

This reuse matrix is accepted when future builders can identify reusable
contracts before adding local one-off types, labels, hotkeys, outputs, fixtures,
or host behavior.
