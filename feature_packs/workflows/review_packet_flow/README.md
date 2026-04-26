# workflow.review_packet_flow

`workflow.review_packet_flow` standardizes how Dex or Codex task results are
turned into review packets.

It owns:

- the review packet draft shape
- validation of required sections
- packet building
- merge recommendation serialization

It does not own:

- code execution
- task orchestration
- UI rendering

## Why It Exists

Without a stable review packet structure, task summaries become inconsistent and
hard to reuse. This workflow keeps the post-task handoff packet buildable and
machine-readable.

## Required Sections

- goal
- files changed
- commands run
- tests run
- risks
- known issues
- follow-up tasks
- merge recommendation

## Current Scope

- parse JSON draft fixtures
- validate missing required sections
- build a normalized packet from complete task metadata
- round-trip packet serialization

## Non-Goals

- no dynamic task ingestion yet
- no library extraction yet
- no visual review dashboard yet
