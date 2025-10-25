# Website

This website is built using [Docusaurus](https://docusaurus.io/), a modern static website generator.

## Multi-Language Support 🌍

The documentation is available in multiple languages:
- **English** (default)
- **繁體中文** (Traditional Chinese)
- **简体中文** (Simplified Chinese)
- **日本語** (Japanese)

### Viewing Different Languages

```bash
# Traditional Chinese
yarn start --locale zh-TW

# Simplified Chinese
yarn start --locale zh-CN

# Japanese
yarn start --locale ja
```

### Translation Status

See [TRANSLATION_GUIDE.md](./TRANSLATION_GUIDE.md) for detailed translation status and how to contribute translations.

**Currently translated:**
- Core documentation pages (intro, zkenc-js getting started, guides overview)
- All UI elements (navbar, footer, sidebars)

## Installation

```bash
yarn
```

## Local Development

```bash
yarn start
```

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

## Build

```bash
yarn build
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.

## Deployment

Using SSH:

```bash
USE_SSH=true yarn deploy
```

Not using SSH:

```bash
GIT_USER=<Your GitHub username> yarn deploy
```

If you are using GitHub pages for hosting, this command is a convenient way to build the website and push to the `gh-pages` branch.
