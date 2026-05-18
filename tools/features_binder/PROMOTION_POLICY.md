# Promotion Policy

`dex_home_final` is a clean runnable UI specimen.

The authority Dex Home repo remains:

```text
/Users/kogaryu/dev/dex_home
```

Rules:

- Do not manually copy final-shell changes into the authority repo.
- Do not sync generated proof files into authority source folders.
- Do not write cockpit JSONL state from this final shell.
- Promote only through an explicit copy/sync script reviewed for the target
  files it will touch.
- The first project added after blank proof should be authored through Settings,
  not preseeded by hand.
