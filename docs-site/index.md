---
layout: home

hero:
  name: clck
  text: A countdown alarm for the terminal
  tagline: Responsive display, flexible scheduling, and configurable alarms.
  actions:
    - theme: brand
      text: Get started
      link: /guide/installation
    - theme: alt
      text: Command reference
      link: /reference/commands
---

## Start an alarm without leaving the terminal

<TerminalTimer />

## Install

::: code-group

```bash [Cargo]
cargo install clck --locked
```

```powershell [GitHub Release]
# Download the archive for your platform, verify SHA256SUMS,
# then place clck on PATH.
```

:::

## Built for terminal workflows

<div class="feature-grid">
  <article><h3>Flexible scheduling</h3><p>Use durations, explicit local times, or extract times from text.</p></article>
  <article><h3>Responsive display</h3><p>The largest fitting ASCII font is selected as the terminal resizes.</p></article>
  <article><h3>Configurable alarms</h3><p>Choose system sounds, custom media, fonts, and notifications.</p></article>
  <article><h3>Cross-platform</h3><p>Supported on macOS and Linux, with best-effort Windows support.</p></article>
</div>
