---
sidebar_position: 3
---

# zkenc-js API 参考

zkenc-js 的完整 API 参考，这是用于见证加密的 JavaScript/TypeScript 库。

## 安装

```bash
npm install zkenc-js
```

## 导入

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

## 类型

### `CircuitFiles`

解密操作所需的电路文件（使用 WASM 进行见证计算）。

```typescript
interface CircuitFiles {
  /** R1CS 电路文件缓冲区（.r1cs）*/
  r1csBuffer: Uint8Array;
  /** Circom WASM 文件缓冲区（.wasm），用于见证计算 */
  wasmBuffer: Uint8Array;
}
```

**范例：**

```typescript
import fs from "fs/promises";

const circuitFiles: CircuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};
```

### `CircuitFilesForEncap`

加密操作所需的电路文件（使用符号文件进行输入映射）。

```typescript
interface CircuitFilesForEncap {
  /** R1CS 电路文件缓冲区（.r1cs）*/
  r1csBuffer: Uint8Array;
  /** 符号文件内容（.sym）作为 UTF-8 字符串 */
  symContent: string;
}
```

**范例：**

```typescript
import fs from "fs/promises";

const circuitFilesForEncap: CircuitFilesForEncap = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  symContent: await fs.readFile("circuit.sym", "utf-8"), // UTF-8 字符串
};
```

### `EncapResult`

封装结果，包含密文和密钥。

```typescript
interface EncapResult {
  /** 可以用有效见证解密的密文（1576 字节）*/
  ciphertext: Uint8Array;
  /** 对称加密密钥（32 字节，AES-256）*/
  key: Uint8Array;
}
```

### `EncryptResult`

加密结果，包含组合密文和密钥。

```typescript
interface EncryptResult {
  /** 组合密文：[4B 长度][见证 CT][AES CT] */
  ciphertext: Uint8Array;
  /** 进阶使用者的加密密钥（32 字节）*/
  key: Uint8Array;
}
```

## 高阶 API

高阶 API 在单一函数调用中提供完整的见证加密功能。

### `encrypt()`

使用见证加密来加密消息。结合密钥生成与 AES-256-GCM 加密。

```typescript
async function encrypt(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>,
  message: Uint8Array
): Promise<EncryptResult>;
```

**参数：**

- `circuitFiles` - 电路文件（R1CS 和 WASM）
- `publicInputs` - 公开输入，作为 JSON 对象（仅公开信号）
- `message` - 要加密的消息，作为 Uint8Array

**返回：**

- `Promise<EncryptResult>` - 组合密文和加密密钥

**密文格式：**

```
[4 字节：见证 CT 长度][见证密文][AES 加密消息]
│                        │           │
│                        │           └─ AES-256-GCM 加密
│                        └─ 见证加密（1576 字节）
└─ Big-endian 长度（总是 1576）
```

**范例：**

```typescript
const { ciphertext, key } = await encrypt(
  {
    r1csBuffer: await fs.readFile("sudoku.r1cs"),
    wasmBuffer: await fs.readFile("sudoku.wasm"),
  },
  {
    puzzle: [5, 3, 0, 0, 7, 0, 0, 0, 0 /* ... */],
  },
  new TextEncoder().encode("秘密消息")
);

console.log("密文大小：", ciphertext.length);
// 密文大小：1608 字节（4 + 1576 + 28）
```

**效能：**

- 首次调用：约 50-100ms（WASM 初始化）
- 后续调用：约 30-50ms

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
- `ciphertext` - 来自 `encrypt()` 的组合密文
- `inputs` - 完整输入，作为 JSON 对象（公开 + 私密信号）

**返回：**

- `Promise<Uint8Array>` - 解密的消息

**抛出异常：**

- `Error` - 如果密文无效或见证不满足电路约束

**范例：**

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

- 首次调用：约 150-200ms（WASM 初始化 + 见证计算）
- 后续调用：约 100-150ms

## 低阶 API

低阶 API 提供对见证加密过程的细粒度控制。用于研究或自定义加密方案。

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
- `publicInputs` - 公开输入，作为 JSON 对象

**返回：**

- `Promise<EncapResult>` - 见证密文（1576 字节）和密钥（32 字节）

**范例：**

```typescript
const { ciphertext: witnessCiphertext, key } = await encap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"),
  },
  { publicValue: 42 }
);

// 现在使用密钥进行自定义加密
const encryptedMessage = await customEncrypt(key, message);
```

**使用情境：**

- 自定义加密方案
- 独立密钥管理
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
- `inputs` - 完整输入，作为 JSON 对象（必须满足电路约束）

**返回：**

- `Promise<Uint8Array>` - 恢复的加密密钥（32 字节）

**抛出异常：**

- `Error` - 如果见证不满足电路约束

