# Interactive Timer Demo Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the static homepage terminal example with a real ten-second countdown that plays a short synthesized chime and resets automatically.

**Architecture:** Implement the interaction as one isolated Vue single-file component registered by the existing VitePress theme. Test countdown, reset, completion, audio fallback, and cleanup behavior with Vitest fake timers and Vue Test Utils; keep visual integration in the existing homepage Markdown and theme CSS.

**Tech Stack:** Vue 3, VitePress 1.6.4, TypeScript, Vitest, Vue Test Utils, jsdom, Web Audio API, CSS

---

## File Structure

| Path | Responsibility |
| --- | --- |
| `docs-site/.vitepress/theme/components/TerminalTimer.vue` | Countdown state, rendering, reset lifecycle, and synthesized chime |
| `docs-site/.vitepress/theme/components/TerminalTimer.test.ts` | Component behavior tests using fake timers and mocked Web Audio |
| `docs-site/.vitepress/theme/index.ts` | Registers `TerminalTimer` for use in Markdown |
| `docs-site/.vitepress/theme/custom.css` | Terminal button, completion pulse, and reduced-motion styling |
| `docs-site/index.md` | Embeds the interactive component in place of static terminal markup |
| `docs-site/package.json` | Adds the component test command and direct test dependencies |
| `docs-site/package-lock.json` | Locks the added test dependencies |

### Task 1: Add the Tested Countdown Component

**Files:**
- Create: `docs-site/.vitepress/theme/components/TerminalTimer.test.ts`
- Create: `docs-site/.vitepress/theme/components/TerminalTimer.vue`
- Modify: `docs-site/package.json`
- Modify: `docs-site/package-lock.json`

- [ ] **Step 1: Add the component-test dependencies and script**

Update `docs-site/package.json`:

```json
{
  "name": "clck-docs",
  "private": true,
  "scripts": {
    "docs:dev": "vitepress dev",
    "docs:build": "vitepress build",
    "docs:preview": "vitepress preview",
    "test": "vitest run"
  },
  "devDependencies": {
    "@vue/test-utils": "2.4.6",
    "jsdom": "26.1.0",
    "vitepress": "1.6.4",
    "vitest": "3.2.4"
  }
}
```

Run:

```bash
cd docs-site
npm install
```

Expected: npm updates `package-lock.json` and exits successfully.

- [ ] **Step 2: Write failing initial-state and countdown tests**

Create `docs-site/.vitepress/theme/components/TerminalTimer.test.ts`:

```ts
// @vitest-environment jsdom

import { mount } from "@vue/test-utils";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import TerminalTimer from "./TerminalTimer.vue";

describe("TerminalTimer", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-06-10T12:00:00Z"));
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
  });

  it("starts from ten seconds and counts down from an absolute end time", async () => {
    const wrapper = mount(TerminalTimer);

    expect(wrapper.get(".timer-output").text()).toBe("00:00:10");
    expect(wrapper.get("button").text()).toBe("Start demo");

    await wrapper.get("button").trigger("click");
    expect(wrapper.get("button").text()).toBe("Reset");

    vi.setSystemTime(new Date("2026-06-10T12:00:04.250Z"));
    await vi.advanceTimersByTimeAsync(250);

    expect(wrapper.get(".timer-output").text()).toBe("00:00:06");
  });

  it("resets immediately while counting", async () => {
    const wrapper = mount(TerminalTimer);

    await wrapper.get("button").trigger("click");
    await vi.advanceTimersByTimeAsync(1_000);
    await wrapper.get("button").trigger("click");

    expect(wrapper.get(".timer-output").text()).toBe("00:00:10");
    expect(wrapper.get(".terminal-muted").text()).toBe("Ready to start the demo.");
    expect(wrapper.get("button").text()).toBe("Start demo");
  });
});
```

- [ ] **Step 3: Run the tests to verify they fail**

Run:

```bash
cd docs-site
npm test
```

Expected: FAIL because `TerminalTimer.vue` does not exist.

- [ ] **Step 4: Implement the initial and running states**

Create `docs-site/.vitepress/theme/components/TerminalTimer.vue`:

