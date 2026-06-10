# Documentation Site Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and deploy a restrained terminal-inspired VitePress site containing a branded `clck` homepage and complete CLI documentation.

**Architecture:** Keep the public site isolated under `docs-site/`, with VitePress configuration and a small custom theme beside Markdown content. Build the site in the existing CI workflow and deploy the generated static output through a dedicated GitHub Pages workflow using the `/clck/` base path.

**Tech Stack:** VitePress 1.6.4, Vue 3 through VitePress, CSS, Markdown, npm, GitHub Actions, GitHub Pages

---

## File Structure

| Path | Responsibility |
| --- | --- |
| `docs-site/package.json` | Pinned documentation build dependency and npm scripts |
| `docs-site/package-lock.json` | Reproducible Node dependency graph |
| `docs-site/.vitepress/config.mts` | Site metadata, base path, navigation, sidebar, search, and social links |
| `docs-site/.vitepress/theme/index.ts` | Loads the default VitePress theme and project CSS |
| `docs-site/.vitepress/theme/custom.css` | Ghostty/PowerShell-inspired tokens, homepage, terminal, and responsive styles |
| `docs-site/index.md` | Branded homepage and quick installation entry points |
| `docs-site/guide/*.md` | User-oriented installation, usage, scheduling, and configuration guides |
| `docs-site/reference/*.md` | Exact command, duration-format, and runtime-control reference |
| `docs-site/development/*.md` | Contributor testing and release documentation |
| `.gitignore` | Excludes installed dependencies and generated VitePress output |
| `.github/workflows/ci.yml` | Validates that documentation builds with the Rust project |
| `.github/workflows/pages.yml` | Builds and deploys the site to GitHub Pages |
| `README.md` | Links repository visitors to the public documentation site |

### Task 1: Scaffold the VitePress Site

**Files:**
- Create: `docs-site/package.json`
- Create: `docs-site/package-lock.json`
- Create: `docs-site/.vitepress/config.mts`
- Create: `docs-site/.vitepress/theme/index.ts`
- Create: `docs-site/index.md`
- Modify: `.gitignore`

- [ ] **Step 1: Create the pinned npm package definition**

Create `docs-site/package.json`:

```json
{
  "name": "clck-docs",
  "private": true,
  "scripts": {
    "docs:dev": "vitepress dev",
    "docs:build": "vitepress build",
    "docs:preview": "vitepress preview"
  },
  "devDependencies": {
    "vitepress": "1.6.4"
  }
}
```

- [ ] **Step 2: Install the pinned dependency and generate the lockfile**

Run:

```bash
cd docs-site
npm install
```

Expected: `package-lock.json` is created and npm exits successfully.

- [ ] **Step 3: Ignore installed dependencies and generated site output**

Append to `.gitignore`:

```gitignore
docs-site/node_modules/
docs-site/.vitepress/cache/
docs-site/.vitepress/dist/
```

- [ ] **Step 4: Add the VitePress configuration**

Create `docs-site/.vitepress/config.mts`:

```ts
import { defineConfig } from "vitepress";

export default defineConfig({
  title: "clck",
  description: "A responsive cross-platform countdown alarm for the terminal.",
  base: "/clck/",
  cleanUrls: true,
  lastUpdated: true,
  appearance: "dark",
  themeConfig: {
    logo: { src: "/terminal.svg", alt: "clck" },
    nav: [
      { text: "Guide", link: "/guide/installation" },
      { text: "Reference", link: "/reference/commands" },
      { text: "GitHub", link: "https://github.com/gkk-dev-ops/clck" }
    ],
    sidebar: {
      "/guide/": [
        {
          text: "Guide",
          items: [
            { text: "Installation", link: "/guide/installation" },
            { text: "Usage", link: "/guide/usage" },
            { text: "Scheduling", link: "/guide/scheduling" },
            { text: "Configuration", link: "/guide/configuration" }
          ]
        }
      ],
      "/reference/": [
        {
          text: "Reference",
          items: [
            { text: "Commands", link: "/reference/commands" },
            { text: "Duration formats", link: "/reference/duration-formats" },
            { text: "Controls", link: "/reference/controls" }
          ]
        }
      ],
      "/development/": [
        {
          text: "Development",
          items: [
            { text: "Testing", link: "/development/testing" },
            { text: "Releases", link: "/development/releases" }
          ]
        }
      ]
    },
    search: { provider: "local" },
    socialLinks: [
      { icon: "github", link: "https://github.com/gkk-dev-ops/clck" }
    ],
    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © clck contributors"
    }
  }
});
```

- [ ] **Step 5: Load the default theme**

Create `docs-site/.vitepress/theme/index.ts`:

