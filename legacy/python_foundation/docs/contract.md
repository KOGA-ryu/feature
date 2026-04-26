# Contract

This repo exists to hold reusable feature-library items in a Dex-queryable
shape.

Current non-negotiable contract:

1. Machine-readable metadata comes first.
2. Templates and concrete entries share one schema family.
3. The CLI must support create, validate, list, search, and show.
4. Validation must fail on missing required fields or broken typed structure.
5. The repo should stay useful without any UI layer.