```vue
<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from "vue";

const durationSeconds = 10;
const remaining = ref(durationSeconds);
const state = ref<"ready" | "running" | "complete">("ready");
let endTime = 0;
let intervalId: ReturnType<typeof setInterval> | undefined;
let resetTimeoutId: ReturnType<typeof setTimeout> | undefined;

const formattedTime = computed(() => `00:00:${String(remaining.value).padStart(2, "0")}`);
const status = computed(() => {
  if (state.value === "running") return "Countdown running.";
  if (state.value === "complete") return "Time is up.";
  return "Ready to start the demo.";
});
const buttonLabel = computed(() => (state.value === "ready" ? "Start demo" : "Reset"));

function clearTimers() {
  if (intervalId !== undefined) clearInterval(intervalId);
  if (resetTimeoutId !== undefined) clearTimeout(resetTimeoutId);
  intervalId = undefined;
  resetTimeoutId = undefined;
}

function reset() {
  clearTimers();
  state.value = "ready";
  remaining.value = durationSeconds;
}

function updateRemaining() {
  remaining.value = Math.max(0, Math.ceil((endTime - Date.now()) / 1_000));
}

function start() {
  clearTimers();
  state.value = "running";
  remaining.value = durationSeconds;
  endTime = Date.now() + durationSeconds * 1_000;
  intervalId = setInterval(updateRemaining, 250);
}

function toggle() {
  if (state.value === "ready") start();
  else reset();
}

onBeforeUnmount(clearTimers);
</script>

<template>
  <div class="terminal-window" :class="{ 'is-complete': state === 'complete' }">
    <div class="terminal-titlebar">
      <span></span><span></span><span></span><strong>PowerShell</strong>
    </div>
    <div class="terminal-body">
      <p><span class="prompt">PS&gt;</span> clck 10s</p>
      <div aria-live="polite">
        <p class="terminal-muted">{{ status }}</p>
        <p class="timer-output">{{ formattedTime }}</p>
      </div>
      <button class="terminal-action" type="button" @click="toggle">{{ buttonLabel }}</button>
    </div>
  </div>
</template>
```

- [ ] **Step 5: Run the tests to verify the initial and running states pass**

Run:

```bash
cd docs-site
npm test
```

Expected: two tests pass.

- [ ] **Step 6: Write failing completion, audio-fallback, and cleanup tests**

Append inside the existing `describe` block:

```ts
  it("plays one chime at zero and resets after two seconds", async () => {
    const start = vi.fn();
    const stop = vi.fn();
    const oscillator = {
      connect: vi.fn(),
      frequency: { setValueAtTime: vi.fn() },
      start,
      stop,
    };
    const gain = {
      connect: vi.fn(),
      gain: {
        setValueAtTime: vi.fn(),
        exponentialRampToValueAtTime: vi.fn(),
      },
    };
    const audioContext = {
      currentTime: 0,
      destination: {},
      createOscillator: vi.fn(() => oscillator),
      createGain: vi.fn(() => gain),
      resume: vi.fn(),
    };
    vi.stubGlobal("AudioContext", vi.fn(() => audioContext));

    const wrapper = mount(TerminalTimer);
    await wrapper.get("button").trigger("click");
    await vi.advanceTimersByTimeAsync(10_000);

    expect(wrapper.get(".timer-output").text()).toBe("00:00:00");
    expect(wrapper.get(".terminal-muted").text()).toBe("Time is up.");
    expect(wrapper.classes()).toContain("is-complete");
    expect(audioContext.createOscillator).toHaveBeenCalledTimes(2);
    expect(start).toHaveBeenCalledTimes(2);
    expect(stop).toHaveBeenCalledTimes(2);

    await vi.advanceTimersByTimeAsync(2_000);
    expect(wrapper.get(".timer-output").text()).toBe("00:00:10");
    expect(wrapper.get("button").text()).toBe("Start demo");
  });

  it("completes when Web Audio is unavailable", async () => {
    vi.stubGlobal("AudioContext", undefined);
    const wrapper = mount(TerminalTimer);

    await wrapper.get("button").trigger("click");
    await vi.advanceTimersByTimeAsync(10_000);

    expect(wrapper.get(".terminal-muted").text()).toBe("Time is up.");
  });

  it("clears active timers when unmounted", async () => {
    const clearIntervalSpy = vi.spyOn(globalThis, "clearInterval");
    const wrapper = mount(TerminalTimer);

    await wrapper.get("button").trigger("click");
    wrapper.unmount();

    expect(clearIntervalSpy).toHaveBeenCalled();
  });
```

- [ ] **Step 7: Run the tests to verify completion behavior fails**

Run:

```bash
cd docs-site
npm test
```

Expected: the three new tests fail because completion and audio are not
implemented.

- [ ] **Step 8: Implement completion, chime, and automatic reset**

Add to the component script:

```ts
type BrowserAudioContext = typeof AudioContext;

let audioContext: AudioContext | undefined;

function prepareAudio() {
  const AudioContextClass = globalThis.AudioContext as BrowserAudioContext | undefined;
  if (!AudioContextClass) return;
  audioContext ??= new AudioContextClass();
  void audioContext.resume();
}

function playTone(frequency: number, offset: number) {
  if (!audioContext) return;
  const oscillator = audioContext.createOscillator();
  const gain = audioContext.createGain();
  const startAt = audioContext.currentTime + offset;
  oscillator.connect(gain);
  gain.connect(audioContext.destination);
  oscillator.frequency.setValueAtTime(frequency, startAt);
  gain.gain.setValueAtTime(0.12, startAt);
  gain.gain.exponentialRampToValueAtTime(0.001, startAt + 0.22);
  oscillator.start(startAt);
  oscillator.stop(startAt + 0.24);
}

function playChime() {
  try {
    playTone(660, 0);
    playTone(880, 0.16);
  } catch {
    // The visual completion state remains functional without Web Audio.
  }
}

function complete() {
  clearTimers();
  state.value = "complete";
  remaining.value = 0;
  playChime();
  resetTimeoutId = setTimeout(reset, 2_000);
}
```

