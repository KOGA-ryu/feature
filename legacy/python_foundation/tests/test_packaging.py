from __future__ import annotations

from pathlib import Path
import tomllib
import unittest

from features_tool.cli import build_parser


class PackagingTests(unittest.TestCase):
    def test_console_script_points_at_cli_main(self) -> None:
        pyproject = Path(__file__).parents[1] / "pyproject.toml"
        config = tomllib.loads(pyproject.read_text())

        self.assertEqual(config["project"]["scripts"]["features"], "features_tool.cli:main")
        self.assertEqual(config["tool"]["setuptools"]["packages"]["find"]["include"], ["features_tool*"])
        self.assertEqual(build_parser().prog, "features")


if __name__ == "__main__":
    unittest.main()
