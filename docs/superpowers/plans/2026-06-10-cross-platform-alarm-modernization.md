# Cross-Platform Alarm Modernization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Modernize `alarm-clock` into a configurable macOS/Linux countdown CLI with guided fallback, responsive ASCII rendering, looping custom/system audio, desktop notifications, and clean terminal shutdown.

**Architecture:** Convert the package into a testable library plus a thin binary. Pure modules handle duration parsing, configuration resolution, sound discovery, font layout, and timer state; adapters handle the terminal, audio device/FFmpeg, notifications, and filesystem. The `app` module owns the runtime state machine and guarantees cleanup through RAII guards.

**Tech Stack:** Rust 2021, `clap`, `inquire`, `serde`, `toml`, `directories`, `crossterm`, `rodio`, `notify-rust`, `ctrlc`, `anyhow`, `thiserror`, and bundled FIGlet-compatible font definitions.

---

## File Structure

- `src/main.rs`: parse top-level command and report final errors.
- `src/lib.rs`: expose modules and `run`.
- `src/cli.rs`: `clap` types, duration parsing, and guided fallback.
- `src/config.rs`: persisted settings, defaults, and precedence.
- `src/timer.rs`: monotonic countdown state calculations.
- `src/display.rs`: terminal session guard, resize/input events, and rendering.
- `src/fonts.rs`: bundled ASCII font catalog and largest-fit selection.
- `src/audio.rs`: sound references, discovery, native playback, FFmpeg fallback.
- `src/notification.rs`: desktop notification adapter.
- `src/app.rs`: countdown/ringing orchestration and cancellation.
- `tests/cli.rs`: binary-level CLI smoke tests.
- `tests/fixtures/sounds/`: controlled system-sound discovery fixtures.
- `README.md`: installation, usage, formats, controls, and platform notes.
- `docs/manual-testing.md`: macOS/Linux manual smoke-test checklist.

### Task 1: Establish The Testable Crate And Dependencies

**Files:**
- Modify: `Cargo.toml`
- Create: `src/lib.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Add a failing library smoke test**

Create `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn package_name_is_stable() {
        assert_eq!(super::APP_NAME, "alarm-clock");
    }
}
```

- [ ] **Step 2: Verify the test fails**

Run: `cargo test package_name_is_stable`

Expected: FAIL because `APP_NAME` is undefined.

- [ ] **Step 3: Add dependencies and minimal library entry point**

Add to `Cargo.toml`:

```toml
[dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
crossterm = "0.29"
ctrlc = "3"
directories = "6"
inquire = "0.9"
notify-rust = "4"
rodio = { version = "0.21", features = ["playback", "symphonia-all"] }
serde = { version = "1", features = ["derive"] }
thiserror = "2"
toml = "0.9"

[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

Complete `src/lib.rs`:

```rust
pub const APP_NAME: &str = "alarm-clock";

pub fn run() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn package_name_is_stable() {
        assert_eq!(super::APP_NAME, "alarm-clock");
    }
}
```

Replace `src/main.rs`:

```rust
fn main() {
    if let Err(error) = alarm_clock::run() {
        eprintln!("alarm-clock: {error:#}");
        std::process::exit(1);
    }
}
```

- [ ] **Step 4: Verify the baseline**

Run: `cargo test && cargo clippy --all-targets -- -D warnings`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock src/lib.rs src/main.rs
git commit -m "Set up testable alarm application"
```

### Task 2: Parse CLI Commands And Duration Formats

**Files:**
- Create: `src/cli.rs`
- Modify: `src/lib.rs`
- Create: `tests/cli.rs`

- [ ] **Step 1: Write failing duration parser tests**

Add tests in `src/cli.rs` for:

```rust
assert_eq!(parse_duration("45s")?, Duration::from_secs(45));
assert_eq!(parse_duration("10m")?, Duration::from_secs(600));
assert_eq!(parse_duration("1h30m")?, Duration::from_secs(5_400));
assert_eq!(parse_duration("1H30")?, Duration::from_secs(5_400));
assert_eq!(parse_duration("01:30:00")?, Duration::from_secs(5_400));
assert!(parse_duration("1H75").is_err());
assert!(parse_duration("nonsense").is_err());
```

Add `tests/cli.rs`:

```rust
use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_lists_duration_examples() {
    Command::cargo_bin("alarm-clock")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("1H30"))
        .stdout(contains("01:30:00"));
}
```

- [ ] **Step 2: Verify tests fail**

Run: `cargo test cli`

Expected: FAIL because `cli` and duration parsing do not exist.

- [ ] **Step 3: Implement CLI types and explicit duration grammar**

Create:

```rust
#[derive(clap::Parser, Debug)]
#[command(name = "alarm-clock", about = "Responsive cross-platform countdown alarm")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
    #[arg(value_name = "DURATION", help = "Examples: 45s, 1h30m, 1H30, 01:30:00")]
    pub duration: Option<String>,
    #[arg(long)]
    pub sound: Option<PathBuf>,
    #[arg(long)]
    pub font: Option<String>,
    #[arg(long)]
    pub no_notification: bool,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Config {
        #[arg(long, conflicts_with = "reset")]
        show: bool,
        #[arg(long, conflicts_with = "show")]
        reset: bool,
    },
    Fonts,
    Sounds,
}
```

Implement `parse_duration` with three deliberately separate parsers:

1. `HH:MM:SS`, requiring minutes and seconds below 60.
2. Case-insensitive unit tokens (`h`, `m`, `s`) with no duplicates.
3. Compact `HhMM`, where trailing minutes are required to be below 60.

Return a typed `DurationParseError` with the accepted-format examples.

Export `pub mod cli;` from `src/lib.rs`, parse `Cli` in `run`, and return after
printing help for subcommands temporarily.

- [ ] **Step 4: Verify parsing and help**

Run: `cargo test cli && cargo run -- --help`

Expected: all tests PASS; help contains examples and subcommands.

- [ ] **Step 5: Commit**

```bash
git add src/cli.rs src/lib.rs tests/cli.rs
git commit -m "Add alarm CLI and duration parsing"
```

### Task 3: Persist And Resolve Configuration

**Files:**
- Create: `src/config.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing configuration tests**

