#!/usr/bin/env bash
set -euo pipefail

tag=${1:?usage: set-release-version.sh vMAJOR.MINOR.PATCH}
[[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || {
  echo "invalid release tag: $tag" >&2
  exit 1
}
version=${tag#v}

perl -0pi -e \
  's/(\[package\][^\[]*\nversion = ")[^"]+(")/${1}'"$version"'${2}/' \
  Cargo.toml
perl -0pi -e \
  's/(\[\[package\]\]\nname = "clck"\nversion = ")[^"]+(")/${1}'"$version"'${2}/' \
  Cargo.lock
