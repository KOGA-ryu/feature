# workflow.packet_archive_index

`workflow.packet_archive_index` scans archived packet waves and reports whether
they are complete, incomplete, or invalid.

It owns:

- scanning `state/packet_waves/*`
- parsing `wave_manifest.json`
- verifying referenced packet files exist
- surfacing lane and feature metadata from archived waves

It does not own:

- packet generation
- archive writing
- UI rendering
- remote synchronization

## Why It Exists

Once packet waves are archived, a fresh ChatGPT or Dex window still needs a
reliable way to discover which wave folders exist and whether they are usable.
This workflow turns on-disk archives into a machine-readable index.

## Current Scope

- deterministic wave discovery by label
- completeness checks for packet files referenced by each manifest
- graceful handling of missing or malformed manifests
- optional root override for tests or alternate local packet roots

## Non-Goals

- no archive mutation
- no packet rehydration API yet
- no visual browser yet
