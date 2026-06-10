# Cross-Platform Alarm Modernization Design

## Goal

Modernize the existing single-binary Rust countdown alarm into a reliable,
configurable CLI application for macOS and Linux, with best-effort Windows
compatibility.

The application must support command-line and guided interactive setup, custom
and system audio, native notifications, responsive ASCII-art countdown
rendering, and clean terminal shutdown.

## Supported Platforms

- macOS and Linux are officially supported.
- Windows remains best-effort and must not be intentionally broken.
- Platform-specific behavior is isolated behind focused modules.

## CLI And Interactive Usage

The primary workflow accepts a duration directly:

```text
alarm-clock 1H30
alarm-clock 45s
alarm-clock 1h30m --sound ~/Music/alarm.mp3 --font banner
alarm-clock 01:30:00
```

Duration parsing is case-insensitive. Supported forms include:

- Unit-based durations such as `45s`, `10m`, and `1h30m`.
- Compact hour/minute form such as `1H30`, meaning 1 hour and 30 minutes.
- Colon-separated `HH:MM:SS` form such as `01:30:00`.

Running `alarm-clock` without a duration opens a guided interactive fallback.
The interactive flow validates duration and allows selection of sound, ASCII
font, and notification behavior. Chosen values can be saved as future
defaults.

Additional commands:

```text
alarm-clock config
alarm-clock fonts
alarm-clock sounds
alarm-clock --help
```

## Configuration

Configuration is stored in the platform-standard application configuration
directory.

Values resolve in this order:

1. Command-line options
2. Saved configuration
3. Built-in defaults

Persisted settings include the preferred ASCII font, notification preference,
and default sound reference.

System sounds are stored as logical names rather than absolute paths. This
allows the same configuration to resolve to an available platform-specific
sound after moving between systems.

## Audio

Common formats such as MP3, WAV, FLAC, OGG, and AIFF are played natively by a
Rust audio library where supported.

Unsupported container or codec formats, including MP4, use an installed
`ffmpeg` executable as a fallback. If FFmpeg is required but unavailable, the
application explains the issue before starting the countdown.

Default sound discovery:

- macOS discovers installed sounds from standard system sound directories and
  defaults to the logical `Glass` sound when available.
- Linux discovers sounds from standard freedesktop sound-theme directories and
  selects an alarm or notification sound from the active or available theme.
- If no playable system sound is available, the application falls back to the
  terminal bell.

`alarm-clock sounds` lists discovered logical sound names. A custom
`--sound PATH` always overrides the saved system sound.

When time expires, sound loops until the user presses any key.

## Notifications

When time expires, the application sends a native desktop notification on
macOS and Linux.

Notification failure does not prevent the alarm from ringing. Instead, the
application displays a warning in the terminal and continues.

## Timer Display

The countdown uses a monotonic clock so wall-clock changes do not alter the
requested duration.

The display:

- Shows the remaining duration throughout the countdown.
- Renders the configured ASCII-art font at the largest size that fits the
  current terminal.
- Recalculates and redraws when the terminal is resized.
- Falls back to compact text if no ASCII-art rendering fits.
- Shows a compact status line containing the selected sound and cancellation
  instructions when space permits.

The initial set of fonts is bundled or otherwise available without requiring
network access. `alarm-clock fonts` previews and lists them.

## Input And Terminal Lifecycle

During countdown, `q`, `Esc`, and `Ctrl+C` cancel the alarm.

When ringing, any key dismisses the alarm.

The application uses a terminal abstraction that supports resize and key
events. Terminal state is guarded so cursor visibility, input mode, and screen
contents are restored on:

- Normal completion
- Countdown cancellation
- Alarm dismissal
- Recoverable and fatal errors
- `Ctrl+C`

The application must not leave the shell in raw mode or emit the current
`tcsetattr: Interrupted system call` error during normal cancellation.

## Architecture

The existing single source file is split into focused modules:

- `cli`: command-line arguments, duration parsing, and guided interactive input
- `config`: persisted defaults and platform-standard configuration paths
- `timer`: monotonic countdown state and transitions
- `display`: font rendering, largest-fit layout, resize handling, and terminal
  cleanup
- `audio`: playback, looping, system-sound discovery, and FFmpeg fallback
- `notification`: macOS and Linux desktop notifications
- `app`: coordinates setup, countdown, ringing, cancellation, and shutdown

The modules expose narrow interfaces so platform-specific audio and
notification details do not leak into timer and display behavior.

## Error Handling

- Invalid durations are rejected with an example of accepted formats.
- Missing or unreadable custom sounds are rejected before countdown begins.
- Unsupported audio without FFmpeg is rejected before countdown begins.
- Missing system sounds fall back to the terminal bell.
- Notification failures produce warnings and do not stop the alarm.
- Terminal and playback resources are cleaned up before returning errors.

## Testing

Automated tests cover:

- Duration parsing, including `1H30`, unit-based, and colon-separated formats
- Invalid and ambiguous duration rejection
- Configuration precedence and serialization
- System-sound discovery using controlled directory fixtures
- Countdown state transitions using controllable time
- Largest-fit font and terminal-size calculations
- Cancellation and dismissal state transitions
- Cleanup guards restoring terminal state after normal and error paths

Manual smoke-test documentation covers:

- Native audio playback on macOS and Linux
- MP4 playback through FFmpeg
- Native notifications on macOS and Linux
- Resize behavior in common terminal emulators
- `Ctrl+C`, countdown cancellation, and ringing dismissal

## Initial Release Boundary

The first modernization release does not include scheduling alarms for a
specific wall-clock time, multiple simultaneous alarms, background daemon
operation, remote control, or a full-screen configuration TUI.
