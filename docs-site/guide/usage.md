# Usage

Pass a duration to start a countdown immediately:

```bash
clck 45s
clck 10m
clck 1h30m
clck 01:30:00
```

The timer uses the largest bundled ASCII font that fits the terminal and
redraws when the terminal is resized.

## Interactive Setup

Run `clck` without a duration to open guided setup:

```bash
clck
```

The prompts collect a duration, font, alarm sound, and notification preference.
You can save those choices as defaults.

## Countdown and Alarm

During a countdown, `q`, `Esc`, or `Ctrl+C` cancels and restores the terminal.
When time expires, the sound loops and a desktop notification is sent when
notifications are enabled. Press any key to dismiss the alarm.

The terminal mode and cursor visibility are restored after normal exit,
cancellation, dismissal, and errors.
