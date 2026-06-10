# alarm-clock

A responsive cross-platform countdown alarm for the terminal.

Officially supported on macOS and Linux. Windows support is best-effort.

## Download

Download the latest prebuilt release:

https://github.com/gkk-dev-ops/rust_alarm/releases/latest

| Platform | Architecture | Artifact |
| --- | --- | --- |
| macOS | Apple Silicon | `alarm-clock-vX.Y.Z-macos-aarch64.tar.gz` |
| macOS | Intel | `alarm-clock-vX.Y.Z-macos-x86_64.tar.gz` |
| Linux | ARM64 musl | `alarm-clock-vX.Y.Z-linux-aarch64-musl.tar.gz` |
| Linux | x86_64 musl | `alarm-clock-vX.Y.Z-linux-x86_64-musl.tar.gz` |
| Windows | x86_64 | `alarm-clock-vX.Y.Z-windows-x86_64.zip` |

Each release also includes `SHA256SUMS`.

### macOS And Linux

Download the archive for your platform, then extract and install it:

```bash
tar -xzf alarm-clock-vX.Y.Z-macos-aarch64.tar.gz
chmod +x alarm-clock
sudo install alarm-clock /usr/local/bin/alarm-clock
alarm-clock --help
```

Use the corresponding Linux archive name on Linux. To update, download the
latest archive and replace the installed executable.

Downloaded macOS binaries are unsigned. Verify the checksum first. If
Gatekeeper blocks the executable, remove the quarantine attribute:

```bash
xattr -d com.apple.quarantine /path/to/alarm-clock
```

### Windows

Download and extract `alarm-clock-vX.Y.Z-windows-x86_64.zip`, then add the
directory containing `alarm-clock.exe` to `PATH`. To update, replace the
executable with the latest release.

### Verify Checksums

Download `SHA256SUMS` beside the archive. On macOS:

```bash
grep 'alarm-clock-vX.Y.Z-macos-aarch64.tar.gz' SHA256SUMS |
  shasum -a 256 -c -
```

On Linux:

```bash
grep 'alarm-clock-vX.Y.Z-linux-x86_64-musl.tar.gz' SHA256SUMS |
  sha256sum -c -
```

On Windows PowerShell, compare this output with the corresponding
`SHA256SUMS` entry:

```powershell
Get-FileHash .\alarm-clock-vX.Y.Z-windows-x86_64.zip -Algorithm SHA256
```

## Install With Cargo

Install directly from the Git repository:

```bash
cargo install --git https://github.com/gkk-dev-ops/rust_alarm.git --locked
```

Update a Cargo installation with:

```bash
cargo install --git https://github.com/gkk-dev-ops/rust_alarm.git --locked --force
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

Maintainers: see [docs/releases.md](docs/releases.md) for CI, versioning, and
release operations.
