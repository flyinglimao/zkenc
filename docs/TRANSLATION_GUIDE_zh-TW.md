# zkenc 文檔多語言翻譯指南

## 已完成的翻譯工作

本次提交已完成以下翻譯設置和內容：

### 1. i18n 基礎設施配置 ✅

- **Docusaurus 配置** (`docusaurus.config.ts`)
  - 添加了三種語言：繁體中文 (zh-TW)、簡體中文 (zh-CN)、日文 (ja)
  - 配置了語言下拉選單
  - 設置了各語言的標籤和方向

### 2. UI 元素翻譯 ✅

所有三種語言的 UI 元素已翻譯：

- **導航欄** (`navbar.json`)
  - Documentation → 文件/文档/ドキュメント
  - Guides → 指南/指南/ガイド
  - Playground → 遊樂場/游乐场/プレイグラウンド

- **頁腳** (`footer.json`)
  - 連結標題和標籤
  - 版權聲明

- **側邊欄** (`current.json`)
  - Getting Started → 入門/入门/はじめに
  - API Reference → API 參考/API 参考/API リファレンス
  - Step-by-Step Guides → 逐步指南/逐步指南/ステップバイステップガイド

### 3. 核心文檔頁面翻譯 ✅

以下重要頁面已完整翻譯為三種語言：

1. **docs/intro.md** - zkenc 介紹頁面
   - 項目概述
   - 可用套件說明
   - 架構圖
   - 快速入門連結

2. **docs/getting-started/zkenc-js.md** - zkenc-js 入門指南
   - 安裝說明
   - 快速範例
   - 高階與低階 API 說明
   - 環境特定設置
   - 疑難排解

3. **docs/guides/intro.md** - 指南概覽
   - Node.js 整合說明
   - React 整合說明
   - 跨工具工作流程
   - 常見模式

## 文件結構

```
docs/
├── docusaurus.config.ts          # 已更新：添加 i18n 配置
├── i18n/
│   ├── zh-TW/                    # 繁體中文
│   │   ├── docusaurus-plugin-content-docs/
│   │   │   ├── current/
│   │   │   │   ├── intro.md
│   │   │   │   ├── getting-started/
│   │   │   │   │   └── zkenc-js.md
│   │   │   │   └── guides/
│   │   │   │       └── intro.md
│   │   │   └── current.json      # 側邊欄翻譯
│   │   └── docusaurus-theme-classic/
│   │       ├── navbar.json       # 導航欄翻譯
│   │       └── footer.json       # 頁腳翻譯
│   ├── zh-CN/                    # 簡體中文（相同結構）
│   └── ja/                       # 日文（相同結構）
```

## 待完成的翻譯

以下文件尚未翻譯（總計約 4000+ 行）：

### 優先級較高
1. **docs/getting-started/zkenc-cli.md** (685 行)
   - CLI 工具完整入門指南

2. **docs/api/zkenc-js.md** (542 行)
   - JavaScript API 完整參考

3. **docs/api/zkenc-cli.md** (539 行)
   - CLI API 完整參考

### 優先級中等
4. **docs/guides/nodejs-integration.md** (576 行)
   - Node.js 整合完整指南

5. **docs/guides/react-integration.md** (578 行)
   - React 整合完整指南

6. **docs/guides/cross-tool-workflow.md** (421 行)
   - 跨工具工作流程指南

7. **docs/api/zkenc-core.md** (400 行)
   - Core Rust 庫 API 參考

### 優先級較低
8. **src/pages/markdown-page.md**
   - 範例 markdown 頁面

## 如何完成剩餘翻譯

### 方法 1：手動翻譯

對於每個待翻譯的文件：

1. 複製原始英文文件內容
2. 創建對應的中文/日文文件：
   ```bash
   # 繁體中文
   mkdir -p i18n/zh-TW/docusaurus-plugin-content-docs/current/[目錄]
   nano i18n/zh-TW/docusaurus-plugin-content-docs/current/[路徑]/[文件].md
   
   # 簡體中文
   mkdir -p i18n/zh-CN/docusaurus-plugin-content-docs/current/[目錄]
   nano i18n/zh-CN/docusaurus-plugin-content-docs/current/[路徑]/[文件].md
   
   # 日文
   mkdir -p i18n/ja/docusaurus-plugin-content-docs/current/[目錄]
   nano i18n/ja/docusaurus-plugin-content-docs/current/[路徑]/[文件].md
   ```

3. 翻譯內容，保持：
   - Frontmatter 不變（`sidebar_position` 等）
   - 代碼區塊不變
   - 命令範例不變
   - 僅翻譯說明文字

### 方法 2：使用翻譯工具

可以使用 AI 翻譯工具（如 ChatGPT、Claude）批量翻譯：

1. 提供原始 markdown 內容
2. 要求翻譯為目標語言，保留代碼和格式
3. 審查並調整技術術語
4. 保存到對應的 i18n 目錄

### 重要技術術語對照

| English | 繁體中文 | 簡體中文 | 日文 |
|---------|---------|---------|------|
| Circuit | 電路 | 电路 | 回路 |
| Witness | 見證 | 见證 | Witness |
| Encryption | 加密 | 加密 | 暗号化 |
| Decryption | 解密 | 解密 | 復号 |
| Ciphertext | 密文 | 密文 | 暗号文 |
| Key | 金鑰 | 密钥 | 鍵 |
| Constraint | 約束 | 约束 | 制約 |
| Public input | 公開輸入 | 公开输入 | 公開入力 |
| Private input | 私有輸入 | 私有输入 | 秘密入力 |

## 測試翻譯

完成翻譯後，測試文檔：

```bash
# 安裝依賴（如果尚未安裝）
cd docs
npm install

# 啟動繁體中文版本
npm start -- --locale zh-TW

# 啟動簡體中文版本
npm start -- --locale zh-CN

# 啟動日文版本
npm start -- --locale ja

# 構建所有語言版本
npm run build
```

## 注意事項

1. **保持一致性**：確保技術術語在所有文件中保持一致
2. **代碼不翻譯**：所有代碼區塊、命令行指令保持英文
3. **連結檢查**：翻譯後檢查內部連結是否正確
4. **格式保持**：保持原始的 markdown 格式和結構
5. **Frontmatter**：不要翻譯 YAML frontmatter 中的鍵名

## 現狀總結

✅ **已完成：**
- 完整的 i18n 基礎設施
- 語言切換器
- 所有 UI 元素翻譯
- 3 個核心文檔頁面（每個 3 種語言 = 9 個文件）

⏳ **待完成：**
- 7 個主要文檔文件（API 參考和指南）
- 每個需要翻譯為 3 種語言
- 總計約 21 個文件，4000+ 行內容

**建議：** 可以按優先級逐步完成翻譯，或者先發布當前版本，後續逐步添加更多翻譯內容。Docusaurus 會自動為缺失的翻譯回退到英文版本。
