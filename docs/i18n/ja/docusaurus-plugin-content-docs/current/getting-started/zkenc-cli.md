---
sidebar_position: 2
---

# zkenc-cli を始める

zkenc-cli は、ウィットネス暗号化操作のためのコマンドラインツールです。Circom 回路を使用してメッセージを暗号化および復号化するためのシンプルなインターフェースを提供します。

## インストール

### ソースから

リポジトリをクローンしてソースからビルドします：

```bash
git clone https://github.com/flyinglimao/zkenc.git
cd zkenc/packages/zkenc-cli
cargo install --path .
```

## 前提条件

zkenc-cli を使用する前に、以下が必要です：

1. **コンパイルされた Circom 回路**（以下を含む）：

   - `.r1cs` ファイル（回路制約）
   - `.wasm` ファイル（ウィットネスジェネレーター）

2. **入力ファイル**（JSON 形式）

## クイックスタート

### 1. シンプルな回路を作成

`example.circom` ファイルを作成します：

```circom
pragma circom 2.0.0;

template Example() {
    signal input publicValue;
    signal input privateValue;
    signal output result;

    result <== publicValue + privateValue;
}

component main = Example();
```

### 2. 回路をコンパイル

```bash
circom example.circom --r1cs --wasm --output circuit_output
```

これにより以下が作成されます：

- `circuit_output/example.r1cs`
- `circuit_output/example_js/example.wasm`

### 3. 入力ファイルを準備

`public_inputs.json` を作成（暗号化時に既知）：

```json
{
  "publicValue": "42"
}
```

`full_inputs.json` を作成（復号化に必要）：

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

### 4. 秘密メッセージを暗号化

`encrypt` を使用してウィットネス暗号化を 1 ステップで実行します：

```bash
echo "Hello, zkenc!" > message.txt
zkenc encrypt \
  --circuit circuit_output/example.r1cs \
  --input public_inputs.json \
  --message message.txt \
  --output encrypted.bin
```

このコマンドは：

- 公開入力からウィットネス暗号化鍵を生成（encap）
- AES-256-GCM でメッセージを暗号化
- すべてを 1 つの暗号文ファイルに結合
- デフォルトで公開入力を暗号文に埋め込む

出力：

```
🔐 ステップ 1: Encap を実行中...
📂 R1CS 回路をロード中...
   - 制約: 2
   - 公開入力: 1
   - ワイヤー: 4

📋 JSON から公開入力をロード中...
   - 1 個のフィールド要素を解析

   ✅ ウィットネス暗号文を生成（123 バイト）

🔒 ステップ 2: メッセージを暗号化中...
   - メッセージサイズ: 14 バイト
   ✅ メッセージを暗号化（42 バイト）

📦 ステップ 3: 結合暗号文を作成中...
   ✅ 結合暗号文を保存（218 バイト）

✨ 暗号化完了！公開入力が暗号文に埋め込まれています。
```

### 5. ウィットネスファイルを生成

復号化する前に、受信者は有効な解答を持っていることを証明するウィットネスを生成する必要があります：

```bash
snarkjs wtns calculate \
  circuit_output/example_js/example.wasm \
  full_inputs.json \
  witness.wtns
```

### 6. メッセージを復号化

`decrypt` を使用してメッセージを復元し復号化します：

```bash
zkenc decrypt \
  --circuit circuit_output/example.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

このコマンドは：

- 結合暗号文を解析
- ウィットネスを使用して鍵を復元（decap）
- AES-256-GCM でメッセージを復号化

出力：

```
📦 ステップ 1: 結合暗号文を解析中...
   - フラグ: 1
   - ウィットネス暗号文: 123 バイト
   - 公開入力: {"publicValue":"42"}
   - 暗号化されたメッセージ: 42 バイト

🔓 ステップ 2: Decap を実行中...
📂 R1CS 回路をロード中...
   - 制約: 2
   - 公開入力: 1

📋 snarkjs からウィットネスをロード中...
   - ウィットネス要素: 4

   ✅ ウィットネスから鍵を復元

🔓 ステップ 3: メッセージを復号化中...
   ✅ 復号化されたメッセージを保存（14 バイト）

