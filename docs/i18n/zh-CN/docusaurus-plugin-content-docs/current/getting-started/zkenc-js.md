---
sidebar_position: 1
---

# zkenc-js 快速开始

zkenc-js 是一个 JavaScript/TypeScript 函数库，用于见证加密，可在 Node.js 和浏览器环境中运作。

## 安装

使用你偏好的套件管理器安装 zkenc-js：

```bash
npm install zkenc-js
# 或
yarn add zkenc-js
# 或
pnpm add zkenc-js
```

## 前置需求

使用 zkenc-js 之前,你需要：

1. **已编译的 Circom 电路**，包含以下文件：
   - `.r1cs` 文件（电路约束）
   - `.wasm` 文件（见证生成器）
   - `.sym` 文件（符号文件）**← 加密时必需**
2. **电路文件**可通过编译 Circom 电路取得：

```bash
circom your_circuit.circom --r1cs --wasm --sym
```

## 快速范例

以下是使用 zkenc-js 的简单范例：

```typescript
import { encrypt, decrypt } from "zkenc-js";
import fs from "fs/promises";

// 载入你的电路文件
const r1csBuffer = await fs.readFile("circuit.r1cs");
const wasmBuffer = await fs.readFile("circuit.wasm");
const symContent = await fs.readFile("circuit.sym", "utf-8"); // 用于 encap 的符号文件

// 定义电路的公开输入
const publicInputs = {
  publicValue: 42,
};

// 要加密的消息
const message = new TextEncoder().encode("Hello, zkenc!");

// 加密消息（使用 r1csBuffer 和 symContent）
const { ciphertext, key } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  message
);

console.log("加密成功！");
console.log("密文大小：", ciphertext.length);

// 要解密，你需要完整的见证（包含私密输入）
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // 这是秘密见证
};

// 解密消息（使用 r1csBuffer 和 wasmBuffer）
const decrypted = await decrypt(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  fullInputs
);

const decryptedMessage = new TextDecoder().decode(decrypted);
console.log("解密消息：", decryptedMessage);
```

## 高级 API vs 低级 API

zkenc-js 提供两种 API：

### 高级 API（建议使用）

高级 API（`encrypt` 和 `decrypt`）处理完整的见证加密流程：

```typescript
// 加密：结合见证加密与 AES（使用 r1csBuffer + symContent）
const { ciphertext, key } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  message
);

// 解密：恢复密钥并解密消息（使用 r1csBuffer + wasmBuffer）
const message = await decrypt(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  fullInputs
);
```

**使用情境：**

- 一般的加密/解密需求
- 当你希望所有事情都自动处理时
- 当你不需要分开管理密钥时

### 低级 API（进阶）

低级 API（`encap` 和 `decap`）提供细粒度控制：

```typescript
// 封装：基于电路生成密钥
// 注意：encap 需要 r1csBuffer 和 symContent 以进行输入映射
const { ciphertext: witnessCiphertext, key } = await encap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    symContent: await fs.readFile("circuit.sym", "utf-8"), // 符号文件必需
  },
  publicInputs
);

// 手动使用 AES 加密消息
const encryptedMessage = await aesEncrypt(key, message);

// 解封装：使用有效见证恢复密钥
// 注意：decap 需要 r1csBuffer 和 wasmBuffer 以计算见证
const recoveredKey = await decap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"), // WASM 文件必需
  },
  witnessCiphertext,
  fullInputs
);

// 手动解密消息
const decryptedMessage = await aesDecrypt(recoveredKey, encryptedMessage);
```

**使用情境：**

- 研究与实验
- 自定义加密方案
- 当你需要分开管理密钥时

## 环境特定设置

### Node.js

zkenc-js 在 Node.js 中可直接使用：

```typescript
import { encrypt, decrypt } from "zkenc-js";
import fs from "fs/promises";

const r1csBuffer = await fs.readFile("circuit.r1cs");
const wasmBuffer = await fs.readFile("circuit.wasm");
const symContent = await fs.readFile("circuit.sym", "utf-8"); // UTF-8 字符串格式的符号文件
```

### 浏览器

在浏览器环境中，你需要用不同的方式载入文件：

```typescript
import { encrypt, decrypt } from "zkenc-js";

// 使用 fetch 载入文件
const [r1csResponse, wasmResponse, symResponse] = await Promise.all([
  fetch("/circuits/circuit.r1cs"),
  fetch("/circuits/circuit.wasm"),
  fetch("/circuits/circuit.sym"),
]);

const r1csBuffer = new Uint8Array(await r1csResponse.arrayBuffer());
const wasmBuffer = new Uint8Array(await wasmResponse.arrayBuffer());
const symContent = await symResponse.text(); // 符号文件读取为 UTF-8 文本

// 用于加密（r1csBuffer + symContent）
const { ciphertext } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  message
);

// 用于解密（r1csBuffer + wasmBuffer）
const decrypted = await decrypt(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  fullInputs
);
```

### React

对于 React 应用程序，请参考我们的 [React 整合指南 →](/docs/guides/react-integration)

## 常见电路模式

以下是适用于 zkenc 的典型 Circom 电路结构：

```circom
pragma circom 2.0.0;

template MyCircuit() {
    // 公开输入（加密者已知）
    signal input publicValue;

    // 私密输入（见证，解密所需）
    signal input privateValue;

    // 输出（必须正确计算）
    signal output result;

    // 约束
    result <== publicValue + privateValue;
}

component main = MyCircuit();
```

**重点：**

- **公开输入**：加密时已知，作为加密条件的一部分
- **私密输入**：解密所需的「见证」
- **约束**：定义必须满足的条件

## 下一步

- **[API 参考 →](/docs/api/zkenc-js)** - 探索完整的 zkenc-js API
- **[Node.js 整合 →](/docs/guides/nodejs-integration)** - 逐步 Node.js 指南
- **[React 整合 →](/docs/guides/react-integration)** - 逐步 React 指南
- **[试用实验场 →](/playground)** - 交互式数独范例

## 疑难排解

### 找不到模块错误

如果遇到模块解析错误，请确保你的 `tsconfig.json` 包含：

```json
{
  "compilerOptions": {
    "moduleResolution": "bundler",
    "esModuleInterop": true
  }
}
```

### 浏览器中的 WebAssembly 错误

确保你的打包工具已设置为处理 WASM 文件。对于 Vite：

```javascript
// vite.config.js
export default {
  optimizeDeps: {
    exclude: ["zkenc-js"],
  },
};
```

### 效能考量

- 电路编译需要大量 CPU 资源
- 首次加密/解密会因 WASM 初始化而较慢
- 考虑在生产环境中缓存电路文件
- 在浏览器应用程序中使用 Web Workers 以避免阻塞主线程

## 支持

如果遇到问题：

1. 查看 [API 参考](/docs/api/zkenc-js) 了解详细文档
2. 检阅 [指南](/docs/guides/intro) 了解常见模式
3. 在 [GitHub](https://github.com/flyinglimao/zkenc) 上开启 issue
