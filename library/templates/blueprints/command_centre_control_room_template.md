# Command Centre Control Room Template

## Purpose

Use this template for operator-facing monitoring surfaces where attention
routing matters more than decorative UI.

## When to Use

Use when:

- many Dex sessions or jobs must be monitored together
- alerts, risks, and next actions need one surface
- the operator needs overview first and details on demand

Avoid when:

- the feature is only a passive report

## Required Fields

- `overview surfaces`
- `alert channels`
- `operator actions`
- `escalation states`
- `drill-down paths`

## Template

```md
# control room: {surface_id}

## purpose

## when to use

## required fields
- overview surfaces:
- alert channels:
- operator actions:
- escalation states:
- drill-down paths:
```

## Example

```md
# control room: multi_dex_monitor

## required fields
- overview surfaces:
  - active lanes
  - blocked lanes
  - latest test status
- alert channels:
  - tests failed
  - shared-file collision risk
  - missing review packet
- operator actions:
  - inspect lane
  - reroute lane
  - request review
```

## Dex Usage Notes

- Keep overview data visible without forcing deep clicks.
- Prefer status, risk, and action routing over ornamental dashboard widgets.
