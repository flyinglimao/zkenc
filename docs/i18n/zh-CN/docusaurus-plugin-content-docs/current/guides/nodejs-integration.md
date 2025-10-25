---
sidebar_position: 2
---

# Node.js 整合指南

本指南展示如何使用 zkenc-js 建立完整的 Node.js 应用程序进行见证加密。

## 我们要建立什么

一个 Node.js CLI 工具：

- 使用数独电路加密文件
- 使用有效的数独解答解密文件
- 优雅地处理错误
- 提供清晰的命令行界面

## 前置需求

- Node.js 18 或更高版本
- 基本的 TypeScript 知识
- 已安装 Circom（`circom --version`）

## 步骤 1：项目设置

建立新项目：

```bash
mkdir zkenc-node-example
cd zkenc-node-example
npm init -y
```

安装依赖包：

```bash
npm install zkenc-js
npm install --save-dev typescript @types/node tsx
```

建立 `tsconfig.json`：

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "esModuleInterop": true,
    "strict": true,
    "skipLibCheck": true,
    "outDir": "./dist"
  },
  "include": ["src/**/*"]
}
```

更新 `package.json`：

```json
{
  "type": "module",
  "scripts": {
    "dev": "tsx src/index.ts",
    "build": "tsc",
    "start": "node dist/index.js"
  }
}
```

## 步骤 2：准备电路文件

建立简单电路 `circuits/simple.circom`：

```circom
pragma circom 2.0.0;

template Simple() {
    signal input publicValue;
    signal input privateValue;
    signal output result;

    // 约束：publicValue + privateValue 必须等于 100
    result <== publicValue + privateValue;
    result === 100;
}

component main {public [publicValue]} = Simple();
```

编译电路：

```bash
mkdir -p circuits/build
circom circuits/simple.circom --r1cs --wasm -o circuits/build
```

这会建立：

- `circuits/build/simple.r1cs`
- `circuits/build/simple_js/simple.wasm`

## 步骤 3：载入电路文件

建立 `src/circuit.ts`：

```typescript
import fs from "fs/promises";
import path from "path";
import { CircuitFiles } from "zkenc-js";

export async function loadCircuitFiles(): Promise<CircuitFiles> {
  const circuitsDir = path.join(process.cwd(), "circuits", "build");

  const [r1csBuffer, wasmBuffer] = await Promise.all([
    fs.readFile(path.join(circuitsDir, "simple.r1cs")),
    fs.readFile(path.join(circuitsDir, "simple_js", "simple.wasm")),
  ]);

  return {
    r1csBuffer,
    wasmBuffer,
  };
}
```

## 步骤 4：实现加密

建立 `src/encrypt.ts`：

```typescript
import fs from "fs/promises";
import { zkenc } from "zkenc-js";
import { loadCircuitFiles } from "./circuit.js";

export async function encryptFile(
  inputFile: string,
  outputFile: string,
  publicValue: number
): Promise<void> {
  console.log("🔐 开始加密...");

  // 载入电路文件
  console.log("📂 载入电路...");
  const circuitFiles = await loadCircuitFiles();

  // 读取消息文件
  console.log("📄 读取消息...");
  const message = await fs.readFile(inputFile);
  console.log(`   消息大小：${message.length} 字节`);

  // 准备公开输入
  const publicInputs = {
    publicValue: publicValue,
  };

  // 加密
  console.log("🔒 加密中...");
  const startTime = Date.now();

  const { ciphertext, key } = await zkenc.encrypt(
    circuitFiles,
    publicInputs,
    message
  );

  const duration = Date.now() - startTime;
  console.log(`   加密耗时 ${duration}ms`);

  // 存储密文
  await fs.writeFile(outputFile, ciphertext);
  console.log(`✅ 密文已存储至：${outputFile}`);
  console.log(`   密文大小：${ciphertext.length} 字节`);

  // 选择性地存储密钥用于调试
  const keyFile = outputFile + ".key";
  await fs.writeFile(keyFile, key);
  console.log(`🔑 密钥已存储至：${keyFile}（用于调试）`);
}
```

## 步骤 5：实现解密

建立 `src/decrypt.ts`：

```typescript
import fs from "fs/promises";
import { zkenc } from "zkenc-js";
import { loadCircuitFiles } from "./circuit.js";