```ts
import DefaultTheme from "vitepress/theme";
import "./custom.css";

export default DefaultTheme;
```

Create an empty `docs-site/.vitepress/theme/custom.css` so the import resolves.

- [ ] **Step 6: Add a minimal homepage that proves the scaffold**

Create `docs-site/index.md`:

```md
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
```

- [ ] **Step 7: Verify the scaffold builds**

Run:

```bash
cd docs-site
npm run docs:build
```

Expected: VitePress reports `build complete` and creates `docs-site/.vitepress/dist/index.html`.

- [ ] **Step 8: Commit the scaffold**

```bash
git add .gitignore docs-site/package.json docs-site/package-lock.json docs-site/.vitepress docs-site/index.md
git commit -m "Scaffold VitePress documentation site"
```

### Task 2: Build the Terminal-Inspired Homepage and Theme

**Files:**
- Create: `docs-site/public/terminal.svg`
- Modify: `docs-site/index.md`
- Modify: `docs-site/.vitepress/theme/custom.css`

- [ ] **Step 1: Add a minimal terminal logo**

Create `docs-site/public/terminal.svg` with an accessible, current-color-style terminal mark:

```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" role="img" aria-label="Terminal">
  <rect x="4" y="8" width="56" height="48" rx="6" fill="#0b1220" stroke="#3b82f6" stroke-width="4"/>
  <path d="m16 23 9 9-9 9M31 41h16" fill="none" stroke="#93c5fd" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
```

- [ ] **Step 2: Replace the minimal homepage with the approved content**

Update `docs-site/index.md` to retain the `layout: home` hero and add:

```md
## Start an alarm without leaving the terminal

<div class="terminal-window" aria-label="PowerShell example">
  <div class="terminal-titlebar"><span></span><span></span><span></span><strong>PowerShell</strong></div>
  <div class="terminal-body">
    <p><span class="prompt">PS&gt;</span> clck 10m</p>
    <p class="terminal-muted">Target: countdown | Sound: Glass | q/Esc/Ctrl+C to cancel</p>
    <p class="timer-output">00:10:00</p>
  </div>
</div>

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
```

- [ ] **Step 3: Implement the visual tokens and component styles**

Populate `docs-site/.vitepress/theme/custom.css` with:

```css
:root {
  --vp-font-family-base: ui-sans-serif, system-ui, sans-serif;
  --vp-font-family-mono: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
  --vp-c-brand-1: #2563eb;
  --vp-c-brand-2: #3b82f6;
  --vp-c-brand-3: #60a5fa;
  --vp-c-brand-soft: rgb(37 99 235 / 14%);
  --vp-button-brand-bg: #2563eb;
  --vp-button-brand-hover-bg: #3b82f6;
  --vp-border-radius: 4px;
}

.dark {
  --vp-c-bg: #080d16;
  --vp-c-bg-alt: #0b1220;
  --vp-c-bg-soft: #101827;
  --vp-c-bg-elv: #111b2d;
  --vp-c-divider: #23314a;
  --vp-c-border: #2b3b57;
  --vp-c-text-1: #dbeafe;
  --vp-c-text-2: #94a3b8;
  --vp-c-text-3: #64748b;
}

h1,
h2,
h3,
.VPNavBarTitle,
.VPNavBarMenuLink,
.VPSidebarItem .text {
  font-family: var(--vp-font-family-mono);
}

.VPHomeHero .name,
.VPHomeHero .text {
  letter-spacing: -0.04em;
}

.VPButton,
.VPFeature {
  border-radius: 4px !important;
}

.terminal-window {
  overflow: hidden;
  margin: 24px 0 40px;
  border: 1px solid var(--vp-c-border);
  border-radius: 6px;
  background: #070c14;
  box-shadow: 0 18px 60px rgb(0 0 0 / 24%);
  color: #dbeafe;
  font-family: var(--vp-font-family-mono);
}

.terminal-titlebar {
  display: flex;
  align-items: center;
  gap: 7px;
  padding: 10px 14px;
  border-bottom: 1px solid #23314a;
  background: #0b1220;
  color: #94a3b8;
  font-size: 12px;
}

.terminal-titlebar span {
  width: 9px;
  height: 9px;
  border: 1px solid #3b82f6;
  border-radius: 50%;
}

.terminal-titlebar strong {
  margin-left: 6px;
  font-weight: 500;
}

.terminal-body {
  padding: 20px;
}

.terminal-body p {
  margin: 0 0 12px;
}

.prompt {
  color: #60a5fa;
}

.terminal-muted {
  color: #64748b;
  font-size: 13px;
}

.timer-output {
  color: #bfdbfe;
  font-size: clamp(2.5rem, 10vw, 6rem);
  line-height: 1;
  letter-spacing: -0.08em;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}

.feature-grid article {
  border: 1px solid var(--vp-c-border);
  border-radius: 4px;
  padding: 18px;
  background: var(--vp-c-bg-soft);
}

.feature-grid h3 {
  margin: 0 0 8px;
  font-size: 15px;
}

.feature-grid p {
  margin: 0;
  color: var(--vp-c-text-2);
}

@media (max-width: 640px) {
  .feature-grid {
    grid-template-columns: 1fr;
  }

  .terminal-body {
    padding: 16px;
  }
}
```

