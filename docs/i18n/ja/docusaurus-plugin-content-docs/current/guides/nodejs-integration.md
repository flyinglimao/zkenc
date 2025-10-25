---
sidebar_position: 2
---

# Node.js 統合ガイド

このガイドでは、zkenc-js を使用してウィットネス暗号化を実装する完全な Node.js アプリケーションの構築方法を説明します。

## 構築するもの

以下の機能を持つ Node.js CLI ツール:

- 数独回路を使用したファイルの暗号化
- 有効な数独の解答によるファイルの復号化
- 適切なエラーハンドリング
- クリーンなコマンドラインインターフェース

## 前提条件

- Node.js 18 以上
- TypeScript の基本知識
- Circom のインストール(`circom --version`)

## ステップ 1:プロジェクトセットアップ

新しいプロジェクトを作成:

```bash
mkdir zkenc-node-example
cd zkenc-node-example
npm init -y
```

依存関係をインストール:

```bash
npm install zkenc-js
npm install --save-dev typescript @types/node tsx
```

`tsconfig.json`を作成:

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

`package.json`を更新:

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

## ステップ 2:回路ファイルの準備

シンプルな回路`circuits/simple.circom`を作成:

```circom
pragma circom 2.0.0;

template Simple() {
    signal input publicValue;
    signal input privateValue;
    signal output result;

    // 制約: publicValue + privateValue は100でなければならない
    result <== publicValue + privateValue;
    result === 100;
}

component main {public [publicValue]} = Simple();
```

回路をコンパイル:

```bash
mkdir -p circuits/build
circom circuits/simple.circom --r1cs --wasm -o circuits/build
```

これにより以下が作成されます:

- `circuits/build/simple.r1cs`
- `circuits/build/simple_js/simple.wasm`

## ステップ 3:回路ファイルのロード

`src/circuit.ts`を作成:

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

## ステップ 4:暗号化の実装

`src/encrypt.ts`を作成:

```typescript
import fs from "fs/promises";
import { zkenc } from "zkenc-js";
import { loadCircuitFiles } from "./circuit.js";

export async function encryptFile(
  inputFile: string,
  outputFile: string,
  publicValue: number
): Promise<void> {
  console.log("🔐 暗号化を開始...");

  // 回路ファイルをロード
  console.log("📂 回路をロード中...");
  const circuitFiles = await loadCircuitFiles();

  // メッセージファイルを読み込み
  console.log("📄 メッセージを読み込み中...");
  const message = await fs.readFile(inputFile);
  console.log(`   メッセージサイズ: ${message.length} バイト`);

  // 公開入力を準備
  const publicInputs = {
    publicValue: publicValue,
  };

  // 暗号化
  console.log("🔒 暗号化中...");
  const startTime = Date.now();

  const { ciphertext, key } = await zkenc.encrypt(
    circuitFiles,
    publicInputs,
    message
  );

  const duration = Date.now() - startTime;
  console.log(`   暗号化にかかった時間: ${duration}ms`);

  // 暗号文を保存
  await fs.writeFile(outputFile, ciphertext);
  console.log(`✅ 暗号文を保存: ${outputFile}`);
  console.log(`   暗号文サイズ: ${ciphertext.length} バイト`);

  // デバッグ用にキーをオプションで保存
  const keyFile = outputFile + ".key";
  await fs.writeFile(keyFile, key);
  console.log(`🔑 キーを保存: ${keyFile} (デバッグ用)`);
}
```

## ステップ 5:復号化の実装

`src/decrypt.ts`を作成:

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
  console.log("🔓 復号化を開始...");

  // 回路ファイルをロード
  console.log("📂 回路をロード中...");
  const circuitFiles = await loadCircuitFiles();

  // 暗号文を読み込み
  console.log("📦 暗号文を読み込み中...");
  const ciphertext = await fs.readFile(inputFile);
  console.log(`   暗号文サイズ: ${ciphertext.length} バイト`);

  // 完全な入力を準備(公開 + 秘密)
  const fullInputs = {
    publicValue: publicValue,
    privateValue: privateValue,
  };

  // 入力が制約を満たすことを確認
  if (publicValue + privateValue !== 100) {
    throw new Error(
      `無効なウィットネス: ${publicValue} + ${privateValue} ≠ 100`
    );
  }

  // 復号化
  console.log("🔓 復号化中...");
  const startTime = Date.now();

  try {
    const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

    const duration = Date.now() - startTime;
    console.log(`   復号化にかかった時間: ${duration}ms`);

    // 復号化されたメッセージを保存
    await fs.writeFile(outputFile, decrypted);
    console.log(`✅ メッセージを復号化: ${outputFile}`);
    console.log(`   メッセージサイズ: ${decrypted.length} バイト`);
  } catch (error) {
    console.error("❌ 復号化に失敗しました!");
    if (error instanceof Error) {
      console.error(`   エラー: ${error.message}`);
    }
    throw error;
  }
}
```

## ステップ 6:CLI インターフェースの作成

`src/index.ts`を作成:

```typescript
#!/usr/bin/env node

