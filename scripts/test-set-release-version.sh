#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

cp "$root/Cargo.toml" "$root/Cargo.lock" "$tmp/"
(
  cd "$tmp"
  CARGO_HOME="$tmp/empty-cargo-home" "$root/scripts/set-release-version.sh" v2.3.4
  grep -Fqx 'version = "2.3.4"' Cargo.toml
  grep -A2 'name = "clck"' Cargo.lock | grep -Fqx 'version = "2.3.4"'
)

if (
  cd "$tmp"
  "$root/scripts/set-release-version.sh" 2.3.4
) 2>/dev/null; then
  echo "set-release-version accepted a tag without a v prefix" >&2
  exit 1
fi

echo "set-release-version tests passed"
