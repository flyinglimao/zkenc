import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
const sidebars: SidebarsConfig = {
  docsSidebar: [
    "intro",
    {
      type: "category",
      label: "Getting Started",
      items: ["getting-started/zkenc-js", "getting-started/zkenc-cli"],
    },
    {
      type: "category",
      label: "API Reference",
      items: ["api/zkenc-core", "api/zkenc-cli", "api/zkenc-js"],
    },
  ],
  guidesSidebar: [
    "guides/intro",
    {
      type: "category",
      label: "Step-by-Step Guides",
      items: ["guides/nodejs-integration", "guides/react-integration"],
    },
    "guides/cross-tool-workflow",
  ],
};

export default sidebars;
