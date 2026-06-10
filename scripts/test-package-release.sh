#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/bin" "$tmp/dist"
printf '# test readme\n' > "$tmp/README.md"
printf 'binary\n' > "$tmp/bin/alarm-clock"
printf 'binary\n' > "$tmp/bin/alarm-clock.exe"

(
  cd "$tmp"
  "$root/scripts/package-release.sh" \
    v1.2.3 aarch64-apple-darwin bin/alarm-clock dist
  "$root/scripts/package-release.sh" \
    v1.2.3 x86_64-pc-windows-msvc bin/alarm-clock.exe dist
)

test -f "$tmp/dist/alarm-clock-v1.2.3-macos-aarch64.tar.gz"
test -f "$tmp/dist/alarm-clock-v1.2.3-windows-x86_64.zip"
tar -tzf "$tmp/dist/alarm-clock-v1.2.3-macos-aarch64.tar.gz" |
  grep -qx 'alarm-clock'
tar -tzf "$tmp/dist/alarm-clock-v1.2.3-macos-aarch64.tar.gz" |
  grep -qx 'README.md'
unzip -Z1 "$tmp/dist/alarm-clock-v1.2.3-windows-x86_64.zip" |
  grep -qx 'alarm-clock.exe'
unzip -Z1 "$tmp/dist/alarm-clock-v1.2.3-windows-x86_64.zip" |
  grep -qx 'README.md'

echo "package-release tests passed"
