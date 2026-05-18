# Search And Replace

## Purpose

Find and change text safely.

## Actions

- find
- find next
- find previous
- search in selection
- case-sensitive search
- whole-word search
- regex search
- replace current
- replace all
- preview replacements

## User Access Pattern

- search bar or drawer for active searches
- command palette entry for every search mode
- keyboard shortcuts follow host profile

## Tests

- search returns stable ranges
- no-match state is explicit
- replace all reports count
- regex failures return errors without mutating text
