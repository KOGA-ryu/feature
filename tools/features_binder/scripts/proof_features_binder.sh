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
  "$OUT/10_features_text_editor_cleanup_1280x800.png" \
  "$OUT/10_features_text_editor_cleanup_ui_tree.json" \
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
capture "10_features_text_editor_cleanup_1280x800" --size 1280x800 --no-settings --worker organizer --tab "Text Editor" --run-text-editor-cleanup-proof --ui-tree-dump "$OUT/10_features_text_editor_cleanup_ui_tree.json"

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
check_png_size "$OUT/10_features_text_editor_cleanup_1280x800.png" 1280 800
test -s "$OUT/07_features_text_editor_ui_tree.json"
test -s "$OUT/08_features_text_editor_ui_tree.json"
test -s "$OUT/09_features_text_editor_palette_ui_tree.json"
test -s "$OUT/10_features_text_editor_cleanup_ui_tree.json"

check_text_editor_tree() {
  local tree="$1"
  local expected_columns="$2"
  local expected_rows="$3"
  grep -q 'workbench.rail.text_editor.documents' "$tree"
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
palette_paths = []
visible_text = []

def walk(node):
    ui_path = node.get("uiPath", "")
    text = node.get("text", "")
    if text:
        visible_text.append(text)
    if ui_path.startswith("workbench.toolbar.primary"):
        buttons.append((ui_path, node.get("geometry", {}), node.get("text", "")))
    if ui_path.startswith("workbench.palette"):
        palette_paths.append(ui_path)
    if ui_path:
        geometries[ui_path] = node.get("geometry", {})
        nodes[ui_path] = node
    for child in node.get("children", []):
        walk(child)

walk(tree)
if buttons:
    raise SystemExit(f"text tool toolbar buttons still render: {buttons}")
if palette_paths:
    raise SystemExit(f"hotkey palette should be hidden in default captures: {palette_paths}")
for forbidden_text in ("Dex Home", "Text Editor / Blank Workspace"):
    if any(forbidden_text in text for text in visible_text):
        raise SystemExit(f"redundant chrome label still visible in Text Editor capture: {forbidden_text}")
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
check_text_editor_tree "$OUT/10_features_text_editor_cleanup_ui_tree.json" 3 2

python3 - "$OUT/09_features_text_editor_palette_ui_tree.json" <<'PY'
import json
import sys

tree = json.load(open(sys.argv[1]))
nodes = {}
selected_rows = []
disabled_rows = []
palette_buttons = []

def walk(node):
    ui_path = node.get("uiPath", "")
    if ui_path:
        nodes[ui_path] = node
    if ui_path.startswith("workbench.palette") and node.get("className") in ("QPushButton", "QToolButton"):
        palette_buttons.append(node)
    if ui_path.startswith("workbench.palette.results.action."):
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
    "workbench.palette.results",
):
    if ui_path not in nodes:
        raise SystemExit(f"missing command palette uiPath: {ui_path}")
palette = nodes["workbench.palette"]
if palette.get("componentState") != "open":
    raise SystemExit(f"palette did not open for proof: {palette}")
props = palette.get("properties", {})
if props.get("paletteOpen") is not True:
    raise SystemExit(f"paletteOpen proof property is not true: {props}")
if props.get("focusedSurface") != "Palette":
    raise SystemExit(f"palette focusedSurface proof property changed: {props}")
if props.get("selectedActionId") in ("", "none", None):
    raise SystemExit(f"palette selectedActionId is missing: {props}")
if int(props.get("selectedRowIndex", -1)) < 0:
    raise SystemExit(f"palette selectedRowIndex is missing: {props}")
if props.get("hotkeyOnly") is not True:
    raise SystemExit(f"palette is not marked hotkey-only: {props}")
if props.get("popout") is not True:
    raise SystemExit(f"palette is not marked as a popout: {props}")
if props.get("hasLauncherButton") is not False:
    raise SystemExit(f"palette still reports a launcher button: {props}")
if props.get("hasTitle") is not False:
    raise SystemExit(f"palette still reports a title: {props}")
if props.get("hasFrameBorder") is not False:
    raise SystemExit(f"palette still reports a frame border: {props}")
if palette_buttons:
    raise SystemExit(f"palette contains button widgets: {palette_buttons}")
if any(path.startswith("workbench.palette.filter.") for path in nodes):
    raise SystemExit("palette still renders filter pill/dropdown paths")
if any(path.startswith("workbench.palette.dropdown") for path in nodes):
    raise SystemExit("palette still renders dropdown launcher/menu paths")
if any(path.startswith("workbench.toolbar.primary") for path in nodes):
    raise SystemExit("text tool toolbar still renders")
if not selected_rows:
    raise SystemExit("palette has no selected command row")
palette_children = palette.get("children", [])
if len(palette_children) < 2:
    raise SystemExit(f"palette does not expose search plus results: {palette}")
if palette_children[0].get("uiPath") != "workbench.palette.search.input":
    raise SystemExit(f"palette first visible child is not search input: {palette_children[0]}")
