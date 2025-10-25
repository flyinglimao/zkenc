---
sidebar_position: 3
---

# zkenc-js API 參考

zkenc-js 的完整 API 參考，一個用於見證加密的 JavaScript/TypeScript 函式庫。

## 安裝

```bash
npm install zkenc-js
```

## 匯入

```typescript
import { zkenc, CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";
```

## 類型

### `CircuitFiles`

見證加密操作所需的電路檔案。

```typescript
interface CircuitFiles {
  /** R1CS 電路檔案緩衝區（.r1cs） */
  r1csBuffer: Uint8Array;
  /** 用於見證計算的 Circom WASM 檔案緩衝區（.wasm） */
  wasmBuffer: Uint8Array;
}
```

**範例：**

```typescript
import fs from "fs/promises";

const circuitFiles: CircuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};
```

### `EncapResult`

封裝的結果，包含密文和金鑰。

```typescript
interface EncapResult {
  /** 可以使用有效見證解密的密文（1576 位元組） */
  ciphertext: Uint8Array;
  /** 對稱加密金鑰（32 位元組，AES-256） */
  key: Uint8Array;
}
```

### `EncryptResult`

加密的結果，包含合併密文和金鑰。

```typescript
interface EncryptResult {
  /** 合併密文：[4B 長度][見證 CT][AES CT] */
  ciphertext: Uint8Array;
  /** 供進階使用者使用的加密金鑰（32 位元組） */
  key: Uint8Array;
}
```

## 高階 API

高階 API 在單一函數呼叫中提供完整的見證加密功能。

### `encrypt()`

使用見證加密來加密訊息。將金鑰生成與 AES-256-GCM 加密結合。

```typescript
async function encrypt(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>,
  message: Uint8Array
): Promise<EncryptResult>;
```

**參數：**

- `circuitFiles` - 電路檔案（R1CS 和 WASM）
- `publicInputs` - 作為 JSON 物件的公開輸入（僅公開訊號）
- `message` - 要加密的訊息，格式為 Uint8Array

**回傳值：**

- `Promise<EncryptResult>` - 合併密文和加密金鑰

**密文格式：**

```
[4 位元組：見證 CT 長度][見證密文][AES 加密訊息]
│                        │              │
│                        │              └─ AES-256-GCM 加密
│                        └─ 見證加密（1576 位元組）
└─ 大端序長度（總是 1576）
```

**範例：**

```typescript
const { ciphertext, key } = await zkenc.encrypt(
  {
    r1csBuffer: await fs.readFile("sudoku.r1cs"),
    wasmBuffer: await fs.readFile("sudoku.wasm"),
  },
  {
    puzzle: [5, 3, 0, 0, 7, 0, 0, 0, 0 /* ... */],
  },
  new TextEncoder().encode("Secret message")
);

console.log("Ciphertext size:", ciphertext.length);
// Ciphertext size: 1608 bytes (4 + 1576 + 28)
```

**效能：**

- 首次呼叫：~50-100ms（WASM 初始化）
- 後續呼叫：~30-50ms

### `decrypt()`

使用見證解密來解密訊息。恢復金鑰並解密訊息。

```typescript
async function decrypt(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array>;
```

**參數：**

- `circuitFiles` - 電路檔案（R1CS 和 WASM）
- `ciphertext` - 來自 `encrypt()` 的合併密文
- `inputs` - 作為 JSON 物件的完整輸入（公開 + 私密訊號）

**回傳值：**

- `Promise<Uint8Array>` - 解密的訊息

**拋出異常：**

- `Error` - 如果密文無效或見證不滿足電路

**範例：**

```typescript
const decrypted = await zkenc.decrypt(
  {
    r1csBuffer: await fs.readFile("sudoku.r1cs"),
    wasmBuffer: await fs.readFile("sudoku.wasm"),
  },
  ciphertext,
  {
    puzzle: [5, 3, 0, 0, 7, 0, 0, 0, 0 /* ... */],
    solution: [5, 3, 4, 6, 7, 8, 9, 1, 2 /* ... */],
  }
);

const message = new TextDecoder().decode(decrypted);
console.log("Decrypted:", message);
```