✨ 復号化完了！
```

結果を確認：

```bash
cat decrypted.txt
# 出力: Hello, zkenc!
```

## コマンドリファレンス

### `zkenc encap`

回路と公開入力から暗号文と暗号化鍵を生成します。

```bash
zkenc encap \
  --circuit <R1CS_FILE> \
  --input <JSON_FILE> \
  --ciphertext <OUTPUT_CT> \
  --key <OUTPUT_KEY>
```

**引数：**

- `--circuit <FILE>` - R1CS 回路ファイルへのパス（Circom の `.r1cs`）
- `--input <FILE>` - 公開入力を含む JSON ファイルへのパス
- `--ciphertext <FILE>` - 暗号文の出力パス
- `--key <FILE>` - 暗号化鍵の出力パス

**例：**

```bash
zkenc encap \
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --ciphertext ciphertext.bin \
  --key key.bin
```

---

### `zkenc decap`

有効なウィットネスと暗号文を使用して暗号化鍵を復元します。

```bash
zkenc decap \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --key <OUTPUT_KEY>
```

**引数：**

- `--circuit <FILE>` - R1CS 回路ファイルへのパス
- `--witness <FILE>` - ウィットネスファイルへのパス（snarkjs の `.wtns`）
- `--ciphertext <FILE>` - 暗号文ファイルへのパス
- `--key <FILE>` - 復元された鍵の出力パス

**例：**

```bash
zkenc decap \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext ciphertext.bin \
  --key recovered_key.bin
```

---

### `zkenc encrypt`

ウィットネス暗号化を使用してメッセージを暗号化します（高レベル、1 ステップ操作）。

```bash
zkenc encrypt \
  --circuit <R1CS_FILE> \
  --input <JSON_FILE> \
  --message <MESSAGE_FILE> \
  --output <OUTPUT_FILE> \
  [--no-public-input]
```

**引数：**

- `--circuit <FILE>` - R1CS 回路ファイルへのパス（Circom の `.r1cs`）
- `--input <FILE>` - 公開入力を含む JSON ファイルへのパス
- `--message <FILE>` - 平文メッセージファイルへのパス
- `--output <FILE>` - 結合暗号文の出力パス
- `--no-public-input` - 暗号文に公開入力を埋め込まない（オプション）

**動作：**

このコマンドは encap と AES 暗号化を 1 ステップに統合します：

1. 公開入力からウィットネス暗号化鍵を生成
2. AES-256-GCM でメッセージを暗号化
3. フォーマット付きの結合暗号文を作成：`[flag][witnessLen][witnessCT][publicLen][publicInput][encryptedMsg]`

**例：**

```bash
zkenc encrypt \
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --message secret.txt \
  --output encrypted.bin
```

**互換性：** 出力は zkenc-js の `decrypt()` 関数と完全に互換性があります。

---

### `zkenc decrypt`

ウィットネス復号化を使用してメッセージを復号化します（高レベル、1 ステップ操作）。

```bash
zkenc decrypt \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --output <OUTPUT_FILE>
```

**引数：**

- `--circuit <FILE>` - R1CS 回路ファイルへのパス
- `--witness <FILE>` - ウィットネスファイルへのパス（snarkjs の `.wtns`）
- `--ciphertext <FILE>` - 結合暗号文ファイルへのパス
- `--output <FILE>` - 復号化されたメッセージの出力パス

**動作：**

このコマンドは decap と AES 復号化を 1 ステップに統合します：

1. 結合暗号文を解析
2. ウィットネスを使用して鍵を復元
3. AES-256-GCM でメッセージを復号化

**例：**

```bash
zkenc decrypt \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

**互換性：** zkenc-js の `encrypt()` 関数で作成されたファイルを復号化できます。

---

## ワークフローの理解

zkenc-cli は 2 つのレベルの API を提供します：

### 高レベル API（推奨）

シンプルな 2 ステッププロセス：

1. **`encrypt`** - encap + AES 暗号化を 1 つのコマンドに統合

   - 入力：回路、公開入力、メッセージ
   - 出力：結合暗号文（zkenc-js と互換性あり）

2. **`decrypt`** - decap + AES 復号化を 1 つのコマンドに統合
   - 入力：回路、ウィットネス、結合暗号文
   - 出力：復号化されたメッセージ

