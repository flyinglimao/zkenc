---
sidebar_position: 1
---

# 指南總覽

歡迎來到 zkenc 指南！這些逐步教學將幫助你將見證加密整合到你的專案中。

## 你將學到什麼

這些指南提供完整、實用的範例，說明如何在實際應用中使用 zkenc：

### 📦 Node.js 整合

學習如何使用見證加密建立完整的 Node.js 應用程式。

- 載入和編譯 Circom 電路
- 加密和解密檔案
- 正確處理電路輸入
- 錯誤處理和最佳實踐

[開始 Node.js 指南 →](/docs/guides/nodejs-integration)

### ⚛️ React 整合

使用見證加密建立互動式 React 應用程式。

- 設定 Vite + React + TypeScript
- 在瀏覽器中處理電路檔案
- 建立加密/解密 UI
- 使用 Web Workers 優化效能

[開始 React 指南 →](/docs/guides/react-integration)

### 🔄 跨工具工作流程

結合使用 zkenc-cli 和 zkenc-js 以獲得最大靈活性。

- 使用 CLI 加密，用 JavaScript 解密
- 跨環境共享密文
- 結合工具優勢以適應你的工作流程
- 批次處理和自動化

[開始跨工具指南 →](/docs/guides/cross-tool-workflow)

## 前置需求

在開始這些指南之前，你應該：

1. **具備基本知識：**

   - JavaScript/TypeScript（用於 JS 指南）
   - 命令列工具（用於 CLI 指南）
   - Circom 電路（基本理解）

2. **安裝所需工具：**

   ```bash
   # Node.js (18+)
   node --version

   # Circom
   circom --version

   # zkenc-cli（用於跨工具指南）
   zkenc --help
   ```

3. **準備好電路：**
   - `.circom` 原始碼檔案
   - 或預先編譯的 `.r1cs` 和 `.wasm` 檔案

## 指南結構

每個指南遵循以下結構：

1. **設定** - 專案初始化和相依性
2. **電路準備** - 編譯和載入你的電路
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

實驗場中使用的實用範例：

```circom
pragma circom 2.0.0;

template Sudoku() {
    signal input puzzle[81];      // 公開：謎題
    signal input solution[81];    // 私密：解答

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

// 2. 準備完整輸入（公開 + 私密）
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

2. **試用實驗場：**

   - [互動式數獨範例](/playground)

3. **檢閱範例程式碼：**

   - 每個指南都包含完整、可執行的範例

4. **開啟 Issue：**
   - [GitHub Issues](https://github.com/flyinglimao/zkenc/issues)

## 選擇你的指南

### 給 Node.js 開發者

適合用於建立：

- CLI 工具
- 後端服務
- 檔案加密工具
- 批次處理器

[Node.js 整合 →](/docs/guides/nodejs-integration)

### 給 React 開發者

適合用於建立：

- Web 應用程式
- 互動式 UI
- 基於瀏覽器的工具
- Progressive Web Apps

[React 整合 →](/docs/guides/react-integration)

### 給自動化需求

適合用於：

- 使用多種工具
- 批次處理檔案
- 建立流程
- 跨平台工作流程

[跨工具工作流程 →](/docs/guides/cross-tool-workflow)

## 下一步

準備好開始了嗎？選擇上方的指南，或者：

- **剛接觸 zkenc？** 從 [zkenc-js 快速開始](/docs/getting-started/zkenc-js) 開始
- **想要實驗？** 試試 [實驗場](/playground)
- **需要 API 詳情？** 查看 [API 參考](/docs/api/zkenc-js)

祝你編碼愉快！🚀
