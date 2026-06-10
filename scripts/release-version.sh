#!/usr/bin/env bash
set -euo pipefail

message_file=${1:?usage: release-version.sh MESSAGE_FILE TAGS_FILE}
tags_file=${2:?usage: release-version.sh MESSAGE_FILE TAGS_FILE}
message=$(cat "$message_file")

if grep -Eqi '\[skip release\]' "$message_file"; then
  echo "skip=true"
  exit 0
fi

latest=$(
  grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$' "$tags_file" |
    sort -t. -k1.2,1n -k2,2n -k3,3n |
    tail -n 1 ||
    true
)
latest=${latest:-v0.1.0}
version=${latest#v}
IFS=. read -r major minor patch <<< "$version"
subject=${message%%$'\n'*}
breaking_subject='^[[:alnum:]_-]+(\([^)]*\))?!:'
feature_subject='^feat(\([^)]*\))?:'

if [[ "$subject" =~ $breaking_subject ]] ||
  grep -Eq '^BREAKING CHANGE:' "$message_file"; then
  ((major += 1))
  minor=0
  patch=0
elif [[ "$subject" =~ $feature_subject ]]; then
  ((minor += 1))
  patch=0
else
  ((patch += 1))
fi

echo "skip=false"
echo "tag=v${major}.${minor}.${patch}"
