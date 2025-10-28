---
sidebar_position: 3
---

# zkenc-js API 參考

zkenc-js 的完整 API 參考，這是用於見證加密的 JavaScript/TypeScript 函式庫。

## 安裝

```bash
npm install zkenc-js
```

## 匯入

```typescript
import {
  encrypt,
  decrypt,
  encap,
  decap,
  getPublicInput,
  type CircuitFiles,
  type CircuitFilesForEncap,
  type EncapResult,
  type EncryptResult,
} from "zkenc-js";
```

## 型別

### `CircuitFiles`

解密操作所需的電路檔案（使用 WASM 進行見證計算）。

```typescript
interface CircuitFiles {
  /** R1CS 電路檔案緩衝區（.r1cs）*/
  r1csBuffer: Uint8Array;
  /** Circom WASM 檔案緩衝區（.wasm），用於見證計算 */
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

### `CircuitFilesForEncap`

加密操作所需的電路檔案（使用符號檔案進行輸入對應）。

```typescript
interface CircuitFilesForEncap {
  /** R1CS 電路檔案緩衝區（.r1cs）*/
  r1csBuffer: Uint8Array;
  /** 符號檔案內容（.sym）作為 UTF-8 字串 */
  symContent: string;
}
```

**範例：**

```typescript
import fs from "fs/promises";

const circuitFilesForEncap: CircuitFilesForEncap = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  symContent: await fs.readFile("circuit.sym", "utf-8"), // UTF-8 字串
};
```

### `EncapResult`

封裝結果，包含密文和金鑰。

```typescript
interface EncapResult {
  /** 可以用有效見證解密的密文（1576 位元組）*/
  ciphertext: Uint8Array;
  /** 對稱加密金鑰（32 位元組，AES-256）*/
  key: Uint8Array;
}
```

### `EncryptResult`

加密結果，包含組合密文和金鑰。

```typescript
interface EncryptResult {
  /** 組合密文：[4B 長度][見證 CT][AES CT] */
  ciphertext: Uint8Array;
  /** 進階使用者的加密金鑰（32 位元組）*/
  key: Uint8Array;
}
```

## 高階 API

高階 API 在單一函式呼叫中提供完整的見證加密功能。

### `encrypt()`

使用見證加密來加密訊息。結合金鑰產生與 AES-256-GCM 加密。

```typescript
async function encrypt(
  circuitFiles: CircuitFilesForEncap,
  publicInputs: Record<string, any>,
  message: Uint8Array,
  options?: EncryptOptions
): Promise<EncryptResult>;
```

**參數：**

- `circuitFiles` - 用於加密的電路檔案（R1CS 和符號檔案）
  - `r1csBuffer: Uint8Array` - R1CS 電路檔案
  - `symContent: string` - 符號檔案內容（UTF-8）
- `publicInputs` - 公開輸入，作為 JSON 物件（僅公開訊號）
- `message` - 要加密的訊息，作為 Uint8Array
- `options` - 可選的加密選項
  - `includePublicInput?: boolean` - 在密文中包含公開輸入（預設：true）

**回傳：**

- `Promise<EncryptResult>` - 組合密文和加密金鑰

**範例：**

```typescript
const { ciphertext, key } = await encrypt(
  {
    r1csBuffer: await fs.readFile("sudoku.r1cs"),
    symContent: await fs.readFile("sudoku.sym", "utf-8"),
  },
  {
    puzzle: [5, 3, 0, 0, 7, 0, 0, 0, 0 /* ... */],
  },
  new TextEncoder().encode("秘密訊息"),
  { includePublicInput: true } // 預設值
);