Cover TOML round-trip and precedence:

```rust
let saved = Config { font: "box".into(), notification: false, sound: SoundSetting::System("Glass".into()) };
let overrides = Overrides { font: Some("banner".into()), notification: Some(true), sound: None };
let resolved = saved.resolve(overrides);
assert_eq!(resolved.font, "banner");
assert!(resolved.notification);
assert_eq!(resolved.sound, SoundSetting::System("Glass".into()));
```

Also test that a missing config file returns `Config::default()` and malformed
TOML returns an actionable error containing the path.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test config`

Expected: FAIL because the module does not exist.

- [ ] **Step 3: Implement configuration storage**

Define:

```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum SoundSetting {
    System(String),
    File(PathBuf),
    TerminalBell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub font: String,
    pub notification: bool,
    pub sound: SoundSetting,
}
```

Use `directories::ProjectDirs::from("", "", "alarm-clock")` and
`config_dir().join("config.toml")`. Implement `load_from`, `save_to`, and
`resolve(Overrides)`. Default to font `standard`, notifications enabled, and a
logical system sound named `Glass`.

- [ ] **Step 4: Verify configuration tests**

Run: `cargo test config`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/config.rs src/lib.rs
git commit -m "Add persisted alarm configuration"
```

### Task 4: Add Guided Interactive Fallback

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/config.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing prompt-model tests**

Separate prompt decisions from `inquire` by testing:

```rust
assert_eq!(
    InteractiveAnswers { duration: "1H30".into(), ..defaults }.validate()?.duration,
    Duration::from_secs(5_400)
);
assert!(InteractiveAnswers { duration: "bad".into(), ..defaults }.validate().is_err());
```

Test that interactive answers become CLI-equivalent overrides and that the
save-defaults choice updates only font, sound, and notification settings, not
the one-off duration.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test interactive`

Expected: FAIL because interactive models are absent.

- [ ] **Step 3: Implement guided prompts**

Add `InteractiveAnswers`, `ValidatedInteractiveAnswers`, and
`prompt_for_alarm(&Config)`. Use `inquire::Text` with a duration validator,
`Select` for discovered sound/font names, `Confirm` for notifications, and a
final `Confirm` for saving defaults.

In `run`, invoke the guided prompts only when there is no duration and no
subcommand. Preserve direct CLI invocation as the primary path.

- [ ] **Step 4: Verify non-interactive tests and manually inspect fallback**

Run: `cargo test && cargo run`

Expected: tests PASS; guided prompts appear and reject an invalid duration.
Cancel the manual prompt with `Ctrl+C`; it must return to a usable shell.

- [ ] **Step 5: Commit**

```bash
git add src/cli.rs src/config.rs src/lib.rs
git commit -m "Add guided interactive alarm setup"
```

### Task 5: Build Timer State And Responsive Font Layout

**Files:**
- Create: `src/timer.rs`
- Create: `src/fonts.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing pure-logic tests**

