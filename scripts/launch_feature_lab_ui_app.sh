#!/bin/zsh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${0}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
APP_NAME="Feature Lab.app"
APP_ROOT="${REPO_ROOT}/target/macos-app/${APP_NAME}"
CONTENTS_DIR="${APP_ROOT}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
PLIST_SOURCE="${REPO_ROOT}/crates/feature_lab_ui/macos/Info.plist"
BINARY_SOURCE="${REPO_ROOT}/target/debug/feature_lab_ui"
BINARY_LINK="${MACOS_DIR}/feature_lab_ui"

cd "${REPO_ROOT}"

cargo build -p feature_lab_ui

mkdir -p "${MACOS_DIR}"
cp "${PLIST_SOURCE}" "${CONTENTS_DIR}/Info.plist"
ln -sfn "${BINARY_SOURCE}" "${BINARY_LINK}"

open -na "${APP_ROOT}"
