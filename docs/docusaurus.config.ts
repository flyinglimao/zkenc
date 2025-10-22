import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: "zkenc",
  tagline: "Witness Encryption for Circom Circuits",
  favicon: "img/favicon.ico",

  // Future flags, see https://docusaurus.io/docs/api/docusaurus-config#future
  future: {
    v4: true, // Improve compatibility with the upcoming Docusaurus v4
  },

  // Set the production url of your site here
  url: "https://zkenc.limaois.me",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "/",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "flyinglimao", // Usually your GitHub org/user name.
  projectName: "zkenc", // Usually your repo name.

  onBrokenLinks: "throw",

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: "https://github.com/flyinglimao/zkenc/tree/main/docs/",
        },
        blog: false,
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  plugins: [
    function wasmPlugin() {
      return {
        name: "wasm-plugin",
        configureWebpack(config, isServer) {
          return {
            experiments: {
              asyncWebAssembly: true,
            },
            module: {
              rules: [
                {
                  test: /\.wasm$/,
                  type: "webassembly/async",
                },
              ],
            },
            resolve: {
              alias: {
                // Prevent server-side import of zkenc-js
                ...(isServer
                  ? {
                      "zkenc-js": false,
                    }
                  : {}),
              },
            },
          };
        },
      };
    },
    [
      "@docusaurus/plugin-google-gtag",
      {
        trackingID: "G-ZSJCSHM3XT",
        anonymizeIP: true,
      },
    ],
  ],

  themeConfig: {
    // Replace with your project's social card
    image: "img/docusaurus-social-card.jpg",
    colorMode: {
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: "zkenc",
      logo: {
        alt: "zkenc Logo",
        src: "img/logo.svg",
      },
      items: [
        {
          type: "docSidebar",
          sidebarId: "docsSidebar",
          position: "left",
          label: "Documentation",
        },
        {
          type: "docSidebar",
          sidebarId: "guidesSidebar",
          position: "left",
          label: "Guides",
        },
        {
          to: "/playground",
          label: "Playground",
          position: "left",
        },
        {
          href: "https://github.com/flyinglimao/zkenc",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Documentation",
          items: [
            {
              label: "Getting Started",
              to: "/docs/getting-started/zkenc-js",
            },
            {
              label: "API Reference",
              to: "/docs/api/zkenc-js",
            },
          ],
        },
        {
          title: "Resources",
          items: [
            {
              label: "GitHub",
              href: "https://github.com/flyinglimao/zkenc",
            },
            {
              label: "npm - zkenc-js",
              href: "https://www.npmjs.com/package/zkenc-js",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} zkenc. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