Test countdown calculations with explicit instants:

```rust
let timer = Countdown::new(start, Duration::from_secs(90));
assert_eq!(timer.remaining(start + Duration::from_secs(30)), Duration::from_secs(60));
assert_eq!(timer.remaining(start + Duration::from_secs(100)), Duration::ZERO);
```

Test largest-fit layout:

```rust
assert_eq!(catalog.largest_fit("01:30", Size::new(120, 30)).unwrap().name, "banner");
assert_eq!(catalog.largest_fit("01:30", Size::new(20, 5)).unwrap().name, "compact");
assert!(catalog.largest_fit("01:30", Size::new(4, 1)).is_none());
```

- [ ] **Step 2: Verify tests fail**

Run: `cargo test`

Expected: FAIL because modules are absent.

- [ ] **Step 3: Implement countdown and bundled fonts**

Implement `Countdown { started_at, duration }`, `remaining(now)`, and
`is_finished(now)`.

Implement a small initial font catalog (`standard`, `box`, `banner`,
`compact`) with glyph dimensions known before rendering. `largest_fit` should
try the configured font first at supported scales, then other fonts from
largest to smallest, then return `None` for plain-text fallback.

- [ ] **Step 4: Verify pure logic**

Run: `cargo test`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/timer.rs src/fonts.rs src/lib.rs
git commit -m "Add countdown state and responsive ASCII fonts"
```

### Task 6: Add Clean Terminal Session And Countdown Display

**Files:**
- Create: `src/display.rs`
- Modify: `src/app.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing display-state tests**

Use an injected `TerminalOps` fake to assert:

```rust
let ops = FakeTerminalOps::default();
{
    let _session = TerminalSession::enter(&ops)?;
}
assert_eq!(ops.calls(), ["enable_raw", "hide_cursor", "show_cursor", "disable_raw"]);
```

Also test cleanup after a render error and mapping `q`, `Esc`, and `Ctrl+C` to
`DisplayEvent::Cancel`.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test display`

Expected: FAIL because display types are absent.

- [ ] **Step 3: Implement terminal guard and rendering loop**

Create `TerminalSession` whose `Drop` implementation always attempts to show
the cursor, leave the alternate screen, and disable raw mode. Use `crossterm`
events for key input and resize. Render only from a complete in-memory frame,
clear before each redraw, and flush once.

Add a global `ctrlc` handler that sets an `Arc<AtomicBool>` cancellation flag;
the app loop observes it and exits through normal guard cleanup rather than
terminating inside the signal handler.

- [ ] **Step 4: Verify cleanup and manually resize**

Run: `cargo test display && cargo run -- 10s`

Expected: tests PASS; timer redraws on resize; `q`, `Esc`, and `Ctrl+C` each
restore the cursor and return a clean shell prompt.

- [ ] **Step 5: Commit**

```bash
git add src/display.rs src/app.rs src/lib.rs
git commit -m "Add responsive terminal countdown and cleanup"
```

### Task 7: Discover System Sounds And Validate Custom Audio

**Files:**
- Create: `src/audio.rs`
- Create: `tests/fixtures/sounds/macos/System/Library/Sounds/Glass.aiff`
- Create: `tests/fixtures/sounds/linux/freedesktop/stereo/alarm-clock-elapsed.oga`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing discovery tests**

Use zero-byte fixtures because discovery tests inspect names and extensions,
not decoding:

```rust
let sounds = discover_macos_sounds(fixture_root)?;
assert_eq!(sounds["Glass"].file_name().unwrap(), "Glass.aiff");

let sounds = discover_linux_sounds(&[fixture_root])?;
assert!(sounds.contains_key("alarm-clock-elapsed"));
```

Test custom-path validation, case-insensitive native extensions, and MP4 being
classified as `PlaybackBackend::Ffmpeg`.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test audio`

