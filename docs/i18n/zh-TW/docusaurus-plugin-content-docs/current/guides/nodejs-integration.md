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

建立 `src/index.ts` 來處理命令列參數並執行加密/解密操作。

## 步驟 7：測試應用程式

建立測試訊息：

```bash
echo "這是秘密訊息！" > message.txt
```

加密：

```bash
npm run dev encrypt -- message.txt encrypted.bin --public 42
```

解密：

```bash
npm run dev decrypt -- encrypted.bin decrypted.txt --public 42 --private 58
```

驗證：

```bash
cat decrypted.txt
# 輸出：這是秘密訊息！
```

## 最佳實踐

1. **錯誤處理**：使用 try-catch 捕獲和處理錯誤
2. **輸入驗證**：在加密/解密前驗證輸入
3. **效能**：快取載入的電路檔案以提升效能
4. **日誌記錄**：提供清晰的進度和錯誤訊息
5. **型別安全**：使用 TypeScript 確保型別安全

## 下一步

- **[React 整合 →](/docs/guides/react-integration)** - 建立 Web UI
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 結合 CLI 和 JS
- **[API 參考 →](/docs/api/zkenc-js)** - 完整 API 文件