export async function decryptFile(
  inputFile: string,
  outputFile: string,
  publicValue: number,
  privateValue: number
): Promise<void> {
  console.log("🔓 开始解密...");

  // 载入电路文件
  console.log("📂 载入电路...");
  const circuitFiles = await loadCircuitFiles();

  // 读取密文
  console.log("📦 读取密文...");
  const ciphertext = await fs.readFile(inputFile);
  console.log(`   密文大小：${ciphertext.length} 字节`);

  // 准备完整输入（公开 + 私密）
  const fullInputs = {
    publicValue: publicValue,
    privateValue: privateValue,
  };

  // 验证输入满足约束
  if (publicValue + privateValue !== 100) {
    throw new Error(`无效的见证：${publicValue} + ${privateValue} ≠ 100`);
  }

  // 解密
  console.log("🔓 解密中...");
  const startTime = Date.now();

  try {
    const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

    const duration = Date.now() - startTime;
    console.log(`   解密耗时 ${duration}ms`);

    // 存储解密消息
    await fs.writeFile(outputFile, decrypted);
    console.log(`✅ 消息已解密至：${outputFile}`);
    console.log(`   消息大小：${decrypted.length} 字节`);
  } catch (error) {
    console.error("❌ 解密失败！");
    if (error instanceof Error) {
      console.error(`   错误：${error.message}`);
    }
    throw error;
  }
}
```
## 步骤 6：建立 CLI 界面

建立 `src/index.ts`：

```typescript
#!/usr/bin/env node

import { Command } from "commander";
import { encryptFile } from "./encrypt.js";
import { decryptFile } from "./decrypt.js";

const program = new Command();

program
  .name("zkenc-example")
  .description("使用 zkenc-js 的见证加密范例")
  .version("1.0.0");

program
  .command("encrypt")
  .description("加密文件")
  .requiredOption("-i, --input <file>", "要加密的输入文件")
  .requiredOption("-o, --output <file>", "输出加密文件")
  .requiredOption("-p, --public <value>", "公开值（数字）", parseInt)
  .action(async (options) => {
    try {
      await encryptFile(options.input, options.output, options.public);
    } catch (error) {
      console.error("加密失败：", error);
      process.exit(1);
    }
  });

program
  .command("decrypt")
  .description("解密文件")
  .requiredOption("-i, --input <file>", "输入加密文件")
  .requiredOption("-o, --output <file>", "输出解密文件")
  .requiredOption("-p, --public <value>", "公开值（数字）", parseInt)
  .requiredOption("--private <value>", "私密值（数字）", parseInt)
  .action(async (options) => {
    try {
      await decryptFile(
        options.input,
        options.output,
        options.public,
        options.private
      );
    } catch (error) {
      console.error("解密失败：", error);
      process.exit(1);
    }
  });

program.parse();
```

安装 commander 用于 CLI：

```bash
npm install commander
```

## 步骤 7：测试应用程序

建立测试消息：

```bash
echo "这是秘密消息！" > message.txt
```

加密消息：

```bash
npm run dev encrypt -- \
  --input message.txt \
  --output encrypted.bin \
  --public 30
```

输出：

```
🔐 开始加密...
📂 载入电路...
📄 读取消息...
   消息大小：26 字节
🔒 加密中...
   加密耗时 45ms
✅ 密文已存储至：encrypted.bin
   密文大小：1630 字节
🔑 密钥已存储至：encrypted.bin.key（用于调试）
```

解密消息（使用正确的见证：30 + 70 = 100）：

```bash
npm run dev decrypt -- \
  --input encrypted.bin \
  --output decrypted.txt \
  --public 30 \
  --private 70
```

输出：

```
🔓 开始解密...
📂 载入电路...
📦 读取密文...
   密文大小：1630 字节
🔓 解密中...
   解密耗时 156ms
✅ 消息已解密至：decrypted.txt
   消息大小：26 字节
```

验证：

```bash
diff message.txt decrypted.txt
echo "成功！"
```

尝试使用错误的见证（会失败）：

```bash
npm run dev decrypt -- \
  --input encrypted.bin \
  --output decrypted.txt \
  --public 30 \
  --private 50