Expected: FAIL because audio discovery does not exist.

- [ ] **Step 3: Implement sound resolution**

Define `ResolvedSound::{Native(PathBuf), Ffmpeg(PathBuf), TerminalBell}`.
Discover macOS sounds under `/System/Library/Sounds` and
`~/Library/Sounds`. Discover Linux sounds under
`$XDG_DATA_HOME/sounds`, `~/.local/share/sounds`, and
`/usr/share/sounds`, preferring names containing `alarm`, then `complete`,
then `notification`.

Resolve logical names case-insensitively. Validate custom files before the
countdown. Classify MP4 and unknown-but-existing formats as FFmpeg candidates.

- [ ] **Step 4: Verify discovery**

Run: `cargo test audio`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/audio.rs src/lib.rs tests/fixtures/sounds
git commit -m "Add cross-platform system sound discovery"
```

### Task 8: Implement Looping Playback And Dismissal

**Files:**
- Modify: `src/audio.rs`
- Modify: `src/app.rs`

- [ ] **Step 1: Write failing playback-controller tests**

Test with fake native and process backends:

```rust
let mut player = FakePlayer::default();
ring_until_dismissed(&mut player, &mut events, ResolvedSound::test())?;
assert_eq!(player.calls(), ["start_loop", "stop"]);
```

Cover dismissal by any key, cancellation flag, native playback error, FFmpeg
missing before countdown, and terminal-bell fallback.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test playback`

Expected: FAIL because playback orchestration is absent.

- [ ] **Step 3: Implement native and FFmpeg playback**

Use `rodio::OutputStreamBuilder::open_default_stream`, `Decoder`, and
`Sink::append(source.repeat_infinite())` for native files. Keep the stream and
sink alive in a `NativePlayer` until `stop`.

For FFmpeg, verify `ffmpeg -version` during preflight, then spawn:

```text
ffmpeg -nostdin -loglevel error -stream_loop -1 -i <path> -f wav -
```

Pipe decoded WAV to native playback and retain the child handle. On dismissal
or cancellation, stop the sink, kill/wait for the child if present, then
return. Terminal bell repeats at a limited interval rather than busy-looping.

- [ ] **Step 4: Verify controller and manual native playback**

Run: `cargo test playback`

Expected: PASS.

Run on macOS: `cargo run -- 2s --sound /System/Library/Sounds/Glass.aiff`

Expected: sound loops after expiry; any key stops it and exits cleanly.

- [ ] **Step 5: Commit**

```bash
git add src/audio.rs src/app.rs
git commit -m "Add looping alarm playback and dismissal"
```

### Task 9: Add Desktop Notifications And Complete App Orchestration

**Files:**
- Create: `src/notification.rs`
- Modify: `src/app.rs`
- Modify: `src/lib.rs`
- Modify: `src/cli.rs`

- [ ] **Step 1: Write failing orchestration tests**

Inject fake clock, display, player, and notifier. Cover:

```rust
assert_eq!(
    run_alarm(&mut deps, request)?,
    Outcome::Dismissed
);
assert_eq!(deps.calls(), ["preflight", "countdown", "notify", "ring", "cleanup"]);
```

Also verify notification failure records a warning and still calls `ring`, and
countdown cancellation never notifies or rings.

- [ ] **Step 2: Verify tests fail**

Run: `cargo test`

Expected: FAIL because notification and final orchestration are incomplete.

- [ ] **Step 3: Implement notifier and application state machine**

Wrap `notify_rust::Notification` behind:

```rust
pub trait Notifier {
    fn notify_time_up(&self) -> anyhow::Result<()>;
}
```

Finish `app::run_alarm`:

1. Resolve and preflight sound.
2. Enter terminal session.
3. Run responsive countdown until elapsed or cancelled.
4. Send notification if enabled; retain failure as a warning.
5. Render ringing state and loop sound.
6. Dismiss on any key.
7. Exit through terminal/audio cleanup guards.

Wire `lib::run` to config loading, CLI/subcommands, interactive fallback,
request resolution, and `app::run_alarm`.

- [ ] **Step 4: Verify integrated behavior**

Run: `cargo test && cargo clippy --all-targets -- -D warnings`

Expected: PASS.

Run: `cargo run -- 2s`

Expected: countdown appears, desktop notification is attempted, default system
sound loops, and any key exits cleanly.