**效能：**

- 首次呼叫：~150-200ms（WASM 初始化 + 見證計算）
- 後續呼叫：~100-150ms

## 低階 API

低階 API 提供對見證加密過程的精細控制。用於研究或自訂加密方案。

### `encap()`

使用見證加密生成加密金鑰（封裝）。

```typescript
async function encap(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>
): Promise<EncapResult>;
```

**參數：**

- `circuitFiles` - 電路檔案（R1CS 和 WASM）
- `publicInputs` - 作為 JSON 物件的公開輸入

**回傳值：**

- `Promise<EncapResult>` - 見證密文（1576 位元組）和金鑰（32 位元組）

**範例：**

```typescript
const { ciphertext: witnessCiphertext, key } = await zkenc.encap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"),
  },
  { publicValue: 42 }
);

// 現在使用金鑰進行您自己的加密
const encryptedMessage = await customEncrypt(key, message);
```

**使用案例：**

- 自訂加密方案
- 單獨的金鑰管理
- 研究和實驗

### `decap()`

使用有效見證恢復加密金鑰（解封裝）。

```typescript
async function decap(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array>;
```

**參數：**

- `circuitFiles` - 電路檔案（R1CS 和 WASM）
- `ciphertext` - 來自 `encap()` 的見證密文（1576 位元組）
- `inputs` - 作為 JSON 物件的完整輸入（必須滿足電路）

**回傳值：**

- `Promise<Uint8Array>` - 恢復的加密金鑰（32 位元組）

**拋出異常：**

- `Error` - 如果見證不滿足電路約束

**範例：**

```typescript
const recoveredKey = await zkenc.decap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"),
  },
  witnessCiphertext,
  {
    publicValue: 42,
    privateValue: 123,
  }
);

// 現在使用恢復的金鑰
const decryptedMessage = await customDecrypt(recoveredKey, encryptedMessage);
```

## 使用模式

### 基本文字加密

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};

// 加密
const message = new TextEncoder().encode("Hello, World!");
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  { publicInput: 42 },
  message
);

// 解密
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, {
  publicInput: 42,
  privateInput: 123,
});

console.log(new TextDecoder().decode(decrypted));
```

### 二進位資料加密

```typescript
// 加密檔案
const fileData = await fs.readFile("document.pdf");
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  fileData
);

await fs.writeFile("document.pdf.enc", ciphertext);

// 解密檔案
const encryptedData = await fs.readFile("document.pdf.enc");
const decryptedData = await zkenc.decrypt(
  circuitFiles,
  encryptedData,
  fullInputs
);

await fs.writeFile("document_decrypted.pdf", decryptedData);
```

### 一次載入電路檔案

```typescript
// 載入一次
const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};

// 重複使用於多個操作
const results = await Promise.all([
  zkenc.encrypt(circuitFiles, inputs1, message1),
  zkenc.encrypt(circuitFiles, inputs2, message2),
  zkenc.encrypt(circuitFiles, inputs3, message3),
]);
```

### 進階：使用自訂加密的低階 API

```typescript
// 生成金鑰
const { ciphertext: witnessCt, key } = await zkenc.encap(
  circuitFiles,
  publicInputs
);

// 使用您自己的加密
import { customEncrypt, customDecrypt } from "./my-crypto";
const encrypted = await customEncrypt(key, message);

// 儲存兩個部分
await fs.writeFile("witness.ct", witnessCt);
await fs.writeFile("message.ct", encrypted);

// 稍後：解密
const witnessCt = await fs.readFile("witness.ct");
const encrypted = await fs.readFile("message.ct");

