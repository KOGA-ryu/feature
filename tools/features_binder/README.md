# Features Binder

This is the repo-specific Dex binder for:

```text
/Users/kogaryu/dev/features
```

It was copied from the blank source shell at:

```text
/Users/kogaryu/dev/dex_home_final
```

The blank source remains empty. This binder is customized through local
registry data and a local binder template.

## Bound Repo

- project id: `features`
- binder template: `features_feature_foundry_v1`
- feature packs: `48`
- groups: `logic 17 | sim 6 | ui 16 | workflows 9`
- generated/reference index: `feature_packs/by_subject/**`

## Worker Team

- `organizer`: keeps feature map, roles, and boundaries clean
- `planner`: turns feature waves into buildable packets
- `stager`: prepares proof-ready slices and handoff state

## Build

```sh
cmake -S /Users/kogaryu/dev/features/tools/features_binder -B /Users/kogaryu/dev/features/tools/features_binder/build
cmake --build /Users/kogaryu/dev/features/tools/features_binder/build
```

## Test

```sh
ctest --test-dir /Users/kogaryu/dev/features/tools/features_binder/build --output-on-failure
```

## Proof

```sh
~/dev/features/tools/features_binder/proof
```

Proof screenshots are written to:

```text
/Users/kogaryu/dev/features/tools/features_binder/proof_reference/final_current/
```

## Active Workbench

Plain app launch opens the current work area:

```text
Settings > Project Spec
```

Use `--no-settings --tab "Text Editor"` to launch the main Text Editor workbench.
Inside the app, the `Shelf` chrome button is the quick jump into that same Text Editor workspace.