**范例：**

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

// 现在使用恢复的密钥
const decryptedMessage = await customDecrypt(recoveredKey, encryptedMessage);
```

## 使用模式

### 基本文本加密

```typescript
import { encrypt, decrypt } from "zkenc-js";
import fs from "fs/promises";

// 用于加密（encap 使用符号文件）
const r1csBuffer = await fs.readFile("circuit.r1cs");
const symContent = await fs.readFile("circuit.sym", "utf-8");

// 加密
const message = new TextEncoder().encode("你好，世界！");
const { ciphertext } = await encrypt(
  { r1csBuffer, symContent },
  { publicInput: 42 },
  message
);

// 用于解密（decap 使用 WASM 文件）
const wasmBuffer = await fs.readFile("circuit.wasm");

// 解密
const decrypted = await decrypt({ r1csBuffer, wasmBuffer }, ciphertext, {
  publicInput: 42,
  privateInput: 123,
});

console.log(new TextDecoder().decode(decrypted));
```

### 二进制数据加密

```typescript
// 加密文件
const fileData = await fs.readFile("document.pdf");
const { ciphertext } = await encrypt(circuitFiles, publicInputs, fileData);

await fs.writeFile("document.pdf.enc", ciphertext);

// 解密文件
const encryptedData = await fs.readFile("document.pdf.enc");
const decryptedData = await decrypt(circuitFiles, encryptedData, fullInputs);

await fs.writeFile("document_decrypted.pdf", decryptedData);
```

### 一次载入电路文件

```typescript
// 载入一次
const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};

// 重复使用于多个操作
const results = await Promise.all([
  encrypt(circuitFiles, inputs1, message1),
  encrypt(circuitFiles, inputs2, message2),
  encrypt(circuitFiles, inputs3, message3),
]);
```

### 进阶：低阶 API 搭配自定义加密

```typescript
// 生成密钥
const { ciphertext: witnessCt, key } = await encap(circuitFiles, publicInputs);

// 使用你自己的加密方式
import { customEncrypt, customDecrypt } from "./my-crypto";
const encrypted = await customEncrypt(key, message);

// 存储两个部分
await fs.writeFile("witness.ct", witnessCt);
await fs.writeFile("message.ct", encrypted);

// 稍后：解密
const witnessCt = await fs.readFile("witness.ct");
const encrypted = await fs.readFile("message.ct");

const recoveredKey = await decap(circuitFiles, witnessCt, fullInputs);
const decrypted = await customDecrypt(recoveredKey, encrypted);
```

## 输入格式

### 公开输入（用于加密）

仅包含标记为公开的信号，或是约束的一部分但不属于见证的信号：

```typescript
const publicInputs = {
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,
};
```

### 完整输入（用于解密）

包含所有信号（公开 + 私密）：

```typescript
const fullInputs = {
  // 公开输入
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,

  // 私密见证
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
  ].flat(), // 将二维数组展平
};
```

## 错误处理

```typescript
try {
  const decrypted = await decrypt(circuitFiles, ciphertext, inputs);
  console.log("成功：", new TextDecoder().decode(decrypted));
} catch (error) {
  if (error.message.includes("Invalid ciphertext")) {
    console.error("密文已损坏或无效");
  } else if (error.message.includes("constraint")) {
    console.error("见证不满足电路约束");
  } else {
    console.error("解密失败：", error.message);
  }
}
```

## 效能考量

### WASM 初始化

首次调用任何函数会初始化 WASM 模块（约 20-50ms）。后续调用会更快。

### 电路复杂度

效能随电路大小扩展：

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

### 浏览器最佳化

使用 Web Workers 进行非阻塞操作：

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

## 浏览器 vs Node.js

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

zkenc-js 使用 TypeScript 撰写，并提供完整的类型定义：

```typescript
import type { CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";

// 类型安全的使用方式
async function encryptMessage(
  files: CircuitFiles,
  inputs: Record<string, any>,
  msg: string
): Promise<EncryptResult> {
  return encrypt(files, inputs, new TextEncoder().encode(msg));
}
```

## 兼容性

- **Node.js**: >= 18.0.0
- **浏览器**: 支持 WebAssembly 的现代浏览器
  - Chrome/Edge >= 90
  - Firefox >= 88
  - Safari >= 15
- **打包工具**: Vite、Webpack 5+、Rollup
- **框架**: React、Vue、Svelte、Next.js

## 下一步

- **[快速入门 →](/docs/getting-started/zkenc-js)** - 安装与基本使用
- **[Node.js 指南 →](/docs/guides/nodejs-integration)** - 完整的 Node.js 整合
- **[React 指南 →](/docs/guides/react-integration)** - 完整的 React 整合
- **[Playground →](/playground)** - 在浏览器中试用
