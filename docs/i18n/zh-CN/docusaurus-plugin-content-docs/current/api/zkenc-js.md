---
sidebar_position: 3
---

# zkenc-js API 参考

zkenc-js 的完整 API 参考，一个用于见证加密的 JavaScript/TypeScript 库。

## 安装

```bash
npm install zkenc-js
```

## 导入

```typescript
import { zkenc, CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";
```

## 类型

### `CircuitFiles`

见证加密操作所需的电路文件。

```typescript
interface CircuitFiles {
  /** R1CS 电路文件缓冲区（.r1cs） */
  r1csBuffer: Uint8Array;
  /** 用于见证计算的 Circom WASM 文件缓冲区（.wasm） */
  wasmBuffer: Uint8Array;
}
```

**示例：**

```typescript
import fs from "fs/promises";

const circuitFiles: CircuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};
```

### `EncapResult`

封装的结果，包含密文和密钥。

```typescript
interface EncapResult {
  /** 可以使用有效见证解密的密文（1576 字节） */
  ciphertext: Uint8Array;
  /** 对称加密密钥（32 字节，AES-256） */
  key: Uint8Array;
}
```

### `EncryptResult`

加密的结果，包含合并密文和密钥。

```typescript
interface EncryptResult {
  /** 合并密文：[4B 长度][见证 CT][AES CT] */
  ciphertext: Uint8Array;
  /** 供高级用户使用的加密密钥（32 字节） */
  key: Uint8Array;
}
```

## 高级 API

高级 API 在单一函数调用中提供完整的见证加密功能。

### `encrypt()`

使用见证加密来加密消息。将密钥生成与 AES-256-GCM 加密结合。

```typescript
async function encrypt(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>,
  message: Uint8Array
): Promise<EncryptResult>;
```

**参数：**

- `circuitFiles` - 电路文件（R1CS 和 WASM）
- `publicInputs` - 作为 JSON 对象的公开输入（仅公开信号）
- `message` - 要加密的消息，格式为 Uint8Array

**返回值：**

- `Promise<EncryptResult>` - 合并密文和加密密钥

**密文格式：**

```
[4 字节：见证 CT 长度][见证密文][AES 加密消息]
│                      │            │
│                      │            └─ AES-256-GCM 加密
│                      └─ 见证加密（1576 字节）
└─ 大端序长度（总是 1576）
```

**示例：**

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

**性能：**

- 首次调用：~50-100ms（WASM 初始化）
- 后续调用：~30-50ms

### `decrypt()`

使用见证解密来解密消息。恢复密钥并解密消息。

```typescript
async function decrypt(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array>;
```

**参数：**

- `circuitFiles` - 电路文件（R1CS 和 WASM）
- `ciphertext` - 来自 `encrypt()` 的合并密文
- `inputs` - 作为 JSON 对象的完整输入（公开 + 私有信号）

**返回值：**

- `Promise<Uint8Array>` - 解密的消息

**抛出异常：**

- `Error` - 如果密文无效或见证不满足电路

**示例：**

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

**性能：**

- 首次调用：~150-200ms（WASM 初始化 + 见证计算）
- 后续调用：~100-150ms

## 低级 API

低级 API 提供对见证加密过程的精细控制。用于研究或自定义加密方案。

### `encap()`

使用见证加密生成加密密钥（封装）。

```typescript
async function encap(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>
): Promise<EncapResult>;
```

**参数：**

- `circuitFiles` - 电路文件（R1CS 和 WASM）
- `publicInputs` - 作为 JSON 对象的公开输入

**返回值：**

- `Promise<EncapResult>` - 见证密文（1576 字节）和密钥（32 字节）

**示例：**

```typescript
const { ciphertext: witnessCiphertext, key } = await zkenc.encap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"),
  },
  { publicValue: 42 }
);

// 现在使用密钥进行您自己的加密
const encryptedMessage = await customEncrypt(key, message);
```

**用例：**

- 自定义加密方案
- 单独的密钥管理
- 研究和实验

### `decap()`

使用有效见证恢复加密密钥（解封装）。

```typescript
async function decap(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array>;
```

**参数：**

- `circuitFiles` - 电路文件（R1CS 和 WASM）
- `ciphertext` - 来自 `encap()` 的见证密文（1576 字节）
- `inputs` - 作为 JSON 对象的完整输入（必须满足电路）

