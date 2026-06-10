#!/usr/bin/env bash
set -euo pipefail

tag=${1:?usage: package-release.sh TAG TARGET EXECUTABLE OUTPUT_DIR}
target=${2:?usage: package-release.sh TAG TARGET EXECUTABLE OUTPUT_DIR}
executable=${3:?usage: package-release.sh TAG TARGET EXECUTABLE OUTPUT_DIR}
output_dir=${4:?usage: package-release.sh TAG TARGET EXECUTABLE OUTPUT_DIR}

[[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] ||
  { echo "invalid release tag: $tag" >&2; exit 1; }

case "$target" in
  aarch64-apple-darwin) suffix=macos-aarch64; executable_name=clck; archive=tar ;;
  x86_64-apple-darwin) suffix=macos-x86_64; executable_name=clck; archive=tar ;;
  aarch64-unknown-linux-musl) suffix=linux-aarch64-musl; executable_name=clck; archive=tar ;;
  x86_64-unknown-linux-musl) suffix=linux-x86_64-musl; executable_name=clck; archive=tar ;;
  x86_64-pc-windows-msvc) suffix=windows-x86_64; executable_name=clck.exe; archive=zip ;;
  *) echo "unsupported release target: $target" >&2; exit 1 ;;
esac

test -f "$executable"
test -f README.md
mkdir -p "$output_dir"
output_dir=$(cd "$output_dir" && pwd)
stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
cp "$executable" "$stage/$executable_name"
cp README.md "$stage/"
entries=("$executable_name" README.md)
for license in LICENSE LICENSE.txt LICENSE.md; do
  if [[ -f "$license" ]]; then
    cp "$license" "$stage/"
    entries+=("$license")
  fi
done

name="clck-${tag}-${suffix}"
if [[ "$archive" == tar ]]; then
  tar -C "$stage" -czf "$output_dir/${name}.tar.gz" "${entries[@]}"
else
  (cd "$stage" && zip -q "$output_dir/${name}.zip" "${entries[@]}")
fi
