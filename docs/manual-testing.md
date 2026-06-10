# Manual Testing

## Verification Matrix

Run on macOS and Linux:

```bash
cargo run -- 5s
cargo run -- 1H30
cargo run -- 2s --sound /path/to/test.mp3
cargo run -- 2s --sound /path/to/test.mp4
cargo run
cargo run -- fonts
cargo run -- sounds
cargo run -- config --show
```

Verify:

- The countdown displays the correct remaining time.
- Resizing selects the largest fitting font and redraws cleanly.
- `q`, `Esc`, and `Ctrl+C` cancel and restore the shell.
- Expiry sends a desktop notification when enabled.
- Sound loops until any key is pressed.
- Any-key dismissal stops sound and restores the shell.
- Missing custom files fail before countdown.
- MP4 reports a useful error when FFmpeg is unavailable.
- The interactive fallback validates duration and can save defaults.

## macOS Smoke Test

Tested on June 10, 2026:

- `cargo run -- 10s --no-notification`: countdown rendered in the alternate
  screen and cancellation restored cursor and terminal state without the old
  `tcsetattr` error.
- `cargo run -- 1s --no-notification`: resolved the installed logical `Glass`
  sound, rang after expiry, stopped on any key, and restored terminal state.
- `cargo run -- sounds`: listed installed `/System/Library/Sounds` entries.
- `cargo run -- config --show`: printed effective TOML configuration.

Notification display, custom MP3/MP4 playback, and interactive saved settings
still require manual confirmation in a normal user terminal session.

## Linux Smoke Test

Requires a Linux host. Verify freedesktop sound discovery and desktop
notification delivery in the active desktop environment.

## Windows Best-Effort Check

Run when the target is installed:

```bash
cargo check --target x86_64-pc-windows-gnu
```
