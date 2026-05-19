#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FEATURES_ROOT="$(cd "$ROOT/../.." && pwd)"
BUILD="$ROOT/build"
APP="$BUILD/dex_home_v2"
OUT="$ROOT/proof_reference/final_current"
RUNNER="$FEATURES_ROOT/target/debug/text_editor_action_runner"

mkdir -p "$OUT"
rm -f \
  "$OUT/01_features_profile_1280x800.png" \
  "$OUT/02_features_inventory_1280x800.png" \
  "$OUT/03_features_quality_1280x800.png" \
  "$OUT/04_features_settings_1280x800.png" \
  "$OUT/05_features_profile_900x700.png" \
  "$OUT/06_features_settings_full_1280x4200.png" \
  "$OUT/07_features_text_actions_1280x800.png" \
  "$OUT/07_features_text_editor_1280x800.png" \
  "$OUT/07_features_text_editor_ui_tree.json" \
  "$OUT/08_features_text_editor_900x700.png" \
  "$OUT/08_features_text_editor_ui_tree.json" \
  "$OUT/09_features_text_editor_palette_1280x800.png" \
  "$OUT/09_features_text_editor_palette_ui_tree.json" \
  "$OUT/manifest.txt"

cmake -S "$ROOT" -B "$BUILD"
cargo build --manifest-path "$FEATURES_ROOT/Cargo.toml" -p text_editor_host_adapter --bin text_editor_action_runner
printf '%s' '{"mode":"render_host_actions","document_text":"alpha","input":{},"profile":"linux_desktop"}' \
  | "$RUNNER" \
  | grep -q '"action_id": "text.copy_plain"'
cmake --build "$BUILD"
ctest --test-dir "$BUILD" --output-on-failure
export TEXT_EDITOR_ACTION_RUNNER="$RUNNER"

capture() {
  local name="$1"
  shift
  QT_QPA_PLATFORM=offscreen "$APP" \
    --project features \
    "$@" \
    --screenshot "$OUT/$name.png"
}

capture "01_features_profile_1280x800" --size 1280x800 --worker organizer --tab Profile
capture "02_features_inventory_1280x800" --size 1280x800 --worker organizer --tab Inventory
capture "03_features_quality_1280x800" --size 1280x800 --worker planner --tab Quality
capture "04_features_settings_1280x800" --size 1280x800 --settings
capture "05_features_profile_900x700" --size 900x700 --worker stager --tab Profile
capture "06_features_settings_full_1280x4200" --size 1280x4200 --settings
capture "07_features_text_editor_1280x800" --size 1280x800 --no-settings --worker organizer --tab "Text Editor" --ui-tree-dump "$OUT/07_features_text_editor_ui_tree.json"
capture "08_features_text_editor_900x700" --size 900x700 --no-settings --worker organizer --tab "Text Editor" --ui-tree-dump "$OUT/08_features_text_editor_ui_tree.json"
capture "09_features_text_editor_palette_1280x800" --size 1280x800 --no-settings --worker organizer --tab "Text Editor" --show-command-palette --ui-tree-dump "$OUT/09_features_text_editor_palette_ui_tree.json"

check_png_size() {
  local file="$1"
  local expected_width="$2"
  local expected_height="$3"
  test -s "$file"
  local width
  local height
  width="$(sips -g pixelWidth "$file" 2>/dev/null | awk '/pixelWidth/ {print $2}')"
  height="$(sips -g pixelHeight "$file" 2>/dev/null | awk '/pixelHeight/ {print $2}')"
  test "$width" = "$expected_width"
  test "$height" = "$expected_height"
}

check_png_size "$OUT/01_features_profile_1280x800.png" 1280 800
check_png_size "$OUT/02_features_inventory_1280x800.png" 1280 800
check_png_size "$OUT/03_features_quality_1280x800.png" 1280 800
check_png_size "$OUT/04_features_settings_1280x800.png" 1280 800
check_png_size "$OUT/05_features_profile_900x700.png" 900 700
check_png_size "$OUT/06_features_settings_full_1280x4200.png" 1280 4200
check_png_size "$OUT/07_features_text_editor_1280x800.png" 1280 800
check_png_size "$OUT/08_features_text_editor_900x700.png" 900 700
check_png_size "$OUT/09_features_text_editor_palette_1280x800.png" 1280 800
test -s "$OUT/07_features_text_editor_ui_tree.json"
test -s "$OUT/08_features_text_editor_ui_tree.json"
test -s "$OUT/09_features_text_editor_palette_ui_tree.json"

