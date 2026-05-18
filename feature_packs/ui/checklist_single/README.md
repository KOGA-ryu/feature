# ui.checklist_single

`ui.checklist_single` is a reusable single-checklist state crate for the Rust
feature lab.

## Purpose

Provide one durable checklist with stable append order so hosts can wire in
task toggles without taking on a larger list-management system.

## V1 Behavior

- one checklist only
- trimmed item titles
- blank adds ignored
- append-only ordering for new items
- toggle, delete, and clear-completed actions

## Non-goals

- multiple named lists
- templates
- per-item notes
- drag reorder
- due dates or tagging

## Intended Host / Use Cases

- operator setup checklists
- embedded task panels in larger app shells
- thin live demo surfaces inside `feature_lab_ui`