- [ ] **Step 4: Build and inspect the generated homepage**

Run:

```bash
cd docs-site
npm run docs:build
rg -n "Start an alarm|PowerShell|Built for terminal workflows" .vitepress/dist/index.html
```

Expected: build succeeds and all three homepage strings are found.

- [ ] **Step 5: Commit the homepage and theme**

```bash
git add docs-site/index.md docs-site/public/terminal.svg docs-site/.vitepress/theme/custom.css
git commit -m "Add terminal-inspired documentation homepage"
```

### Task 3: Write the User Guides

**Files:**
- Create: `docs-site/guide/installation.md`
- Create: `docs-site/guide/usage.md`
- Create: `docs-site/guide/scheduling.md`
- Create: `docs-site/guide/configuration.md`

- [ ] **Step 1: Write the installation guide**

Create `docs-site/guide/installation.md` covering:

- Supported macOS and Linux platforms and best-effort Windows support.
- `cargo install clck --locked`.
- GitHub Release archive names from `README.md`.
- `SHA256SUMS` verification for macOS/Linux and Windows.
- Placing `clck` or `clck.exe` on `PATH`.
- Optional `ffmpeg` requirement for formats such as MP4.

- [ ] **Step 2: Write the basic usage guide**

Create `docs-site/guide/usage.md` with tested examples:

```bash
clck 45s
clck 10m
clck 1h30m
clck 01:30:00
clck
```

Explain interactive setup, responsive font selection, notification behavior,
sound playback, cancellation, and dismissal.

- [ ] **Step 3: Write the scheduling guide**

Create `docs-site/guide/scheduling.md` documenting:

```bash
clck at 2:50pm
clck at "tomorrow at 9am"
clck at "June 12 at 09:00"
clck from-text
printf 'Meet tomorrow at 9am\n' | clck from-text
```

State explicitly that scheduled targets use the local IANA time zone, display
the complete resolved target before confirmation, reject past/nonexistent/
ambiguous local times, and require explicit selection when text contains
multiple candidates.

- [ ] **Step 4: Write the configuration guide**

Create `docs-site/guide/configuration.md` documenting:

```bash
clck config --show
clck config --reset
clck 5m --font banner
clck 5m --sound ~/Music/alarm.mp3
clck 5m --no-notification
clck fonts
clck sounds
```

Include the effective TOML shape:

```toml
font = "standard"
notification = true

[sound]
kind = "system"
value = "Glass"
```

Explain that command-line values override saved values and that unavailable
sounds fall back according to the behavior described in `README.md`.

- [ ] **Step 5: Verify every guide is reachable in the build**

Run:

```bash
cd docs-site
npm run docs:build
find .vitepress/dist/guide -name '*.html' -print
```

Expected: four generated guide HTML files are listed.

- [ ] **Step 6: Commit the guides**

```bash
git add docs-site/guide
git commit -m "Add clck user guides"
```

### Task 4: Write Reference and Development Documentation

**Files:**
- Create: `docs-site/reference/commands.md`
- Create: `docs-site/reference/duration-formats.md`
- Create: `docs-site/reference/controls.md`
- Create: `docs-site/development/testing.md`
- Create: `docs-site/development/releases.md`
- Modify: `README.md`

- [ ] **Step 1: Write the exact command reference**

Create `docs-site/reference/commands.md` with a table for:

| Command | Purpose |
| --- | --- |
| `clck [DURATION]` | Start a countdown or open interactive setup |
| `clck at <VALUE>` | Resolve and confirm one local scheduled target |
| `clck from-text` | Extract, select, and confirm a target from text |
| `clck config --show` | Print effective configuration |
| `clck config --reset` | Restore default configuration |
| `clck fonts` | Preview bundled fonts |
| `clck sounds` | List discovered logical system sounds |

Document the global `--sound <PATH>`, `--font <NAME>`, and
`--no-notification` options.

- [ ] **Step 2: Write duration and control references**

Create `docs-site/reference/duration-formats.md` documenting the accepted,
case-insensitive forms and examples:

- Unit durations: `45s`, `10m`, `1h30m`.
- Compact hours/minutes: `1H30`.
- Clock durations: `01:30:00`.
- Durations must be valid and nonzero.

