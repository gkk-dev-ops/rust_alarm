#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

calculate() {
  local message=$1
  shift
  printf '%s\n' "$message" > "$tmp/message"
  printf '%s\n' "$@" > "$tmp/tags"
  "$root/scripts/release-version.sh" "$tmp/message" "$tmp/tags"
}

assert_output() {
  local expected=$1
  shift
  local actual
  actual=$("$@")
  if [[ "$actual" != "$expected" ]]; then
    printf 'expected:\n%s\nactual:\n%s\n' "$expected" "$actual" >&2
    exit 1
  fi
}

assert_output $'skip=false\ntag=v0.1.1' calculate 'Fix typo'
assert_output $'skip=false\ntag=v0.2.0' calculate 'feat: add scheduling'
assert_output $'skip=false\ntag=v2.0.0' calculate 'feat!: replace CLI' v1.4.3
assert_output $'skip=false\ntag=v2.0.0' calculate \
  $'fix: retain CLI\n\nBREAKING CHANGE: remove old flags' v1.4.3
assert_output $'skip=false\ntag=v1.5.0' calculate 'feat(cli): add command' v1.4.3
assert_output $'skip=false\ntag=v1.4.4' calculate 'docs: update readme' v1.4.3
assert_output $'skip=false\ntag=v2.1.0' calculate \
  'feat: use highest tag' v1.9.9 invalid v2.0.4
assert_output $'skip=true' calculate 'docs: no release [SKIP RELEASE]' v2.0.4

ci="$root/.github/workflows/ci.yml"
test -f "$ci"
grep -Fq 'contents: read' "$ci"
grep -Fq 'libasound2-dev' "$ci"
grep -Fq 'pkg-config' "$ci"
grep -Fq 'cargo test --locked' "$ci"
grep -Fq 'cargo clippy --locked --all-targets -- -D warnings' "$ci"

release="$root/.github/workflows/release.yml"
test -f "$release"
grep -Fq 'contents: write' "$release"
grep -Fq 'cancel-in-progress: false' "$release"
grep -Fq 'workflow_run:' "$release"
grep -Fq 'workflow_dispatch:' "$release"
grep -Fq 'houseabsolute/actions-rust-cross@v1' "$release"
grep -Fq 'actions/upload-artifact@v4' "$release"
grep -Fq 'actions/download-artifact@v4' "$release"
grep -Fq 'SHA256SUMS' "$release"
grep -Fq 'gh release upload' "$release"
grep -Fq 'args: --locked --release --no-default-features' "$release"

readme="$root/README.md"
grep -Fq 'cargo install clck --locked' "$readme"
grep -Fq 'https://github.com/gkk-dev-ops/clck/releases/latest' "$readme"
grep -Fq 'clck-vX.Y.Z-linux-x86_64-musl.tar.gz' "$readme"

releases="$root/docs/releases.md"
test -f "$releases"
grep -Fq '[skip release]' "$releases"
grep -Fq 'workflow_dispatch' "$releases"
grep -Fq 'contents: write' "$releases"
grep -Fq 'gh workflow run Release' "$releases"
grep -Fq 'releases/latest' "$root/docs/manual-testing.md"
grep -Fq 'SHA256SUMS' "$root/docs/manual-testing.md"

echo "release-version tests passed"
