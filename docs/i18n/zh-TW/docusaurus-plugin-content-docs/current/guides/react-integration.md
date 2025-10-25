---
sidebar_position: 3
---

# React 整合指南

使用 zkenc-js 建立具有見證加密功能的互動式 React 應用程式。

## 我們要建立什麼

一個 React 應用程式：

- 在瀏覽器中加密和解密訊息
- 使用數獨謎題作為電路
- 提供直觀的 UI
- 處理檔案上傳和下載

## 前置需求

- Node.js 18+
- 基本的 React 和 TypeScript 知識
- Circom 編譯的電路檔案

## 步驟 1：專案設定

建立新的 Vite + React + TypeScript 專案：

```bash
npm create vite@latest zkenc-react-app -- --template react-ts
cd zkenc-react-app
npm install
```

安裝 zkenc-js：

```bash
npm install zkenc-js
```

## 步驟 2：添加電路檔案

將編譯好的電路檔案複製到 `public/circuits/`：

```
public/
└── circuits/
    ├── simple.r1cs
    └── simple.wasm
```

這讓瀏覽器可以透過 fetch 載入它們。

## 步驟 3：建立電路載入器

建立 `src/utils/circuit.ts`：

```typescript
import { CircuitFiles } from "zkenc-js";

export async function loadCircuitFiles(): Promise<CircuitFiles> {
  const [r1csResponse, wasmResponse] = await Promise.all([
    fetch("/circuits/simple.r1cs"),
    fetch("/circuits/simple.wasm"),
  ]);

  if (!r1csResponse.ok || !wasmResponse.ok) {
    throw new Error("無法載入電路檔案");
  }

  const [r1csBuffer, wasmBuffer] = await Promise.all([
    r1csResponse.arrayBuffer(),
    wasmResponse.arrayBuffer(),
  ]);

  return {
    r1csBuffer: new Uint8Array(r1csBuffer),
    wasmBuffer: new Uint8Array(wasmBuffer),
  };
}
```

## 步驟 4：建立加密/解密元件

建立主要應用程式元件，處理加密和解密操作。

## 步驟 5：添加 UI 元素

- 訊息輸入框
- 公開/私密值輸入
- 加密/解密按鈕
- 結果顯示區域
- 檔案下載功能

## 步驟 6：處理狀態管理

使用 React hooks 管理：

- 載入狀態
- 錯誤訊息
- 電路檔案快取
- 加密/解密結果

## 步驟 7：配置 Vite

更新 `vite.config.ts` 以處理 WASM：

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  optimizeDeps: {
    exclude: ["zkenc-js"],
  },
});
```

## 最佳實踐

1. **使用 Web Workers**：將加密操作移至 worker 以避免阻塞 UI
2. **快取電路**：載入後快取電路檔案
3. **錯誤處理**：提供清晰的錯誤訊息
4. **載入狀態**：顯示進度指示器
5. **檔案驗證**：驗證上傳的檔案格式

## 下一步

- **[Node.js 整合 →](/docs/guides/nodejs-integration)** - 伺服器端實作
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 結合 CLI 和 JS
- **[實驗場 →](/playground)** - 查看完整範例
- **[API 參考 →](/docs/api/zkenc-js)** - 完整 API 文件