check_text_editor_tree() {
  local tree="$1"
  local expected_columns="$2"
  local expected_rows="$3"
  grep -q 'workbench.rail.text_editor.documents' "$tree"
  grep -q 'workbench.toolbar.primary' "$tree"
  grep -q 'workbench.editor.surface.document' "$tree"
  grep -q 'workbench.editor.surface.text' "$tree"
  grep -q 'workbench.inspector.text_editor.options' "$tree"
  grep -q 'workbench.inspector.text_editor.context' "$tree"
  grep -q 'workbench.inspector.text_editor.result' "$tree"
  grep -q 'workbench.inspector.text_editor.receipts' "$tree"
  grep -q 'workbench.fixture_bench.results.expected' "$tree"
  grep -q 'workbench.fixture_bench.results.actual' "$tree"
  grep -q 'workbench.editor.snapshot' "$tree"
  grep -q '"workspace": "text_editor"' "$tree"
  ! grep -q 'workbench.rail.repo' "$tree"
  ! grep -q 'workbench.rail.agent' "$tree"
  ! grep -q 'workbench.inspector.repo' "$tree"
  ! grep -q 'workbench.inspector.agent' "$tree"
  python3 - "$tree" "$expected_columns" "$expected_rows" <<'PY'
import json
import sys

path, expected_columns, expected_rows = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
tree = json.load(open(path))
buttons = []
geometries = {}
nodes = {}

def walk(node):
    ui_path = node.get("uiPath", "")
    if ui_path.startswith("workbench.toolbar.primary.") and ui_path != "workbench.toolbar.primary":
        buttons.append((ui_path, node.get("geometry", {}), node.get("text", "")))
    if ui_path:
        geometries[ui_path] = node.get("geometry", {})
        nodes[ui_path] = node
    for child in node.get("children", []):
        walk(child)

walk(tree)
if len(buttons) != 6:
    raise SystemExit(f"expected 6 primary toolbar actions, saw {len(buttons)}")
labels = [button[2] for button in sorted(buttons, key=lambda item: (item[1].get("y", 0), item[1].get("x", 0)))]
expected_labels = ["Copy", "Markdown", "Prompt", "Fence", "All", "Clean"]
if labels != expected_labels:
    raise SystemExit(f"toolbar labels mismatch: expected {expected_labels}, saw {labels}")
columns = len({button[1].get("x") for button in buttons})
rows = len({button[1].get("y") for button in buttons})
if columns != expected_columns or rows != expected_rows:
    raise SystemExit(f"toolbar grid mismatch: expected {expected_columns}x{expected_rows}, saw {columns}x{rows}")
toolbar = geometries.get("workbench.toolbar.primary", {})
for ui_path, geometry, _label in buttons:
    if geometry.get("x", 0) < 0 or geometry.get("y", 0) < 0:
        raise SystemExit(f"{ui_path} has negative geometry: {geometry}")
    if geometry.get("x", 0) + geometry.get("width", 0) > toolbar.get("width", 0):
        raise SystemExit(f"{ui_path} overflows toolbar width: {geometry} > {toolbar}")
workspace = geometries.get("workbench.editor.workspace", {})
scroll = geometries.get("workbench.editor.scroll", {})
if scroll and workspace and workspace.get("width", 0) > scroll.get("width", 0):
    raise SystemExit(f"workspace width exceeds scroll viewport: {workspace} > {scroll}")
for ui_path in ("workbench.rail.text_editor.documents", "workbench.rail.text_editor.clipboard", "workbench.rail.text_editor.drafts", "workbench.rail.text_editor.fixtures"):
    props = nodes.get(ui_path, {}).get("properties", {})
    if props.get("defaultSize") != 208 or props.get("minSize") != 208:
        raise SystemExit(f"{ui_path} rail descriptor width changed: {props}")
for ui_path in ("workbench.inspector.text_editor.options", "workbench.inspector.text_editor.context", "workbench.inspector.text_editor.result", "workbench.inspector.text_editor.receipts"):
    width = geometries.get(ui_path, {}).get("width", 0)
    if width < 250 or width > 300:
        raise SystemExit(f"{ui_path} inspector panel width outside clamp envelope: {width}")
    props = nodes.get(ui_path, {}).get("properties", {})
    if props.get("defaultSize") != 300 or props.get("minSize") != 280:
        raise SystemExit(f"{ui_path} inspector descriptor clamp changed: {props}")
for ui_path in ("workbench.fixture_bench.results.expected", "workbench.fixture_bench.results.actual"):
    node = nodes.get(ui_path)
    if not node:
        raise SystemExit(f"missing output pane {ui_path}")
    if node.get("className") != "QPlainTextEdit":
        raise SystemExit(f"{ui_path} is not a QPlainTextEdit: {node.get('className')}")
    if not node.get("readOnly"):
        raise SystemExit(f"{ui_path} is not read-only")
    if not node.get("properties", {}).get("scrollsInternally"):
        raise SystemExit(f"{ui_path} missing internal scroll contract")
snapshot = nodes.get("workbench.editor.snapshot", {}).get("properties", {})
for key in ("snapshotLineCount", "snapshotCharacterCount", "snapshotVisibleBlockStart", "snapshotVisibleBlockEnd", "snapshotScrollValue", "snapshotScrollMaximum"):
    if key not in snapshot:
        raise SystemExit(f"snapshot missing {key}: {snapshot}")
if int(snapshot["snapshotVisibleBlockEnd"]) < int(snapshot["snapshotVisibleBlockStart"]):
    raise SystemExit(f"snapshot visible range is inverted: {snapshot}")
PY
}