search_geometry = nodes["workbench.palette.search.input"].get("geometry", {})
results_geometry = nodes["workbench.palette.results"].get("geometry", {})
if results_geometry.get("y", 0) <= search_geometry.get("y", 0):
    raise SystemExit(f"palette results are not below search: search={search_geometry} results={results_geometry}")
root_geometry = palette.get("geometry", {})
window_geometry = tree.get("geometry", {})
if window_geometry and abs((root_geometry.get("width", 0) / 2 + root_geometry.get("x", 0)) - (window_geometry.get("width", 0) / 2)) > 32:
    raise SystemExit(f"palette popout is not centered in main window: palette={root_geometry} window={window_geometry}")
if props.get("positionAnchor") != "main_window":
    raise SystemExit(f"palette popout is not anchored to main window: {props}")
selected = selected_rows[0]
selected_props = selected.get("properties", {})
for key in ("actionId", "actionLabel", "displayText", "category", "iconName", "hotkeyLabel", "disabledReason", "paletteRole", "paletteActionIndex", "paletteActionEnabled", "accessibleLabel"):
    if key not in selected_props:
        raise SystemExit(f"selected palette row missing {key}: {selected_props}")
if selected_props.get("paletteRole") != "menuitem":
    raise SystemExit(f"selected palette row is not a menuitem: {selected_props}")
if selected_props.get("actionId") != props.get("selectedActionId"):
    raise SystemExit(f"selected row/action proof mismatch: {selected_props} vs {props}")
display_text = selected_props.get("displayText", "")
if selected_props.get("actionLabel") not in display_text:
    raise SystemExit(f"selected row text does not include action label: {selected}")
if "[" + selected_props.get("category", "") + "]" not in display_text:
    raise SystemExit(f"selected row text does not include category: {selected}")
if not selected_props.get("accessibleLabel"):
    raise SystemExit(f"selected row missing accessibleName: {selected}")
if selected.get("className") != "QFrame":
    raise SystemExit(f"selected palette row is not a QFrame menu row: {selected}")
if selected.get("geometry", {}).get("height", 999) > 24:
    raise SystemExit(f"selected palette row is outside compact row height envelope: {selected}")
if disabled_rows and "disabledReason" not in disabled_rows[0].get("properties", {}):
    raise SystemExit(f"disabled row missing disabledReason: {disabled_rows[0]}")
if len(nodes) < 4:
    raise SystemExit("command palette tree is unexpectedly sparse")
PY

python3 - "$OUT/10_features_text_editor_cleanup_ui_tree.json" <<'PY'
import json
import sys

tree = json.load(open(sys.argv[1]))
nodes = {}

def walk(node):
    ui_path = node.get("uiPath", "")
    if ui_path:
        nodes[ui_path] = node
    for child in node.get("children", []):
        walk(child)

walk(tree)
actual = nodes.get("workbench.fixture_bench.results.actual")
receipt = nodes.get("workbench.fixture_bench.results.expected")
status = nodes.get("workbench.fixture_bench.runner.status")
if not actual or not receipt or not status:
    raise SystemExit("cleanup proof missing result panes or status")
actual_props = actual.get("properties", {})
receipt_props = receipt.get("properties", {})
if actual_props.get("previewMode") != "cleanup_before_after":
    raise SystemExit(f"actual pane did not enter cleanup preview mode: {actual_props}")
if actual_props.get("cleanupActionId") != "text.clean_basic":
    raise SystemExit(f"actual pane cleanup action mismatch: {actual_props}")
if actual_props.get("cleanupBeforeLabel") != "BEFORE" or actual_props.get("cleanupAfterLabel") != "AFTER":
    raise SystemExit(f"cleanup preview labels missing: {actual_props}")
if int(actual_props.get("cleanupChangeCount", 0)) <= 0:
    raise SystemExit(f"cleanup change count did not prove changes: {actual_props}")
if int(actual_props.get("cleanupWarningCount", -1)) < 0:
    raise SystemExit(f"cleanup warning count missing: {actual_props}")
if receipt_props.get("receiptMode") != "cleanup_receipt":
    raise SystemExit(f"receipt pane did not enter cleanup receipt mode: {receipt_props}")
if receipt_props.get("receiptActionId") != "text.clean_basic":
    raise SystemExit(f"receipt pane cleanup action mismatch: {receipt_props}")
if receipt_props.get("receiptChangeCount") != actual_props.get("cleanupChangeCount"):
    raise SystemExit(f"receipt/change proof mismatch: {receipt_props} vs {actual_props}")
if "text.clean_basic" not in status.get("text", ""):
    raise SystemExit(f"cleanup status missing action id: {status}")
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
$OUT/10_features_text_editor_cleanup_1280x800.png
ui_tree:
$OUT/07_features_text_editor_ui_tree.json
$OUT/08_features_text_editor_ui_tree.json
$OUT/09_features_text_editor_palette_ui_tree.json
$OUT/10_features_text_editor_cleanup_ui_tree.json
MANIFEST

echo "proof written to $OUT"
