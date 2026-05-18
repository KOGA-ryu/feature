# Design Implementation Matrix

This matrix turns the small visual choices into implementation rules.

## Runtime Surface

| Choice | Decision | Code owner | Default | Test |
| --- | --- | --- | --- | --- |
| Runtime chrome | Display only the capsule pill | host widget | no shell, no toolbar | SVG excludes app-shell text and controls |
| Options | Separate options window only | host/options slice | not shown during reading | docs and contract name `separate_options_window` |
| Focus anchor | One centered eye anchor word | reader model | `lens = current token` | viewport lens contains no spaces |
| Context band | Functional peripheral reading surface | renderer/host | readable words before and after anchor | tests require context on both sides |
| Sentence line | Continuous tape | renderer/host | before + anchor + after line | render preview keeps neighboring words close |
| Render plan | Host-agnostic drawing recipe | reader crate | rects, fades, progress, text placements | render plan geometry tests |
| Theme controls | Simple eye-comfort seed | options/host | accent, background, foreground, contrast | role colors derive from seed |

## Magnification

| Choice | Decision | Code owner | Default | Test |
| --- | --- | --- | --- | --- |
| Main emphasis | Color, opacity, and contrast | renderer/palette | focus opacity 100%, context 72% | SVG opacity checks |
| Type scale | Slight only | typography tokens | large `18 -> 20`, compact `11 -> 12` | scale cap <= 112% |
| Spacing | Slight clarity only | typography tokens | focus letter spacing `0.2px` | SVG letter-spacing check |
| Glow | Not literal glow | renderer | no bloom/halo requirement | docs golden check |

## Tape Window

| Choice | Decision | Code owner | Default | Test |
| --- | --- | --- | --- | --- |
| Large context | Roughly 90 chars around focus | viewport | `focus_context_target_chars_large = 90` | mode viewport test |
| Compact context | Preserve context with less width | viewport | `focus_context_target_chars_compact = 44` | fixture round trip |
| Paragraphs | Affect pacing, not layout shell | later timing slice | normalized LF boundaries | normalization tests |
| Punctuation | Stays attached in V1 | tokenizer | punctuation in token text | token span tests |

## Motion Model

V1 exposes headless tape motion and a host-agnostic render plan. A later host
slice owns live drawing and animation:

- text line slides under a fixed center word
- frame does not move
- wheel scrub maps to token index changes
- timer ticks advance one focus anchor word at a time
- easing must be subtle enough that the text still feels like a normal sentence
- SVG preview consumes the same render plan intended for Qt or egui
