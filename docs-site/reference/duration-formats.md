# Duration Formats

Durations are case-insensitive and must be valid and nonzero.

## Seconds

A number without a unit is interpreted as seconds:

```text
45
```

## Unit Durations

Combine hours, minutes, and seconds:

```text
45s
10m
1h30m
2H15M30S
```

## Compact Hours and Minutes

A compact value beginning with hours may omit unit letters:

```text
1H30
```

This means one hour and thirty minutes. Minutes must be below 60.

## Clock Durations

Use an `HH:MM:SS` clock duration:

```text
01:30:00
```

Minutes and seconds must each be below 60.
