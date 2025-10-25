---
sidebar_position: 1
---

# 指南概覽

歡迎來到 zkenc 指南！這些逐步教學將幫助您將見證加密整合到您的專案中。

## 您將學到什麼

這些指南提供在實際應用中使用 zkenc 的完整實用範例：

### 📦 Node.js 整合

學習如何建構完整的 Node.js 應用程式與見證加密。

- 載入和編譯 Circom 電路
- 加密和解密檔案
- 正確處理電路輸入
- 錯誤處理和最佳實踐

[開始 Node.js 指南 →](/docs/guides/nodejs-integration)

### ⚛️ React 整合

建構具有見證加密的互動式 React 應用程式。

- 設定 Vite + React + TypeScript
- 在瀏覽器中處理電路檔案
- 建立加密/解密 UI
- 使用 Web Workers 優化效能

[開始 React 指南 →](/docs/guides/react-integration)

### 🔄 跨工具工作流程

結合使用 zkenc-cli 和 zkenc-js 以獲得最大靈活性。

- 使用 CLI 加密，使用 JavaScript 解密
- 跨環境共享密文
- 結合工具優勢以適應您的工作流程
- 批次處理和自動化

[開始跨工具指南 →](/docs/guides/cross-tool-workflow)

## 前置需求

開始這些指南之前，您應該：

1. **具備基本知識：**

   - JavaScript/TypeScript（用於 JS 指南）
   - 命令列工具（用於 CLI 指南）
   - Circom 電路（基本理解）

2. **安裝必要工具：**

   ```bash
   # Node.js (18+)
   node --version

   # Circom
   circom --version

   # zkenc-cli（用於跨工具指南）
   zkenc --help
   ```

3. **準備好電路：**
   - `.circom` 原始檔案
   - 或預編譯的 `.r1cs` 和 `.wasm` 檔案

## 指南結構

每個指南遵循以下結構：

1. **設定** - 專案初始化和相依性
2. **電路準備** - 編譯和載入您的電路
3. **實作** - 逐步程式碼範例
4. **測試** - 驗證一切正常運作
5. **優化** - 效能改進
6. **部署** - 生產環境考量

## 範例電路

指南使用這些範例電路：

### 簡單範例電路

用於學習的基本電路：

```circom
pragma circom 2.0.0;

template Example() {
    signal input publicValue;
    signal input privateValue;
    signal output result;

    result <== publicValue + privateValue;
}

component main = Example();
```

### 數獨電路

遊樂場中使用的實用範例：

```circom
pragma circom 2.0.0;

template Sudoku() {
    signal input puzzle[81];      // 公開：謎題
    signal input solution[81];    // 私有：解答

    // 驗證解答有效
    // ... 約束 ...
}

component main = Sudoku();
```

## 常見模式

### 加密模式

```typescript
// 1. 載入電路檔案
const circuitFiles = {
  r1csBuffer: await loadFile('circuit.r1cs'),
  wasmBuffer: await loadFile('circuit.wasm'),
};

// 2. 準備公開輸入
const publicInputs = { puzzle: [...] };

// 3. 加密
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);
```

### 解密模式

```typescript
// 1. 載入密文
const ciphertext = await loadFile('encrypted.bin');

// 2. 準備完整輸入（公開 + 私有）
const fullInputs = {
  puzzle: [...],
  solution: [...],
};

// 3. 解密
const decrypted = await zkenc.decrypt(
  circuitFiles,
  ciphertext,
  fullInputs
);
```

## 取得協助

如果遇到困難：

1. **查看 API 參考：**

   - [zkenc-js API](/docs/api/zkenc-js)
   - [zkenc-cli API](/docs/api/zkenc-cli)
   - [zkenc-core API](/docs/api/zkenc-core)

2. **試用遊樂場：**

   - [互動式數獨範例](/playground)

3. **查看範例程式碼：**

   - 每個指南都包含完整、可執行的範例

4. **開啟問題：**
   - [GitHub Issues](https://github.com/flyinglimao/zkenc/issues)

## 選擇您的指南

<div className="guides-grid">

### 對於 Node.js 開發者

適合您正在建構：

- CLI 工具
- 後端服務
- 檔案加密工具
- 批次處理器

[Node.js 整合 →](/docs/guides/nodejs-integration)

### 對於 React 開發者

適合您正在建構：

- Web 應用程式
- 互動式 UI
- 基於瀏覽器的工具
- 漸進式 Web 應用程式

[React 整合 →](/docs/guides/react-integration)

### 對於自動化

適合您：

- 使用多種工具
- 批次處理檔案
- 建構管線
- 跨平台工作流程

[跨工具工作流程 →](/docs/guides/cross-tool-workflow)

</div>

## 下一步

準備好開始了嗎？選擇上面的指南，或：

- **新手？** 從 [zkenc-js 入門](/docs/getting-started/zkenc-js) 開始
- **想要實驗？** 試試 [遊樂場](/playground)
- **需要 API 細節？** 查看 [API 參考](/docs/api/zkenc-js)

編碼愉快！🚀
