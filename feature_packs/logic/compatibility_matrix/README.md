# logic.compatibility_matrix

`logic.compatibility_matrix` evaluates explicit pairwise compatibility for a selected feature set.

It is intentionally narrow:

- exact feature-id matching only
- symmetric rule lookup
- deterministic pair ordering
- no inference from tags, names, or feature kinds

Missing rules are not hard failures. They become warnings and are treated as `risky` pairings so downstream planning can continue without silently assuming compatibility.