const recoveredKey = await zkenc.decap(circuitFiles, witnessCt, fullInputs);
const decrypted = await customDecrypt(recoveredKey, encrypted);
```

## 輸入格式

### 公開輸入（用於加密）

僅包含標記為公開的訊號或屬於約束但不屬於見證的訊號：

```typescript
const publicInputs = {
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,
};
```

### 完整輸入（用於解密）

包含所有訊號（公開 + 私密）：

```typescript
const fullInputs = {
  // 公開輸入
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,

  // 私密見證
  solutionGrid: [5, 3, 4, 6, 7, 8, 9, 1, 2 /* ... */],
};
```

### 陣列訊號

支援陣列：

```typescript
const inputs = {
  singleValue: 42,
  arrayValue: [1, 2, 3, 4, 5],
  matrix: [
    [1, 2],
    [3, 4],
  ].flat(), // 展平二維陣列
};
```

## 錯誤處理

```typescript
try {
  const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, inputs);
  console.log("Success:", new TextDecoder().decode(decrypted));
} catch (error) {
  if (error.message.includes("Invalid ciphertext")) {
    console.error("Ciphertext is corrupted or invalid");
  } else if (error.message.includes("constraint")) {
    console.error("Witness does not satisfy circuit constraints");
  } else {
    console.error("Decryption failed:", error.message);
  }
}
```

## 效能考量

### WASM 初始化

首次呼叫任何函數會初始化 WASM 模組（~20-50ms）。後續呼叫更快。

### 電路複雜度

效能隨電路大小而變化：

- 小型電路（< 1000 個約束）：< 50ms
- 中型電路（1000-10000 個約束）：50-200ms
- 大型電路（> 10000 個約束）：200ms+

### 快取

在記憶體中快取電路檔案：

```typescript
let cachedCircuitFiles: CircuitFiles | null = null;

async function getCircuitFiles(): Promise<CircuitFiles> {
  if (!cachedCircuitFiles) {
    cachedCircuitFiles = {
      r1csBuffer: await fs.readFile("circuit.r1cs"),
      wasmBuffer: await fs.readFile("circuit.wasm"),
    };
  }
  return cachedCircuitFiles;
}
```

### 瀏覽器最佳化

使用 Web Workers 進行非阻塞操作：

```typescript
// worker.ts
import { zkenc } from "zkenc-js";

self.onmessage = async (e) => {
  const { circuitFiles, ciphertext, inputs } = e.data;

  try {
    const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, inputs);
    self.postMessage({ success: true, decrypted });
  } catch (error) {
    self.postMessage({ success: false, error: error.message });
  }
};
```

## 瀏覽器與 Node.js

### Node.js

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};
```

### 瀏覽器

```typescript
import { zkenc } from "zkenc-js";

const [r1cs, wasm] = await Promise.all([
  fetch("/circuits/circuit.r1cs").then((r) => r.arrayBuffer()),
  fetch("/circuits/circuit.wasm").then((r) => r.arrayBuffer()),
]);

const circuitFiles = {
  r1csBuffer: new Uint8Array(r1cs),
  wasmBuffer: new Uint8Array(wasm),
};
```

## TypeScript 支援

zkenc-js 使用 TypeScript 編寫，並提供完整的類型定義：

```typescript
import type { CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";

// 類型安全的使用
async function encryptMessage(
  files: CircuitFiles,
  inputs: Record<string, any>,
  msg: string
): Promise<EncryptResult> {
  return zkenc.encrypt(files, inputs, new TextEncoder().encode(msg));
}
```

## 相容性

- **Node.js**：>= 18.0.0
- **瀏覽器**：支援 WebAssembly 的現代瀏覽器
  - Chrome/Edge >= 90
  - Firefox >= 88
  - Safari >= 15
- **打包工具**：Vite、Webpack 5+、Rollup
- **框架**：React、Vue、Svelte、Next.js

## 下一步

- **[入門 →](/docs/getting-started/zkenc-js)** - 安裝和基本用法
- **[Node.js 指南 →](/docs/guides/nodejs-integration)** - 完整的 Node.js 整合
- **[React 指南 →](/docs/guides/react-integration)** - 完整的 React 整合
- **[遊樂場 →](/playground)** - 在瀏覽器中試用