check_text_editor_tree "$OUT/07_features_text_editor_ui_tree.json" 3 2
check_text_editor_tree "$OUT/08_features_text_editor_ui_tree.json" 2 3

python3 - "$OUT/09_features_text_editor_palette_ui_tree.json" <<'PY'
import json
import sys

tree = json.load(open(sys.argv[1]))
nodes = {}
selected_rows = []
disabled_rows = []

def walk(node):
    ui_path = node.get("uiPath", "")
    if ui_path:
        nodes[ui_path] = node
    if ui_path.startswith("workbench.palette.action."):
        state = node.get("componentState", "")
        if state == "selected":
            selected_rows.append(node)
        if state == "disabled":
            disabled_rows.append(node)
    for child in node.get("children", []):
        walk(child)

walk(tree)
for ui_path in (
    "workbench.palette",
    "workbench.palette.search.input",
    "workbench.palette.section.all",
    "workbench.palette.empty_state",
):
    if ui_path not in nodes:
        raise SystemExit(f"missing command palette uiPath: {ui_path}")
palette = nodes["workbench.palette"]
if palette.get("componentState") != "open":
    raise SystemExit(f"palette did not open for proof: {palette}")
if not selected_rows:
    raise SystemExit("command palette has no selected command row")
if len(nodes) < 4:
    raise SystemExit("command palette tree is unexpectedly sparse")
PY

grep -q '"project_id": "features"' "$ROOT/data/projects.json"
grep -q '"path": "/Users/kogaryu/dev/features"' "$ROOT/data/projects.json"
grep -q '"total": 48' "$ROOT/data/projects.json"
grep -q '"worker_id": "organizer"' "$ROOT/data/projects.json"
grep -q '"worker_id": "planner"' "$ROOT/data/projects.json"
grep -q '"worker_id": "stager"' "$ROOT/data/projects.json"
test -e "$ROOT/data/binder_templates/features_feature_foundry_v1.json"

cat > "$OUT/manifest.txt" <<MANIFEST
Features binder proof
root: $ROOT
bound_repo: /Users/kogaryu/dev/features
project_id: features
template: features_feature_foundry_v1
feature_packs: 48
workers: organizer planner stager
screenshots:
$OUT/01_features_profile_1280x800.png
$OUT/02_features_inventory_1280x800.png
$OUT/03_features_quality_1280x800.png
$OUT/04_features_settings_1280x800.png
$OUT/05_features_profile_900x700.png
$OUT/06_features_settings_full_1280x4200.png
$OUT/07_features_text_editor_1280x800.png
$OUT/08_features_text_editor_900x700.png
$OUT/09_features_text_editor_palette_1280x800.png
ui_tree:
$OUT/07_features_text_editor_ui_tree.json
$OUT/08_features_text_editor_ui_tree.json
$OUT/09_features_text_editor_palette_ui_tree.json
MANIFEST

echo "proof written to $OUT"
