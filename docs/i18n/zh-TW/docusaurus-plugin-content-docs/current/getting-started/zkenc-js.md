---
sidebar_position: 1
---

# zkenc-js 快速開始

zkenc-js 是一個 JavaScript/TypeScript 函式庫，用於見證加密，可在 Node.js 和瀏覽器環境中運作。

## 安裝

使用你偏好的套件管理器安裝 zkenc-js：

```bash
npm install zkenc-js
# 或
yarn add zkenc-js
# 或
pnpm add zkenc-js
```

## 前置需求

使用 zkenc-js 之前，你需要：

1. **已編譯的 Circom 電路**，包含以下檔案：

   - `.r1cs` 檔案（電路約束）
   - `.wasm` 檔案（見證產生器，用於 decap）
   - `.sym` 檔案（符號檔案，用於 encap）**← encap 時必需**

2. **電路檔案**可透過編譯 Circom 電路取得：

```bash
circom your_circuit.circom --r1cs --wasm --sym
```

**注意：**`.sym` 旗標在 v0.2.0 中為必需，以確保 JSON 鍵順序獨立性。

## 快速範例

以下是使用 zkenc-js 的簡單範例：

```typescript
import { encrypt, decrypt } from "zkenc-js";
import fs from "fs/promises";

// 載入你的電路檔案
const r1csBuffer = await fs.readFile("circuit.r1cs");
const wasmBuffer = await fs.readFile("circuit.wasm");
const symContent = await fs.readFile("circuit.sym", "utf-8"); // 用於 encap 的符號檔案

// 定義電路的公開輸入
const publicInputs = {
  publicValue: 42,
};

// 要加密的訊息
const message = new TextEncoder().encode("Hello, zkenc!");

// 加密訊息（使用 r1csBuffer 和 symContent）
const { ciphertext, key } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  message
);

console.log("加密成功！");
console.log("密文大小：", ciphertext.length);

// 要解密，你需要完整的見證（包含私密輸入）
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // 這是秘密見證
};

// 解密訊息（使用 r1csBuffer 和 wasmBuffer）
const decrypted = await decrypt(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  fullInputs
);

const decryptedMessage = new TextDecoder().decode(decrypted);
console.log("解密訊息：", decryptedMessage);
```

## 高階 API vs 低階 API

zkenc-js 提供兩種 API：

### 高階 API（建議使用）

高階 API（`encrypt` 和 `decrypt`）處理完整的見證加密流程：

```typescript
// 加密：結合見證加密與 AES（使用 r1csBuffer + symContent）
const { ciphertext, key } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  message
);

// 解密：恢復金鑰並解密訊息（使用 r1csBuffer + wasmBuffer）
const message = await decrypt(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  fullInputs
);
```

**使用情境：**

- 一般的加密/解密需求
- 當你希望所有事情都自動處理時
- 當你不需要分開管理金鑰時

### 低階 API（進階）

低階 API（`encap` 和 `decap`）提供細粒度控制：

```typescript
// 封裝：基於電路產生金鑰
// 注意：encap 需要 r1csBuffer 和 symContent 以進行輸入對應
const { ciphertext: witnessCiphertext, key } = await encap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    symContent: await fs.readFile("circuit.sym", "utf-8"), // 符號檔案必需
  },
  publicInputs
);

// 手動使用 AES 加密訊息
const encryptedMessage = await aesEncrypt(key, message);

// 解封裝：使用有效見證恢復金鑰
// 注意：decap 需要 r1csBuffer 和 wasmBuffer 以計算見證
const recoveredKey = await decap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"), // WASM 檔案必需
  },
  witnessCiphertext,
  fullInputs
);

// 手動解密訊息
const decryptedMessage = await aesDecrypt(recoveredKey, encryptedMessage);
```

**使用情境：**

- 研究與實驗
- 自訂加密方案
- 當你需要分開管理金鑰時

## 環境特定設定

### Node.js

zkenc-js 在 Node.js 中可直接使用：

```typescript
import { encrypt, decrypt } from "zkenc-js";
import fs from "fs/promises";

const r1csBuffer = await fs.readFile("circuit.r1cs");
const wasmBuffer = await fs.readFile("circuit.wasm");
const symContent = await fs.readFile("circuit.sym", "utf-8"); // UTF-8 字串格式的符號檔案
```

### 瀏覽器

在瀏覽器環境中，你需要用不同的方式載入檔案：

```typescript
import { encrypt, decrypt } from "zkenc-js";

// 使用 fetch 載入檔案
const [r1csResponse, wasmResponse, symResponse] = await Promise.all([
  fetch("/circuits/circuit.r1cs"),
  fetch("/circuits/circuit.wasm"),
  fetch("/circuits/circuit.sym"),
]);

const r1csBuffer = new Uint8Array(await r1csResponse.arrayBuffer());
const wasmBuffer = new Uint8Array(await wasmResponse.arrayBuffer());
const symContent = await symResponse.text(); // 符號檔案讀取為 UTF-8 文字

// 用於加密（r1csBuffer + symContent）
const { ciphertext } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  message
);

// 用於解密（r1csBuffer + wasmBuffer）
const decrypted = await decrypt(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  fullInputs
);
```

### React

對於 React 應用程式，請參考我們的 [React 整合指南 →](/docs/guides/react-integration)

## 常見電路模式

以下是適用於 zkenc 的典型 Circom 電路結構：

```circom
pragma circom 2.0.0;

template MyCircuit() {
    // 公開輸入（加密者已知）
    signal input publicValue;

    // 私密輸入（見證，解密所需）
    signal input privateValue;

    // 輸出（必須正確計算）
    signal output result;

    // 約束
    result <== publicValue + privateValue;
}

component main = MyCircuit();
```

**重點：**

- **公開輸入**：加密時已知，作為加密條件的一部分
- **私密輸入**：解密所需的「見證」
- **約束**：定義必須滿足的條件

## 下一步

- **[API 參考 →](/docs/api/zkenc-js)** - 探索完整的 zkenc-js API
- **[Node.js 整合 →](/docs/guides/nodejs-integration)** - 逐步 Node.js 指南
- **[React 整合 →](/docs/guides/react-integration)** - 逐步 React 指南
- **[試用實驗場 →](/playground)** - 互動式數獨範例

## 疑難排解

### 找不到模組錯誤

如果遇到模組解析錯誤，請確保你的 `tsconfig.json` 包含：

```json
{
  "compilerOptions": {
    "moduleResolution": "bundler",
    "esModuleInterop": true
  }
}
```

### 瀏覽器中的 WebAssembly 錯誤

確保你的打包工具已設定為處理 WASM 檔案。對於 Vite：

```javascript
// vite.config.js
export default {
  optimizeDeps: {
    exclude: ["zkenc-js"],
  },
};
```

### 效能考量

- 電路編譯需要大量 CPU 資源
- 首次加密/解密會因 WASM 初始化而較慢
- 考慮在生產環境中快取電路檔案
- 在瀏覽器應用程式中使用 Web Workers 以避免阻塞主執行緒

## 支援

如果遇到問題：

1. 查看 [API 參考](/docs/api/zkenc-js) 了解詳細文件
2. 檢閱 [指南](/docs/guides/intro) 了解常見模式
3. 在 [GitHub](https://github.com/flyinglimao/zkenc) 上開啟 issue
