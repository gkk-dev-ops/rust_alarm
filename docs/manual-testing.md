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
cargo run -- at 2:50pm
cargo run -- at "tomorrow at 9am"
cargo run -- from-text
printf 'Meet tomorrow at 9am\n' | cargo run -- from-text
printf 'Choose June 12 at 09:00 or June 12 at 14:30\n' | cargo run -- from-text
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
- Every scheduled target displays its local date, time, UTC offset, and time
  zone before confirmation.
- Rejecting confirmation exits without starting a countdown.
- Past, nonexistent DST, and ambiguous DST targets are rejected rather than
  guessed.
- Interactive multiline input ends on a single `.` line.
- Multiple text candidates require explicit selection.
- Piped text uses a controlling terminal for selection and confirmation when
  available.
- Piped text without a controlling terminal prints candidates and exits
  non-zero.
- Scheduled countdowns and time-up notifications include the confirmed target.

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

### Scheduling Smoke Test

Tested on macOS in the `Europe/Warsaw` time zone on June 10, 2026:

- `cargo run -- at "tomorrow at 9am"` resolved the complete local target,
  required confirmation, and exited without starting when answered `no`.
- `cargo run -- at later` rejected the vague expression, accepted a replacement
  expression, displayed its resolved target, and required confirmation.
- Interactive `cargo run -- from-text` accepted multiline input ending with
  `.`, displayed two candidates, required explicit selection, and exited
  without starting when confirmation was rejected.
- Piped one- and two-candidate text printed resolved candidates and exited
  non-zero when no controlling terminal was available.
- Piped vague text printed accepted examples and exited non-zero.

Starting a scheduled countdown through expiry, checking scheduled notification
text, and smoke testing scheduling on Linux still require manual confirmation.

## Linux Smoke Test

Requires a Linux host. Verify freedesktop sound discovery and desktop
notification delivery in the active desktop environment.

## Windows Best-Effort Check

Run when the target is installed:

```bash
cargo check --target x86_64-pc-windows-gnu
```

## GitHub Release Smoke Test

After the first automatic release:

1. Open `https://github.com/gkk-dev-ops/clck/releases/latest`.
2. Confirm all five archives and `SHA256SUMS` are present.
3. Download the current macOS archive and `SHA256SUMS`.
4. Filter its entry from `SHA256SUMS` and verify it with
   `shasum -a 256 -c -`.
5. Extract the archive and confirm it contains `clck`, `README.md`, and
   `LICENSE`.
6. Run `./clck --help`.
7. Confirm the GitHub tag and release use the same `vMAJOR.MINOR.PATCH`.
8. Manually rebuild that existing tag and confirm artifacts are replaced rather
   than a duplicate release being created.

On Linux, filter the appropriate archive entry from `SHA256SUMS`, verify it
with `sha256sum -c -`, extract it, install the executable on `PATH`, and run
`clck --help`.

On Windows, compare `Get-FileHash -Algorithm SHA256` with `SHA256SUMS`, extract
the ZIP, add its directory to `PATH`, and run `clck.exe --help`.

### Release Automation Local Validation

Tested on macOS on June 10, 2026:

- Release version tests passed for initial, patch, minor, breaking, highest-tag,
  and `[skip release]` behavior.
- Release packaging tests passed for macOS tar archives and Windows ZIP
  archives.
- `bash -n scripts/*.sh` and actionlint passed.
- The exact CI formatting, test, Clippy, and release-build commands passed.
- A local macOS x86_64 release archive contained `clck` and `README.md`, and
  `SHA256SUMS` was generated for it.

After merge, GitHub-hosted validation must confirm the CI triggers, tag-push
permission, all five runner builds, generated release notes, latest-release
link, manual rebuild behavior, and execution of a downloaded macOS artifact.
