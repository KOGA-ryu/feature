# Text Input

## Purpose

Support predictable writing and deletion.

## Regular Applications

- typing prompts
- editing notes
- replacing selected text
- fixing terminal/chat paste mistakes

## Actions

- insert text
- insert newline
- backspace
- delete forward
- delete word backward
- delete word forward
- delete line
- duplicate line
- join lines
- split line

## User Access Pattern

- keyboard first
- command palette for less common line operations
- toolbar only for operations users would click repeatedly

## Tests

- typed text preserves spaces
- line endings normalize to `\n`
- replacement preserves expected cursor position
- delete/join behavior handles line edges
