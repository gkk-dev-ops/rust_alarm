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

  it("renders terminal content in a compact stacked order", () => {
    const wrapper = mount(TerminalTimer);
    const bodyChildren = wrapper.get(".terminal-body").element.children;

    expect(bodyChildren[0]?.classList).toContain("terminal-command");
    expect(bodyChildren[1]?.classList).toContain("terminal-output");
    expect(bodyChildren[2]?.classList).toContain("terminal-action");
    expect(wrapper.get("output.timer-output").text()).toBe("00:00:10");
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
});
