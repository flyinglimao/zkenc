---
sidebar_position: 4
---

# クロスツールワークフローガイド

zkenc-cli と zkenc-js を組み合わせて使用し、最大の柔軟性とパワーを実現する方法を学びます。

## なぜ両方のツールを使用するのか?

zkenc-cli と zkenc-js を組み合わせることで、強力なワークフローが可能になります:

- **サーバーで暗号化、ブラウザで復号化**
- **バッチ処理には CLI、UI には JS**
- **異なる環境で同じ暗号文**
- **各ツールの強みを活用**

## 互換性

zkenc-cli と zkenc-js は**完全に互換性があり**、同じ結合暗号文形式を使用します:

✅ CLI で暗号化したファイルは JS で復号化可能
✅ JS で暗号化したファイルは CLI で復号化可能
✅ 同じ回路ファイルが両方のツールで動作
✅ 両方のツールで同じ入力形式
✅ ファイル形式変換不要

**両方のツールは同じ結合形式を使用:**

```
[1バイトフラグ][4バイトウィットネスCT長][証拠暗号文]
[4バイト公開入力長(フラグ=1の場合)][公開入力JSON(フラグ=1の場合)]
[暗号化メッセージ]
```

## ワークフロー 1:CLI 暗号化 → JS 復号化

**ユースケース:** サーバーで機密ファイルを暗号化し、Web アプリケーションで復号化。

### ステップ 1:回路の準備(CLI)

```bash
# 回路をコンパイル
circom circuit.circom --r1cs --wasm -o build

# 必要なファイル:
# - build/circuit.r1cs (CLIとJS両方用)
# - build/circuit_js/circuit.wasm (CLIとJS両方用)
```

### ステップ 2:公開入力の作成(CLI)

`public_inputs.json`を作成:

```json
{
  "publicValue": "42"
}
```

### ステップ 3:CLI で暗号化

```bash
# 1ステップ暗号化(推奨)
zkenc encrypt \
  --circuit build/circuit.r1cs \
  --input public_inputs.json \
  --message secret.txt \
  --output encrypted.bin
```

出力された`encrypted.bin`は以下を含む結合暗号文です:

- 証拠暗号化暗号文
- 公開入力(デフォルトで埋め込み)
- AES 暗号化メッセージ

**ファイルサイズ:**

- `encrypted.bin`(結合) ≈ ウィットネス CT(1576 バイト) + 公開入力 + メッセージ + オーバーヘッド

### ステップ 4:JS で復号化

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

// 結合暗号文をロード
const ciphertext = await fs.readFile("encrypted.bin");

// 回路ファイルをロード
const circuitFiles = {
  r1csBuffer: await fs.readFile("build/circuit.r1cs"),
  wasmBuffer: await fs.readFile("build/circuit_js/circuit.wasm"),
};

// 完全な入力を準備(公開 + 秘密)
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // ウィットネス
};

// 1ステップで復号化
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

console.log(new TextDecoder().decode(decrypted));
// 出力: (secret.txtの内容)
```

**これだけです!** ファイル変換は不要です。

## ワークフロー 2:JS 暗号化 → CLI 復号化

**ユースケース:** ブラウザで暗号化、サーバーで処理/復号化。

### ステップ 1:JS で暗号化

```typescript
import { zkenc } from "zkenc-js";

const circuitFiles = {
  r1csBuffer: await fetch("/circuits/circuit.r1cs")
    .then((r) => r.arrayBuffer())
    .then((b) => new Uint8Array(b)),
  wasmBuffer: await fetch("/circuits/circuit.wasm")
    .then((r) => r.arrayBuffer())
    .then((b) => new Uint8Array(b)),
};

const publicInputs = { publicValue: "42" };
const message = new TextEncoder().encode("Secret from browser");

// 1ステップ暗号化
const { ciphertext } = await zkenc.encrypt(circuitFiles, publicInputs, message);

