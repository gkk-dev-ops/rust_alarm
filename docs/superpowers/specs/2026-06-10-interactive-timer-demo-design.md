# Interactive Timer Demo Design

## Goal

Replace the static homepage terminal example with a real, predefined ten-second
countdown that lets visitors experience the core `clck` interaction directly
on the documentation site.

## Architecture

Implement the demo as an isolated Vue component rendered inside the existing
VitePress homepage. The component owns countdown state, interval cleanup, the
completion animation, and synthesized audio. The surrounding homepage and
documentation structure remain unchanged.

The component is registered globally by the custom VitePress theme and used
from `docs-site/index.md`.

## Interaction

The initial terminal display shows:

```text
PS> clck 10s
Ready to start the demo.
00:00:10
```

A `Start demo` button begins the countdown. While counting:

- The display updates from `00:00:10` to `00:00:00`.
- The status line indicates that the countdown is running.
- The button label changes to `Reset`.
- Pressing `Reset` immediately returns the demo to its initial state.

At zero:

- The terminal display receives a brief blue completion pulse.
- A short terminal-style synthesized chime plays once.
- The status line announces that time is up.
- After two seconds, the component resets automatically.

Starting again after a reset creates a fresh ten-second countdown.

## Timing

Use an absolute end timestamp rather than decrementing a counter on every
interval tick. Each update calculates the remaining whole seconds from the
current time and the stored end time. This prevents visible drift when browser
timers are throttled or delayed.

The component clears active intervals and reset timeouts when reset or
unmounted.

## Audio

Generate the completion sound with the browser Web Audio API. No audio asset is
stored in the repository.

The visitor's `Start demo` click creates or resumes the audio context, satisfying
browser autoplay restrictions. At completion, two short oscillator tones form
a restrained terminal-style chime and stop automatically.

If Web Audio is unavailable or blocked, the countdown and visual completion
state still work without showing an error.

## Accessibility

- Use an actual button with visible focus styles.
- Expose the timer and status text through an `aria-live="polite"` region.
- Avoid updating the live region more often than once per displayed second.
- Respect `prefers-reduced-motion` by disabling the completion pulse animation.
- Keep sufficient contrast in dark and light themes.

## Visual Design

Reuse the existing terminal panel and PowerShell-inspired palette. Add only:

- A compact terminal-style action button.
- Running and completed status treatments.
- A brief blue border/background pulse at completion.

The component remains responsive at mobile widths and does not introduce
horizontal scrolling.

## Verification

The implementation is complete when:

- The VitePress build succeeds with the Vue component.
- Initial rendering shows `00:00:10` and `Start demo`.
- Starting counts down to zero using an absolute end timestamp.
- Reset returns immediately to the initial state.
- Completion plays one short chime when Web Audio is available.
- Completion resets automatically after two seconds.
- Timers are cleared when the component is unmounted.
- Reduced-motion users do not receive the completion animation.
- The existing documentation routes and GitHub Pages base path still work.

## Out of Scope

- User-selectable durations.
- Pause and resume.
- Looping alarm audio or a dismiss interaction.
- Desktop notifications.
- Reusing the Rust timer implementation in the browser.
