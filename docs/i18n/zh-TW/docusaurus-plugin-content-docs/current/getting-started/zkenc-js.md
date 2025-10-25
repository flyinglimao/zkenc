---
sidebar_position: 1
---

# zkenc-js 入門

zkenc-js 是一個用於見證加密的 JavaScript/TypeScript 函式庫，可在 Node.js 和瀏覽器環境中運作。

## 安裝

使用您偏好的套件管理器安裝 zkenc-js：

```bash
npm install zkenc-js
# 或
yarn add zkenc-js
# 或
pnpm add zkenc-js
```

## 前置需求

使用 zkenc-js 之前，您需要：

1. **已編譯的 Circom 電路**，包含以下檔案：
   - `.r1cs` 檔案（電路約束）
   - `.wasm` 檔案（見證產生器）
2. **電路檔案**可透過編譯 Circom 電路獲得：

```bash
circom your_circuit.circom --r1cs --wasm
```

## 快速範例

以下是使用 zkenc-js 的簡單範例：

```typescript
import { zkenc, CircuitFiles } from "zkenc-js";

// 載入您的電路檔案
const circuitFiles: CircuitFiles = {
  r1cs: await fs.readFile("circuit.r1cs"),
  wasm: await fs.readFile("circuit.wasm"),
};

// 定義電路的公開輸入
const publicInputs = {
  publicValue: 42,
};

// 要加密的訊息
const message = new TextEncoder().encode("Hello, zkenc!");

// 加密訊息
const { ciphertext, key } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);

console.log("加密成功！");
console.log("密文大小：", ciphertext.length);

// 要解密，您需要完整的見證（包括私有輸入）
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // 這是秘密見證
};

// 解密訊息
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

const decryptedMessage = new TextDecoder().decode(decrypted);
console.log("解密訊息：", decryptedMessage);
```

## 高階與低階 API

zkenc-js 提供兩種 API：

### 高階 API（推薦）

高階 API（`encrypt` 和 `decrypt`）處理完整的見證加密流程：

```typescript
// 加密：結合見證加密與 AES
const { ciphertext, key } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);

// 解密：恢復金鑰並解密訊息
const message = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

**使用案例：**

- 一般加密/解密需求
- 當您希望一切自動處理時
- 當您不需要單獨的金鑰管理時

### 低階 API（進階）

低階 API（`encap` 和 `decap`）提供細粒度控制：

```typescript
// 封裝：根據電路產生金鑰
const { ciphertext: witnessCiphertext, key } = await zkenc.encap(
  circuitFiles,
  publicInputs
);

// 使用 AES 手動加密訊息
const encryptedMessage = await aesEncrypt(key, message);

// 解封：使用有效見證恢復金鑰
const recoveredKey = await zkenc.decap(
  circuitFiles,
  witnessCiphertext,
  fullInputs
);

// 手動解密訊息
const decryptedMessage = await aesDecrypt(recoveredKey, encryptedMessage);
```

**使用案例：**

- 研究和實驗
- 自訂加密方案
- 當您需要單獨的金鑰管理時

## 環境特定設定

### Node.js

zkenc-js 在 Node.js 中開箱即用：

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1cs: await fs.readFile("circuit.r1cs"),
  wasm: await fs.readFile("circuit.wasm"),
};
```

### 瀏覽器

在瀏覽器環境中，您需要以不同方式載入檔案：

```typescript
import { zkenc } from "zkenc-js";

// 使用 fetch 載入檔案
const [r1csResponse, wasmResponse] = await Promise.all([
  fetch("/circuits/circuit.r1cs"),
  fetch("/circuits/circuit.wasm"),
]);

const circuitFiles = {
  r1cs: new Uint8Array(await r1csResponse.arrayBuffer()),
  wasm: new Uint8Array(await wasmResponse.arrayBuffer()),
};
```

### React

對於 React 應用程式，請參閱我們的 [React 整合指南 →](/docs/guides/react-integration)

## 常見電路模式

以下是與 zkenc 配合使用的典型 Circom 電路結構：

```circom
pragma circom 2.0.0;

template MyCircuit() {
    // 公開輸入（加密者已知）
    signal input publicValue;

    // 私有輸入（見證，解密所需）
    signal input privateValue;

    // 輸出（必須正確計算）
    signal output result;

    // 約束
    result <== publicValue + privateValue;
}

component main = MyCircuit();
```

**重點：**

- **公開輸入**：加密時已知，是加密條件的一部分
- **私有輸入**：解密所需的「見證」
- **約束**：定義必須滿足的條件

## 下一步

- **[API 參考 →](/docs/api/zkenc-js)** - 探索完整的 zkenc-js API
- **[Node.js 整合 →](/docs/guides/nodejs-integration)** - 逐步 Node.js 指南
- **[React 整合 →](/docs/guides/react-integration)** - 逐步 React 指南
- **[試用遊樂場 →](/playground)** - 互動式數獨範例

## 疑難排解

### 找不到模組錯誤

如果遇到模組解析錯誤，請確保您的 `tsconfig.json` 包含：

```json
{
  "compilerOptions": {
    "moduleResolution": "bundler",
    "esModuleInterop": true
  }
}
```

### 瀏覽器中的 WebAssembly 錯誤

確保您的打包器已配置為處理 WASM 檔案。對於 Vite：

```javascript
// vite.config.js
export default {
  optimizeDeps: {
    exclude: ["zkenc-js"],
  },
};
```

### 效能考量

- 電路編譯是 CPU 密集型的
- 首次加密/解密較慢，因為 WASM 初始化
- 考慮在生產環境中快取電路檔案
- 對於瀏覽器應用程式，使用 Web Workers 避免阻塞主執行緒

## 支援

如果遇到問題：

1. 查看 [API 參考](/docs/api/zkenc-js) 取得詳細文件
2. 查閱 [指南](/docs/guides/intro) 瞭解常見模式
3. 在 [GitHub](https://github.com/flyinglimao/zkenc) 上開啟問題
