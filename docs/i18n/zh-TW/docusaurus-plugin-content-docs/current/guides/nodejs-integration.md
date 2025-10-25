---
sidebar_position: 2
---

# Node.js 整合指南

本指南展示如何使用 zkenc-js 建立完整的 Node.js 應用程式進行見證加密。

## 我們要建立什麼

一個 Node.js CLI 工具：

- 使用數獨電路加密檔案
- 使用有效的數獨解答解密檔案
- 優雅地處理錯誤
- 提供清晰的命令列介面

## 前置需求

- Node.js 18 或更高版本
- 基本的 TypeScript 知識
- 已安裝 Circom（`circom --version`）

## 步驟 1：專案設定

建立新專案：

```bash
mkdir zkenc-node-example
cd zkenc-node-example
npm init -y
```

安裝相依套件：

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

## 步驟 2：準備電路檔案

建立簡單電路 `circuits/simple.circom`：

```circom
pragma circom 2.0.0;

template Simple() {
    signal input publicValue;
    signal input privateValue;
    signal output result;

    // 約束：publicValue + privateValue 必須等於 100
    result <== publicValue + privateValue;
    result === 100;
}

component main {public [publicValue]} = Simple();
```

編譯電路：

```bash
mkdir -p circuits/build
circom circuits/simple.circom --r1cs --wasm -o circuits/build
```

這會建立：

- `circuits/build/simple.r1cs`
- `circuits/build/simple_js/simple.wasm`

## 步驟 3：載入電路檔案

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

## 步驟 4：實作加密

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
  console.log("🔐 開始加密...");

  // 載入電路檔案
  console.log("📂 載入電路...");
  const circuitFiles = await loadCircuitFiles();

  // 讀取訊息檔案
  console.log("📄 讀取訊息...");
  const message = await fs.readFile(inputFile);
  console.log(`   訊息大小：${message.length} 位元組`);

  // 準備公開輸入
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
  console.log(`   加密耗時 ${duration}ms`);

  // 儲存密文
  await fs.writeFile(outputFile, ciphertext);
  console.log(`✅ 密文已儲存至：${outputFile}`);
  console.log(`   密文大小：${ciphertext.length} 位元組`);

  // 選擇性地儲存金鑰用於除錯
  const keyFile = outputFile + ".key";
  await fs.writeFile(keyFile, key);
  console.log(`🔑 金鑰已儲存至：${keyFile}（用於除錯）`);
}
```

## 步驟 5：實作解密

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
  console.log("🔓 開始解密...");

  // 載入電路檔案
  console.log("📂 載入電路...");
  const circuitFiles = await loadCircuitFiles();

  // 讀取密文
  console.log("📦 讀取密文...");
  const ciphertext = await fs.readFile(inputFile);
  console.log(`   密文大小：${ciphertext.length} 位元組`);

  // 準備完整輸入（公開 + 私密）
  const fullInputs = {
    publicValue: publicValue,
    privateValue: privateValue,
  };

  // 驗證輸入滿足約束
  if (publicValue + privateValue !== 100) {
    throw new Error(`無效的見證：${publicValue} + ${privateValue} ≠ 100`);
  }

  // 解密
  console.log("🔓 解密中...");
  const startTime = Date.now();

  try {
    const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

    const duration = Date.now() - startTime;
    console.log(`   解密耗時 ${duration}ms`);

    // 儲存解密訊息
    await fs.writeFile(outputFile, decrypted);
    console.log(`✅ 訊息已解密至：${outputFile}`);
    console.log(`   訊息大小：${decrypted.length} 位元組`);
  } catch (error) {
    console.error("❌ 解密失敗！");
    if (error instanceof Error) {
      console.error(`   錯誤：${error.message}`);
    }
    throw error;
  }
}
```

## 步驟 6：建立 CLI 介面

建立 `src/index.ts`：

```typescript
#!/usr/bin/env node

import { Command } from "commander";
import { encryptFile } from "./encrypt.js";
import { decryptFile } from "./decrypt.js";

const program = new Command();

program
  .name("zkenc-example")
  .description("使用 zkenc-js 的見證加密範例")
  .version("1.0.0");

program
  .command("encrypt")
  .description("加密檔案")
  .requiredOption("-i, --input <file>", "要加密的輸入檔案")
  .requiredOption("-o, --output <file>", "輸出加密檔案")
  .requiredOption("-p, --public <value>", "公開值（數字）", parseInt)
  .action(async (options) => {
    try {
      await encryptFile(options.input, options.output, options.public);
    } catch (error) {
      console.error("加密失敗：", error);
      process.exit(1);
    }
  });

program
  .command("decrypt")
  .description("解密檔案")
  .requiredOption("-i, --input <file>", "輸入加密檔案")
  .requiredOption("-o, --output <file>", "輸出解密檔案")
  .requiredOption("-p, --public <value>", "公開值（數字）", parseInt)
  .requiredOption("--private <value>", "私密值（數字）", parseInt)
  .action(async (options) => {
    try {
      await decryptFile(
        options.input,
        options.output,
        options.public,
        options.private
      );
    } catch (error) {
      console.error("解密失敗：", error);
      process.exit(1);
    }
  });

program.parse();
```

安裝 commander 用於 CLI：

```bash
npm install commander
```

## 步驟 7：測試應用程式

