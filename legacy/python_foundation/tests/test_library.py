from __future__ import annotations

import json
from pathlib import Path
import tempfile
import unittest

from features_tool.cli import build_parser
from features_tool.library import (
    create_library_item,
    list_library_items,
    search_library_items,
    seed_library_root,
    show_library_item,
    validate_library,
)


REPO_ROOT = Path(__file__).parents[1]
TRACKED_LIBRARY_ROOT = REPO_ROOT / "library"


class LibraryItemTests(unittest.TestCase):
    def test_tracked_library_root_validates(self) -> None:
        result = validate_library(TRACKED_LIBRARY_ROOT)

        self.assertEqual(result["status"], "pass")
        self.assertGreaterEqual(result["item_count"], 15)
        self.assertEqual(result["warning_count"], 0)

    def test_init_library_writes_schema_and_templates(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "library"
            seeded = seed_library_root(root)

            self.assertTrue((root / "schemas" / "library_item_schema_v1.json").exists())
            self.assertGreaterEqual(seeded["asset_count"], 16)

    def test_create_item_bootstraps_library_root(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "library"
            created = create_library_item(
                root,
                item_type="ui_pattern",
                name="Right Inspector",
                summary="Reusable side inspector pattern for focused detail work.",
                ui_surfaces=["right_inspector"],
                tags=["desktop", "detail_panel"],
                frameworks=["qt"],
                languages=["qml"],
                dependencies=["selection_state"],
                app_archetypes=["wiki_browser"],
                example_projects=["codex_shell"],
                risk="low",
            )

            self.assertTrue((root / "schemas" / "library_item_schema_v1.json").exists())
            self.assertTrue(Path(created["path"]).exists())
            self.assertTrue(created["valid"])

            shown = show_library_item(root, "right_inspector")
            self.assertEqual(shown["item"]["ui_surfaces"], ["right_inspector"])

            validation = validate_library(root)
            self.assertEqual(validation["status"], "pass")

    def test_list_and_search_support_metadata_filters(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "library"
            seed_library_root(root)
            create_library_item(
                root,
                item_type="ui_pattern",
                name="Right Inspector",
                summary="Persistent detail panel for the active selection.",
                ui_surfaces=["right_inspector"],
                tags=["desktop"],
                frameworks=["qt"],
                languages=["qml"],
                app_archetypes=["wiki_browser"],
                example_projects=["codex_shell"],
                risk="low",
            )
            create_library_item(
                root,
                item_type="logic_pattern",
                name="Validation Pipeline",
                summary="Reusable validation pipeline for staged input checks.",
                logic_patterns=["validation_pipeline"],
                tags=["validation"],
                frameworks=["python"],
                languages=["python"],
                app_archetypes=["agent_dashboard"],
                example_projects=["features"],
                risk="low",
            )

            listed = list_library_items(
                root,
                item_type="ui_pattern",
                ui_surfaces=["right_inspector"],
                frameworks=["qt"],
                template_mode="exclude",
            )
            self.assertEqual([item["slug"] for item in listed["items"]], ["right_inspector"])

            searched = search_library_items(
                root,
                "validation pipeline",
                item_type="logic_pattern",
                template_mode="exclude",
            )
            self.assertEqual(searched["items"][0]["slug"], "validation_pipeline")

    def test_invalid_component_contract_is_flagged(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "library"
            created = create_library_item(
                root,
                item_type="component_contract",
                name="Review Inspector",
                summary="Contract for a review-oriented inspector component.",
                ui_surfaces=["right_inspector"],
                risk="low",
            )
            path = Path(created["path"])
            payload = json.loads(path.read_text())
            del payload["component_contract"]["test_cases"]
            path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")

            result = validate_library(path)
            self.assertEqual(result["status"], "fail")
            self.assertIn("component_contract.test_cases", " ".join(result["errors"]))

    def test_cli_parser_accepts_feature_commands(self) -> None:
        parser = build_parser()
        args = parser.parse_args(["validate-library"])

        self.assertEqual(args.func.__name__, "cmd_validate_library")


if __name__ == "__main__":
    unittest.main()
