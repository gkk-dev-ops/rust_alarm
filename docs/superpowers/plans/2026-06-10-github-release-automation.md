# GitHub Release Automation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add validated GitHub Actions CI and automatic semantic-versioned GitHub Releases containing checksummed binaries for macOS, Linux, and Windows, plus complete user and maintainer documentation.

**Architecture:** Keep version calculation and archive assembly in locally testable Bash scripts, while GitHub Actions owns orchestration, permissions, concurrency, cross-platform compilation, artifact transfer, and release publication. CI validates Rust and release tooling on pull requests and `master`; a separate serialized release workflow runs only after successful `master` CI or a manual existing-tag rebuild.

**Tech Stack:** GitHub Actions, Bash, Git, GitHub CLI, Rust/Cargo, `actions/checkout@v6`, `actions/upload-artifact@v4`, `actions/download-artifact@v4`, `Swatinem/rust-cache@v2`, `dtolnay/rust-toolchain@stable`, `houseabsolute/actions-rust-cross@v1`, and `rhysd/actionlint`.

---

## File Structure

- `scripts/release-version.sh`: pure semantic-version and `[skip release]` calculation from a commit-message file and tag list.
- `scripts/test-release-version.sh`: regression tests for initial, patch, minor, major, highest-tag, and skip behavior.
- `scripts/package-release.sh`: map Rust targets to release archive names and assemble archives from built executables.
- `scripts/test-package-release.sh`: verify archive names and required archive contents.
- `.github/workflows/ci.yml`: read-only validation for pull requests, `master`, and manual dispatch.
- `.github/workflows/release.yml`: serialized automatic/manual tag preparation, five-platform build matrix, checksums, and GitHub Release publication.
- `README.md`: release download link, platform matrix, installation, updates, checksums, Gatekeeper, Cargo install, and maintainer link.
- `docs/releases.md`: maintainer release operation and recovery guide.
- `docs/manual-testing.md`: downloaded-release verification checklist and first-release validation record.

### Task 1: Implement And Test Semantic Version Calculation

**Files:**
- Create: `scripts/release-version.sh`
- Create: `scripts/test-release-version.sh`

- [ ] **Step 1: Write the failing version-calculation test script**

Create `scripts/test-release-version.sh`:

```bash
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
assert_output $'skip=false\ntag=v2.0.0' calculate $'fix: retain CLI\n\nBREAKING CHANGE: remove old flags' v1.4.3
assert_output $'skip=false\ntag=v1.5.0' calculate 'feat(cli): add command' v1.4.3
assert_output $'skip=false\ntag=v1.4.4' calculate 'docs: update readme' v1.4.3
assert_output $'skip=false\ntag=v2.1.0' calculate 'feat: use highest tag' v1.9.9 invalid v2.0.4
assert_output $'skip=true' calculate 'docs: no release [SKIP RELEASE]' v2.0.4

echo "release-version tests passed"
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `bash scripts/test-release-version.sh`

Expected: FAIL because `scripts/release-version.sh` does not exist.

- [ ] **Step 3: Implement the version calculator**

Create `scripts/release-version.sh`:

```bash
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
```

Make both scripts executable:

```bash
chmod +x scripts/release-version.sh scripts/test-release-version.sh
```

The workflow will generate `TAGS_FILE` with `git tag --list`, so the script remains deterministic and locally testable.

- [ ] **Step 4: Verify version behavior**

Run: `bash scripts/test-release-version.sh && bash -n scripts/release-version.sh scripts/test-release-version.sh`

Expected: PASS and print `release-version tests passed`.

- [ ] **Step 5: Commit**

```bash
git add scripts/release-version.sh scripts/test-release-version.sh
git commit -m "Add release version calculation"
```

### Task 2: Implement And Test Release Archive Assembly

**Files:**
- Create: `scripts/package-release.sh`
- Create: `scripts/test-package-release.sh`

- [ ] **Step 1: Write the failing package test script**

Create `scripts/test-package-release.sh`:

```bash
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
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `bash scripts/test-package-release.sh`

Expected: FAIL because `scripts/package-release.sh` does not exist.

- [ ] **Step 3: Implement archive-name mapping and packaging**

Create `scripts/package-release.sh`:

