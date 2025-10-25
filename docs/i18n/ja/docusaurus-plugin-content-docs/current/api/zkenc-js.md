---
sidebar_position: 3
---

# zkenc-js API リファレンス

zkenc-js の完全な API リファレンス、ウィットネス暗号化のための JavaScript/TypeScript ライブラリです。

## インストール

```bash
npm install zkenc-js
```

## インポート

```typescript
import { zkenc, CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";
```

## 型

### `CircuitFiles`

ウィットネス暗号化操作に必要な回路ファイル。

```typescript
interface CircuitFiles {
  /** R1CS回路ファイルバッファ(.r1cs) */
  r1csBuffer: Uint8Array;
  /** ウィットネス計算用のCircom WASMファイルバッファ(.wasm) */
  wasmBuffer: Uint8Array;
}
```

**例:**

```typescript
import fs from "fs/promises";

const circuitFiles: CircuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};
```

### `EncapResult`

暗号文とキーを含むカプセル化の結果。

```typescript
interface EncapResult {
  /** 有効なウィットネスで復号化できる暗号文(1576バイト) */
  ciphertext: Uint8Array;
  /** 対称暗号化キー(32バイト、AES-256) */
  key: Uint8Array;
}
```

### `EncryptResult`

結合暗号文とキーを含む暗号化の結果。

```typescript
interface EncryptResult {
  /** 結合暗号文: [4B長][ウィットネスCT][AES CT] */
  ciphertext: Uint8Array;
  /** 上級ユーザー向けの暗号化キー(32バイト) */
  key: Uint8Array;
}
```

## 高レベル API

高レベル API は、単一の関数呼び出しで完全なウィットネス暗号化機能を提供します。

### `encrypt()`

ウィットネス暗号化を使用してメッセージを暗号化します。キー生成と AES-256-GCM 暗号化を組み合わせます。

```typescript
async function encrypt(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>,
  message: Uint8Array
): Promise<EncryptResult>;
```

**パラメータ:**

- `circuitFiles` - 回路ファイル(R1CS と WASM)
- `publicInputs` - JSON オブジェクトとしての公開入力(公開シグナルのみ)
- `message` - Uint8Array として暗号化するメッセージ

**戻り値:**

- `Promise<EncryptResult>` - 結合暗号文と暗号化キー

**暗号文形式:**

```
[4バイト: ウィットネスCT長][ウィットネス暗号文][AES暗号化メッセージ]
│                          │                     │
│                          │                     └─ AES-256-GCM暗号化
│                          └─ ウィットネス暗号化(1576バイト)
└─ ビッグエンディアン長(常に1576)
```

**例:**

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

