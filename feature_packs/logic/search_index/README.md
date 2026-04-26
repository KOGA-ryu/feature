# logic.search_index

`logic.search_index` is a reusable in-memory search index for library items in the Rust feature lab.

It stores searchable items and supports:

- case-insensitive query matching
- title matching
- summary matching
- tag matching
- kind filtering
- tag filtering
- stable result ordering
- empty-query passthrough

It does not do fuzzy search, semantic search, async indexing, or external storage.

## Contract

- Feature id: `logic.search_index`
- Kind: `logic_pattern`
- Status: `experimental`
- Inputs: search items, query, filters
- Outputs: search results, result count, applied filters

## Search Model

- Query matching is case-insensitive.
- Query matches title, summary, and tags only.
- Kind filtering is exact and case-insensitive.
- Tag filtering is exact and case-insensitive.
- Result order is the original insertion order of the indexed items.

## Harness / Integration Note

This crate is intentionally isolated. A later integrator can add it to the root workspace and wire any demo surface or CLI visibility required for broader repo proof.
