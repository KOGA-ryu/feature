from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys
from typing import Any

from features_tool.library import (
    DEFAULT_LIBRARY_ROOT,
    ITEM_TYPES,
    LEVEL_VALUES,
    STATUS_VALUES,
    create_library_item,
    list_library_items,
    search_library_items,
    seed_library_root,
    show_library_item,
    validate_library,
)


def main(argv: list[str] | None = None) -> None:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        payload = args.func(args)
    except Exception as exc:  # pragma: no cover - CLI boundary
        print(f"error: {exc}", file=sys.stderr)
        raise SystemExit(1) from exc
    if payload is not None:
        print_payload(payload, json_output=args.json)
    if getattr(args, "exit_fail_on_status", False) and isinstance(payload, dict):
        if payload.get("status") == "fail":
            raise SystemExit(1)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="features", description="Dex-usable feature library tooling")
    parser.add_argument("--json", action="store_true", help="emit JSON instead of text")
    sub = parser.add_subparsers(required=True)

    init_cmd = sub.add_parser("init-library", help="seed the library schema and starter templates")
    add_json_flag(init_cmd)
    init_cmd.add_argument("--root", type=Path, default=DEFAULT_LIBRARY_ROOT)
    init_cmd.add_argument("--overwrite", action="store_true")
    init_cmd.set_defaults(func=cmd_init_library)

    create = sub.add_parser("create-item", help="create a new library item from the typed scaffold")
    add_json_flag(create)
    create.add_argument("type", choices=ITEM_TYPES)
    create.add_argument("name")
    create.add_argument("--slug")
    create.add_argument("--root", type=Path, default=DEFAULT_LIBRARY_ROOT)
    create.add_argument("--summary")
    create.add_argument("--status", choices=STATUS_VALUES, default=None)
    create.add_argument("--template", action="store_true")
    create.add_argument("--force", action="store_true")
    create.add_argument("--tag", action="append", default=[])
    create.add_argument("--surface", action="append", default=[])
    create.add_argument("--logic-pattern", action="append", default=[])
    create.add_argument("--archetype", action="append", default=[])
    create.add_argument("--project", action="append", default=[])
    create.add_argument("--dependency", action="append", default=[])
    create.add_argument("--language", action="append", default=[])
    create.add_argument("--framework", action="append", default=[])
    create.add_argument("--complexity", choices=LEVEL_VALUES, default="medium")
    create.add_argument("--reusability", choices=LEVEL_VALUES, default="high")
    create.add_argument("--risk", choices=LEVEL_VALUES, default="medium")
    create.set_defaults(func=cmd_create_item)

    validate = sub.add_parser("validate-library", help="validate the library root or one library item")
    add_json_flag(validate)
    validate.add_argument("target", nargs="?", type=Path)
    validate.add_argument("--root", type=Path, default=DEFAULT_LIBRARY_ROOT)
    validate.set_defaults(func=cmd_validate_library, exit_fail_on_status=True)

    list_cmd = sub.add_parser("list", help="list library items with metadata filters")
    add_json_flag(list_cmd)
    list_cmd.add_argument("--root", type=Path, default=DEFAULT_LIBRARY_ROOT)
    add_filter_args(list_cmd)
    list_cmd.set_defaults(func=cmd_list_items)

    search = sub.add_parser("search", help="search library items by text and metadata")
    add_json_flag(search)
    search.add_argument("query")
    search.add_argument("--root", type=Path, default=DEFAULT_LIBRARY_ROOT)
    search.add_argument("--limit", type=int, default=10)
    add_filter_args(search)
    search.set_defaults(func=cmd_search_items)

    show = sub.add_parser("show", help="show one library item by slug or path")
    add_json_flag(show)
    show.add_argument("identifier")
    show.add_argument("--root", type=Path, default=DEFAULT_LIBRARY_ROOT)
    show.add_argument("--type", choices=ITEM_TYPES)
    show.set_defaults(func=cmd_show_item)
    return parser


