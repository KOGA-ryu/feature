#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FEATURE_ROOT="$REPO_ROOT/feature_packs"
BY_SUBJECT_ROOT="$FEATURE_ROOT/by_subject"
INDEX_PATH="$REPO_ROOT/docs/feature_subject_index.md"
TMP_DIR="$(mktemp -d)"
TMP_ENTRIES="$TMP_DIR/entries.tsv"

cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

mkdir -p "$BY_SUBJECT_ROOT"
rm -rf "$BY_SUBJECT_ROOT"
mkdir -p "$BY_SUBJECT_ROOT"

while IFS= read -r -d '' feature_toml; do
  crate_dir="$(dirname "$feature_toml")"
  crate_rel="${crate_dir#$FEATURE_ROOT/}"
  id="$(sed -n 's/^[[:space:]]*id[[:space:]]*=[[:space:]]*"\([^"]*\)"/\1/p' "$feature_toml")"
  if [[ -z "$id" ]]; then
    id="${crate_dir##*/}"
  fi

  while IFS= read -r tag; do
    tag="${tag#subject:}"
    [[ -z "$tag" ]] && continue

    subject_dir="$BY_SUBJECT_ROOT/$tag"
    mkdir -p "$subject_dir"
    ln -sfn "$crate_dir" "$subject_dir/$id"

    feature_name="${id##*.}"
    feature_toml_rel="$crate_rel/feature.toml"

    printf '%s|%s|%s|%s\n' "$tag" "$id" "$feature_toml_rel" "$feature_name" >>"$TMP_ENTRIES"
  done < <(grep -o '"subject:[^"]*"' "$feature_toml" | tr -d '"' || true)
done < <(find "$FEATURE_ROOT" -type f -name feature.toml -print0)

{
  echo '# Feature Subject Index'
  echo
  echo 'Generated from `tags` in each `feature.toml`.'
  echo
} >"$INDEX_PATH"

if [[ -s "$TMP_ENTRIES" ]]; then
  sort -u -t '|' -k1,1 -k2,2 "$TMP_ENTRIES" >"$TMP_DIR/sorted.tsv"

  current_subject=""
  while IFS='|' read -r subject feature_id feature_toml_rel feature_name; do
    if [[ "$subject" != "$current_subject" ]]; then
      echo "## subject:$subject" >>"$INDEX_PATH"
      echo >>"$INDEX_PATH"
      current_subject="$subject"
    fi

    echo "- \`$feature_id\` (\`$feature_toml_rel\`) — \`$feature_name\`" >>"$INDEX_PATH"
  done <"$TMP_DIR/sorted.tsv"

else
  echo '_No subject-tagged features found._' >>"$INDEX_PATH"
fi

printf 'Rebuilt subject views in %s and index at %s\n' "$BY_SUBJECT_ROOT" "$INDEX_PATH"