import { Command } from "commander";
import { encryptFile } from "./encrypt.js";
import { decryptFile } from "./decrypt.js";

const program = new Command();

program
  .name("zkenc-example")
  .description("zkenc-jsを使用したウィットネス暗号化の例")
  .version("1.0.0");

program
  .command("encrypt")
  .description("ファイルを暗号化")
  .requiredOption("-i, --input <file>", "暗号化する入力ファイル")
  .requiredOption("-o, --output <file>", "暗号化された出力ファイル")
  .requiredOption("-p, --public <value>", "公開値(数値)", parseInt)
  .action(async (options) => {
    try {
      await encryptFile(options.input, options.output, options.public);
    } catch (error) {
      console.error("暗号化に失敗しました:", error);
      process.exit(1);
    }
  });

program
  .command("decrypt")
  .description("ファイルを復号化")
  .requiredOption("-i, --input <file>", "暗号化された入力ファイル")
  .requiredOption("-o, --output <file>", "復号化された出力ファイル")
  .requiredOption("-p, --public <value>", "公開値(数値)", parseInt)
  .requiredOption("--private <value>", "秘密値(数値)", parseInt)
  .action(async (options) => {
    try {
      await decryptFile(
        options.input,
        options.output,
        options.public,
        options.private
      );
    } catch (error) {
      console.error("復号化に失敗しました:", error);
      process.exit(1);
    }
  });

program.parse();
```

CLI 用に commander をインストール:

```bash
npm install commander
```

## ステップ 7:アプリケーションのテスト

テストメッセージを作成:

```bash
echo "This is a secret message!" > message.txt
```

メッセージを暗号化:

```bash
npm run dev encrypt -- \
  --input message.txt \
  --output encrypted.bin \
  --public 30
```

出力:

```
🔐 暗号化を開始...
📂 回路をロード中...
📄 メッセージを読み込み中...
   メッセージサイズ: 26 バイト
🔒 暗号化中...
   暗号化にかかった時間: 45ms
✅ 暗号文を保存: encrypted.bin
   暗号文サイズ: 1630 バイト
🔑 キーを保存: encrypted.bin.key (デバッグ用)
```

メッセージを復号化(正しいウィットネスを使用: 30 + 70 = 100):

```bash
npm run dev decrypt -- \
  --input encrypted.bin \
  --output decrypted.txt \
  --public 30 \
  --private 70
```

出力:

```
🔓 復号化を開始...
📂 回路をロード中...
📦 暗号文を読み込み中...
   暗号文サイズ: 1630 バイト
🔓 復号化中...
   復号化にかかった時間: 156ms
✅ メッセージを復号化: decrypted.txt
   メッセージサイズ: 26 バイト
```

確認:

```bash
diff message.txt decrypted.txt
echo "Success!"
```

間違ったウィットネスで試す(失敗します):

```bash
npm run dev decrypt -- \
  --input encrypted.bin \
  --output decrypted.txt \
  --public 30 \
  --private 50
```

出力:

```
❌ 復号化に失敗しました!
   エラー: 無効なウィットネス: 30 + 50 ≠ 100
```

## ステップ 8:高度な機能

### 回路ファイルのキャッシング

`src/circuit-cache.ts`を作成:

```typescript
import { CircuitFiles } from "zkenc-js";
import { loadCircuitFiles } from "./circuit.js";

let circuitCache: CircuitFiles | null = null;

