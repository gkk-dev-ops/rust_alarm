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
      { text: "GitHub", link: "https://github.com/gkk-dev-ops/clck" },
    ],
    sidebar: {
      "/guide/": [
        {
          text: "Guide",
          items: [
            { text: "Installation", link: "/guide/installation" },
            { text: "Usage", link: "/guide/usage" },
            { text: "Scheduling", link: "/guide/scheduling" },
            { text: "Configuration", link: "/guide/configuration" },
          ],
        },
      ],
      "/reference/": [
        {
          text: "Reference",
          items: [
            { text: "Commands", link: "/reference/commands" },
            { text: "Duration formats", link: "/reference/duration-formats" },
            { text: "Controls", link: "/reference/controls" },
          ],
        },
      ],
      "/development/": [
        {
          text: "Development",
          items: [
            { text: "Testing", link: "/development/testing" },
            { text: "Releases", link: "/development/releases" },
          ],
        },
      ],
    },
    search: { provider: "local" },
    socialLinks: [
      { icon: "github", link: "https://github.com/gkk-dev-ops/clck" },
    ],
    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © clck contributors",
    },
  },
});
