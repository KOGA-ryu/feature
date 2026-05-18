# Selection

## Purpose

Select exact text for copy, export, replacement, or formatting.

## Actions

- select all
- select word
- select line
- select paragraph
- extend selection by character
- extend selection by word
- extend selection to line start/end
- clear selection
- set selection programmatically

## User Access Pattern

- keyboard selection is required.
- mouse selection belongs to host adapters.
- context menus should expose copy/export actions when selection exists.

## Tests

- select all returns the full document exactly
- reversed anchor/caret selections work
- out-of-range selections clamp safely
- selected text preserves newlines and spaces