**利点：**

- シンプルなワークフロー（2 ステップ vs 4 ステップ）
- 管理する暗号文ファイルは 1 つだけ
- zkenc-js との完全な互換性
- 公開入力を暗号文に埋め込み可能

### 低レベル API（上級者向け）

きめ細かい制御のための 4 ステッププロセス：

1. **`encap`** - 公開入力からウィットネス暗号化された暗号文と鍵を生成
2. メッセージを個別に暗号化（任意の AES ツールを使用）
3. **`decap`** - 有効なウィットネスを使用して鍵を復元
4. メッセージを個別に復号化（任意の AES ツールを使用）

**使用例：**

- カスタム暗号化スキーム
- 複数のメッセージで鍵を再利用
- 既存の暗号化パイプラインとの統合
- プロトコルを理解するための教育目的

**注意：** ほとんどの使用例では、互換性を確保しワークフローを簡素化するため、高レベル API が推奨されます。

## 入力ファイル形式

### R1CS 回路ファイル（`.r1cs`）

Circom コンパイラによって生成されます：

```bash
circom circuit.circom --r1cs --wasm --sym
```

### ウィットネスファイル（`.wtns`）

完全な入力から snarkjs によって生成されます：

```bash
# 入力からウィットネスを計算
snarkjs wtns calculate circuit.wasm input.json witness.wtns

# ウィットネスを検証（オプション）
snarkjs wtns check circuit.r1cs witness.wtns
```

### 入力 JSON ファイル

信号名をキーとする JSON オブジェクト：

```json
{
  "publicValue": "42",
  "privateValue": "123",
  "arraySignal": ["1", "2", "3"]
}
```

**重要な注意事項：**

- すべての値は文字列である必要があります（数値も含む）
- 配列信号はサポートされています
- 信号名は回路で定義されたものと一致する必要があります
- `encrypt` の場合、公開入力のみを提供
- `decrypt` の場合、完全な入力（公開 + プライベート）から生成されたウィットネスファイルを提供

## 結合暗号文形式

`encrypt` コマンドは以下の構造で結合暗号文を作成します：

```
[1 バイト フラグ]
[4 バイト ウィットネス CT 長]
[ウィットネス暗号文]
[4 バイト 公開入力長]  (フラグ = 1 の場合)
[公開入力 JSON]         (フラグ = 1 の場合)
[暗号化されたメッセージ]
```

**フラグバイト：**

- `1` = 公開入力を含む（デフォルト）
- `0` = 公開入力を含まない（`--no-public-input` を使用）

この形式は zkenc-js と互換性があり、以下が可能です：

- 自己完結型の暗号文（必要なデータをすべて含む）
- クロスツール互換性
- オプションの公開入力埋め込み

## バイナリファイルの操作

### バイナリファイルの暗号化

高レベル API で任意のファイルタイプを暗号化できます：

```bash
# 画像を1ステップで暗号化
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message photo.jpg \
  --output encrypted_photo.bin

# ウィットネスを持つ人が画像を1ステップで復号化
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted_photo.bin \
  --output decrypted_photo.jpg
```

### バイナリファイルで低レベル API を使用

上級者向けの使用例：

```bash
# ステップ 1: 回路から鍵を生成
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness_ct.bin \
  --key key.bin

# ステップ 2: 外部ツールまたはカスタムメソッドで暗号化
# （key.bin は AES-256 に適した 32 バイトの鍵です）

# ステップ 3: 受信者が鍵を復元
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness_ct.bin \
  --key recovered_key.bin

# ステップ 4: ステップ 2 で使用したのと同じ方法で復号化
```

## 高度な使用法

### 公開入力を埋め込まずに暗号化

デフォルトでは、`encrypt` は公開入力を暗号文に埋め込みます。除外するには：

```bash
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin \
  --no-public-input
```

**`--no-public-input` を使用する場合：**

- 公開入力が非常に大きい
- 公開入力を別に配布する予定
- より小さな暗号文ファイルが必要

**注意：** 受信者はウィットネスを検証するために公開入力が必要です。

### バッチ処理

同じ回路と公開入力で複数のメッセージを暗号化：

