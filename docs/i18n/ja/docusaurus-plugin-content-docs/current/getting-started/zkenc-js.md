---
sidebar_position: 1
---

# zkenc-js 入門

zkenc-js は、Node.js とブラウザ環境の両方で動作する Witness 暗号用の JavaScript/TypeScript ライブラリです。

## インストール

お好みのパッケージマネージャーを使用して zkenc-js をインストールします：

```bash
npm install zkenc-js
# または
yarn add zkenc-js
# または
pnpm add zkenc-js
```

## 前提条件

zkenc-js を使用する前に、以下が必要です：

1. **コンパイル済みの Circom 回路**（以下のファイルを含む）：
   - `.r1cs` ファイル（回路制約）
   - `.wasm` ファイル（Witness ジェネレータ）
2. **回路ファイル**は Circom 回路をコンパイルすることで取得できます：

```bash
circom your_circuit.circom --r1cs --wasm
```

## クイック例

zkenc-js を使用する簡単な例：

```typescript
import { zkenc, CircuitFiles } from "zkenc-js";

// 回路ファイルを読み込む
const circuitFiles: CircuitFiles = {
  r1cs: await fs.readFile("circuit.r1cs"),
  wasm: await fs.readFile("circuit.wasm"),
};

// 回路の公開入力を定義
const publicInputs = {
  publicValue: 42,
};

// 暗号化するメッセージ
const message = new TextEncoder().encode("Hello, zkenc!");

// メッセージを暗号化
const { ciphertext, key } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);

console.log("暗号化成功！");
console.log("暗号文サイズ：", ciphertext.length);

// 復号するには、完全な Witness（秘密入力を含む）が必要
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // これが秘密の Witness
};

// メッセージを復号
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

const decryptedMessage = new TextDecoder().decode(decrypted);
console.log("復号されたメッセージ：", decryptedMessage);
```

## 高レベル API vs 低レベル API

zkenc-js は 2 つの API を提供します：

### 高レベル API（推奨）

高レベル API（`encrypt` と `decrypt`）は完全な Witness 暗号フローを処理します：

```typescript
// 暗号化：Witness 暗号と AES を組み合わせる
const { ciphertext, key } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);

// 復号：鍵を復元してメッセージを復号
const message = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

**ユースケース：**

- 一般的な暗号化/復号のニーズ
- すべてを自動的に処理したい場合
- 個別の鍵管理が不要な場合

### 低レベル API（高度）

低レベル API（`encap` と `decap`）は細かい制御を提供します：

```typescript
// カプセル化：回路に基づいて鍵を生成
const { ciphertext: witnessCiphertext, key } = await zkenc.encap(
  circuitFiles,
  publicInputs
);

// AES でメッセージを手動で暗号化
const encryptedMessage = await aesEncrypt(key, message);

// デカプセル化：有効な Witness で鍵を復元
const recoveredKey = await zkenc.decap(
  circuitFiles,
  witnessCiphertext,
  fullInputs
);

// メッセージを手動で復号
const decryptedMessage = await aesDecrypt(recoveredKey, encryptedMessage);
```

**ユースケース：**

- 研究と実験
- カスタム暗号化スキーム
- 個別の鍵管理が必要な場合

## 環境固有のセットアップ

### Node.js

zkenc-js は Node.js ですぐに使えます：

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1cs: await fs.readFile("circuit.r1cs"),
  wasm: await fs.readFile("circuit.wasm"),
};
```

### ブラウザ

ブラウザ環境では、ファイルを異なる方法で読み込む必要があります：

```typescript
import { zkenc } from "zkenc-js";

// fetch を使用してファイルを読み込む
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

React アプリケーションについては、[React 統合ガイド →](/docs/guides/react-integration)をご覧ください

## 一般的な回路パターン

zkenc と連携する典型的な Circom 回路構造：

```circom
pragma circom 2.0.0;

template MyCircuit() {
    // 公開入力（暗号化者が知っている）
    signal input publicValue;

    // 秘密入力（Witness、復号に必要）
    signal input privateValue;

    // 出力（正しく計算される必要がある）
    signal output result;

    // 制約
    result <== publicValue + privateValue;
}

component main = MyCircuit();
```

**ポイント：**

- **公開入力**：暗号化時に既知、暗号化条件の一部
- **秘密入力**：復号に必要な「Witness」
- **制約**：満たす必要がある条件を定義

## 次のステップ

- **[API リファレンス →](/docs/api/zkenc-js)** - 完全な zkenc-js API を探索
- **[Node.js 統合 →](/docs/guides/nodejs-integration)** - Node.js のステップバイステップガイド
- **[React 統合 →](/docs/guides/react-integration)** - React のステップバイステップガイド
- **[プレイグラウンドを試す →](/playground)** - インタラクティブな数独の例

## トラブルシューティング

### モジュールが見つからないエラー

モジュール解決エラーが発生した場合、`tsconfig.json` に以下が含まれていることを確認してください：

```json
{
  "compilerOptions": {
    "moduleResolution": "bundler",
    "esModuleInterop": true
  }
}
```

### ブラウザでの WebAssembly エラー

バンドラーが WASM ファイルを処理するように設定されていることを確認してください。Vite の場合：

```javascript
// vite.config.js
export default {
  optimizeDeps: {
    exclude: ["zkenc-js"],
  },
};
```

### パフォーマンスに関する考慮事項

- 回路のコンパイルは CPU 集約的
- WASM の初期化のため、最初の暗号化/復号は遅い
- 本番環境では回路ファイルのキャッシュを検討
- ブラウザアプリケーションでは、メインスレッドのブロックを避けるため Web Workers を使用

## サポート

問題が発生した場合：

1. 詳細なドキュメントについては [API リファレンス](/docs/api/zkenc-js)を確認
2. 一般的なパターンについては[ガイド](/docs/guides/intro)を参照
3. [GitHub](https://github.com/flyinglimao/zkenc) で issue を開く