建立測試訊息：

```bash
echo "這是秘密訊息！" > message.txt
```

加密訊息：

```bash
npm run dev encrypt -- \
  --input message.txt \
  --output encrypted.bin \
  --public 30
```

輸出：

```
🔐 開始加密...
📂 載入電路...
📄 讀取訊息...
   訊息大小：26 位元組
🔒 加密中...
   加密耗時 45ms
✅ 密文已儲存至：encrypted.bin
   密文大小：1630 位元組
🔑 金鑰已儲存至：encrypted.bin.key（用於除錯）
```

解密訊息（使用正確的見證：30 + 70 = 100）：

```bash
npm run dev decrypt -- \
  --input encrypted.bin \
  --output decrypted.txt \
  --public 30 \
  --private 70
```

輸出：

```
🔓 開始解密...
📂 載入電路...
📦 讀取密文...
   密文大小：1630 位元組
🔓 解密中...
   解密耗時 156ms
✅ 訊息已解密至：decrypted.txt
   訊息大小：26 位元組
```

驗證：

```bash
diff message.txt decrypted.txt
echo "成功！"
```

嘗試使用錯誤的見證（會失敗）：

```bash
npm run dev decrypt -- \
  --input encrypted.bin \
  --output decrypted.txt \
  --public 30 \
  --private 50
```

輸出：

```
❌ 解密失敗！
   錯誤：無效的見證：30 + 50 ≠ 100
```

## 步驟 8：進階功能

### 電路檔案快取

建立 `src/circuit-cache.ts`：

```typescript
import { CircuitFiles } from "zkenc-js";
import { loadCircuitFiles } from "./circuit.js";

let circuitCache: CircuitFiles | null = null;

export async function getCachedCircuitFiles(): Promise<CircuitFiles> {
  if (!circuitCache) {
    console.log("💾 快取電路檔案...");
    circuitCache = await loadCircuitFiles();
  }
  return circuitCache;
}
```

### 進度報告

```typescript
export async function encryptFileWithProgress(
  inputFile: string,
  outputFile: string,
  publicValue: number
): Promise<void> {
  const steps = [
    "載入電路",
    "讀取訊息",
    "加密",
    "儲存密文",
  ];

  for (let i = 0; i < steps.length; i++) {
    console.log(`[${i + 1}/${steps.length}] ${steps[i]}...`);
    // ... 執行步驟
  }
}
```

### 批次處理

```typescript
export async function encryptMultiple(
  files: string[],
  outputDir: string,
  publicValue: number
): Promise<void> {
  const circuitFiles = await getCachedCircuitFiles();

  for (const file of files) {
    const outputFile = path.join(outputDir, path.basename(file) + ".enc");

    console.log(`\n處理中：${file}`);
    // ... 加密檔案
  }
}
```

## 完整範例

完整原始碼可在以下位置取得：`examples/nodejs-integration/`

專案結構：

```
zkenc-node-example/
├── circuits/
│   ├── simple.circom
│   └── build/
│       ├── simple.r1cs
│       └── simple_js/
│           └── simple.wasm
├── src/
│   ├── index.ts          # CLI 介面
│   ├── circuit.ts        # 電路載入
│   ├── encrypt.ts        # 加密邏輯
│   └── decrypt.ts        # 解密邏輯
├── package.json
└── tsconfig.json
```

## 效能優化

### 1. 快取電路檔案

```typescript
// 載入一次，重複使用多次
const circuitFiles = await loadCircuitFiles();

for (const file of files) {
  await zkenc.encrypt(circuitFiles, inputs, message);
}
```

### 2. 對大型檔案使用串流

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

### 3. 平行處理

```typescript
await Promise.all(
  files.map((file) => encryptFile(file, `${file}.enc`, publicValue))
);
```

## 錯誤處理

```typescript
try {
  await decryptFile(input, output, pubVal, privVal);
} catch (error) {
  if (error instanceof Error) {
    if (error.message.includes("Invalid ciphertext")) {
      console.error("檔案損毀或不是有效的密文");
    } else if (error.message.includes("constraint")) {
      console.error("見證不滿足電路約束");
    } else {
      console.error("未預期的錯誤：", error.message);
    }
  }
  process.exit(1);
}
```

## 生產部署

### 1. 為生產環境建置

```bash
npm run build
```

### 2. 全域安裝

```bash
npm install -g .
zkenc-example --help
```

### 3. 建立二進位檔（選用）

使用 `pkg`：

```bash
npm install -g pkg
pkg . --targets node18-linux-x64,node18-macos-x64,node18-win-x64
```

## 下一步

- **[React 整合 →](/docs/guides/react-integration)** - 建立 Web UI
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 結合 CLI 和 JS
- **[API 參考 →](/docs/api/zkenc-js)** - 探索所有函式
- **[實驗場 →](/playground)** - 在瀏覽器中試用

## 疑難排解

**電路載入失敗：**

- 檢查檔案路徑是否正確
- 驗證電路已成功編譯
- 確保 R1CS 和 WASM 檔案存在

**加密速度慢：**

- 第一次呼叫會初始化 WASM（約 20-50ms 的開銷）
- 對多個操作快取電路檔案
- 考慮電路複雜度

**解密失敗：**

- 驗證見證滿足約束
- 檢查公開輸入與加密時相符
- 確保密文未損毀