console.log("暗号文サイズ:", ciphertext.length);
// 暗号文サイズ: 1608バイト(4 + 1576 + 28)
```

**パフォーマンス:**

- 最初の呼び出し: 約 50-100ms(WASM 初期化)
- 以降の呼び出し: 約 30-50ms

### `decrypt()`

ウィットネス復号化を使用してメッセージを復号化します。キーを回復してメッセージを復号化します。

```typescript
async function decrypt(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array>;
```

**パラメータ:**

- `circuitFiles` - 回路ファイル(R1CS と WASM)
- `ciphertext` - `encrypt()`からの結合暗号文
- `inputs` - JSON オブジェクトとしての完全な入力(公開 + 秘密シグナル)

**戻り値:**

- `Promise<Uint8Array>` - 復号化されたメッセージ

**例外:**

- `Error` - 暗号文が無効またはウィットネスが回路を満たさない場合

**例:**

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
console.log("復号化:", message);
```

**パフォーマンス:**

- 最初の呼び出し: 約 150-200ms(WASM 初期化 + ウィットネス計算)
- 以降の呼び出し: 約 100-150ms

## 低レベル API

低レベル API は、ウィットネス暗号化プロセスの細かい制御を提供します。研究やカスタム暗号化スキームに使用します。

### `encap()`

ウィットネス暗号化を使用して暗号化キーを生成(カプセル化)。

```typescript
async function encap(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>
): Promise<EncapResult>;
```

**パラメータ:**

- `circuitFiles` - 回路ファイル(R1CS と WASM)
- `publicInputs` - JSON オブジェクトとしての公開入力

**戻り値:**

- `Promise<EncapResult>` - ウィットネス暗号文(1576 バイト)とキー(32 バイト)

**例:**

```typescript
const { ciphertext: witnessCiphertext, key } = await zkenc.encap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"),
  },
  { publicValue: 42 }
);

// 独自の暗号化にキーを使用
const encryptedMessage = await customEncrypt(key, message);
```

**ユースケース:**

- カスタム暗号化スキーム
- 個別のキー管理
- 研究と実験

### `decap()`

有効なウィットネスを使用して暗号化キーを回復(デカプセル化)。

```typescript
async function decap(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array>;
```

**パラメータ:**

- `circuitFiles` - 回路ファイル(R1CS と WASM)
- `ciphertext` - `encap()`からのウィットネス暗号文(1576 バイト)
- `inputs` - JSON オブジェクトとしての完全な入力(回路を満たす必要)

**戻り値:**

- `Promise<Uint8Array>` - 回復された暗号化キー(32 バイト)

**例外:**

- `Error` - ウィットネスが回路の制約を満たさない場合

**例:**

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

// 回復されたキーを使用
const decryptedMessage = await customDecrypt(recoveredKey, encryptedMessage);
```

## 使用パターン

### 基本的なテキスト暗号化

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};

// 暗号化
const message = new TextEncoder().encode("Hello, World!");
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  { publicInput: 42 },
  message
);

// 復号化
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, {
  publicInput: 42,
  privateInput: 123,
});

console.log(new TextDecoder().decode(decrypted));
```

### バイナリデータ暗号化

```typescript
// ファイルを暗号化
const fileData = await fs.readFile("document.pdf");
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  fileData
);

await fs.writeFile("document.pdf.enc", ciphertext);

// ファイルを復号化
const encryptedData = await fs.readFile("document.pdf.enc");
const decryptedData = await zkenc.decrypt(
  circuitFiles,
  encryptedData,
  fullInputs
);

await fs.writeFile("document_decrypted.pdf", decryptedData);
```

### 回路ファイルを一度保存

```typescript
// 一度ロード
const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};

// 複数の操作で再利用
const results = await Promise.all([
  zkenc.encrypt(circuitFiles, inputs1, message1),
  zkenc.encrypt(circuitFiles, inputs2, message2),
  zkenc.encrypt(circuitFiles, inputs3, message3),
]);
```

### 高度:カスタム暗号化を使用した低レベル

```typescript
// キーを生成
const { ciphertext: witnessCt, key } = await zkenc.encap(
  circuitFiles,
  publicInputs
);

// 独自の暗号化を使用
import { customEncrypt, customDecrypt } from "./my-crypto";
const encrypted = await customEncrypt(key, message);

// 両方の部分を保存
await fs.writeFile("witness.ct", witnessCt);
await fs.writeFile("message.ct", encrypted);

// 後で: 復号化
const witnessCt = await fs.readFile("witness.ct");
const encrypted = await fs.readFile("message.ct");

const recoveredKey = await zkenc.decap(circuitFiles, witnessCt, fullInputs);
const decrypted = await customDecrypt(recoveredKey, encrypted);
```

## 入力形式

### 公開入力(暗号化用)

公開としてマークされているシグナル、または制約の一部ではあるがウィットネスの一部ではないシグナルのみを含めます:

```typescript
const publicInputs = {
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,
};
```

### 完全な入力(復号化用)

すべてのシグナル(公開 + 秘密)を含めます:

```typescript
const fullInputs = {
  // 公開入力
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,

  // 秘密ウィットネス
  solutionGrid: [5, 3, 4, 6, 7, 8, 9, 1, 2 /* ... */],
};
```

### 配列シグナル

配列がサポートされています:

```typescript
const inputs = {
  singleValue: 42,
  arrayValue: [1, 2, 3, 4, 5],
  matrix: [
    [1, 2],
    [3, 4],
  ].flat(), // 2D配列をフラット化
};
```

## エラーハンドリング

```typescript
try {
  const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, inputs);
  console.log("成功:", new TextDecoder().decode(decrypted));
} catch (error) {
  if (error.message.includes("Invalid ciphertext")) {
    console.error("暗号文が破損しているか無効です");
  } else if (error.message.includes("constraint")) {
    console.error("ウィットネスが回路の制約を満たしていません");
  } else {
    console.error("復号化に失敗しました:", error.message);
  }
}
```

## パフォーマンスの考慮事項

### WASM 初期化

任意の関数への最初の呼び出しは WASM モジュールを初期化します(約 20-50ms)。以降の呼び出しはより高速です。

### 回路の複雑さ

パフォーマンスは回路サイズに比例します:

- 小規模回路(< 1000 制約): < 50ms
- 中規模回路(1000-10000 制約): 50-200ms
- 大規模回路(> 10000 制約): 200ms 以上

### キャッシング

回路ファイルをメモリにキャッシュ:

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

### ブラウザの最適化

ノンブロッキング操作に Web Workers を使用:

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

## ブラウザ vs Node.js

### Node.js

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};
```

### ブラウザ

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

## TypeScript サポート

zkenc-js は TypeScript で書かれており、完全な型定義を提供します:

```typescript
import type { CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";

// 型安全な使用
async function encryptMessage(
  files: CircuitFiles,
  inputs: Record<string, any>,
  msg: string
): Promise<EncryptResult> {
  return zkenc.encrypt(files, inputs, new TextEncoder().encode(msg));
}
```

## 互換性

- **Node.js**: >= 18.0.0
- **ブラウザ**: WebAssembly をサポートするモダンブラウザ
  - Chrome/Edge >= 90
  - Firefox >= 88
  - Safari >= 15
- **バンドラー**: Vite、Webpack 5+、Rollup
- **フレームワーク**: React、Vue、Svelte、Next.js

## 次のステップ

- **[入門 →](/docs/getting-started/zkenc-js)** - インストールと基本的な使用法
- **[Node.js ガイド →](/docs/guides/nodejs-integration)** - 完全な Node.js 統合
- **[React ガイド →](/docs/guides/react-integration)** - 完全な React 統合
- **[プレイグラウンド →](/playground)** - ブラウザで試す
