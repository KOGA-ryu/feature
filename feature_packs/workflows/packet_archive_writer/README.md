# workflow.packet_archive_writer

`workflow.packet_archive_writer` turns a `FeatureWavePacketBundle` into a
launch-ready archive under `state/packet_waves/`.

It owns:

- deterministic archive folder creation
- worker, integrator, and reviewer packet file emission
- a machine-readable wave manifest
- a human-readable archive README for fresh coding windows

It does not own:

- packet generation
- feature selection
- prompt rewriting
- packet indexing across several waves

## Why It Exists

The feature lab can already assemble single-feature and multi-feature packet
bundles in memory. This workflow makes them operational by writing a stable
handoff archive that a fresh ChatGPT or Dex coding window can open immediately.

## Current Scope

- one archive per explicit wave label
- collision-safe writes by default
- absolute manifest paths for local-first launch tooling
- markdown wrappers around the exact generated prompt packets

## Non-Goals

- no archive index yet
- no remote sync
- no packet diffing or history management
