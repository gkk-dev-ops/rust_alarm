# Wall-Clock And Text Scheduling Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add confirmed foreground alarms scheduled from direct local wall-clock expressions or explicit date/time candidates extracted from pasted and piped text.

**Architecture:** Add a pure `schedule` module that parses, resolves, validates, deduplicates, and converts local targets into countdown durations. Extend `cli` with the `at` and `from-text` commands plus line-oriented source, selection, and confirmation helpers; then pass optional target metadata through the existing monotonic alarm lifecycle to display and notifications.

**Tech Stack:** Rust 2021, `clap`, `chrono`, `chrono-tz`, `iana-time-zone`, `regex`, existing `crossterm`, `notify-rust`, `anyhow`, `thiserror`, `assert_cmd`, and `predicates`.

---

## File Structure

- `Cargo.toml`: add date/time parsing and extraction dependencies.
- `src/schedule.rs`: scheduling types, constrained parsing, local-time resolution, extraction, deduplication, past-time validation, and countdown conversion.
- `src/cli.rs`: add scheduling commands and own text input, candidate selection, and confirmation behavior.
- `src/lib.rs`: route scheduling commands, require confirmation, and build a shared alarm request.
- `src/app.rs`: carry optional confirmed target metadata through the alarm lifecycle.
- `src/display.rs`: include target metadata in countdown and ringing output when space permits.
- `src/notification.rs`: include confirmed target metadata in the time-up notification.
- `tests/cli.rs`: binary help and non-interactive scheduling behavior.
- `docs/manual-testing.md`: direct, pasted, piped, selection, confirmation, and cancellation smoke tests.

### Task 1: Add Scheduling Types And Countdown Conversion

**Files:**
- Modify: `Cargo.toml`
- Create: `src/schedule.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing scheduling-model tests**

Create `src/schedule.rs` with tests that define the required public contract:

```rust
#[cfg(test)]
mod tests {
    use super::{duration_until, Candidate, ScheduleError};
    use chrono::{FixedOffset, TimeZone};
    use std::time::Duration;

    fn fixed(hour: u32, minute: u32) -> chrono::DateTime<FixedOffset> {
        FixedOffset::east_opt(7_200)
            .unwrap()
            .with_ymd_and_hms(2026, 6, 10, hour, minute, 0)
            .unwrap()
    }

    #[test]
    fn converts_future_target_to_countdown_duration() {
        assert_eq!(
            duration_until(fixed(12, 0), fixed(12, 1)).unwrap(),
            Duration::from_secs(60)
        );
    }

    #[test]
    fn rejects_past_and_equal_targets() {
        assert_eq!(
            duration_until(fixed(12, 0), fixed(11, 59)).unwrap_err(),
            ScheduleError::PastTarget(fixed(11, 59))
        );
        assert!(duration_until(fixed(12, 0), fixed(12, 0)).is_err());
    }

    #[test]
    fn candidate_formats_resolved_target_with_zone() {
        let candidate = Candidate::new("tomorrow at 9am", fixed(9, 0), "CEST");
        assert_eq!(
            candidate.display_target(),
            "2026-06-10 09:00:00 +02:00 (CEST)"
        );
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test schedule`

Expected: FAIL because `schedule` is not exported and its types do not exist.

- [ ] **Step 3: Add dependencies and implement the scheduling model**

Add to `Cargo.toml`:

```toml
chrono = { version = "0.4", features = ["clock"] }
chrono-tz = "0.10"
iana-time-zone = "0.1"
regex = "1"
```

Implement and export:

```rust
use chrono::{DateTime, FixedOffset};
use std::time::Duration;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    pub source: String,
    pub target: DateTime<FixedOffset>,
    pub timezone: String,
}

impl Candidate {
    pub fn new(
        source: impl Into<String>,
        target: DateTime<FixedOffset>,
        timezone: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            target,
            timezone: timezone.into(),
        }
    }

