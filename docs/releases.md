# Release Operations

## CI

The `CI` GitHub Actions workflow runs for pull requests targeting `master`,
pushes to `master`, and manual dispatches. It uses read-only repository
permissions and runs:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

It also validates workflow YAML and runs the release versioning and packaging
tests.

## Automatic Releases

After CI succeeds for a push to `master`, the serialized `Release` workflow:

1. Reads the head commit message.
2. Exits without tagging when the message contains case-insensitive
   `[skip release]`.
3. Finds the highest valid `vMAJOR.MINOR.PATCH` tag.
4. Calculates and pushes the next version tag.
5. Builds all required targets from that tag.
6. Packages the executables and generates `SHA256SUMS`.
7. Creates the GitHub Release with generated release notes after every build
   succeeds.

Only one release workflow runs at a time. New runs wait and are not cancelled.
Tags created by the workflow do not trigger another release.

## Version Rules

- A Conventional Commit subject containing `!`, or a `BREAKING CHANGE:`
  footer, increments the major version.
- `feat:` and `feat(scope):` increment the minor version.
- Every other commit increments the patch version.
- If no valid release tag exists, calculation starts from `v0.1.0`.
- Invalid tags are ignored and the highest valid semantic version is used.
- `[skip release]` is case-insensitive and skips release publication, not CI.

## Artifacts

| Rust target | Release artifact |
| --- | --- |
| `aarch64-apple-darwin` | `clck-vX.Y.Z-macos-aarch64.tar.gz` |
| `x86_64-apple-darwin` | `clck-vX.Y.Z-macos-x86_64.tar.gz` |
| `aarch64-unknown-linux-musl` | `clck-vX.Y.Z-linux-aarch64-musl.tar.gz` |
| `x86_64-unknown-linux-musl` | `clck-vX.Y.Z-linux-x86_64-musl.tar.gz` |
| `x86_64-pc-windows-msvc` | `clck-vX.Y.Z-windows-x86_64.zip` |

Every release also contains `SHA256SUMS`. Archives contain the executable and
`README.md`, plus a plain-text license file if one exists.

## Manual Rebuild

The release workflow's `workflow_dispatch` trigger rebuilds an existing valid
tag without calculating or creating a new tag:

```bash
gh workflow run Release --ref master -f tag=vX.Y.Z
```

The workflow validates that the tag exists. If the release already exists,
assets with matching names are replaced using `--clobber`.

## Failure Recovery

- CI failure: fix the issue and push again. No release tag was created.
- Platform build failure after tagging: inspect the failed matrix job, fix the
  workflow or infrastructure issue, then manually rebuild the existing tag.
- Invalid manual tag: select or create an existing valid `vMAJOR.MINOR.PATCH`
  tag before dispatching.
- Existing release: manual rebuild replaces generated assets instead of
  creating a duplicate release.

## Repository Settings

In repository Actions settings, allow workflows to read and write repository
contents. The release workflow requests `contents: write` so `GITHUB_TOKEN` can
push tags and create releases. No external secrets are required.