Change `updateRemaining`:

```ts
function updateRemaining() {
  remaining.value = Math.max(0, Math.ceil((endTime - Date.now()) / 1_000));
  if (remaining.value === 0) complete();
}
```

Call `prepareAudio()` at the beginning of `start()`.

- [ ] **Step 9: Run all component tests**

Run:

```bash
cd docs-site
npm test
```

Expected: all five component tests pass.

- [ ] **Step 10: Commit the tested component**

```bash
git add docs-site/package.json docs-site/package-lock.json docs-site/.vitepress/theme/components
git commit -m "Add interactive countdown component"
```

### Task 2: Integrate and Style the Homepage Demo

**Files:**
- Modify: `docs-site/.vitepress/theme/index.ts`
- Modify: `docs-site/index.md`
- Modify: `docs-site/.vitepress/theme/custom.css`

- [ ] **Step 1: Verify the component is not yet available to Markdown**

Run:

```bash
cd docs-site
npm run docs:build
rg -n "Start demo|TerminalTimer" .vitepress/dist/index.html
```

Expected: build succeeds, but the generated homepage does not contain
`Start demo` or `TerminalTimer`.

- [ ] **Step 2: Register the component globally**

Replace `docs-site/.vitepress/theme/index.ts` with:

```ts
import DefaultTheme from "vitepress/theme";
import type { Theme } from "vitepress";
import TerminalTimer from "./components/TerminalTimer.vue";
import "./custom.css";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("TerminalTimer", TerminalTimer);
  },
} satisfies Theme;
```

- [ ] **Step 3: Replace the static terminal markup**

In `docs-site/index.md`, replace the complete `<div class="terminal-window">`
block with:

```md
<TerminalTimer />
```

- [ ] **Step 4: Add action and completion styles**

Append to `docs-site/.vitepress/theme/custom.css`:

```css
.terminal-action {
  border: 1px solid #3b82f6;
  border-radius: 4px;
  padding: 8px 14px;
  background: transparent;
  color: #bfdbfe;
  font: inherit;
  cursor: pointer;
  transition:
    background-color 160ms ease,
    border-color 160ms ease,
    color 160ms ease;
}

.terminal-action:hover {
  background: rgb(37 99 235 / 18%);
  color: #eff6ff;
}

.terminal-action:focus-visible {
  outline: 2px solid #93c5fd;
  outline-offset: 3px;
}

.terminal-window.is-complete {
  animation: terminal-complete 600ms ease-in-out 2;
}

@keyframes terminal-complete {
  50% {
    border-color: #60a5fa;
    background: #0b1d3a;
    box-shadow: 0 0 32px rgb(59 130 246 / 30%);
  }
}

@media (prefers-reduced-motion: reduce) {
  .terminal-window.is-complete {
    animation: none;
    border-color: #60a5fa;
  }
}
```

- [ ] **Step 5: Verify tests and generated homepage integration**

Run:

```bash
cd docs-site
npm test
npm run docs:build
rg -n "Start demo|clck 10s|aria-live=\"polite\"" .vitepress/dist/index.html
rg -n "prefers-reduced-motion|terminal-complete|terminal-action" .vitepress/dist/assets/*.css
```

Expected: component tests and build pass; generated HTML contains the
interactive initial state; generated CSS contains action, completion, and
reduced-motion rules.

- [ ] **Step 6: Commit the homepage integration**

```bash
git add docs-site/.vitepress/theme/index.ts docs-site/.vitepress/theme/custom.css docs-site/index.md
git commit -m "Embed interactive timer on documentation homepage"
```

### Task 3: Final Verification

**Files:**
- Modify only files found incorrect during verification.

- [ ] **Step 1: Run the complete automated suite**

Run:

```bash
npm ci --prefix docs-site
npm test --prefix docs-site
npm run docs:build --prefix docs-site
go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
git diff --check
```

Expected: every command exits successfully.

- [ ] **Step 2: Preview and exercise the demo**

Run:

```bash
npm run docs:preview --prefix docs-site -- --host 127.0.0.1
```

At `http://127.0.0.1:4173/clck/`, verify:

- Initial display is `00:00:10` with `Start demo`.
- Clicking starts the countdown and changes the button to `Reset`.
- Reset immediately restores the initial state.
- A complete run reaches `00:00:00`, plays one short two-tone chime, pulses
  blue, and resets after two seconds.
- The timer and button fit at a mobile viewport no wider than 390 pixels.

- [ ] **Step 3: Inspect final repository state**

Run:

```bash
git status --short
git diff --check
git log --oneline -5
```

Expected: no unintended files or whitespace errors remain, and the two
implementation commits are visible.

- [ ] **Step 4: Commit any verification fixes**

If verification required corrections:

```bash
git add docs-site
git commit -m "Polish interactive timer demo"
```

If no corrections were required, do not create an empty commit.