console.log("密文大小：", ciphertext.length);
```

**參數：**

- `circuitFiles` - 電路檔案（R1CS 和 WASM）
- `publicInputs` - 公開輸入，作為 JSON 物件（僅公開訊號）
- `message` - 要加密的訊息，作為 Uint8Array

**回傳：**

- `Promise<EncryptResult>` - 組合密文和加密金鑰

**密文格式：**

```
[4 位元組：見證 CT 長度][見證密文][AES 加密訊息]
│                        │           │
│                        │           └─ AES-256-GCM 加密
│                        └─ 見證加密（1576 位元組）
└─ Big-endian 長度（總是 1576）
```

**範例：**

```typescript
const { ciphertext, key } = await encrypt(
  {
    r1csBuffer: await fs.readFile("sudoku.r1cs"),
    wasmBuffer: await fs.readFile("sudoku.wasm"),
  },
  {
    puzzle: [5, 3, 0, 0, 7, 0, 0, 0, 0 /* ... */],
  },
  new TextEncoder().encode("秘密訊息")
);

console.log("密文大小：", ciphertext.length);
// 密文大小：1608 位元組（4 + 1576 + 28）
```

**效能：**

- 首次呼叫：約 50-100ms（WASM 初始化）
- 後續呼叫：約 30-50ms

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
- `ciphertext` - 來自 `encrypt()` 的組合密文
- `inputs` - 完整輸入，作為 JSON 物件（公開 + 私密訊號）

**回傳：**

- `Promise<Uint8Array>` - 解密的訊息

**拋出例外：**

- `Error` - 如果密文無效或見證不滿足電路約束

**範例：**

```typescript
const decrypted = await decrypt(
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
console.log("解密：", message);
```

**效能：**

- 首次呼叫：約 150-200ms（WASM 初始化 + 見證計算）
- 後續呼叫：約 100-150ms

## 低階 API

低階 API 提供對見證加密過程的細粒度控制。用於研究或自訂加密方案。

### `encap()`

使用見證加密產生加密金鑰（封裝）。

```typescript
async function encap(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>
): Promise<EncapResult>;
```

**參數：**

- `circuitFiles` - 電路檔案（R1CS 和 WASM）
- `publicInputs` - 公開輸入，作為 JSON 物件

**回傳：**

- `Promise<EncapResult>` - 見證密文（1576 位元組）和金鑰（32 位元組）

**範例：**

```typescript
const { ciphertext: witnessCiphertext, key } = await encap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"),
  },
  { publicValue: 42 }
);

// 現在使用金鑰進行自訂加密
const encryptedMessage = await customEncrypt(key, message);
```

**使用情境：**

- 自訂加密方案
- 獨立金鑰管理
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
- `inputs` - 完整輸入，作為 JSON 物件（必須滿足電路約束）

**回傳：**

- `Promise<Uint8Array>` - 恢復的加密金鑰（32 位元組）

**拋出例外：**

- `Error` - 如果見證不滿足電路約束

**範例：**

```typescript
const recoveredKey = await decap(
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
import { encrypt, decrypt } from "zkenc-js";
import fs from "fs/promises";

// 用於加密（encap 使用符號檔案）
const r1csBuffer = await fs.readFile("circuit.r1cs");
const symContent = await fs.readFile("circuit.sym", "utf-8");

// 加密
const message = new TextEncoder().encode("你好，世界！");
const { ciphertext } = await encrypt(
  { r1csBuffer, symContent },
  { publicInput: 42 },
  message
);

// 用於解密（decap 使用 WASM 檔案）
const wasmBuffer = await fs.readFile("circuit.wasm");

// 解密
const decrypted = await decrypt({ r1csBuffer, wasmBuffer }, ciphertext, {
  publicInput: 42,
  privateInput: 123,
});

console.log(new TextDecoder().decode(decrypted));
```

### 二進位資料加密

````typescript
### 二進位資料加密

```typescript
import { encrypt, decrypt } from "zkenc-js";
import fs from "fs/promises";

// 載入電路檔案
const r1csBuffer = await fs.readFile("circuit.r1cs");
const wasmBuffer = await fs.readFile("circuit.wasm");
const symContent = await fs.readFile("circuit.sym", "utf-8");

// 加密檔案
const fileData = await fs.readFile("document.pdf");
const { ciphertext } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  fileData
);

