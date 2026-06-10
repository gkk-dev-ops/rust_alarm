# Usage

Pass a duration to start a countdown immediately:

```bash
clck 45
clck 45s
clck 10m
clck 1h30m
clck 01:30:00
```

Numbers without a unit are interpreted as seconds.

The timer uses the largest bundled ASCII font that fits the terminal and
redraws when the terminal is resized.

## Interactive Setup

Run `clck` without a duration to open guided setup:

```bash
clck
```

The prompts collect a duration, optional title, font, alarm sound, and notification
preference. You can save those choices as defaults.

## Countdown and Alarm

During a countdown, your title is displayed in the status line if provided.
`q`, `Esc`, or `Ctrl+C` cancels and restores the terminal.
When time expires, the sound loops, the title is displayed on the screen,
and a desktop notification is sent. Press any key to dismiss the alarm.

The terminal mode and cursor visibility are restored after normal exit,
cancellation, dismissal, and errors.