def add_json_flag(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--json", action="store_true", default=argparse.SUPPRESS, help=argparse.SUPPRESS)


def add_filter_args(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--type", choices=ITEM_TYPES)
    parser.add_argument("--status", choices=STATUS_VALUES)
    parser.add_argument("--complexity", choices=LEVEL_VALUES)
    parser.add_argument("--reusability", choices=LEVEL_VALUES)
    parser.add_argument("--risk", choices=LEVEL_VALUES)
    parser.add_argument("--tag", action="append", default=[])
    parser.add_argument("--surface", action="append", default=[])
    parser.add_argument("--logic-pattern", action="append", default=[])
    parser.add_argument("--archetype", action="append", default=[])
    parser.add_argument("--project", action="append", default=[])
    parser.add_argument("--dependency", action="append", default=[])
    parser.add_argument("--language", action="append", default=[])
    parser.add_argument("--framework", action="append", default=[])
    template_group = parser.add_mutually_exclusive_group()
    template_group.add_argument("--templates-only", action="store_true")
    template_group.add_argument("--exclude-templates", action="store_true")


def cmd_init_library(args: argparse.Namespace) -> dict[str, Any]:
    return seed_library_root(args.root, overwrite=args.overwrite)


def cmd_create_item(args: argparse.Namespace) -> dict[str, Any]:
    return create_library_item(
        args.root,
        item_type=args.type,
        name=args.name,
        slug=args.slug,
        summary=args.summary,
        template=args.template,
        status=args.status,
        tags=args.tag,
        ui_surfaces=args.surface,
        logic_patterns=args.logic_pattern,
        app_archetypes=args.archetype,
        example_projects=args.project,
        dependencies=args.dependency,
        languages=args.language,
        frameworks=args.framework,
        complexity=args.complexity,
        reusability=args.reusability,
        risk=args.risk,
        force=args.force,
    )


def cmd_validate_library(args: argparse.Namespace) -> dict[str, Any]:
    target = args.target or args.root
    return validate_library(target)


def cmd_list_items(args: argparse.Namespace) -> dict[str, Any]:
    return list_library_items(args.root, **collect_filters(args))


def cmd_search_items(args: argparse.Namespace) -> dict[str, Any]:
    return search_library_items(args.root, args.query, limit=args.limit, **collect_filters(args))


def cmd_show_item(args: argparse.Namespace) -> dict[str, Any]:
    return show_library_item(args.root, args.identifier, item_type=args.type)


def collect_filters(args: argparse.Namespace) -> dict[str, Any]:
    template_mode = "all"
    if args.templates_only:
        template_mode = "only"
    elif args.exclude_templates:
        template_mode = "exclude"
    return {
        "item_type": args.type,
        "status": args.status,
        "complexity": args.complexity,
        "reusability": args.reusability,
        "risk": args.risk,
        "template_mode": template_mode,
        "tags": args.tag,
        "ui_surfaces": args.surface,
        "logic_patterns": args.logic_pattern,
        "app_archetypes": args.archetype,
        "example_projects": args.project,
        "dependencies": args.dependency,
        "languages": args.language,
        "frameworks": args.framework,
    }


def print_payload(payload: Any, *, json_output: bool) -> None:
    if json_output:
        print(json.dumps(payload, indent=2, sort_keys=True))
        return
    print(render_text(payload))


def render_text(payload: Any) -> str:
    if not isinstance(payload, dict):
        return str(payload)
    lines: list[str] = []
    for key, value in payload.items():
        lines.append(f"{key}:")
        lines.extend(indent(render_value(value)))
    return "\n".join(lines)


def render_value(value: Any) -> list[str]:
    if isinstance(value, list):
        if not value:
            return ["  []"]
        return [f"  - {compact(item)}" for item in value]
    if isinstance(value, dict):
        if not value:
            return ["  {}"]
        return [f"  {key}: {compact(val)}" for key, val in value.items()]
    return [f"  {value}"]


def compact(value: Any) -> str:
    if isinstance(value, dict):
        preferred = [
            "path",
            "title",
            "name",
            "slug",
            "type",
            "status",
            "summary",
            "score",
            "valid",
            "count",
        ]
        parts = [f"{key}={value[key]!r}" for key in preferred if key in value]
        return ", ".join(parts) if parts else json.dumps(value, sort_keys=True)
    return repr(value)


def indent(lines: list[str]) -> list[str]:
    return [f"  {line}" for line in lines]
