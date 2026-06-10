# Wall-Clock And Text Scheduling Design

## Goal

Extend `alarm-clock` so users can schedule an alarm for a specific local
wall-clock date and time, or paste arbitrary text and select an explicit time
found inside it.

The feature remains private, deterministic, and offline. It does not use an AI
service, create a background daemon, or survive closing the foreground
terminal process.

## CLI

New commands:

```text
alarm-clock at 2:50pm
alarm-clock at "tomorrow at 9am"
alarm-clock at "June 12 at 09:00"
alarm-clock from-text
pbpaste | alarm-clock from-text
cat message.txt | alarm-clock from-text
```

The existing countdown-duration interface remains unchanged:

```text
alarm-clock 10m
alarm-clock 1H30
```

Existing sound, font, and notification options apply to wall-clock and
text-derived alarms.

## Direct Wall-Clock Scheduling

`alarm-clock at VALUE` parses one explicit local date and time.

Supported forms include:

- Twelve-hour times such as `2:50pm`, `3 p.m.`, and `9 AM`
- Twenty-four-hour times such as `14:30`
- Relative dates `today` and `tomorrow`
- Explicit dates such as `June 12 at 09:00`

A time without a date resolves to today in the system's current local time
zone.

Before starting, the application displays the fully resolved local date, time,
and time zone and asks the user to confirm it.

If the resolved value is in the past, the application rejects it and asks the
user to enter another value. It does not silently schedule the time for
tomorrow and does not ring immediately.

## Text Input And Candidate Selection

`alarm-clock from-text` reads arbitrary text:

- When stdin is piped, it reads the complete stream until EOF.
- When stdin is an interactive terminal, it opens multiline paste mode and
  provides a clear platform-neutral instruction for ending input.

The application extracts every explicit date/time candidate that it can
resolve. It does not guess vague phrases such as `later`, `afternoon`, or
`after lunch`.

If multiple unique candidates are found, the application shows all resolved
values in a selection menu. If one candidate is found, it is selected
automatically. In both cases, the selected value requires confirmation before
the alarm starts.

Candidates are deduplicated by their resolved local date and time while
preserving the source text for display.

If no candidates are found, the application explains accepted examples and,
when interactive, allows the user to paste or enter another value.

## Interactivity And Piped Input

Confirmation is required for every wall-clock or text-derived alarm.

Piped text supplies the source material but does not authorize starting an
alarm without confirmation. When a controlling interactive terminal is
available, selection and confirmation use it.

If no interactive terminal is available, the command prints the detected
candidates and exits with a non-zero status, explaining how to rerun the
command interactively. It does not choose or start an alarm automatically.

## Runtime Behavior

After confirmation, the application calculates the duration between the
current local time and the selected target. It then starts the existing
foreground alarm lifecycle.

The countdown uses the existing monotonic timer after scheduling begins. A
wall-clock change while the process is running does not unexpectedly shorten
or extend the alarm.

The countdown display shows:

- Remaining duration in the largest ASCII font that fits
- The target local wall-clock date and time when terminal space permits
- The existing sound and cancellation instructions

The time-up desktop notification includes the confirmed target time.

Closing or cancelling the foreground process cancels the scheduled alarm.

## Architecture

Add a focused `schedule` module responsible for:

- Parsing one direct wall-clock expression
- Extracting explicit date/time candidates from arbitrary text
- Resolving candidates in the current local time zone
- Rejecting past and ambiguous values
- Deduplicating candidates
- Converting a confirmed target into a countdown duration

The `cli` module owns the `at` and `from-text` commands, source input, candidate
selection, and confirmation.

The `app` and `display` modules accept optional target-time metadata but keep
the existing monotonic countdown and alarm lifecycle.

The scheduling parser uses maintained Rust date/time parsing libraries such as
`chrono` and `dateparser`, plus deliberately constrained extraction patterns
for finding candidate spans in arbitrary text. Extraction is conservative:
only explicit time expressions are offered.

## Date And Time Rules

- Resolution uses the system's current local time zone.
- A time without a date means today.
- `today` and `tomorrow` use the current local calendar date.
- An explicit date without a year uses the current year.
- An explicit date that resolves into the past is rejected rather than moved
  into the next year.
- Seconds default to zero when absent.
- Duplicate resolved candidates appear only once.
- A nonexistent or ambiguous local time caused by daylight-saving transitions
  is rejected with an explanation.

## Error Handling

- Invalid direct input shows accepted examples and asks for another value when
  interactive.
- Past direct input shows the resolved past value and asks for another.
- No text candidates shows accepted examples and permits another paste when
  interactive.
- Ambiguous daylight-saving time is rejected rather than guessed.
- Non-interactive execution without a controlling terminal prints candidates
  and instructions, then exits without starting an alarm.
- Existing terminal, audio, notification, and cancellation cleanup behavior is
  preserved.

## Testing

Automated tests cover:

- Twelve-hour and twenty-four-hour direct parsing
- Time-only resolution to today
- `today` and `tomorrow`
- Explicit month/day parsing
- Current-year behavior
- Past-time rejection
- Daylight-saving nonexistent and ambiguous times
- Extraction of one and multiple explicit candidates
- Rejection of vague phrases
- Candidate deduplication
- Confirmation requirements
- Piped-input behavior without an interactive terminal
- Conversion of confirmed targets into monotonic countdown durations
- Display and notification target-time metadata

Manual smoke tests cover:

- `alarm-clock at 2:50pm`
- `alarm-clock at "tomorrow at 9am"`
- Interactive multiline paste mode
- `pbpaste | alarm-clock from-text`
- Selection from text containing multiple times
- Cancellation and clean terminal restoration

## Relationship To Release Automation

The previously approved GitHub release-automation design remains a separate
project. Its implementation can include this scheduling feature in release
artifacts once both projects are implemented, but scheduling requirements do
not alter CI versioning or packaging behavior.

## Initial Boundary

This feature does not add multiple simultaneous alarms, recurring alarms,
background daemon operation, system scheduler integration, calendar
integration, natural-language AI parsing, or persistence across process exit.
