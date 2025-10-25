---
sidebar_position: 1
---

# zkenc-js を始める

zkenc-js は、Node.js とブラウザ環境の両方で動作する、証拠暗号化のための JavaScript/TypeScript ライブラリです。

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

1. **コンパイルされた Circom 回路**（以下のファイル）：
   - `.r1cs` ファイル（回路制約）
   - `.wasm` ファイル（ウィットネスジェネレーター）
2. **回路ファイル**は Circom 回路をコンパイルすることで取得できます：

```bash
circom your_circuit.circom --r1cs --wasm
```

## クイック例

zkenc-js を使用した簡単な例です：

```typescript
import { zkenc, CircuitFiles } from "zkenc-js";

// 回路ファイルをロード
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

console.log("暗号化に成功しました！");
console.log("暗号文のサイズ:", ciphertext.length);

// 復号化するには、完全なウィットネス（プライベート入力を含む）が必要です
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // これが秘密のウィットネスです
};

// メッセージを復号化
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

const decryptedMessage = new TextDecoder().decode(decrypted);
console.log("復号化されたメッセージ:", decryptedMessage);
```

## 高レベル API vs 低レベル API

zkenc-js は 2 つの API を提供します：

### 高レベル API（推奨）

高レベル API（`encrypt` と `decrypt`）は、完全な証拠暗号化フローを処理します：

```typescript
// 暗号化：証拠暗号化と AES を組み合わせる
const { ciphertext, key } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);

// 復号化：鍵を復元してメッセージを復号化
const message = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

**使用例：**

- 一般的な暗号化/復号化のニーズ
- すべてを自動的に処理したい場合
- 個別の鍵管理が不要な場合

### 低レベル API（上級者向け）

低レベル API（`encap` と `decap`）は、きめ細かい制御を提供します：

```typescript
// カプセル化：回路に基づいて鍵を生成
const { ciphertext: witnessCiphertext, key } = await zkenc.encap(
  circuitFiles,
  publicInputs
);

// AES でメッセージを手動で暗号化
const encryptedMessage = await aesEncrypt(key, message);

// カプセル化解除：有効なウィットネスで鍵を復元
const recoveredKey = await zkenc.decap(
  circuitFiles,
  witnessCiphertext,
  fullInputs
);

// メッセージを手動で復号化
const decryptedMessage = await aesDecrypt(recoveredKey, encryptedMessage);
```

**使用例：**

- 研究と実験
- カスタム暗号化スキーム
- 個別の鍵管理が必要な場合

## 環境固有のセットアップ

### Node.js

zkenc-js は Node.js ですぐに動作します：

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1cs: await fs.readFile("circuit.r1cs"),
  wasm: await fs.readFile("circuit.wasm"),
};
```

### ブラウザ

ブラウザ環境では、ファイルを異なる方法でロードする必要があります：

```typescript
import { zkenc } from "zkenc-js";

// fetch を使用してファイルをロード
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

React アプリケーションについては、[React 統合ガイド →](/docs/guides/react-integration) をご覧ください。

## 一般的な回路パターン

zkenc で動作する典型的な Circom 回路構造は以下の通りです：

```circom
pragma circom 2.0.0;

template MyCircuit() {
    // 公開入力（暗号化者に知られている）
    signal input publicValue;

    // プライベート入力（ウィットネス、復号化に必要）
    signal input privateValue;

    // 出力（正しく計算されなければならない）
    signal output result;

    // 制約
    result <== publicValue + privateValue;
}

component main = MyCircuit();
```

**重要なポイント：**

- **公開入力**：暗号化時に知られており、暗号化条件の一部
- **プライベート入力**：復号化に必要な「ウィットネス」
- **制約**：満たされなければならない条件を定義

## 次のステップ

- **[API リファレンス →](/docs/api/zkenc-js)** - 完全な zkenc-js API を探索
- **[Node.js 統合 →](/docs/guides/nodejs-integration)** - ステップバイステップの Node.js ガイド
- **[React 統合 →](/docs/guides/react-integration)** - ステップバイステップの React ガイド
- **[プレイグラウンドを試す →](/playground)** - インタラクティブな数独の例

## トラブルシューティング

### モジュールが見つからないエラー

モジュール解決エラーが発生した場合は、`tsconfig.json` に以下を含めてください：

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

### パフォーマンスの考慮事項

- 回路のコンパイルは CPU を多用します
- 最初の暗号化/復号化は WASM の初期化により遅くなります
- 本番環境では回路ファイルのキャッシュを検討してください
- ブラウザアプリケーションではメインスレッドをブロックしないよう Web Worker を使用してください

## サポート

問題が発生した場合：

1. 詳細なドキュメントについては [API リファレンス](/docs/api/zkenc-js) を確認してください
2. 一般的なパターンについては [ガイド](/docs/guides/intro) を参照してください
3. [GitHub](https://github.com/flyinglimao/zkenc) で issue を開いてください
