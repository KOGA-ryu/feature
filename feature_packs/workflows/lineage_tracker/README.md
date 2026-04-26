# workflow.lineage_tracker

`workflow.lineage_tracker` builds a deterministic lineage record from the artifacts already produced by the feature lab.

It is the chain-of-custody seam between:

- idea intake
- generated specs
- generated prompt packets
- review packets
- extraction packets

The crate accepts partial chains. Missing downstream artifacts become warning stage summaries instead of hard failures. The only hard requirements are the identity envelope (`project`, `idea_id`, `branch_name`) and the root `app_idea`.

This crate does not write any files, open the UI harness, or mutate the library. It only normalizes provenance into a machine-readable record that later library intelligence or command-centre features can consume.