```

输出：

```
❌ 解密失败！
   错误：无效的见证：30 + 50 ≠ 100
```

## 步骤 8：进阶功能

### 电路文件缓存

建立 `src/circuit-cache.ts`：

```typescript
import { CircuitFiles } from "zkenc-js";
import { loadCircuitFiles } from "./circuit.js";

let circuitCache: CircuitFiles | null = null;

export async function getCachedCircuitFiles(): Promise<CircuitFiles> {
  if (!circuitCache) {
    console.log("💾 缓存电路文件...");
    circuitCache = await loadCircuitFiles();
  }
  return circuitCache;
}
```

### 进度报告

```typescript
export async function encryptFileWithProgress(
  inputFile: string,
  outputFile: string,
  publicValue: number
): Promise<void> {
  const steps = ["载入电路", "读取消息", "加密", "存储密文"];

  for (let i = 0; i < steps.length; i++) {
    console.log(`[${i + 1}/${steps.length}] ${steps[i]}...`);
    // ... 执行步骤
  }
}
```

### 批处理

```typescript
export async function encryptMultiple(
  files: string[],
  outputDir: string,
  publicValue: number
): Promise<void> {
  const circuitFiles = await getCachedCircuitFiles();

  for (const file of files) {
    const outputFile = path.join(outputDir, path.basename(file) + ".enc");

    console.log(`\n处理中：${file}`);
    // ... 加密文件
  }
}
```

## 完整范例

完整源码可在以下位置获取：`examples/nodejs-integration/`

项目结构：

```
zkenc-node-example/
├── circuits/
│   ├── simple.circom
│   └── build/
│       ├── simple.r1cs
│       └── simple_js/
│           └── simple.wasm
├── src/
│   ├── index.ts          # CLI 界面
│   ├── circuit.ts        # 电路载入
│   ├── encrypt.ts        # 加密逻辑
│   └── decrypt.ts        # 解密逻辑
├── package.json
└── tsconfig.json
```
## 效能优化

### 1. 缓存电路文件

```typescript
// 载入一次，重复使用多次
const circuitFiles = await loadCircuitFiles();

for (const file of files) {
  await zkenc.encrypt(circuitFiles, inputs, message);
}
```

### 2. 对大型文件使用串流

```typescript
import { createReadStream, createWriteStream } from "fs";

async function encryptLargeFile(input: string, output: string) {
  const chunks: Buffer[] = [];
  const stream = createReadStream(input);

  for await (const chunk of stream) {
    chunks.push(chunk);
  }

  const message = Buffer.concat(chunks);
  // ... 加密
}
```

### 3. 平行处理

```typescript
await Promise.all(
  files.map((file) => encryptFile(file, `${file}.enc`, publicValue))
);
```

## 错误处理

```typescript
try {
  await decryptFile(input, output, pubVal, privVal);
} catch (error) {
  if (error instanceof Error) {
    if (error.message.includes("Invalid ciphertext")) {
      console.error("文件损坏或不是有效的密文");
    } else if (error.message.includes("constraint")) {
      console.error("见证不满足电路约束");
    } else {
      console.error("未预期的错误：", error.message);
    }
  }
  process.exit(1);
}
```

## 生产部署

### 1. 为生产环境建置

```bash
npm run build
```

### 2. 全局安装

```bash
npm install -g .
zkenc-example --help
```

### 3. 建立二进制文件（选用）

使用 `pkg`：

```bash
npm install -g pkg
pkg . --targets node18-linux-x64,node18-macos-x64,node18-win-x64
```

## 下一步

- **[React 整合 →](/docs/guides/react-integration)** - 建立 Web UI
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 结合 CLI 和 JS
- **[API 参考 →](/docs/api/zkenc-js)** - 探索所有函数
- **[实验场 →](/playground)** - 在浏览器中试用

## 疑难排解

**电路载入失败：**

- 检查文件路径是否正确
- 验证电路已成功编译
- 确保 R1CS 和 WASM 文件存在

**加密速度慢：**

- 第一次调用会初始化 WASM（约 20-50ms 的开销）
- 对多个操作缓存电路文件
- 考虑电路复杂度

**解密失败：**

- 验证见证满足约束
- 检查公开输入与加密时相符
- 确保密文未损坏
