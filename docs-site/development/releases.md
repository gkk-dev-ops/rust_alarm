# Releases

## CI

The `CI` workflow runs for pull requests targeting `master`, pushes to
`master`, and manual dispatches. It validates Rust, release tooling, workflow
syntax, and the documentation build.

## Automatic Releases

After CI succeeds for a push to `master`, the serialized `Release` workflow:

1. Reads the head commit message.
2. Skips tagging when the message contains `[skip release]`.
3. Calculates the next semantic version.
4. Updates `Cargo.toml` and `Cargo.lock`, then commits and tags that version.
5. Builds all supported targets.
6. Packages executables and generates `SHA256SUMS`.
7. Creates the GitHub Release after all builds succeed.

## Version Rules

- A Conventional Commit subject containing `!`, or a `BREAKING CHANGE:`
  footer, increments the major version.
- `feat:` and `feat(scope):` increment the minor version.
- Every other commit increments the patch version.
- If no release tag exists, calculation starts from `v0.1.0`.
- The version in `Cargo.toml` and `Cargo.lock` matches the release tag.

## Artifacts

| Rust target | Release artifact |
| --- | --- |
| `aarch64-apple-darwin` | `clck-vX.Y.Z-macos-aarch64.tar.gz` |
| `x86_64-apple-darwin` | `clck-vX.Y.Z-macos-x86_64.tar.gz` |
| `aarch64-unknown-linux-musl` | `clck-vX.Y.Z-linux-aarch64-musl.tar.gz` |
| `x86_64-unknown-linux-musl` | `clck-vX.Y.Z-linux-x86_64-musl.tar.gz` |
| `x86_64-pc-windows-msvc` | `clck-vX.Y.Z-windows-x86_64.zip` |

Every release also contains `SHA256SUMS`.

## Manual Rebuild

The Release workflow's `workflow_dispatch` trigger rebuilds an existing valid
tag without calculating a new version:

```bash
gh workflow run Release --ref master -f tag=vX.Y.Z
```

Existing assets with matching names are replaced.

## Repository Settings

The Release workflow requests `contents: write` permission so `GITHUB_TOKEN`
can push tags and publish releases. No external secrets are required.

## Release Verification

After publishing, open the
[latest release](https://github.com/gkk-dev-ops/clck/releases/latest) and
confirm all five archives and `SHA256SUMS` are present.

## Failure Recovery

- CI failure: fix the issue and push again; no release tag was created.
- Platform build failure after tagging: fix the issue, then manually rebuild
  the existing tag.
- Invalid manual tag: select or create an existing `vMAJOR.MINOR.PATCH` tag.
