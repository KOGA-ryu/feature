# Terminal Text Helpers

## Purpose

Make terminal and chat transcript text safe to copy, clean, and reuse.

## Actions

- strip ANSI
- strip box drawing borders
- normalize prompt prefixes
- unwrap terminal soft wraps
- remove repeated margin glyphs
- preserve command output blocks
- copy command-only selection
- copy output-only selection

## Reference Behavior

Konsole is the first reference for terminal copy/paste expectations. Terminal
hosts often reserve shortcuts, so host adapters must use terminal-safe profiles.

## Tests

- `/status`-style box output can be cleaned
- command prompts can be removed without deleting output
- soft-wrap cleanup does not join intentional paragraph breaks
- trailing spaces are preserved only when meaningful