```bash
#!/usr/bin/env bash
set -euo pipefail

tag=${1:?usage: package-release.sh TAG TARGET EXECUTABLE OUTPUT_DIR}
target=${2:?usage: package-release.sh TAG TARGET EXECUTABLE OUTPUT_DIR}
executable=${3:?usage: package-release.sh TAG TARGET EXECUTABLE OUTPUT_DIR}
output_dir=${4:?usage: package-release.sh TAG TARGET EXECUTABLE OUTPUT_DIR}

[[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] ||
  { echo "invalid release tag: $tag" >&2; exit 1; }

case "$target" in
  aarch64-apple-darwin) suffix=macos-aarch64; executable_name=alarm-clock; archive=tar ;;
  x86_64-apple-darwin) suffix=macos-x86_64; executable_name=alarm-clock; archive=tar ;;
  aarch64-unknown-linux-musl) suffix=linux-aarch64-musl; executable_name=alarm-clock; archive=tar ;;
  x86_64-unknown-linux-musl) suffix=linux-x86_64-musl; executable_name=alarm-clock; archive=tar ;;
  x86_64-pc-windows-msvc) suffix=windows-x86_64; executable_name=alarm-clock.exe; archive=zip ;;
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

name="alarm-clock-${tag}-${suffix}"
if [[ "$archive" == tar ]]; then
  tar -C "$stage" -czf "$output_dir/${name}.tar.gz" "${entries[@]}"
else
  (cd "$stage" && zip -q "$output_dir/${name}.zip" "${entries[@]}")
fi
```

Explicitly listing `entries` keeps all archive contents at archive root without
a leading `./`.

Make both scripts executable:

```bash
chmod +x scripts/package-release.sh scripts/test-package-release.sh
```

- [ ] **Step 4: Verify archive behavior**

Run:

```bash
bash scripts/test-package-release.sh
bash -n scripts/package-release.sh scripts/test-package-release.sh
```

Expected: PASS and print `package-release tests passed`.

- [ ] **Step 5: Commit**

```bash
git add scripts/package-release.sh scripts/test-package-release.sh
git commit -m "Add release archive packaging"
```

### Task 3: Add Read-Only CI Workflow

**Files:**
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: Add a failing workflow-presence assertion**

Add this check to the end of `scripts/test-release-version.sh`, before the success message:

```bash
test -f "$root/.github/workflows/ci.yml"
grep -Fq 'contents: read' "$root/.github/workflows/ci.yml"
grep -Fq 'cargo test --locked' "$root/.github/workflows/ci.yml"
grep -Fq 'cargo clippy --locked --all-targets -- -D warnings' "$root/.github/workflows/ci.yml"
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `bash scripts/test-release-version.sh`

Expected: FAIL because `.github/workflows/ci.yml` does not exist.

- [ ] **Step 3: Create CI workflow**

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  pull_request:
    branches: [master]
  push:
    branches: [master]
  workflow_dispatch:

permissions:
  contents: read

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - name: Validate GitHub Actions workflows
        run: go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7
      - name: Test release tooling
        run: |
          bash scripts/test-release-version.sh
          bash scripts/test-package-release.sh
          bash -n scripts/*.sh
      - run: cargo fmt --check
      - run: cargo test --locked
      - run: cargo clippy --locked --all-targets -- -D warnings
      - run: cargo build --locked --release
```

This workflow has read-only repository permissions, uses the checked-in lockfile, caches Cargo outputs, and runs for all three required triggers.

- [ ] **Step 4: Verify CI workflow and local commands**

Run:

```bash
bash scripts/test-release-version.sh
go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

Expected: all commands PASS.

- [ ] **Step 5: Commit**

```bash
git add .github/workflows/ci.yml scripts/test-release-version.sh
git commit -m "Add GitHub Actions CI"
```

### Task 4: Add Serialized Automatic And Manual Release Workflow

**Files:**
- Create: `.github/workflows/release.yml`
- Modify: `scripts/test-release-version.sh`

- [ ] **Step 1: Add failing release-workflow assertions**

Add these checks to `scripts/test-release-version.sh`:

```bash
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
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `bash scripts/test-release-version.sh`

Expected: FAIL because `.github/workflows/release.yml` does not exist.

- [ ] **Step 3: Create workflow triggers, permissions, concurrency, and prepare job**

Start `.github/workflows/release.yml` with:

