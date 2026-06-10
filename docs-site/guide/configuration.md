# Configuration

Inspect or reset the effective configuration:

```bash
clck config --show
clck config --reset
```

Configuration is stored in the platform-standard application configuration
directory. Its default shape is:

```toml
font = "standard"
notification = true

[sound]
kind = "system"
value = "Glass"
```

Command-line options override saved values for one invocation:

```bash
clck 5m --font banner
clck 5m --sound ~/Music/alarm.mp3
clck 5m --no-notification
```

List available bundled fonts and discovered logical system sounds:

```bash
clck fonts
clck sounds
```

macOS defaults to the logical system sound `Glass`. Linux discovers sounds from
freedesktop sound locations. If no system sound is available, `clck` uses the
terminal bell.
