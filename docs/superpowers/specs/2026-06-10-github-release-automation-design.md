# GitHub Release Automation Design

## Goal

Add GitHub Actions automation that validates every pull request and `master`
commit, automatically versions each successful releasable `master` commit,
builds downloadable cross-platform binaries, and publishes a GitHub Release.

Update project documentation so end users can easily download, install, and
update the application, while maintainers can understand and operate the
release process.

## Current State

The repository has no GitHub Actions workflows. Commits pushed to `master` are
not automatically tested, tagged, packaged, or published.

The GitHub repository is:

`https://github.com/gkk-dev-ops/clck`

## Release Trigger And Versioning

Every push to `master` runs CI. After CI succeeds, the release workflow creates
a release unless the head commit message contains `[skip release]`.

The version bump follows Conventional Commit semantics:

- A breaking change increments the major version. Breaking changes are
  detected by a Conventional Commit type or scope followed by `!`, or by a
  `BREAKING CHANGE:` footer.
- A commit whose subject begins with `feat:` or `feat(scope):` increments the
  minor version.
- Every other commit, including commits that do not follow Conventional
  Commits, increments the patch version.

The workflow finds the highest existing valid `vMAJOR.MINOR.PATCH` tag. If the
repository has no valid version tag, version calculation begins from `v0.1.0`.
For example, the first patch release becomes `v0.1.1`, while the first feature
release becomes `v0.2.0`.

`[skip release]` is case-insensitive and may appear anywhere in the head commit
message. It skips tagging and publishing but does not skip CI. GitHub-standard
`[skip ci]` behavior is not changed.

## Workflows

### CI Workflow

The CI workflow runs on:

- Pull requests targeting `master`
- Pushes to `master`
- Manual workflow dispatch

It runs:

```text
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

CI uses the checked-in `Cargo.lock` and Cargo dependency/build caching.

### Release Workflow

The release workflow runs after the CI workflow completes successfully for a
push to `master`. It also supports manual workflow dispatch for rebuilding a
specific existing version tag.

For an automatic release, it:

1. Checks the head commit message for `[skip release]`.
2. Fetches all version tags.
3. Determines the required semantic version bump.
4. Creates and pushes the new version tag.
5. Builds every required platform artifact from that tagged commit.
6. Generates `SHA256SUMS`.
7. Creates a GitHub Release with generated release notes only after all
   required artifacts succeed.

Workflow-created tags do not recursively trigger another release because the
release workflow is not triggered by tag pushes.

Manual dispatch accepts an existing `vMAJOR.MINOR.PATCH` tag. It rebuilds and
publishes artifacts for that tag without calculating or creating another tag.

## Concurrency And Permissions

Only one release workflow may run at a time. New release runs wait for the
active release rather than cancelling it, preventing concurrent commits from
calculating the same next version.

The release workflow uses GitHub's built-in `GITHUB_TOKEN` and requests only:

```text
contents: write
```

No external secrets are required.

The CI workflow uses read-only repository permissions.

Third-party GitHub Actions are pinned to stable major versions. The workflows
use official GitHub and Rust toolchain actions where practical.

## Release Artifacts

Each GitHub Release contains:

```text
alarm-clock-vX.Y.Z-macos-aarch64.tar.gz
alarm-clock-vX.Y.Z-macos-x86_64.tar.gz
alarm-clock-vX.Y.Z-linux-aarch64-musl.tar.gz
alarm-clock-vX.Y.Z-linux-x86_64-musl.tar.gz
alarm-clock-vX.Y.Z-windows-x86_64.zip
SHA256SUMS
```

Each archive contains:

- The `alarm-clock` executable, or `alarm-clock.exe` on Windows
- `README.md`
- A plain-text license file if the repository adds one before release

Linux artifacts use musl targets to maximize portability:

- `aarch64-unknown-linux-musl`
- `x86_64-unknown-linux-musl`

macOS artifacts target:

- `aarch64-apple-darwin`
- `x86_64-apple-darwin`

Windows targets:

- `x86_64-pc-windows-msvc`

## Failure Behavior

CI failures prevent automatic tagging and publishing.

If a platform build fails after the automatic version tag has been created,
the tag remains available for diagnosis, but no incomplete GitHub Release is
created. After fixing infrastructure or workflow problems, a maintainer can
manually dispatch the release workflow for the existing tag.

If a release already exists for a manually selected tag, the workflow replaces
the release's generated artifacts rather than creating a second release.

Invalid manual-dispatch tags fail before builds begin with a clear error.

## Documentation

### README

The README will add:

- A prominent link to the latest GitHub Release
- A supported platform and architecture table
- End-user installation steps for macOS, Linux, Windows, and Cargo
- Download, extraction, installation, and update examples
- Checksum verification examples
- macOS Gatekeeper instructions for unsigned downloaded binaries
- A short maintainer section linking to detailed release documentation

The latest-release URL is:

`https://github.com/gkk-dev-ops/clck/releases/latest`

### Maintainer Release Documentation

`docs/releases.md` will document:

- CI and release triggers
- Conventional Commit version bump rules
- `[skip release]`
- Artifact names and target mappings
- Automatic release flow
- Manual rebuild flow
- Failure recovery and troubleshooting
- Required repository Actions permissions

### Manual Testing Documentation

`docs/manual-testing.md` will include checks for downloaded release archives,
checksums, executable startup, and platform-specific installation behavior.

## Validation

Automated validation covers:

- Rust formatting, tests, Clippy, and release build
- Workflow YAML syntax and action configuration
- Version calculation for major, minor, patch, non-conventional, initial, and
  `[skip release]` cases
- Archive naming and expected executable presence
- Documentation links and commands

After merging, maintainers verify the first automatic release on GitHub,
download at least the current macOS artifact, verify its checksum, and run
`alarm-clock --help`.

## Initial Boundary

This release automation does not publish to crates.io, Homebrew, Linux package
repositories, WinGet, or macOS notarization services. Those distribution
channels can be added separately after GitHub Release packaging is stable.