Create `docs-site/reference/controls.md` documenting:

- During countdown: `q`, `Esc`, or `Ctrl+C` cancels.
- While ringing: any key dismisses.
- Terminal mode and cursor are restored after exit, cancellation, dismissal,
  and errors.

- [ ] **Step 3: Adapt development documentation for the public site**

Create `docs-site/development/testing.md` from `docs/manual-testing.md`, keeping
the verification matrix and platform notes while omitting dated local test
results.

Create `docs-site/development/releases.md` from `docs/releases.md`, keeping CI,
automatic release rules, artifacts, manual rebuild, and failure recovery.

- [ ] **Step 4: Link the repository README to the site**

Add directly after the README introduction:

```md
Read the full documentation at
[gkk-dev-ops.github.io/clck](https://gkk-dev-ops.github.io/clck/).
```

- [ ] **Step 5: Verify the complete content tree**

Run:

```bash
cd docs-site
npm run docs:build
find .vitepress/dist -name '*.html' -print | sort
```

Expected: homepage plus four guide, three reference, and two development pages
are listed, and VitePress reports no dead-link errors.

- [ ] **Step 6: Commit the reference and development docs**

```bash
git add docs-site/reference docs-site/development README.md
git commit -m "Add CLI reference and contributor docs"
```

### Task 5: Add CI and GitHub Pages Deployment

**Files:**
- Modify: `.github/workflows/ci.yml`
- Create: `.github/workflows/pages.yml`

- [ ] **Step 1: Verify CI does not yet build the documentation**

Run:

```bash
rg -n "setup-node|docs:build" .github/workflows/ci.yml
```

Expected: command exits nonzero because CI does not yet configure Node or build
the documentation.

- [ ] **Step 2: Add pinned Node setup and documentation build to CI**

Add:

```yaml
- uses: actions/setup-node@v4
  with:
    node-version: 22
    cache: npm
    cache-dependency-path: docs-site/package-lock.json
- run: npm ci --prefix docs-site
- run: npm run docs:build --prefix docs-site
```

Place it after checkout and before Rust setup so site failures return quickly.

- [ ] **Step 3: Add the Pages deployment workflow**

Create `.github/workflows/pages.yml`:

```yaml
name: Pages

on:
  push:
    branches: [master]
    paths:
      - docs-site/**
      - .github/workflows/pages.yml
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: npm
          cache-dependency-path: docs-site/package-lock.json
      - uses: actions/configure-pages@v5
      - run: npm ci --prefix docs-site
      - run: npm run docs:build --prefix docs-site
      - uses: actions/upload-pages-artifact@v3
        with:
          path: docs-site/.vitepress/dist
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

- [ ] **Step 4: Validate workflows and the local build**

Run:

```bash
go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7
npm ci --prefix docs-site
npm run docs:build --prefix docs-site
```

Expected: actionlint exits successfully, npm installs from the lockfile, and
VitePress builds the site.

- [ ] **Step 5: Commit CI and deployment**

```bash
git add .github/workflows/ci.yml .github/workflows/pages.yml
git commit -m "Deploy documentation site to GitHub Pages"
```

### Task 6: Final Verification

**Files:**
- Modify only files found incorrect during verification.

- [ ] **Step 1: Run all automated repository checks**

Run:

```bash
npm ci --prefix docs-site
npm run docs:build --prefix docs-site
go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

Expected: every command exits successfully.

- [ ] **Step 2: Preview the site at its configured base path**

Run:

```bash
npm run docs:preview --prefix docs-site -- --host 127.0.0.1
```

Open `http://127.0.0.1:4173/clck/`.

Verify:

- The homepage loads without missing assets.
- Dark mode uses restrained navy surfaces and PowerShell-blue accents.
- The terminal panel is readable and does not overflow.
- Guide, Reference, GitHub, sidebar, and local search controls work.
- All ten content pages are reachable.

- [ ] **Step 3: Check responsive behavior**

At desktop and a mobile viewport no wider than 390 pixels, verify:

- The feature grid changes from two columns to one.
- The timer example fits without horizontal scrolling.
- Navigation and sidebar remain usable.
- Code blocks scroll rather than widening the page.

- [ ] **Step 4: Inspect the final diff**

Run:

```bash
git status --short
git diff --check
git log --oneline -6
```

Expected: no unintended generated output is tracked, no whitespace errors are
reported, and the implementation commits are visible.

- [ ] **Step 5: Commit any verification fixes**

If verification required corrections:

```bash
git add docs-site README.md .github/workflows
git commit -m "Polish documentation site"
```

If no corrections were required, do not create an empty commit.
