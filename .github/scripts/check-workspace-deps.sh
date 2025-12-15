#!/bin/bash
set -euo pipefail

# Check that all workspace crates share the workspace version.
# Also check that workspace.dependencies versions match the workspace version.

WS_VERSION=$(
  awk '
    $0 ~ /^\[workspace\.package\]/ {in_section=1; next}
    in_section && $0 ~ /^\[/ {in_section=0}
    in_section && $0 ~ /^version[[:space:]]*=/ {
      gsub(/"/, "", $0);
      sub(/.*=[[:space:]]*/, "", $0);
      print $0;
      exit
    }
  ' Cargo.toml
)

if [ -z "${WS_VERSION:-}" ]; then
  echo "❌ ERROR: failed to read [workspace.package] version from Cargo.toml"
  exit 1
fi

pkg_version() {
  local crate="$1"
  cargo metadata --no-deps --format-version 1 \
    | python3 -c "import json,sys; d=json.load(sys.stdin); print([p['version'] for p in d['packages'] if p['name']=='$crate'][0])"
}

DE_VERSION="$(pkg_version dynamic_expressions)"
SR_VERSION="$(pkg_version symbolic_regression)"
SRW_VERSION="$(pkg_version symbolic_regression_wasm)"

echo "workspace version: $WS_VERSION"
echo "dynamic_expressions version: $DE_VERSION"
echo "symbolic_regression version: $SR_VERSION"
echo "symbolic_regression_wasm version: $SRW_VERSION"

if [ "$WS_VERSION" != "$DE_VERSION" ] || [ "$WS_VERSION" != "$SR_VERSION" ]; then
  echo "❌ ERROR: core workspace crates must share the same version as [workspace.package]"
  exit 1
fi

WS_DE_DEP_VERSION=$(
  awk '
    $0 ~ /^\[workspace\.dependencies\]/ {in_section=1; next}
    in_section && $0 ~ /^\[/ {in_section=0}
    in_section && $0 ~ /^dynamic_expressions[[:space:]]*=/ {print $0; exit}
  ' Cargo.toml \
  | sed -nE 's/.*version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p'
)
WS_SR_DEP_VERSION=$(
  awk '
    $0 ~ /^\[workspace\.dependencies\]/ {in_section=1; next}
    in_section && $0 ~ /^\[/ {in_section=0}
    in_section && $0 ~ /^symbolic_regression[[:space:]]*=/ {print $0; exit}
  ' Cargo.toml \
  | sed -nE 's/.*version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p'
)

echo "workspace dependency dynamic_expressions version: $WS_DE_DEP_VERSION"
echo "workspace dependency symbolic_regression version: $WS_SR_DEP_VERSION"

if [ "$WS_VERSION" != "$WS_DE_DEP_VERSION" ] || [ "$WS_VERSION" != "$WS_SR_DEP_VERSION" ]; then
  echo "❌ ERROR: workspace.dependencies versions must match [workspace.package].version"
  exit 1
fi

echo "✅ Workspace versions are in sync"
