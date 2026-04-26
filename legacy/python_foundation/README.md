# Dex Feature Library

This repo is the dedicated machine-first feature library for Dex and Codex.
The point is retrieval, assembly, validation, and reuse. Pretty browsing can
come later.

Current foundation cut:

- library item schema
- core templates
- folder skeleton entries
- `create-item` CLI
- `validate-library` CLI
- `list` and `search` CLI
- regression tests

Install the editable CLI:

```bash
python3 -m pip install -e .
```

Seed or refresh the starter library:

```bash
python3 -m features_tool init-library --json
```

Create a new item:

```bash
python3 -m features_tool create-item ui_pattern "Right Inspector" --surface right_inspector --archetype wiki_browser --json
```

Validate the library:

```bash
python3 -m features_tool validate-library --json
```

List or search reusable pieces:

```bash
python3 -m features_tool list --type ui_pattern --templates-only --json
python3 -m features_tool search "repo foundation" --json
```

Starter template coverage:

- `feature_card`
- `ui_pattern`
- `logic_pattern`
- `component_contract`
- `app_archetype`
- `build_recipe`
- `spec_sheet`
- `dex_prompt`
- `review_packet_template`
- `decision_log`
- `teardown`
- `source_reference_rules`
- `app_idea_intake`
- `folder_skeleton`
