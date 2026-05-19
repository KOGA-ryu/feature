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
capture "08_features_text_editor_900x700" --size 900x700 --no-settings --worker organizer --tab "Text Editor"

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
test -s "$OUT/07_features_text_editor_ui_tree.json"
grep -q 'workbench.rail.text_editor.documents' "$OUT/07_features_text_editor_ui_tree.json"
grep -q 'workbench.toolbar.primary' "$OUT/07_features_text_editor_ui_tree.json"
grep -q 'workbench.editor.surface.document' "$OUT/07_features_text_editor_ui_tree.json"
grep -q 'workbench.inspector.text_editor.options' "$OUT/07_features_text_editor_ui_tree.json"
grep -q 'workbench.fixture_bench.results.expected' "$OUT/07_features_text_editor_ui_tree.json"
grep -q 'workbench.fixture_bench.results.actual' "$OUT/07_features_text_editor_ui_tree.json"
grep -q '"workspace": "text_editor"' "$OUT/07_features_text_editor_ui_tree.json"

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
ui_tree:
$OUT/07_features_text_editor_ui_tree.json
MANIFEST

echo "proof written to $OUT"