```bash
# 公開入力を埋め込んで複数のファイルを暗号化
for file in documents/*.txt; do
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input public.json \
    --message "$file" \
    --output "encrypted/$(basename $file).enc"
done
```

各暗号化ファイルは自己完結型で、個別に復号化できます。

### クロスツール互換性

zkenc-cli は zkenc-js と**完全に互換性があります**！ 一方のツールで暗号化し、もう一方で復号化できます：

**CLI → JS：**

```bash
# CLI で暗号化
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin

# Node.js またはブラウザで zkenc-js で復号化
# encrypted.bin は zkenc-js の decrypt() で読み取れます
```

**JS → CLI：**

```bash
# zkenc-js の encrypt() で暗号化した後...
# CLI で復号化
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

両方のツールは同じ結合暗号文形式を使用し、シームレスな相互運用性を保証します。

[クロスツールワークフローについて詳しく学ぶ →](/docs/guides/cross-tool-workflow)

## パフォーマンスのヒント

1. **高レベル API を使用**：`encrypt`/`decrypt` コマンドがすべてを効率的に処理
2. **公開入力を埋め込む**：暗号文を自己完結型に保つ（デフォルト動作）
3. **回路を事前コンパイル**：回路を一度コンパイルし、何度も再利用
4. **回路サイズを考慮**：大きな回路 = 遅い encap/decap 操作
5. **バイナリ形式**：すべてのファイルは効率的なバイナリシリアライゼーションを使用

## 一般的なパターン

### 条件付きアクセス制御

```bash
# パズルを解くユーザーのみが復号化できる
zkenc encrypt \
  --circuit puzzle.r1cs \
  --input question.json \
  --message "秘密の答え: 42" \
  --output secret.bin
```

### タイムロック暗号化

```bash
# ウィットネスを生成するために計算作業が必要
zkenc encrypt \
  --circuit timelock.r1cs \
  --input params.json \
  --message future_message.txt \
  --output locked.bin
```

### 暗号化ファイルの配布

```bash
# 公開入力を埋め込んで暗号化
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message secret.txt \
  --output package.bin

# package.bin を公開で共有
# 有効なウィットネスを生成できる人だけが復号化可能
```

## 次のステップ

- **[API リファレンス →](/docs/api/zkenc-cli)** - 完全な CLI コマンドリファレンス
- **[クロスツールワークフロー →](/docs/guides/cross-tool-workflow)** - CLI を zkenc-js と併用
- **[zkenc-js を始める →](/docs/getting-started/zkenc-js)** - JavaScript の代替

## トラブルシューティング

### "Circuit file not found"

R1CS ファイルパスが正しいことを確認してください：

```bash
# ファイルが存在するか確認
ls -lh circuit.r1cs
```

### "Invalid inputs"

JSON ファイルを確認してください：

- 有効な JSON 形式である
- 必要なすべての信号名が含まれている
- すべての数値に文字列値を使用

```bash
# JSON を検証
cat inputs.json | jq .
```

### "Invalid ciphertext: too short"

これは暗号文ファイルが破損しているか、有効な zkenc 暗号文ではないことを意味します。確認してください：

- ファイルが zkenc-cli の `encrypt` または zkenc-js の `encrypt()` によって作成された
- ファイルが変更または切り詰められていない
- 正しいファイルを使用している

### "Decap failed"

これは通常次のことを意味します：

- ウィットネスが回路制約を満たしていない
- ウィットネスファイルが破損している
- 誤った回路ファイルを使用している
- ウィットネスが暗号化に使用された公開入力と一致しない

まずウィットネスを検証してください：

```bash
snarkjs wtns check circuit.r1cs witness.wtns
```

### "Decryption failed" または "Message decryption failed"

確認してください：

- ウィットネスが回路制約を満たしている
- 暗号文ファイルが破損していない
- 正しい回路ファイルを使用している
- ウィットネスが暗号化時の公開入力と一致している

## サポート

問題や質問がある場合：

1. [API リファレンス](/docs/api/zkenc-cli) を確認
2. [ワークフロー例](/docs/guides/cross-tool-workflow) を参照
3. [GitHub](https://github.com/flyinglimao/zkenc) で issue を開く
