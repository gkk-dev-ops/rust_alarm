# alarm-clock

A responsive cross-platform countdown alarm for the terminal.

Officially supported on macOS and Linux. Windows support is best-effort.

## Install

Install Rust, clone this repository, then run:

```bash
cargo install --path .
```

## Usage

Pass the countdown duration directly:

```bash
alarm-clock 45s
alarm-clock 10m
alarm-clock 1h30m
alarm-clock 1H30
alarm-clock 01:30:00
```

`1H30` means one hour and thirty minutes. Duration units are
case-insensitive.

Running `alarm-clock` without a duration opens guided interactive setup.
Interactive font, sound, and notification choices can be saved as defaults.

## Options

```bash
alarm-clock 5m --sound ~/Music/alarm.mp3
alarm-clock 5m --sound ~/Movies/alarm.mp4
alarm-clock 5m --font banner
alarm-clock 5m --no-notification
```

MP3, WAV, FLAC, OGG, and AIFF play natively. Other formats, including MP4,
require `ffmpeg` to be installed.

The default sound is resolved by logical name from installed OS sounds.
macOS defaults to `Glass`; Linux discovers freedesktop sound themes. If no
sound is available, the terminal bell is used.

## Commands

```bash
alarm-clock fonts
alarm-clock sounds
alarm-clock config --show
alarm-clock config --reset
alarm-clock --help
```

Configuration is stored in the platform-standard application config directory.
Command-line options override saved defaults.

## Controls

- During countdown: `q`, `Esc`, or `Ctrl+C` cancels.
- While ringing: any key dismisses the alarm.

The timer selects the largest bundled ASCII font that fits and redraws when
the terminal is resized. Terminal mode and cursor visibility are restored on
normal exit, cancellation, dismissal, and errors.

## Development

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
```

See [docs/manual-testing.md](docs/manual-testing.md) for platform smoke tests.
