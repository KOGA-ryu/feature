# logic.spec_generator

`logic.spec_generator` turns a structured app idea and an explicit list of
selected reusable features into a deterministic build spec.

It owns:

- idea-input validation
- grouping selected features into UI, logic, and workflow slices
- deterministic spec assembly
- reusable folder-structure output

It does not own:

- feature search
- compatibility recommendation
- prompt text generation

## Why It Exists

The feature lab needs a stable assembly step between "idea intake" and "worker
prompt generation". This crate keeps that spec deterministic and
machine-readable.

## Current Scope

- validate required idea fields
- preserve selected feature order
- split selected features into grouped spec sections
- emit a folder structure that matches feature-lab conventions
- generate a compact Dex prompt seed and review checklist
