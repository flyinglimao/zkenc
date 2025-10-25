# zkenc Documentation Multi-Language Translation Guide

## Completed Translation Work

This commit completes the following translation setup and content:

### 1. i18n Infrastructure Configuration ✅

- **Docusaurus Configuration** (`docusaurus.config.ts`)
  - Added three languages: Traditional Chinese (zh-TW), Simplified Chinese (zh-CN), Japanese (ja)
  - Configured language dropdown selector
  - Set up labels and directions for each language

### 2. UI Elements Translation ✅

All UI elements translated for all three languages:

- **Navbar** (`navbar.json`)
  - Documentation → 文件/文档/ドキュメント
  - Guides → 指南/指南/ガイド
  - Playground → 遊樂場/游乐场/プレイグラウンド

- **Footer** (`footer.json`)
  - Link titles and labels
  - Copyright notice

- **Sidebar** (`current.json`)
  - Getting Started → 入門/入门/はじめに
  - API Reference → API 參考/API 参考/API リファレンス
  - Step-by-Step Guides → 逐步指南/逐步指南/ステップバイステップガイド

### 3. Core Documentation Pages Translation ✅

The following important pages are fully translated into all three languages:

1. **docs/intro.md** - zkenc Introduction
   - Project overview
   - Available packages description
   - Architecture diagram
   - Quick start links

2. **docs/getting-started/zkenc-js.md** - zkenc-js Getting Started Guide
   - Installation instructions
   - Quick examples
   - High-level vs Low-level API explanation
   - Environment-specific setup
   - Troubleshooting

3. **docs/guides/intro.md** - Guides Overview
   - Node.js integration overview
   - React integration overview
   - Cross-tool workflows
   - Common patterns

## File Structure

```
docs/
├── docusaurus.config.ts          # Updated: Added i18n configuration
├── i18n/
│   ├── zh-TW/                    # Traditional Chinese
│   │   ├── docusaurus-plugin-content-docs/
│   │   │   ├── current/
│   │   │   │   ├── intro.md
│   │   │   │   ├── getting-started/
│   │   │   │   │   └── zkenc-js.md
│   │   │   │   └── guides/
│   │   │   │       └── intro.md
│   │   │   └── current.json      # Sidebar translations
│   │   └── docusaurus-theme-classic/
│   │       ├── navbar.json       # Navbar translations
│   │       └── footer.json       # Footer translations
│   ├── zh-CN/                    # Simplified Chinese (same structure)
│   └── ja/                       # Japanese (same structure)
```

## Remaining Translations

The following files have not yet been translated (total ~4000+ lines):

### High Priority
1. **docs/getting-started/zkenc-cli.md** (685 lines)
   - Complete CLI tool getting started guide

2. **docs/api/zkenc-js.md** (542 lines)
   - Complete JavaScript API reference

3. **docs/api/zkenc-cli.md** (539 lines)
   - Complete CLI API reference

### Medium Priority
4. **docs/guides/nodejs-integration.md** (576 lines)
   - Complete Node.js integration guide

5. **docs/guides/react-integration.md** (578 lines)
   - Complete React integration guide

6. **docs/guides/cross-tool-workflow.md** (421 lines)
   - Cross-tool workflow guide

7. **docs/api/zkenc-core.md** (400 lines)
   - Core Rust library API reference

### Lower Priority
8. **src/pages/markdown-page.md**
   - Example markdown page

## How to Complete Remaining Translations

### Method 1: Manual Translation

For each file to be translated:

1. Copy the original English file content
2. Create corresponding Chinese/Japanese files:
   ```bash
   # Traditional Chinese
   mkdir -p i18n/zh-TW/docusaurus-plugin-content-docs/current/[directory]
   nano i18n/zh-TW/docusaurus-plugin-content-docs/current/[path]/[file].md
   
   # Simplified Chinese
   mkdir -p i18n/zh-CN/docusaurus-plugin-content-docs/current/[directory]
   nano i18n/zh-CN/docusaurus-plugin-content-docs/current/[path]/[file].md
   
   # Japanese
   mkdir -p i18n/ja/docusaurus-plugin-content-docs/current/[directory]
   nano i18n/ja/docusaurus-plugin-content-docs/current/[path]/[file].md
   ```

3. Translate content while preserving:
   - Frontmatter unchanged (`sidebar_position`, etc.)
   - Code blocks unchanged
   - Command examples unchanged
   - Translate only descriptive text

### Method 2: Using Translation Tools

You can use AI translation tools (such as ChatGPT, Claude) for batch translation:

1. Provide original markdown content
2. Request translation to target language, preserving code and format
3. Review and adjust technical terminology
4. Save to corresponding i18n directory

### Important Technical Term Glossary

| English | Traditional Chinese | Simplified Chinese | Japanese |
|---------|-------------------|-------------------|----------|
| Circuit | 電路 | 电路 | 回路 |
| Witness | 見證 | 见證 | Witness |
| Encryption | 加密 | 加密 | 暗号化 |
| Decryption | 解密 | 解密 | 復号 |
| Ciphertext | 密文 | 密文 | 暗号文 |
| Key | 金鑰 | 密钥 | 鍵 |
| Constraint | 約束 | 约束 | 制約 |
| Public input | 公開輸入 | 公开输入 | 公開入力 |
| Private input | 私有輸入 | 私有输入 | 秘密入力 |

## Testing Translations

After completing translations, test the documentation:

```bash
# Install dependencies (if not already installed)
cd docs
npm install

# Start Traditional Chinese version
npm start -- --locale zh-TW

# Start Simplified Chinese version
npm start -- --locale zh-CN

# Start Japanese version
npm start -- --locale ja

# Build all language versions
npm run build
```

## Important Notes

1. **Consistency**: Ensure technical terms are consistent across all files
2. **Don't translate code**: Keep all code blocks and command-line instructions in English
3. **Check links**: Verify internal links work correctly after translation
4. **Preserve formatting**: Maintain original markdown format and structure
5. **Frontmatter**: Don't translate key names in YAML frontmatter

## Current Status Summary

✅ **Completed:**
- Complete i18n infrastructure
- Language switcher
- All UI elements translated
- 3 core documentation pages (each in 3 languages = 9 files)

⏳ **Remaining:**
- 7 major documentation files (API references and guides)
- Each needs translation to 3 languages
- Total of ~21 files, 4000+ lines of content

**Recommendation:** Translations can be completed incrementally by priority, or the current version can be released first with more translations added progressively. Docusaurus will automatically fallback to English for missing translations.
