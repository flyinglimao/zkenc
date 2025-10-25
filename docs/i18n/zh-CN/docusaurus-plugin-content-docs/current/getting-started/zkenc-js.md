---
sidebar_position: 1
---

# zkenc-js 入门

zkenc-js 是一个用于见证加密的 JavaScript/TypeScript 库，可在 Node.js 和浏览器环境中工作。

## 安装

使用您偏好的包管理器安装 zkenc-js：

```bash
npm install zkenc-js
# 或
yarn add zkenc-js
# 或
pnpm add zkenc-js
```

## 前置需求

使用 zkenc-js 之前，您需要：

1. **已编译的 Circom 电路**，包含以下文件：
   - `.r1cs` 文件（电路约束）
   - `.wasm` 文件（见证生成器）
2. **电路文件**可通过编译 Circom 电路获得：

```bash
circom your_circuit.circom --r1cs --wasm
```

## 快速示例

以下是使用 zkenc-js 的简单示例：

```typescript
import { zkenc, CircuitFiles } from "zkenc-js";

// 加载您的电路文件
const circuitFiles: CircuitFiles = {
  r1cs: await fs.readFile("circuit.r1cs"),
  wasm: await fs.readFile("circuit.wasm"),
};

// 定义电路的公开输入
const publicInputs = {
  publicValue: 42,
};

// 要加密的消息
const message = new TextEncoder().encode("Hello, zkenc!");

// 加密消息
const { ciphertext, key } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);

console.log("加密成功！");
console.log("密文大小：", ciphertext.length);

// 要解密，您需要完整的见证（包括私有输入）
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // 这是秘密见证
};

// 解密消息
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

const decryptedMessage = new TextDecoder().decode(decrypted);
console.log("解密消息：", decryptedMessage);
```

## 高级与低级 API

zkenc-js 提供两种 API：

### 高级 API（推荐）

高级 API（`encrypt` 和 `decrypt`）处理完整的见证加密流程：

```typescript
// 加密：结合见证加密与 AES
const { ciphertext, key } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);

// 解密：恢复密钥并解密消息
const message = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

**使用案例：**

- 一般加密/解密需求
- 当您希望一切自动处理时
- 当您不需要单独的密钥管理时

### 低级 API（高级）

低级 API（`encap` 和 `decap`）提供细粒度控制：

```typescript
// 封装：根据电路生成密钥
const { ciphertext: witnessCiphertext, key } = await zkenc.encap(
  circuitFiles,
  publicInputs
);

// 使用 AES 手动加密消息
const encryptedMessage = await aesEncrypt(key, message);

// 解封：使用有效见证恢复密钥
const recoveredKey = await zkenc.decap(
  circuitFiles,
  witnessCiphertext,
  fullInputs
);

// 手动解密消息
const decryptedMessage = await aesDecrypt(recoveredKey, encryptedMessage);
```

**使用案例：**

- 研究和实验
- 自定义加密方案
- 当您需要单独的密钥管理时

## 环境特定设置

### Node.js

zkenc-js 在 Node.js 中开箱即用：

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1cs: await fs.readFile("circuit.r1cs"),
  wasm: await fs.readFile("circuit.wasm"),
};
```

### 浏览器

在浏览器环境中，您需要以不同方式加载文件：

```typescript
import { zkenc } from "zkenc-js";

// 使用 fetch 加载文件
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

对于 React 应用程序，请参阅我们的 [React 集成指南 →](/docs/guides/react-integration)

## 常见电路模式

以下是与 zkenc 配合使用的典型 Circom 电路结构：

```circom
pragma circom 2.0.0;

template MyCircuit() {
    // 公开输入（加密者已知）
    signal input publicValue;

    // 私有输入（见证，解密所需）
    signal input privateValue;

    // 输出（必须正确计算）
    signal output result;

    // 约束
    result <== publicValue + privateValue;
}

component main = MyCircuit();
```

**重点：**

- **公开输入**：加密时已知，是加密条件的一部分
- **私有输入**：解密所需的「见证」
- **约束**：定义必须满足的条件

## 下一步

- **[API 参考 →](/docs/api/zkenc-js)** - 探索完整的 zkenc-js API
- **[Node.js 集成 →](/docs/guides/nodejs-integration)** - 逐步 Node.js 指南
- **[React 集成 →](/docs/guides/react-integration)** - 逐步 React 指南
- **[试用游乐场 →](/playground)** - 交互式数独示例

## 疑难排解

### 找不到模块错误

如果遇到模块解析错误，请确保您的 `tsconfig.json` 包含：

```json
{
  "compilerOptions": {
    "moduleResolution": "bundler",
    "esModuleInterop": true
  }
}
```

### 浏览器中的 WebAssembly 错误

确保您的打包器已配置为处理 WASM 文件。对于 Vite：

```javascript
// vite.config.js
export default {
  optimizeDeps: {
    exclude: ["zkenc-js"],
  },
};
```

### 性能考量

- 电路编译是 CPU 密集型的
- 首次加密/解密较慢，因为 WASM 初始化
- 考虑在生产环境中缓存电路文件
- 对于浏览器应用程序，使用 Web Workers 避免阻塞主线程

## 支持

如果遇到问题：

1. 查看 [API 参考](/docs/api/zkenc-js) 获取详细文档
2. 查阅 [指南](/docs/guides/intro) 了解常见模式
3. 在 [GitHub](https://github.com/flyinglimao/zkenc) 上开启问题
