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
