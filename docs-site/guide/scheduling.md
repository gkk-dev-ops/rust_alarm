# Scheduling

Schedule an alarm for an explicit local time with `at`:

```bash
clck at 2:50pm
clck at "tomorrow at 9am"
clck at "June 12 at 09:00"
```

`clck` resolves the expression in the system's local IANA time zone and shows
the full local date, time, UTC offset, and time-zone name before asking for
confirmation.

Past targets are rejected. Local times that are nonexistent or ambiguous
during daylight-saving transitions are also rejected instead of being guessed.

## Extract Times from Text

Use `from-text` to find explicit dates and times in text:

```bash
clck from-text
printf 'Meet tomorrow at 9am\n' | clck from-text
printf 'Choose June 12 at 09:00 or June 12 at 14:30\n' | clck from-text
```

Interactive multiline input ends when you enter a single `.` on its own line.
When multiple valid targets are found, you must select one explicitly.

Piped input uses the controlling terminal for selection and confirmation when
one is available. Without a controlling terminal, candidates are printed and
the command exits without starting an alarm.