```yaml
name: Release

on:
  workflow_run:
    workflows: [CI]
    types: [completed]
  workflow_dispatch:
    inputs:
      tag:
        description: Existing vMAJOR.MINOR.PATCH tag to rebuild
        required: true
        type: string

permissions:
  contents: write

concurrency:
  group: alarm-clock-release
  cancel-in-progress: false

jobs:
  prepare:
    if: >-
      github.event_name == 'workflow_dispatch' ||
      (github.event.workflow_run.conclusion == 'success' &&
       github.event.workflow_run.event == 'push' &&
       github.event.workflow_run.head_branch == 'master')
    runs-on: ubuntu-latest
    outputs:
      skip: ${{ steps.prepare.outputs.skip }}
      tag: ${{ steps.prepare.outputs.tag }}
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 0
          ref: ${{ github.event_name == 'workflow_run' && github.event.workflow_run.head_sha || 'master' }}
      - id: prepare
        env:
          EVENT_NAME: ${{ github.event_name }}
          MANUAL_TAG: ${{ inputs.tag }}
          HEAD_SHA: ${{ github.event.workflow_run.head_sha }}
        run: |
          set -euo pipefail
          if [[ "$EVENT_NAME" == workflow_dispatch ]]; then
            [[ "$MANUAL_TAG" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] ||
              { echo "invalid release tag: $MANUAL_TAG" >&2; exit 1; }
            git rev-parse --verify "refs/tags/$MANUAL_TAG" >/dev/null ||
              { echo "release tag does not exist: $MANUAL_TAG" >&2; exit 1; }
            echo "skip=false" >> "$GITHUB_OUTPUT"
            echo "tag=$MANUAL_TAG" >> "$GITHUB_OUTPUT"
            exit 0
          fi

          git show -s --format=%B "$HEAD_SHA" > "$RUNNER_TEMP/message"
          git tag --list > "$RUNNER_TEMP/tags"
          result=$(scripts/release-version.sh "$RUNNER_TEMP/message" "$RUNNER_TEMP/tags")
          echo "$result" >> "$GITHUB_OUTPUT"
          grep -qx 'skip=true' <<< "$result" && exit 0
          tag=$(sed -n 's/^tag=//p' <<< "$result")
          git config user.name "github-actions[bot]"
          git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
          git tag "$tag" "$HEAD_SHA"
          git push origin "$tag"
```

The automatic path creates the tag before platform builds. If a later build fails, the tag remains and the same tag can be rebuilt through manual dispatch.

- [ ] **Step 4: Add five-target build matrix**

Add:

```yaml
  build:
    needs: prepare
    if: needs.prepare.outputs.skip == 'false'
    strategy:
      fail-fast: false
      matrix:
        include:
          - runner: macos-14
            target: aarch64-apple-darwin
            executable: alarm-clock
            cross: false
          - runner: macos-14
            target: x86_64-apple-darwin
            executable: alarm-clock
            cross: false
          - runner: ubuntu-latest
            target: aarch64-unknown-linux-musl
            executable: alarm-clock
            cross: true
          - runner: ubuntu-latest
            target: x86_64-unknown-linux-musl
            executable: alarm-clock
            cross: true
          - runner: windows-2022
            target: x86_64-pc-windows-msvc
            executable: alarm-clock.exe
            cross: false
    runs-on: ${{ matrix.runner }}
    steps:
      - uses: actions/checkout@v6
        with:
          ref: ${{ needs.prepare.outputs.tag }}
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - uses: Swatinem/rust-cache@v2
        with:
          key: release-${{ matrix.target }}
      - name: Build Linux target
        if: matrix.cross
        uses: houseabsolute/actions-rust-cross@v1
        with:
          command: build
          target: ${{ matrix.target }}
          args: --locked --release
      - name: Build native or Apple target
        if: ${{ !matrix.cross }}
        run: cargo build --locked --release --target ${{ matrix.target }}
      - name: Stage executable
        shell: bash
        run: |
          mkdir -p staged
          cp "target/${{ matrix.target }}/release/${{ matrix.executable }}" staged/
      - uses: actions/upload-artifact@v4
        with:
          name: binary-${{ matrix.target }}
          path: staged/${{ matrix.executable }}
          if-no-files-found: error
```

- [ ] **Step 5: Add checksum and publish job**

Add:

