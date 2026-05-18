# UI Logic Rules

The final shell is repo-binder-first.

## Blank Slate

Production data starts with:

```json
{
  "projects": [],
  "workers": []
}
```

No project-specific content renders until a project is saved in Settings.

## Settings Owns Authoring

Settings is the only UI path for:

- project identity
- worker roster
- boundaries
- docs
- commands
- IDE seam fields
- binder template selection

The left rail is navigation only.

## Binder Content

Repo binder pages may show:

- saved registry values
- blank or `not set` states
- read-only scanner/contract/proof facts after a saved project exists

Repo binder pages must not show seeded demo facts.

## Workers

Workers are repo-scoped. A worker appears only when it exists in `workers` and
its id is assigned to the selected project.

## Agent Subjects

Agent tabs are retained as binder subjects. They render real backend records or
honest empty states only.