export async function getCachedCircuitFiles(): Promise<CircuitFiles> {
  if (!circuitCache) {
    console.log("💾 回路ファイルをキャッシュ中...");
    circuitCache = await loadCircuitFiles();
  }
  return circuitCache;
}
```

### 進行状況レポート

```typescript
export async function encryptFileWithProgress(
  inputFile: string,
  outputFile: string,
  publicValue: number
): Promise<void> {
  const steps = [
    "回路をロード中",
    "メッセージを読み込み中",
    "暗号化中",
    "暗号文を保存中",
  ];

  for (let i = 0; i < steps.length; i++) {
    console.log(`[${i + 1}/${steps.length}] ${steps[i]}...`);
    // ... ステップを実行
  }
}
```

### バッチ処理

```typescript
export async function encryptMultiple(
  files: string[],
  outputDir: string,
  publicValue: number
): Promise<void> {
  const circuitFiles = await getCachedCircuitFiles();

  for (const file of files) {
    const outputFile = path.join(outputDir, path.basename(file) + ".enc");

    console.log(`\n処理中: ${file}`);
    // ... ファイルを暗号化
  }
}
```

## 完全な例

完全なソースコードは以下で入手できます: `examples/nodejs-integration/`

プロジェクト構造:

```
zkenc-node-example/
├── circuits/
│   ├── simple.circom
│   └── build/
│       ├── simple.r1cs
│       └── simple_js/
│           └── simple.wasm
├── src/
│   ├── index.ts          # CLIインターフェース
│   ├── circuit.ts        # 回路のロード
│   ├── encrypt.ts        # 暗号化ロジック
│   └── decrypt.ts        # 復号化ロジック
├── package.json
└── tsconfig.json
```

## パフォーマンスの最適化

### 1. 回路ファイルのキャッシュ

```typescript
// 一度ロードして何度も再利用
const circuitFiles = await loadCircuitFiles();

for (const file of files) {
  await zkenc.encrypt(circuitFiles, inputs, message);
}
```

### 2. 大きなファイルにはストリームを使用

```typescript
import { createReadStream, createWriteStream } from "fs";

async function encryptLargeFile(input: string, output: string) {
  const chunks: Buffer[] = [];
  const stream = createReadStream(input);

  for await (const chunk of stream) {
    chunks.push(chunk);
  }

  const message = Buffer.concat(chunks);
  // ... 暗号化
}
```

### 3. 並列処理

```typescript
await Promise.all(
  files.map((file) => encryptFile(file, `${file}.enc`, publicValue))
);
```

## エラーハンドリング

```typescript
try {
  await decryptFile(input, output, pubVal, privVal);
} catch (error) {
  if (error instanceof Error) {
    if (error.message.includes("Invalid ciphertext")) {
      console.error("ファイルが破損しているか、有効な暗号文ではありません");
    } else if (error.message.includes("constraint")) {
      console.error("ウィットネスが回路の制約を満たしていません");
    } else {
      console.error("予期しないエラー:", error.message);
    }
  }
  process.exit(1);
}
```

## 本番環境へのデプロイ

### 1. 本番環境用にビルド

```bash
npm run build
```

### 2. グローバルにインストール

```bash
npm install -g .
zkenc-example --help
```

### 3. バイナリの作成(オプション)

`pkg`を使用:

```bash
npm install -g pkg
pkg . --targets node18-linux-x64,node18-macos-x64,node18-win-x64
```

## 次のステップ

- **[React 統合 →](/docs/guides/react-integration)** - WebUI を構築
- **[クロスツールワークフロー →](/docs/guides/cross-tool-workflow)** - CLI と JS を組み合わせる
- **[API リファレンス →](/docs/api/zkenc-js)** - すべての関数を探索
- **[プレイグラウンド →](/playground)** - ブラウザで試す

## トラブルシューティング

**回路のロードに失敗する:**

- ファイルパスが正しいことを確認
- 回路が正常にコンパイルされたことを確認
- R1CS と WASM ファイルが存在することを確認

**暗号化が遅い:**

- 最初の呼び出しは WASM を初期化します(約 20-50ms のオーバーヘッド)
- 複数の操作のために回路ファイルをキャッシュ
- 回路の複雑さを考慮

**復号化に失敗する:**

- ウィットネスが制約を満たすことを確認
- 公開入力が暗号化時と一致することを確認
- 暗号文が破損していないことを確認