```yaml
  publish:
    needs: [prepare, build]
    if: needs.prepare.outputs.skip == 'false'
    runs-on: ubuntu-latest
    env:
      GH_TOKEN: ${{ github.token }}
      TAG: ${{ needs.prepare.outputs.tag }}
    steps:
      - uses: actions/checkout@v6
        with:
          ref: ${{ env.TAG }}
      - uses: actions/download-artifact@v4
        with:
          pattern: binary-*
          path: raw
      - name: Package release archives
        run: |
          set -euo pipefail
          mkdir -p dist
          scripts/package-release.sh "$TAG" aarch64-apple-darwin \
            raw/binary-aarch64-apple-darwin/alarm-clock dist
          scripts/package-release.sh "$TAG" x86_64-apple-darwin \
            raw/binary-x86_64-apple-darwin/alarm-clock dist
          scripts/package-release.sh "$TAG" aarch64-unknown-linux-musl \
            raw/binary-aarch64-unknown-linux-musl/alarm-clock dist
          scripts/package-release.sh "$TAG" x86_64-unknown-linux-musl \
            raw/binary-x86_64-unknown-linux-musl/alarm-clock dist
          scripts/package-release.sh "$TAG" x86_64-pc-windows-msvc \
            raw/binary-x86_64-pc-windows-msvc/alarm-clock.exe dist
          (cd dist && sha256sum alarm-clock-* > SHA256SUMS)
          test "$(find dist -maxdepth 1 -type f | wc -l)" -eq 6
      - name: Create or replace GitHub Release artifacts
        run: |
          set -euo pipefail
          if gh release view "$TAG" >/dev/null 2>&1; then
            gh release upload "$TAG" dist/* --clobber
          else
            gh release create "$TAG" dist/* --verify-tag --generate-notes --title "$TAG"
          fi
```

Because `publish` depends on the complete build matrix and packages all five downloaded executables before invoking `gh release`, no incomplete release is created after a platform failure. Workflow-created tags do not retrigger this workflow because it has no tag-push trigger.

- [ ] **Step 6: Verify release workflow**

Run:

```bash
bash scripts/test-release-version.sh
go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7
```

Expected: PASS with no actionlint findings.

- [ ] **Step 7: Commit**

```bash
git add .github/workflows/release.yml scripts/test-release-version.sh
git commit -m "Add automated GitHub releases"
```

### Task 5: Add User Installation And Update Documentation

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Add failing README assertions**

Add to `scripts/test-release-version.sh`:

```bash
readme="$root/README.md"
grep -Fq 'https://github.com/gkk-dev-ops/clck/releases/latest' "$readme"
grep -Fq 'SHA256SUMS' "$readme"
grep -Fq 'xattr -d com.apple.quarantine' "$readme"
grep -Fq 'docs/releases.md' "$readme"
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `bash scripts/test-release-version.sh`

Expected: FAIL because README release installation documentation is absent.

- [ ] **Step 3: Expand README installation documentation**

Update `README.md` with these sections and exact artifact mappings:

```markdown
## Download

Download the latest prebuilt release:

https://github.com/gkk-dev-ops/clck/releases/latest

| Platform | Architecture | Artifact |
| --- | --- | --- |
| macOS | Apple Silicon | `alarm-clock-vX.Y.Z-macos-aarch64.tar.gz` |
| macOS | Intel | `alarm-clock-vX.Y.Z-macos-x86_64.tar.gz` |
| Linux | ARM64 musl | `alarm-clock-vX.Y.Z-linux-aarch64-musl.tar.gz` |
| Linux | x86_64 musl | `alarm-clock-vX.Y.Z-linux-x86_64-musl.tar.gz` |
| Windows | x86_64 | `alarm-clock-vX.Y.Z-windows-x86_64.zip` |
```

Document:

- macOS/Linux download, extraction, `chmod +x`, and installation into a directory on `PATH`.
- Windows ZIP extraction and adding the chosen directory to `PATH`.
- Cargo source installation with `cargo install --git https://github.com/gkk-dev-ops/clck.git --locked`.
- Updating by downloading/replacing the binary or rerunning Cargo install with `--force`.
- `shasum -a 256 -c SHA256SUMS` on macOS and `sha256sum -c SHA256SUMS` on Linux.
- Windows checksum verification with `Get-FileHash -Algorithm SHA256`.
- Unsigned macOS Gatekeeper recovery using `xattr -d com.apple.quarantine /path/to/alarm-clock`, with a warning to verify the checksum first.
- A maintainer link: `See [docs/releases.md](docs/releases.md) for CI, versioning, and release operations.`

- [ ] **Step 4: Verify README requirements**

Run: `bash scripts/test-release-version.sh`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add README.md scripts/test-release-version.sh
git commit -m "Document release installation"
```

### Task 6: Add Maintainer Release And Downloaded-Artifact Testing Guides

**Files:**
- Create: `docs/releases.md`
- Modify: `docs/manual-testing.md`
- Modify: `scripts/test-release-version.sh`

- [ ] **Step 1: Add failing documentation assertions**

Add:

```bash
releases="$root/docs/releases.md"
test -f "$releases"
grep -Fq '[skip release]' "$releases"
grep -Fq 'workflow_dispatch' "$releases"
grep -Fq 'contents: write' "$releases"
grep -Fq 'gh workflow run Release' "$releases"
grep -Fq 'releases/latest' "$root/docs/manual-testing.md"
grep -Fq 'SHA256SUMS' "$root/docs/manual-testing.md"
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `bash scripts/test-release-version.sh`

