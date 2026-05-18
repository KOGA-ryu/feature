# Navigation

## Purpose

Move through text without damaging the document.

## Actions

- move left/right by character
- move up/down by line
- move by word
- move to line start/end
- move to document start/end
- go to line
- jump to previous edit location
- scroll without cursor movement

## Reference Behavior

- KDE/Kate and common Linux text fields for desktop movement.
- Konsole-like terminal hosts must avoid shortcuts captured by the shell.
- macOS uses `Cmd` and `Option` conventions where hosted natively.

## Tests

- cursor clamps to valid document positions
- vertical movement preserves preferred column
- word movement handles punctuation and whitespace
- movement does not dirty the document
