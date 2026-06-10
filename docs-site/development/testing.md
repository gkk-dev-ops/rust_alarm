# Testing

## Automated Checks

Run the same core checks used by CI:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
npm ci --prefix docs-site
npm run docs:build --prefix docs-site
```

## Manual Verification Matrix

Run on macOS and Linux:

```bash
cargo run -- 5s
cargo run -- 1H30
cargo run -- 2s --sound /path/to/test.mp3
cargo run -- 2s --title "Resuming"
cargo run -- 2s --sound /path/to/test.mp4
cargo run
cargo run -- fonts
cargo run -- sounds
cargo run -- config --show
cargo run -- at 2:50pm
cargo run -- at "tomorrow at 9am"
cargo run -- from-text
printf 'Meet tomorrow at 9am\n' | cargo run -- from-text
```

Verify countdown accuracy, terminal resizing, cancellation, notification
delivery, looping sound, any-key dismissal, custom-file errors, interactive
defaults, schedule confirmation, DST rejection, and explicit selection of
multiple extracted targets.

Linux testing should include freedesktop sound discovery and desktop
notifications. Windows best-effort validation can use:

```bash
cargo check --target x86_64-pc-windows-gnu
```

See the repository's `docs/manual-testing.md` for the full release and
platform smoke-test checklist.