**返回值：**

- `Promise<Uint8Array>` - 恢复的加密密钥（32 字节）

**抛出异常：**

- `Error` - 如果见证不满足电路约束

**示例：**

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

// 现在使用恢复的密钥
const decryptedMessage = await customDecrypt(recoveredKey, encryptedMessage);
```

## 使用模式

### 基本文本加密

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

### 二进制数据加密

```typescript
// 加密文件
const fileData = await fs.readFile("document.pdf");
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  fileData
);

await fs.writeFile("document.pdf.enc", ciphertext);

// 解密文件
const encryptedData = await fs.readFile("document.pdf.enc");
const decryptedData = await zkenc.decrypt(
  circuitFiles,
  encryptedData,
  fullInputs
);

await fs.writeFile("document_decrypted.pdf", decryptedData);
```

### 一次加载电路文件

```typescript
// 加载一次
const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};

// 重复使用于多个操作
const results = await Promise.all([
  zkenc.encrypt(circuitFiles, inputs1, message1),
  zkenc.encrypt(circuitFiles, inputs2, message2),
  zkenc.encrypt(circuitFiles, inputs3, message3),
]);
```

### 高级：使用自定义加密的低级 API

```typescript
// 生成密钥
const { ciphertext: witnessCt, key } = await zkenc.encap(
  circuitFiles,
  publicInputs
);

// 使用您自己的加密
import { customEncrypt, customDecrypt } from "./my-crypto";
const encrypted = await customEncrypt(key, message);

// 存储两个部分
await fs.writeFile("witness.ct", witnessCt);
await fs.writeFile("message.ct", encrypted);

// 稍后：解密
const witnessCt = await fs.readFile("witness.ct");
const encrypted = await fs.readFile("message.ct");

const recoveredKey = await zkenc.decap(circuitFiles, witnessCt, fullInputs);
const decrypted = await customDecrypt(recoveredKey, encrypted);
```

## 输入格式

### 公开输入（用于加密）

仅包含标记为公开的信号或属于约束但不属于见证的信号：

```typescript
const publicInputs = {
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,
};
```

### 完整输入（用于解密）

包含所有信号（公开 + 私有）：

```typescript
const fullInputs = {
  // 公开输入
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,

  // 私有见证
  solutionGrid: [5, 3, 4, 6, 7, 8, 9, 1, 2 /* ... */],
};
```

### 数组信号

支持数组：

```typescript
const inputs = {
  singleValue: 42,
  arrayValue: [1, 2, 3, 4, 5],
  matrix: [
    [1, 2],
    [3, 4],
  ].flat(), // 展平二维数组
};
```

## 错误处理

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

## 性能考量

### WASM 初始化

首次调用任何函数会初始化 WASM 模块（~20-50ms）。后续调用更快。

### 电路复杂度

性能随电路大小而变化：

- 小型电路（< 1000 个约束）：< 50ms
- 中型电路（1000-10000 个约束）：50-200ms
- 大型电路（> 10000 个约束）：200ms+

### 缓存

在内存中缓存电路文件：

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

### 浏览器优化

使用 Web Workers 进行非阻塞操作：

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

## 浏览器与 Node.js

### Node.js

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};
```

### 浏览器

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

## TypeScript 支持

zkenc-js 使用 TypeScript 编写，并提供完整的类型定义：

```typescript
import type { CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";

// 类型安全的使用
async function encryptMessage(
  files: CircuitFiles,
  inputs: Record<string, any>,
  msg: string
): Promise<EncryptResult> {
  return zkenc.encrypt(files, inputs, new TextEncoder().encode(msg));
}
```

## 兼容性

- **Node.js**：>= 18.0.0
- **浏览器**：支持 WebAssembly 的现代浏览器
  - Chrome/Edge >= 90
  - Firefox >= 88
  - Safari >= 15
- **打包工具**：Vite、Webpack 5+、Rollup
- **框架**：React、Vue、Svelte、Next.js

## 下一步

- **[入门 →](/docs/getting-started/zkenc-js)** - 安装和基本用法
- **[Node.js 指南 →](/docs/guides/nodejs-integration)** - 完整的 Node.js 集成
- **[React 指南 →](/docs/guides/react-integration)** - 完整的 React 集成
- **[游乐场 →](/playground)** - 在浏览器中试用