Expected: FAIL because maintainer and downloaded-release documentation is absent.

- [ ] **Step 3: Create maintainer release guide**

Create `docs/releases.md` with:

- CI triggers and the exact four Cargo commands.
- Automatic release flow after successful `master` CI.
- Version rules for breaking, `feat`, patch/default, initial `v0.1.0`, highest valid tag, and case-insensitive `[skip release]`.
- The five target/artifact mappings and `SHA256SUMS`.
- Manual rebuild command:

```bash
gh workflow run Release --ref master -f tag=vX.Y.Z
```

- Recovery procedures:
  - CI failure: fix and push; no tag was created.
  - Build failure after tagging: inspect the failed matrix job, fix workflow infrastructure, then manually rebuild the existing tag.
  - Invalid manual tag: create or select an existing valid `vMAJOR.MINOR.PATCH` tag.
  - Existing release: manual rebuild replaces named assets using `--clobber`.
- Repository setting requirement: Actions must be allowed to read and write repository contents so `GITHUB_TOKEN` can push tags and create releases; no external secrets are required.
- Explicit statement that only one release workflow runs at once and queued runs are not cancelled.

- [ ] **Step 4: Add downloaded-release manual tests**

Append to `docs/manual-testing.md`:

```markdown
## GitHub Release Smoke Test

After the first automatic release:

1. Open `https://github.com/gkk-dev-ops/clck/releases/latest`.
2. Confirm all five archives and `SHA256SUMS` are present.
3. Download the current macOS archive and `SHA256SUMS`.
4. Verify the archive checksum with `shasum -a 256 -c SHA256SUMS`.
5. Extract the archive and confirm it contains `alarm-clock` and `README.md`.
6. Run `./alarm-clock --help`.
7. Confirm the GitHub tag and release use the same `vMAJOR.MINOR.PATCH`.
8. Manually rebuild that existing tag and confirm artifacts are replaced rather
   than a duplicate release being created.
```

Also add Linux and Windows checklist entries for archive extraction, checksum verification, executable startup, and platform-specific installation behavior.

- [ ] **Step 5: Verify documentation requirements**

Run: `bash scripts/test-release-version.sh`

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add docs/releases.md docs/manual-testing.md scripts/test-release-version.sh
git commit -m "Document release operations"
```

### Task 7: Run Complete Local Validation And Prepare First Release Verification

**Files:**
- Modify: `docs/manual-testing.md`

- [ ] **Step 1: Validate scripts and workflow configuration**

Run:

```bash
bash scripts/test-release-version.sh
bash scripts/test-package-release.sh
bash -n scripts/*.sh
go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7
```

Expected: all commands PASS with no actionlint findings.

- [ ] **Step 2: Validate the Rust project exactly as CI will**

Run:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

Expected: all commands PASS.

- [ ] **Step 3: Verify local archive contents and checksum generation**

Run:

```bash
tmp=$(mktemp -d)
mkdir -p "$tmp/raw/binary-x86_64-apple-darwin" "$tmp/dist"
cp target/release/alarm-clock "$tmp/raw/binary-x86_64-apple-darwin/alarm-clock"
scripts/package-release.sh v0.1.1 x86_64-apple-darwin \
  "$tmp/raw/binary-x86_64-apple-darwin/alarm-clock" "$tmp/dist"
(cd "$tmp/dist" && shasum -a 256 alarm-clock-* > SHA256SUMS)
tar -tzf "$tmp/dist/alarm-clock-v0.1.1-macos-x86_64.tar.gz"
cat "$tmp/dist/SHA256SUMS"
rm -rf "$tmp"
```

Expected: archive listing contains `alarm-clock` and `README.md`; checksum output names the archive.

- [ ] **Step 4: Record local validation and remaining hosted checks**

Append a dated Release Automation subsection to `docs/manual-testing.md` recording:

- Local release-tool tests, actionlint, Rust CI commands, archive inspection, and checksum generation.
- Hosted checks that cannot occur until merge: CI triggers, tag push permission, all five runner builds, generated release notes, latest-release link, manual rebuild, and downloaded macOS artifact execution.

- [ ] **Step 5: Re-run complete validation**

Run:

```bash
bash scripts/test-release-version.sh
bash scripts/test-package-release.sh
go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
git diff --check
```

Expected: all commands PASS.

- [ ] **Step 6: Commit**

```bash
git add docs/manual-testing.md
git commit -m "Record release automation validation"
```
