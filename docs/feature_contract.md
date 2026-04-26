# Feature Contract

Every feature crate must include:

- `Cargo.toml`
- `feature.toml`
- `README.md`
- `src/lib.rs`
- `tests/`
- `fixtures/`

Required metadata:

- `id`
- `name`
- `kind`
- `status`
- `summary`
- `inputs`
- `outputs`
- `dependencies`
- `compatible_features`
- `tags`
- `owner`
- `created_at`
- `updated_at`

Feature crates are the unit of reuse. No loose feature files at repo root.