    pub fn display_target(&self) -> String {
        format!(
            "{} ({})",
            self.target.format("%Y-%m-%d %H:%M:%S %:z"),
            self.timezone
        )
    }
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ScheduleError {
    #[error("could not find an explicit date and time; try examples such as `2:50pm`, `tomorrow at 9am`, or `June 12 at 09:00`")]
    InvalidExpression,
    #[error("resolved target is not in the future: {0}")]
    PastTarget(DateTime<FixedOffset>),
    #[error("local time does not exist because of a daylight-saving transition")]
    NonexistentLocalTime,
    #[error("local time is ambiguous because of a daylight-saving transition")]
    AmbiguousLocalTime,
    #[error("target is too far in the future to represent as a countdown")]
    DurationOutOfRange,
    #[error("could not determine the system's local IANA time zone")]
    LocalTimeZoneUnavailable,
}

pub fn duration_until(
    now: DateTime<FixedOffset>,
    target: DateTime<FixedOffset>,
) -> Result<Duration, ScheduleError> {
    let delta = target.signed_duration_since(now);
    if delta <= chrono::Duration::zero() {
        return Err(ScheduleError::PastTarget(target));
    }
    delta.to_std().map_err(|_| ScheduleError::DurationOutOfRange)
}
```

Add `pub mod schedule;` to `src/lib.rs`.

- [ ] **Step 4: Verify the scheduling model**

Run: `cargo test schedule`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock src/schedule.rs src/lib.rs
git commit -m "Add wall-clock scheduling model"
```

### Task 2: Parse And Resolve Direct Wall-Clock Expressions

**Files:**
- Modify: `src/schedule.rs`

- [ ] **Step 1: Write failing direct-parser tests**

Add deterministic tests using `chrono_tz::America::New_York`:

```rust
use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::America::New_York;

fn context_now() -> chrono::DateTime<chrono_tz::Tz> {
    New_York.with_ymd_and_hms(2026, 6, 10, 8, 0, 0).unwrap()
}

#[test]
fn parses_supported_direct_expressions() {
    for (input, expected_day, expected_hour, expected_minute) in [
        ("2:50pm", 10, 14, 50),
        ("3 p.m.", 10, 15, 0),
        ("14:30", 10, 14, 30),
        ("today at 9 AM", 10, 9, 0),
        ("tomorrow at 9am", 11, 9, 0),
        ("June 12 at 09:00", 12, 9, 0),
    ] {
        let candidate = parse_direct_in(input, context_now(), New_York).unwrap();
        assert_eq!(candidate.target.day(), expected_day);
        assert_eq!(candidate.target.hour(), expected_hour);
        assert_eq!(candidate.target.minute(), expected_minute);
        assert_eq!(candidate.target.second(), 0);
    }
}

#[test]
fn rejects_past_explicit_and_time_only_values() {
    assert!(matches!(
        parse_direct_in("7am", context_now(), New_York),
        Err(ScheduleError::PastTarget(_))
    ));
    assert!(matches!(
        parse_direct_in("June 9 at 09:00", context_now(), New_York),
        Err(ScheduleError::PastTarget(_))
    ));
}

#[test]
fn rejects_nonexistent_and_ambiguous_dst_values() {
    let spring = New_York.with_ymd_and_hms(2026, 3, 7, 12, 0, 0).unwrap();
    assert_eq!(
        parse_direct_in("March 8 at 02:30", spring, New_York).unwrap_err(),
        ScheduleError::NonexistentLocalTime
    );
    let fall = New_York.with_ymd_and_hms(2026, 10, 31, 12, 0, 0).unwrap();
    assert_eq!(
        parse_direct_in("November 1 at 01:30", fall, New_York).unwrap_err(),
        ScheduleError::AmbiguousLocalTime
    );
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test schedule`

Expected: FAIL because `parse_direct_in` does not exist.

- [ ] **Step 3: Implement constrained direct parsing**

Implement `parse_direct_in<Tz>` as a small parser pipeline:

```rust
pub fn parse_direct(input: &str) -> Result<Candidate, ScheduleError> {
    let name =
        iana_time_zone::get_timezone().map_err(|_| ScheduleError::LocalTimeZoneUnavailable)?;
    let timezone: chrono_tz::Tz = name
        .parse()
        .map_err(|_| ScheduleError::LocalTimeZoneUnavailable)?;
    let now = chrono::Utc::now().with_timezone(&timezone);
    parse_direct_in(input, now, timezone)
}

pub fn parse_direct_in<Tz>(
    input: &str,
    now: DateTime<Tz>,
    timezone: Tz,
) -> Result<Candidate, ScheduleError>
where
    Tz: chrono::TimeZone + Copy + std::fmt::Display,
    Tz::Offset: std::fmt::Display,
{
    let normalized = normalize_expression(input);
    let (date, time) = parse_date_and_time(&normalized, now.date_naive())
        .ok_or(ScheduleError::InvalidExpression)?;
    let naive = date.and_time(time);
    let resolved = match timezone.from_local_datetime(&naive) {
        chrono::LocalResult::Single(value) => value,
        chrono::LocalResult::None => return Err(ScheduleError::NonexistentLocalTime),
        chrono::LocalResult::Ambiguous(_, _) => return Err(ScheduleError::AmbiguousLocalTime),
    };
    let target = resolved.fixed_offset();
    duration_until(now.fixed_offset(), target)?;
    Ok(Candidate::new(input.trim(), target, timezone.to_string()))
}
```

Add focused private helpers:

- `normalize_expression`: lowercase, trim, convert `a.m.`/`p.m.` to `am`/`pm`, and collapse whitespace.
- `parse_date_and_time`: accept time-only, `today`, `tomorrow`, or one English month plus day, optionally separated from the time by `at`.
- `parse_time`: accept `H am`, `H:MM am`, `HH:MM`, and reject invalid hour/minute values.
- `parse_date`: use the current year for explicit month/day and never roll a past date into the next year.

Use `chrono::NaiveDate::from_ymd_opt` and `chrono::NaiveTime::from_hms_opt` for all validation. Do not use permissive free-form parsing, because the spec requires deterministic rejection of vague input.

- [ ] **Step 4: Verify direct parsing**

Run: `cargo test schedule`

Expected: PASS, including past-time and DST cases.

- [ ] **Step 5: Commit**

```bash
git add src/schedule.rs
git commit -m "Parse direct local alarm times"
```

### Task 3: Extract And Deduplicate Explicit Candidates From Text

**Files:**
- Modify: `src/schedule.rs`

- [ ] **Step 1: Write failing extraction tests**

Add:

```rust
#[test]
fn extracts_one_and_multiple_explicit_candidates() {
    let one = extract_candidates_in(
        "Please call me tomorrow at 9am.",
        context_now(),
        New_York,
    );
    assert_eq!(one.len(), 1);
    assert_eq!(one[0].source, "tomorrow at 9am");

    let many = extract_candidates_in(
        "Try tomorrow at 9am or June 12 at 14:30.",
        context_now(),
        New_York,
    );
    assert_eq!(many.len(), 2);
}

#[test]
fn ignores_vague_phrases_and_deduplicates_resolved_targets() {
    assert!(extract_candidates_in(
        "Let's talk later in the afternoon after lunch.",
        context_now(),
        New_York,
    )
    .is_empty());
    let candidates = extract_candidates_in(
        "Use tomorrow at 9am, or June 11 at 09:00.",
        context_now(),
        New_York,
    );
    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].source, "tomorrow at 9am");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test schedule::tests::extracts schedule::tests::ignores`

Expected: FAIL because candidate extraction does not exist.

- [ ] **Step 3: Implement conservative extraction and deduplication**

Add:

```rust
pub fn extract_candidates(text: &str) -> Result<Vec<Candidate>, ScheduleError> {
    let name =
        iana_time_zone::get_timezone().map_err(|_| ScheduleError::LocalTimeZoneUnavailable)?;
    let timezone = name
        .parse::<chrono_tz::Tz>()
        .map_err(|_| ScheduleError::LocalTimeZoneUnavailable)?;
    let now = chrono::Utc::now().with_timezone(&timezone);
    Ok(extract_candidates_in(text, now, timezone))
}

pub fn extract_candidates_in<Tz>(
    text: &str,
    now: DateTime<Tz>,
    timezone: Tz,
) -> Vec<Candidate>
where
    Tz: chrono::TimeZone + Copy + std::fmt::Display,
    Tz::Offset: std::fmt::Display,
{
    let patterns = candidate_patterns();
    let mut seen = std::collections::HashSet::new();
    let mut candidates = Vec::new();
    for matched in patterns.iter().flat_map(|pattern| pattern.find_iter(text)) {
        if let Ok(candidate) = parse_direct_in(matched.as_str(), now.clone(), timezone) {
            if seen.insert(candidate.target) {
                candidates.push(candidate);
            }
        }
    }
    candidates.sort_by_key(|candidate| candidate.target);
    candidates
}
```

Build `candidate_patterns()` from these case-insensitive `regex::Regex` patterns, ordered from most specific to least specific:

```rust
fn candidate_patterns() -> Vec<regex::Regex> {
    let time = r"(?:(?:1[0-2]|0?[1-9])(?::[0-5]\d)?\s*(?:a\.?m\.?|p\.?m\.?)|(?:[01]?\d|2[0-3]):[0-5]\d)";
    let month = r"(?:january|february|march|april|may|june|july|august|september|october|november|december)";
    [
        format!(r"(?i)\b(?:today|tomorrow)\s+(?:at\s+)?{time}\b"),
        format!(r"(?i)\b{month}\s+\d{{1,2}}\s+(?:at\s+)?{time}\b"),
        r"(?i)\b(?:1[0-2]|0?[1-9])(?::[0-5]\d)?\s*(?:a\.?m\.?|p\.?m\.?)\b".to_owned(),
        r"(?i)\b(?:[01]?\d|2[0-3]):[0-5]\d\b".to_owned(),
    ]
    .into_iter()
    .map(|pattern| regex::Regex::new(&pattern).expect("candidate regex is valid"))
    .collect()
}
```

Preserve the exact matched source slice in `Candidate::source`. Skip invalid, past, nonexistent, and ambiguous matches. Deduplicate by resolved `DateTime<FixedOffset>` while retaining the first source occurrence, then sort candidates chronologically for selection.

- [ ] **Step 4: Verify extraction**

Run: `cargo test schedule`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/schedule.rs
git commit -m "Extract scheduling candidates from text"
```

### Task 4: Add Scheduling Commands, Text Input, Selection, And Confirmation

**Files:**
- Modify: `src/cli.rs`

- [ ] **Step 1: Write failing CLI-model tests**

Add tests for scheduling command parsing and pure prompt decisions:

```rust
use clap::Parser;
use super::{choose_candidate, parse_confirmation, Cli, Command};

#[test]
fn parses_scheduling_commands_with_existing_options() {
    let cli = Cli::try_parse_from(["alarm-clock", "at", "tomorrow at 9am", "--font", "box"])
        .unwrap();
    assert!(matches!(cli.command, Some(Command::At { .. })));
    assert_eq!(cli.font.as_deref(), Some("box"));

    let cli = Cli::try_parse_from(["alarm-clock", "from-text", "--no-notification"]).unwrap();
    assert!(matches!(cli.command, Some(Command::FromText)));
    assert!(cli.no_notification);
}

#[test]
fn selection_and_confirmation_are_explicit() {
    assert_eq!(choose_candidate(1, "1").unwrap(), 0);
    assert_eq!(choose_candidate(3, "2").unwrap(), 1);
    assert!(choose_candidate(3, "4").is_err());
    assert!(parse_confirmation("yes").unwrap());
    assert!(!parse_confirmation("n").unwrap());
    assert!(parse_confirmation("").is_err());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test cli`

Expected: FAIL because the commands and prompt helpers do not exist.

- [ ] **Step 3: Add command types and line-oriented interaction helpers**

Mark the existing `sound`, `font`, and `no_notification` arguments with `global = true`, then extend `Command`:

```rust
#[derive(Subcommand, Debug)]
pub enum Command {
    At {
        #[arg(value_name = "VALUE")]
        value: String,
    },
    FromText,
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

Add pure validation helpers:

```rust
pub fn choose_candidate(count: usize, input: &str) -> Result<usize> {
    let selected: usize = input.trim().parse().context("enter a candidate number")?;
    if selected == 0 || selected > count {
        anyhow::bail!("choose a number from 1 to {count}");
    }
    Ok(selected - 1)
}

pub fn parse_confirmation(input: &str) -> Result<bool> {
    match input.trim().to_ascii_lowercase().as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => anyhow::bail!("enter yes or no"),
    }
}
```

Add focused I/O helpers:

- `read_text_source<R: BufRead, W: Write>(reader, writer, interactive)`: read to EOF for piped input; for interactive input, print `Paste or enter text. Finish with a single '.' on its own line.` and stop on that sentinel.
- `select_candidate<R: BufRead, W: Write>(candidates, reader, writer)`: automatically return the only candidate; otherwise print numbered source and resolved target values and require a valid number.
- `confirm_candidate<R: BufRead, W: Write>(candidate, reader, writer)`: print the fully resolved date, time, offset, and time-zone label, then require an explicit yes.
- `open_controlling_terminal()`: on Unix open `/dev/tty` for read/write; on Windows open `CONIN$` and `CONOUT$`; return `None` when no controlling terminal exists.

Keep these helpers line-oriented and separate from `crossterm` raw-mode countdown handling.

- [ ] **Step 4: Verify CLI models**

Run: `cargo test cli && cargo run -- --help && cargo run -- at --help && cargo run -- from-text --help`

Expected: tests PASS; help lists `at` and `from-text`; existing sound, font, and notification options are accepted after scheduling subcommands.

- [ ] **Step 5: Commit**

```bash
git add src/cli.rs
git commit -m "Add scheduling CLI interaction"
```

### Task 5: Route Confirmed Scheduling Commands Into The Alarm Lifecycle

**Files:**
- Modify: `src/lib.rs`
- Modify: `src/app.rs`

- [ ] **Step 1: Write failing request-construction tests**

Move alarm-request construction into a testable helper and add:

```rust
#[test]
fn scheduled_request_keeps_target_metadata() {
    let effective = Config::default();
    let request = build_alarm_request(
        Duration::from_secs(60),
        effective,
        Some("2026-06-11 09:00:00 -04:00 (EDT)".into()),
    )
    .unwrap();
    assert_eq!(
        request.target.as_deref(),
        Some("2026-06-11 09:00:00 -04:00 (EDT)")
    );
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test scheduled_request_keeps_target_metadata`

Expected: FAIL because `build_alarm_request` and `AlarmRequest::target` do not exist.

- [ ] **Step 3: Implement shared request construction and scheduling flow**

Extend `AlarmRequest`:

```rust
pub struct AlarmRequest {
    pub duration: Duration,
    pub font: String,
    pub sound_name: String,
    pub sound: ResolvedSound,
    pub notification: bool,
    pub target: Option<String>,
}
```

Refactor the existing sound resolution into:

```rust
fn build_alarm_request(
    duration: Duration,
    effective: Config,
    target: Option<String>,
) -> anyhow::Result<app::AlarmRequest>
```

In `run`, handle scheduling commands before the existing duration/interactive fallback:

1. `at VALUE`: obtain a controlling terminal, call `schedule::parse_direct`, and confirm the fully resolved target. If parsing fails while a terminal is available, prompt for another expression until parsing succeeds or input closes. Without a controlling terminal, print the resolved candidate or parsing error and exit non-zero because confirmation is impossible.
2. `from-text`: read the source from stdin; when stdin is piped, open the controlling terminal for selection and confirmation; when stdin is interactive, use multiline mode and stdin for interaction.
3. If interactive text contains no candidate, print accepted examples and offer multiline input again. If piped text contains no candidate, print accepted examples and return a non-zero error.
4. If piped input has no controlling terminal, print every detected source and resolved target, explain that confirmation is required, and return a non-zero error without starting an alarm.
5. After explicit confirmation, calculate `duration_until(Local::now().fixed_offset(), candidate.target)`. If the target became past while the user was selecting or confirming, reject it and return to entry/selection instead of ringing immediately.
6. Resolve existing sound/font/notification options through `Config::resolve`, build the request with `Some(candidate.display_target())`, and call `app::run_alarm`.

For the existing duration and guided fallback paths, call the same helper with `target: None`.

- [ ] **Step 4: Verify routing and existing behavior**

Run: `cargo test && cargo clippy --all-targets -- -D warnings`

Expected: PASS; no existing duration, config, font, or sound behavior regresses.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs src/app.rs
git commit -m "Route confirmed schedules into alarms"
```

### Task 6: Display And Notify With Confirmed Target Metadata

**Files:**
- Modify: `src/display.rs`
- Modify: `src/app.rs`
- Modify: `src/notification.rs`

- [ ] **Step 1: Write failing metadata rendering tests**

Extract pure status/body builders and add:

```rust
#[test]
fn countdown_status_includes_optional_target() {
    assert_eq!(
        countdown_status("Glass", Some("2026-06-11 09:00 EDT")),
        "Target: 2026-06-11 09:00 EDT | Sound: Glass | q/Esc/Ctrl+C to cancel"
    );
    assert_eq!(
        countdown_status("Glass", None),
        "Sound: Glass | q/Esc/Ctrl+C to cancel"
    );
}

#[test]
fn notification_body_includes_optional_target() {
    assert_eq!(
        notification_body(Some("2026-06-11 09:00 EDT")),
        "Time is up for 2026-06-11 09:00 EDT."
    );
    assert_eq!(notification_body(None), "Time is up!");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test countdown_status && cargo test notification_body`

Expected: FAIL because the builders do not exist.

- [ ] **Step 3: Thread metadata through display and notification adapters**

Change display APIs:

```rust
pub fn countdown_status(sound: &str, target: Option<&str>) -> String

pub fn render_countdown(
    &self,
    remaining: Duration,
    preferred_font: &str,
    sound: &str,
    target: Option<&str>,
) -> Result<()>

pub fn render_ringing(&self, target: Option<&str>) -> Result<()>
```

Only include the target status line when it fits the current terminal width; retain the existing compact countdown fallback when height is constrained.

Change notification APIs:

```rust
pub fn notification_body(target: Option<&str>) -> String

pub fn notify_time_up(target: Option<&str>) -> Result<()>
```

Update `app::run_alarm` to pass `request.target.as_deref()` into countdown rendering, ringing rendering, and notification delivery. Notification failure remains non-fatal.

- [ ] **Step 4: Verify presentation behavior**

Run: `cargo test && cargo clippy --all-targets -- -D warnings`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/display.rs src/app.rs src/notification.rs
git commit -m "Show scheduled target in alarm output"
```

### Task 7: Cover Non-Interactive Behavior And Update Manual Tests

**Files:**
- Modify: `tests/cli.rs`
- Modify: `docs/manual-testing.md`

- [ ] **Step 1: Write failing binary-level scheduling tests**

Add:

```rust
#[test]
fn help_lists_scheduling_commands() {
    Command::cargo_bin("alarm-clock")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("at"))
        .stdout(contains("from-text"));
}

#[test]
fn piped_text_without_terminal_never_starts_alarm() {
    Command::cargo_bin("alarm-clock")
        .unwrap()
        .arg("from-text")
        .write_stdin("Meet tomorrow at 9am")
        .assert()
        .failure()
        .stderr(contains("confirmation"))
        .stderr(contains("tomorrow at 9am"));
}

#[test]
fn vague_piped_text_reports_accepted_examples() {
    Command::cargo_bin("alarm-clock")
        .unwrap()
        .arg("from-text")
        .write_stdin("Let's talk later after lunch")
        .assert()
        .failure()
        .stderr(contains("2:50pm"))
        .stderr(contains("tomorrow at 9am"));
}
```

- [ ] **Step 2: Run tests to verify failures are meaningful**

Run: `cargo test --test cli`

Expected: the new tests FAIL until stderr output and no-terminal detection exactly match the required behavior.

- [ ] **Step 3: Align error output and document smoke tests**

Adjust only scheduling error messages needed by the binary tests. Add these commands to `docs/manual-testing.md`:

```bash
cargo run -- at 2:50pm
cargo run -- at "tomorrow at 9am"
cargo run -- from-text
printf 'Meet tomorrow at 9am\n' | cargo run -- from-text
printf 'Choose June 12 at 09:00 or June 12 at 14:30\n' | cargo run -- from-text
```

Document verification that:

- Every resolved target displays local date, time, UTC offset, and time-zone label before confirmation.
- Rejecting confirmation exits without starting a countdown.
- Past, nonexistent DST, and ambiguous DST targets are rejected rather than guessed.
- Interactive multiline input ends on a single `.` line.
- Multiple candidates require explicit selection.
- Piped input uses a controlling terminal for selection and confirmation when available.
- Piped input without a controlling terminal prints candidates and exits non-zero.
- Countdown target metadata appears when terminal space permits.
- Time-up notification includes the confirmed target.
- Cancellation still restores the terminal cleanly.

- [ ] **Step 4: Run complete verification**

Run:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

Expected: all commands PASS.

- [ ] **Step 5: Commit**

```bash
git add tests/cli.rs docs/manual-testing.md
git commit -m "Test and document scheduled alarms"
```

### Task 8: Perform Platform Smoke Tests

**Files:**
- Modify: `docs/manual-testing.md`

- [ ] **Step 1: Run direct and text scheduling on the current platform**

Run the commands documented in Task 7 with targets a few minutes in the future.

Expected: direct and extracted targets require confirmation, the countdown uses the resolved duration, cancellation restores the terminal, and expiry includes target metadata.

- [ ] **Step 2: Exercise selection and rejection paths**

Test multiple candidates, vague text, a past time, a known local DST gap, and a known local DST overlap.

Expected: selection is explicit; vague, past, nonexistent, and ambiguous values never start an alarm.

- [ ] **Step 3: Record dated smoke-test results**

Append a dated scheduling subsection to `docs/manual-testing.md` listing the platform, commands exercised, successful behaviors, and any checks still requiring another platform.

- [ ] **Step 4: Re-run automated verification after documentation changes**

Run:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

Expected: all commands PASS.

- [ ] **Step 5: Commit**

```bash
git add docs/manual-testing.md
git commit -m "Record scheduled alarm smoke tests"
```