- [ ] **Step 5: Commit**

```bash
git add src/notification.rs src/app.rs src/lib.rs src/cli.rs
git commit -m "Integrate countdown notifications and alarm lifecycle"
```

### Task 10: Implement Fonts, Sounds, And Config Commands

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/lib.rs`
- Modify: `src/config.rs`
- Modify: `src/fonts.rs`
- Modify: `src/audio.rs`
- Modify: `tests/cli.rs`

- [ ] **Step 1: Write failing command smoke tests**

Assert:

```rust
Command::cargo_bin("alarm-clock")?.arg("fonts").assert().success().stdout(contains("standard"));
Command::cargo_bin("alarm-clock")?.arg("sounds").assert().success();
Command::cargo_bin("alarm-clock")?.args(["config", "--show"]).assert().success().stdout(contains("notification"));
```

- [ ] **Step 2: Verify tests fail**

Run: `cargo test --test cli`

Expected: FAIL because subcommands are placeholders.

- [ ] **Step 3: Implement utility commands**

Make `fonts` list names and previews, `sounds` list logical names and resolved
paths, and `config --show` print effective TOML. Make `config --reset` require
interactive confirmation before replacing saved settings with defaults.

- [ ] **Step 4: Verify commands**

Run: `cargo test --test cli && cargo run -- fonts && cargo run -- sounds && cargo run -- config --show`

Expected: PASS and readable command output.

- [ ] **Step 5: Commit**

```bash
git add src/cli.rs src/lib.rs src/config.rs src/fonts.rs src/audio.rs tests/cli.rs
git commit -m "Add font sound and config utility commands"
```

### Task 11: Document, Format, And Perform Platform Smoke Tests

**Files:**
- Modify: `README.md`
- Create: `docs/manual-testing.md`
- Modify: `.gitignore`

- [ ] **Step 1: Write the manual verification matrix**

Document exact checks for macOS and Linux:

```text
cargo run -- 5s
cargo run -- 1H30
cargo run -- 2s --sound /path/to/test.mp3
cargo run -- 2s --sound /path/to/test.mp4
cargo run
cargo run -- fonts
cargo run -- sounds
```

Include expected notification, resize, any-key dismissal, `q`, `Esc`,
`Ctrl+C`, missing-file, missing-FFmpeg, and terminal-restoration results.

- [ ] **Step 2: Rewrite README usage and installation**

Document direct CLI usage first, interactive fallback second, accepted duration
formats, configuration precedence, system/custom sounds, FFmpeg fallback,
controls, supported platforms, and troubleshooting.

Add `.superpowers/` to `.gitignore`.

- [ ] **Step 3: Run automated verification**

Run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
cargo check --target x86_64-pc-windows-gnu
git diff --check
```

Expected: every command PASS when the Windows target is installed. If it is
not installed, record that limitation in `docs/manual-testing.md`; do not
install a target without user approval. Windows-specific code must remain
behind `cfg` boundaries so normal macOS/Linux builds do not depend on it.

- [ ] **Step 4: Run available host smoke tests**

On the current macOS host, execute the manual matrix entries that do not
require unavailable user-owned media. Record results in
`docs/manual-testing.md`, including the terminal emulator and date. Leave Linux
checks explicitly marked as requiring a Linux host, not as passed.

- [ ] **Step 5: Commit**

```bash
git add README.md docs/manual-testing.md .gitignore
git commit -m "Document modern alarm usage and verification"
```

### Task 12: Final Requirement Audit

**Files:**
- Modify only files required by audit findings

- [ ] **Step 1: Audit every design requirement**

Check the implementation against
`docs/superpowers/specs/2026-06-10-cross-platform-alarm-modernization-design.md`
and explicitly verify: `1H30`, interactive fallback, saved defaults, system
sounds, custom MP3/MP4, notifications, live timer, selectable fonts,
largest-fit resize, any-key dismissal, and clean `Ctrl+C`.

- [ ] **Step 2: Run the full verification suite**

Run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
git status --short
```

Expected: all checks PASS; status contains only intentional changes, if any.

- [ ] **Step 3: Commit audit fixes if needed**

```bash
git add src Cargo.toml Cargo.lock README.md docs tests
git commit -m "Finish cross-platform alarm modernization"
```

If no fixes are needed, do not create an empty commit.