await fs.writeFile("document.pdf.enc", ciphertext);

// 解密檔案
const encryptedData = await fs.readFile("document.pdf.enc");
const decryptedData = await decrypt(
  { r1csBuffer, wasmBuffer },
  encryptedData,
  fullInputs
);

await fs.writeFile("document_decrypted.pdf", decryptedData);
````

### 一次載入電路檔案

```typescript
import { encrypt } from "zkenc-js";
import fs from "fs/promises";

// 載入一次用於加密操作
const encapFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  symContent: await fs.readFile("circuit.sym", "utf-8"),
};

// 重複使用於多個加密操作
```

````

### 一次載入電路檔案

```typescript
// 載入一次
const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};

// 重複使用於多個操作
const results = await Promise.all([
  encrypt(circuitFiles, inputs1, message1),
  encrypt(circuitFiles, inputs2, message2),
  encrypt(circuitFiles, inputs3, message3),
]);
````

### 進階：低階 API 搭配自訂加密

```typescript
// 產生金鑰
const { ciphertext: witnessCt, key } = await encap(circuitFiles, publicInputs);

// 使用你自己的加密方式
import { customEncrypt, customDecrypt } from "./my-crypto";
const encrypted = await customEncrypt(key, message);

// 儲存兩個部分
await fs.writeFile("witness.ct", witnessCt);
await fs.writeFile("message.ct", encrypted);

// 稍後：解密
const witnessCt = await fs.readFile("witness.ct");
const encrypted = await fs.readFile("message.ct");

const recoveredKey = await decap(circuitFiles, witnessCt, fullInputs);
const decrypted = await customDecrypt(recoveredKey, encrypted);
```

## 輸入格式

### 公開輸入（用於加密）

僅包含標記為公開的訊號，或是約束的一部分但不屬於見證的訊號：

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
  ].flat(), // 將二維陣列展平
};
```

## 錯誤處理

```typescript
try {
  const decrypted = await decrypt(circuitFiles, ciphertext, inputs);
  console.log("成功：", new TextDecoder().decode(decrypted));
} catch (error) {
  if (error.message.includes("Invalid ciphertext")) {
    console.error("密文已損壞或無效");
  } else if (error.message.includes("constraint")) {
    console.error("見證不滿足電路約束");
  } else {
    console.error("解密失敗：", error.message);
  }
}
```

## 效能考量

### WASM 初始化

首次呼叫任何函式會初始化 WASM 模組（約 20-50ms）。後續呼叫會更快。

### 電路複雜度

效能隨電路大小擴展：

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
    const decrypted = await decrypt(circuitFiles, ciphertext, inputs);
    self.postMessage({ success: true, decrypted });
  } catch (error) {
    self.postMessage({ success: false, error: error.message });
  }
};
```

## 瀏覽器 vs Node.js

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

zkenc-js 使用 TypeScript 撰寫，並提供完整的型別定義：

```typescript
import type { CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";

// 型別安全的使用方式
async function encryptMessage(
  files: CircuitFiles,
  inputs: Record<string, any>,
  msg: string
): Promise<EncryptResult> {
  return encrypt(files, inputs, new TextEncoder().encode(msg));
}
```

## 相容性

- **Node.js**: >= 18.0.0
- **瀏覽器**: 支援 WebAssembly 的現代瀏覽器
  - Chrome/Edge >= 90
  - Firefox >= 88
  - Safari >= 15
- **打包工具**: Vite、Webpack 5+、Rollup
- **框架**: React、Vue、Svelte、Next.js

## 下一步

- **[快速入門 →](/docs/getting-started/zkenc-js)** - 安裝與基本使用
- **[Node.js 指南 →](/docs/guides/nodejs-integration)** - 完整的 Node.js 整合
- **[React 指南 →](/docs/guides/react-integration)** - 完整的 React 整合
- **[Playground →](/playground)** - 在瀏覽器中試用
