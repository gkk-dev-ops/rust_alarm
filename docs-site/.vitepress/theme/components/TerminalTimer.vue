<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from "vue";

const durationSeconds = 10;
const remaining = ref(durationSeconds);
const state = ref<"ready" | "running" | "complete">("ready");
let endTime = 0;
let intervalId: ReturnType<typeof setInterval> | undefined;
let resetTimeoutId: ReturnType<typeof setTimeout> | undefined;
let audioContext: AudioContext | undefined;

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
  if (remaining.value === 0) complete();
}

function prepareAudio() {
  try {
    const AudioContextClass = globalThis.AudioContext;
    if (!AudioContextClass) return;
    audioContext ??= new AudioContextClass();
    void audioContext.resume();
  } catch {
    audioContext = undefined;
  }
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

function start() {
  clearTimers();
  prepareAudio();
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