// 暗号文をダウンロード
const blob = new Blob([ciphertext]);
const url = URL.createObjectURL(blob);
const a = document.createElement("a");
a.href = url;
a.download = "encrypted.bin";
a.click();
```

`ciphertext`はすでに CLI が直接読み取れる結合形式になっています。

### ステップ 2:ウィットネスの生成(CLI)

完全な入力`full_inputs.json`を作成:

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

snarkjs を使用してウィットネスを生成:

```bash
snarkjs wtns calculate \
  build/circuit_js/circuit.wasm \
  full_inputs.json \
  witness.wtns
```

### ステップ 3:CLI で復号化

```bash
# 1ステップ復号化
zkenc decrypt \
  --circuit build/circuit.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt

cat decrypted.txt
# 出力: Secret from browser
```

**これだけです!** CLI は JS 暗号化ファイルを直接読み取ることができます。

## ワークフロー 3:ハイブリッド処理

**ユースケース:** バッチ操作には CLI、インタラクティブ UI には JS を使用。

### 例:写真暗号化サービス

**サーバー(CLI):**

```bash
#!/bin/bash
# encrypt-photos.sh

for photo in uploads/*.jpg; do
  echo "処理中 $photo..."

  # ユニークな公開入力を生成
  PUBLIC_VALUE=$(date +%s)
  echo "{\"timestamp\": \"$PUBLIC_VALUE\"}" > inputs.json

  # 1ステップで暗号化
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input inputs.json \
    --message "$photo" \
    --output "${photo}.enc"

  # メタデータを保存
  echo "$photo,$PUBLIC_VALUE" >> metadata.csv

  rm inputs.json
done
```

**クライアント(JS):**

```typescript
// 選択した写真を復号化
async function decryptPhoto(photoId: string, privateValue: number) {
  // 暗号化された写真を取得(結合形式)
  const response = await fetch(`/api/photos/${photoId}/encrypted`);
  const ciphertext = new Uint8Array(await response.arrayBuffer());

  // メタデータから公開値を取得
  const metadata = await fetch(`/api/photos/${photoId}/metadata`).then((r) =>
    r.json()
  );

  // 1ステップで復号化
  const fullInputs = {
    timestamp: metadata.timestamp,
    userSecret: privateValue,
  };

  const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

  // 写真を表示
  const blob = new Blob([decrypted], { type: "image/jpeg" });
  const url = URL.createObjectURL(blob);
  imageElement.src = url;
}
```

**注意:** 埋め込まれている場合、`getPublicInput()`を使用して暗号文から公開入力を抽出できます:

```typescript
import { getPublicInput } from "zkenc-js";

// 埋め込まれた公開入力を抽出
const publicInputs = getPublicInput(ciphertext);
console.log(publicInputs.timestamp); // メタデータを取得する必要はありません!
```

## ワークフロー 4:マルチプラットフォーム配布

**ユースケース:** 一度暗号化し、どのプラットフォームでも復号化。

### セットアップ

```bash
# 回路をコンパイル
circom puzzle.circom --r1cs --wasm -o dist

# 配布パッケージを作成
mkdir -p package/circuits
cp dist/puzzle.r1cs package/circuits/
cp dist/puzzle_js/puzzle.wasm package/circuits/
cp README.md package/
```

### 一度暗号化

```bash
# パズルを作成
cat > puzzle.json <<EOF
{
  "puzzle": ["5", "3", "0", "0", "7", "0", "0", "0", "0"]
}
EOF

# メッセージを暗号化(結合形式を作成)
zkenc encrypt \
  --circuit package/circuits/puzzle.r1cs \
  --input puzzle.json \
  --message treasure.txt \
  --output package/treasure.enc
```

### 配布

```
package/
├── circuits/
│   ├── puzzle.r1cs     # 回路ファイル
│   └── puzzle.wasm      # ウィットネス生成器
├── treasure.enc         # 結合暗号文(両方のツールで動作!)
└── README.md            # 手順
```

### ユーザーはどちらのツールでも復号化可能

**CLI ユーザー:**

```bash
# 解答ウィットネスを生成
cat > solution.json <<EOF
{
  "puzzle": ["5", "3", "0", ...],
  "solution": ["5", "3", "4", "6", "7", "8", "9", "1", "2", ...]
}
EOF

snarkjs wtns calculate puzzle.wasm solution.json solution.wtns

# 直接復号化
zkenc decrypt \
  --circuit puzzle.r1cs \
  --witness solution.wtns \
  --ciphertext treasure.enc \
  --output treasure.txt
```

**JS ユーザー:**

```typescript
// 同じ暗号化ファイルをロード
const ciphertext = await fetch('treasure.enc')
  .then(r => r.arrayBuffer())
  .then(b => new Uint8Array(b));

const solution = {
  puzzle: ["5", "3", "0", ...],
  solution: ["5", "3", "4", "6", "7", "8", "9", "1", "2", ...],
};

// 直接復号化
const treasure = await zkenc.decrypt(circuitFiles, ciphertext, solution);
```

**変換不要!** 両方のツールが同じファイル形式を読み取ります。

## 高度:低レベル API の使用

高度なユースケースでは、低レベルの`encap`/`decap`コマンドを個別に使用することもできます:

### CLI 低レベルコマンド

```bash
# ステップ1:証拠暗号文とキーを生成
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key key.bin

# ステップ2:任意のAESツールまたはカスタム実装で暗号化
# (key.binはAES-256に適した32バイトキー)

# ステップ3:復号化 - キーを回復
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness.ct \
  --key recovered_key.bin

# ステップ4:ステップ2で使用したのと同じ方法で復号化
```

### 低レベル API を使用するタイミング

- カスタム暗号化スキーム
- 既存の暗号化パイプラインとの統合
- 教育目的
- 暗号化/復号化を個別にデバッグ

**注意:** ほとんどのユースケースでは、高レベルの`encrypt`/`decrypt`コマンドをお勧めします。

## ベストプラクティス

1. **高レベル API を使用**: シンプルさと互換性のために`encrypt`/`decrypt`コマンドを使用
2. **回路ファイルの一貫性を保つ**: ツール間で同じコンパイル済み回路ファイルを使用
3. **公開入力を文書化**: どの入力が公開か秘密かを明確に文書化
4. **公開入力を埋め込む**: 自己完結型暗号文のためにデフォルトの動作(埋め込み)を使用
5. **回路のバージョン管理**: 互換性を確保するために回路のバージョンを追跡
6. **両方向をテスト**: 常に CLI→JS と JS→CLI の両方のワークフローをテスト

## トラブルシューティング

**復号化時に"Invalid ciphertext":**

- ファイルが有効な zkenc 暗号文であることを確認(`encrypt`コマンドで作成)
- 転送中にファイルが破損していないことを確認
- 正しい回路ファイルを使用していることを確認

**"Witness doesn't satisfy constraints":**

- 暗号化と復号化の間で公開入力が一致することを確認
- 秘密入力が回路の制約を満たすことを確認
- 同じ回路バージョンを使用していることを確認
- ウィットネスを検証するために`snarkjs wtns check`を使用

**ファイル形式の問題:**

- ファイルはすでに互換性があります - 変換不要!
- すべてのファイル操作でバイナリモードを使用
- バイナリファイルを破損する可能性のあるテキストエディタを避ける
- 必要に応じて`xxd`または`hexdump`を使用してファイルを検査

**公開入力の不一致:**

- CLI と JS は両方ともデフォルトで公開入力を埋め込みます
- JS で`getPublicInput()`を使用して暗号文から抽出
- CLI は復号化時に公開入力を表示(埋め込まれている場合)

## 次のステップ

- **[Node.js ガイド →](/docs/guides/nodejs-integration)** - CLI ツールを構築
- **[React ガイド →](/docs/guides/react-integration)** - WebUI を構築
- **[API リファレンス →](/docs/api/zkenc-js)** - 詳細なドキュメント
- **[プレイグラウンド →](/playground)** - ブラウザで試す
