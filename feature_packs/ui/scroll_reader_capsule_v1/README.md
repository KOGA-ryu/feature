# scroll_reader_capsule_v1

`ui.scroll_reader_capsule_v1` is the first build slice for the scroll reader
capsule.

It defines stable geometry tokens, color token names, display layers, beauty
constraints, control bindings, a small text-snapshot runtime model, a headless
tape motion model, and a deterministic SVG preview renderer that a later Qt or
egui surface can consume.

The runtime preview is pill-only. It does not draw a surrounding app shell,
numeric WPM readout, or persistent play/pause indicator. A future options window
may tune shape, color, and function placement separately.

Design/spec source:

- `/Users/kogaryu/dev/features/docs/scroll_reader_capsule/README.md`
- `/Users/kogaryu/dev/features/docs/scroll_reader_capsule/01_visual_contract.md`
- `/Users/kogaryu/dev/features/docs/scroll_reader_capsule/02_interaction_model.md`
- `/Users/kogaryu/dev/features/docs/scroll_reader_capsule/03_state_machine.md`

## Contract

The capsule is a temporary reading tool:

```text
text snapshot -> hotkey -> floating capsule reader -> pace control -> Esc closes
```

Hard exclusions:

- no document library
- no persistence
- no history storage
- no AI summary
- no MIDI
- no dashboard
- no sidebar
- no full editor
- no clipboard manager
- no cleanup panel
- no large settings page
- no account/auth
- no repo scanner integration

Allowed lightweight surface:

- separate options window for saved parameters, color tuning, shape, and
  function placement, opened only on user request

## Built Surface

- `ReaderInputPayload` with selected-text, clipboard-snapshot, and
  text-editor-pipeline sources
- selected text normalization
- word token spans
- continuous sentence tape viewport
- exactly one centered focus anchor word
- readable context words ahead of and behind the focus anchor
- slightly ahead-weighted context budget for left-to-right English
- large-mode context target of roughly 90 characters
- deterministic `TapeMotionFrame` word placements
- smoothstep tape transition frames over 160ms
- host-agnostic `CapsuleRenderPlan` geometry for capsule, lens, clipping,
  fades, progress, and text placements
- default desktop shortcut contract: `Ctrl+Option+R` summons the capsule
- `ReaderThemeSeed` with only `accent`, `background`, `foreground`, and
  `contrast` as user-facing theme controls
- two built-in theme options, `warm_lens` and `eye_comfort`
- custom theme control by color wheel seed, not an 11-color role editor
- play/pause/close state
- wheel and trackpad scrub direction
- internal pace timing
- large and compact pill-only SVG preview rendering from the render plan
- checked-in SVG golden snapshots for large, compact, empty, edge, long-word,
  and transition render-plan states

The SVG preview is still a fixture-quality renderer, not a standalone app shell.

The crate does not touch the OS clipboard directly. Host code may read the
system clipboard or receive text from the parallel text editor, then pass a
plain text snapshot into the capsule. Extraction artifacts are preserved as
copied text instead of hidden behind cleanup UI.

## Golden Snapshots

Checked-in SVG fixtures live in `fixtures/render_snapshots/`:

- `large_mid.svg`
- `compact_mid.svg`
- `large_start.svg`
- `large_end.svg`
- `compact_empty.svg`
- `large_long_focus_word.svg`
- `large_transition_50.svg`

These files are exact render-plan fixtures for tests. They are not a live UI,
host shell, or app preview. Update them intentionally with:

```bash
cargo run --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/scroll_reader_capsule_v1/Cargo.toml --example render_snapshots -- /Users/kogaryu/dev/features/feature_packs/ui/scroll_reader_capsule_v1/fixtures/render_snapshots
```

## Binder-Compatible Theme Seed

`ReaderThemeSeed` is intentionally a four-field payload that can later receive a
theme from `features_binder` without coupling this crate to the binder:

```json
{
  "accent": "#39E83C",
  "background": "#6B6F78",
  "foreground": "#33085D",
  "contrast": 76
}
```

The fixture lives at `fixtures/theme_seed_binder_compatible.json`. Keep the
reader local for now; promote the seed shape only when a shared theme primitive
exists.

## Theme Options

V1 exposes only two built-in theme options:

- `warm_lens`
- `eye_comfort`

Custom themes are expected to come from a small color-wheel surface:

- accent color wheel
- background color wheel
- foreground color wheel
- contrast slider

The fixture lives at `fixtures/theme_options.json`. The options surface should
not expose the derived 11 internal role colors as direct user controls.

## Desktop Shortcut

The default summon shortcut is `Ctrl+Option+R`.

The fixture lives at `fixtures/desktop_shortcut.json`. It is a host-owned
desktop shortcut contract, not a reader-owned keyboard hook. A later desktop
host registers the shortcut, handles conflicts, captures selected or copied
text, and opens the pill.

## Worker Proof

```bash
cargo test --manifest-path /Users/kogaryu/dev/features/feature_packs/ui/scroll_reader_capsule_v1/Cargo.toml
```
