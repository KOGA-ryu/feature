# Codex Appearance Theme Teardown

## Purpose

Record the useful implementation pattern visible in the Codex Appearance page,
with specific attention to how custom color is allowed without turning the UI
into an incoherent pile of direct color overrides.

## Source

- source: direct visual teardown of the Codex Appearance page on 2026-04-26
- focus: theme configuration model, not branding
- confidence: medium
- note: implementation details below are inferred from the visible UI contract

## What Is Clearly Visible

The screen exposes a compact set of theme controls:

- theme mode: `Light`, `Dark`, `System`
- live theme preview
- editable color fields:
  - `Accent`
  - `Background`
  - `Foreground`
- `Contrast` control
- structural toggle:
  - `Translucent sidebar`
- typography controls:
  - UI font
  - code font
- theme transport actions:
  - `Import`
  - `Copy theme`

## Why This Matters

This is not a component-by-component paint menu.

The page suggests a data-first theme model where a small user-owned config is
converted into a larger semantic token set. That allows custom color while
keeping surfaces, borders, text, and interaction states visually coherent.

## Likely Theme Model

The visible contract implies two layers.

### Layer 1: user-owned theme config

```ts
type ThemeConfig = {
  mode: "light" | "dark" | "system";
  accent: string;
  background: string;
  foreground: string;
  contrast: number;
  translucentSidebar: boolean;
  uiFont: string;
  codeFont: string;
};
```

### Layer 2: derived semantic theme tokens

```ts
type ThemeTokens = {
  appBg: string;
  panelBg: string;
  panelElevatedBg: string;
  sidebarBg: string;
  sidebarElevatedBg: string;
  borderSubtle: string;
  borderStrong: string;
  textPrimary: string;
  textSecondary: string;
  textMuted: string;
  textDisabled: string;
  accent: string;
  accentHover: string;
  accentActive: string;
  accentSoft: string;
  focusRing: string;
  selectionBg: string;
  codeBg: string;
};
```

## Core Design Rule

Components should consume semantic tokens only.

Good:

- `surface.sidebar`
- `surface.panel`
- `text.primary`
- `text.muted`
- `accent.base`
- `border.subtle`

Bad:

- hardcoded `#hex`
- ad hoc `rgba(...)`
- component-specific custom color knobs

## Why the UI Stays Coherent

The user only controls a few meaningful anchors:

- one interaction color: `accent`
- one surface anchor: `background`
- one text anchor: `foreground`
- one spread control: `contrast`

That gives freedom without letting the user manually break every surface and
state.

## Likely Derivation Flow

1. Read `background`, `foreground`, `accent`, `contrast`, and structural
   toggles.
2. Build a surface ramp:
   - app
   - panel
   - elevated panel
   - sidebar
   - elevated sidebar
3. Build a text ramp:
   - primary
   - secondary
   - muted
   - disabled
4. Build interaction tokens from accent:
   - base
   - hover
   - active
   - soft tint
   - focus ring
5. Apply structural variants:
   - translucent sidebar
6. Render the preview from derived tokens, not raw config values.

## Recommended Color Math

- prefer OKLCH or another perceptual color space for derivation
- avoid naive RGB offsets for tone ladders
- treat `contrast` as a spread parameter over tone distance, not a single alpha

## Reusable Extraction

This teardown should feed three reusable artifacts:

- `logic.theme_token_generator`
- `ui.theme_editor`
- `workflow.theme_import_export`

## Risks and Failure Modes

- allowing direct component color overrides breaks coherence
- deriving text tokens without contrast checks causes unreadable combinations
- using accent everywhere creates a candy-colored UI
- translucency without fallback tokens creates muddy sidebars
- import/export without a stable schema makes themes non-portable

## Build Guidance

If this pattern is implemented in the feature lab, keep the seam split:

- logic crate owns config validation and token derivation
- UI crate owns controls, preview, and import/export affordances
- components consume tokens and never do local color invention
