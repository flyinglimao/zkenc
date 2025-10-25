---
sidebar_position: 2
---

# zkenc-cli入門

zkenc-cliは、ウィットネス暗号化操作のためのコマンドラインツールです。Circom回路を使用してメッセージを暗号化および復号化するためのシンプルなインターフェースを提供します。

## インストール

### ソースからのインストール

リポジトリをクローンしてソースからビルドします：

```bash
git clone https://github.com/flyinglimao/zkenc.git
cd zkenc/packages/zkenc-cli
cargo install --path .
```

## 前提条件

zkenc-cliを使用する前に、次のものが必要です：

1. **コンパイル済みのCircom回路**、以下を含む：

   - `.r1cs`ファイル（回路制約）
   - `.wasm`ファイル（ウィットネスジェネレータ）

2. **JSON形式の入力ファイル**

## クイックスタート

### 1. シンプルな回路を作成

ファイル`example.circom`を作成します：

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

`public_inputs.json`を作成（暗号化時に既知）：

```json
{
  "publicValue": "42"
}
```

`full_inputs.json`を作成（復号化に必要）：

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

### 4. 秘密メッセージを暗号化

`encrypt`を使用してワンステップでウィットネス暗号化を実行：

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
- AES-256-GCMでメッセージを暗号化
- すべてを単一の暗号文ファイルに結合
- 暗号文に公開入力を埋め込む（デフォルト）

出力：

```
🔐 Step 1: Running Encap...
📂 Loading R1CS circuit...
   - Constraints: 2
   - Public inputs: 1
   - Wires: 4

📋 Loading public inputs from JSON...
   - Parsed 1 field elements

   ✅ Witness ciphertext generated (123 bytes)

🔒 Step 2: Encrypting message...
   - Message size: 14 bytes
   ✅ Message encrypted (42 bytes)

📦 Step 3: Creating combined ciphertext...
   ✅ Combined ciphertext saved (218 bytes)

✨ Encryption complete! Public inputs are embedded in the ciphertext.
```

### 5. ウィットネスファイルを生成

復号化の前に、受信者は有効な解を持っていることを証明するウィットネスを生成する必要があります：

```bash
snarkjs wtns calculate \
  circuit_output/example_js/example.wasm \
  full_inputs.json \
  witness.wtns
```

### 6. メッセージを復号化

`decrypt`を使用してワンステップでメッセージを復元および復号化：

```bash
zkenc decrypt \
  --circuit circuit_output/example.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

このコマンドは：

- 結合された暗号文を解析
- ウィットネスを使用して鍵を復元（decap）
- AES-256-GCMでメッセージを復号化

出力：

```
📦 Step 1: Parsing combined ciphertext...
   - Flag: 1
   - Witness ciphertext: 123 bytes
   - Public input: {"publicValue":"42"}
   - Encrypted message: 42 bytes

🔓 Step 2: Running Decap...
📂 Loading R1CS circuit...
   - Constraints: 2
   - Public inputs: 1

📋 Loading witness from snarkjs...
   - Witness elements: 4

   ✅ Key recovered from witness

🔓 Step 3: Decrypting message...
   ✅ Decrypted message saved (14 bytes)

✨ Decryption complete!
```

結果を確認：

```bash
cat decrypted.txt
# 出力：Hello, zkenc!
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

- `--circuit <FILE>` - R1CS回路ファイルのパス（Circomからの`.r1cs`）
- `--input <FILE>` - 公開入力を含むJSONファイルのパス
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

- `--circuit <FILE>` - R1CS回路ファイルのパス
- `--witness <FILE>` - ウィットネスファイルのパス（snarkjsからの`.wtns`）
- `--ciphertext <FILE>` - 暗号文ファイルのパス
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

ウィットネス暗号化を使用してメッセージを暗号化（高レベル、ワンステップ操作）。

```bash
zkenc encrypt \
  --circuit <R1CS_FILE> \
  --input <JSON_FILE> \
  --message <MESSAGE_FILE> \
  --output <OUTPUT_FILE> \
  [--no-public-input]
```

**引数：**

- `--circuit <FILE>` - R1CS回路ファイルのパス（Circomからの`.r1cs`）
- `--input <FILE>` - 公開入力を含むJSONファイルのパス
- `--message <FILE>` - 平文メッセージファイルのパス
- `--output <FILE>` - 結合された暗号文の出力パス
- `--no-public-input` - 暗号文に公開入力を埋め込まない（オプション）

**機能：**

このコマンドはencapとAES暗号化を1つのステップに統合します：

1. 公開入力からウィットネス暗号化鍵を生成
2. AES-256-GCMでメッセージを暗号化
3. 以下の形式で結合された暗号文を作成：`[flag][witnessLen][witnessCT][publicLen][publicInput][encryptedMsg]`

**例：**

```bash
zkenc encrypt \
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --message secret.txt \
  --output encrypted.bin
```

**互換性：** 出力はzkenc-jsの`decrypt()`関数と完全に互換性があります。

---

### `zkenc decrypt`

ウィットネス復号化を使用してメッセージを復号化（高レベル、ワンステップ操作）。

```bash
zkenc decrypt \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --output <OUTPUT_FILE>
```

**引数：**

- `--circuit <FILE>` - R1CS回路ファイルのパス
- `--witness <FILE>` - ウィットネスファイルのパス（snarkjsからの`.wtns`）
- `--ciphertext <FILE>` - 結合された暗号文ファイルのパス
- `--output <FILE>` - 復号化されたメッセージの出力パス

**機能：**

このコマンドはdecapとAES復号化を1つのステップに統合します：

1. 結合された暗号文を解析
2. ウィットネスを使用して鍵を復元
3. AES-256-GCMでメッセージを復号化

**例：**

```bash
zkenc decrypt \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

**互換性：** zkenc-jsの`encrypt()`関数で作成されたファイルを復号化できます。

---

### 低レベルコマンド

高度な使用例では、低レベルのencap/decapコマンドを個別に使用できます：

#### `zkenc encap`

## ワークフローの理解

zkenc-cliは2つのレベルのAPIを提供します：

### 高レベルAPI（推奨）

シンプルな2ステップのプロセス：

1. **`encrypt`** - 1つのコマンドでencap + AES暗号化を統合

   - 入力：回路、公開入力、メッセージ
   - 出力：結合された暗号文（zkenc-jsと互換）

2. **`decrypt`** - 1つのコマンドでdecap + AES復号化を統合
   - 入力：回路、ウィットネス、結合された暗号文
   - 出力：復号化されたメッセージ

**利点：**

- よりシンプルなワークフロー（4ステップに対して2ステップ）
- 単一の暗号文ファイル管理
- zkenc-jsと完全互換
- 公開入力を暗号文に埋め込み可能

### 低レベルAPI（高度）

きめ細かい制御のための4ステップのプロセス：

1. **`encap`** - 公開入力からウィットネス暗号化された暗号文と鍵を生成
2. メッセージを個別に暗号化（任意のAESツールを使用）
3. **`decap`** - 有効なウィットネスを使用して鍵を復元
4. メッセージを個別に復号化（任意のAESツールを使用）

**使用例：**

- カスタム暗号化スキーム
- 複数のメッセージにわたる鍵の再利用
- 既存の暗号化パイプラインとの統合
- プロトコルを理解するための教育目的

**注意：** ほとんどの使用例では、互換性を確保しワークフローを簡素化する高レベルAPIの使用をお勧めします。

## 入力ファイル形式

### R1CS回路ファイル（`.r1cs`）

Circomコンパイラによって生成されます：

```bash
circom circuit.circom --r1cs --wasm --sym
```

### ウィットネスファイル（`.wtns`）

完全な入力からsnarkjsによって生成されます：

```bash
# 入力からウィットネスを計算
snarkjs wtns calculate circuit.wasm input.json witness.wtns

# ウィットネスを検証（オプション）
snarkjs wtns check circuit.r1cs witness.wtns
```

### 入力JSONファイル

シグナル名をキーとするJSONオブジェクト：

```json
{
  "publicValue": "42",
  "privateValue": "123",
  "arraySignal": ["1", "2", "3"]
}
```

**重要な注意事項：**

- すべての値は文字列でなければなりません（数値も含む）
- 配列シグナルがサポートされています
- シグナル名は回路で定義された名前と一致する必要があります
- `encrypt`の場合、公開入力のみを提供します
- `decrypt`の場合、完全な入力（公開 + プライベート）から生成されたウィットネスファイルを提供します

## 結合された暗号文形式

`encrypt`コマンドは、以下の構造を持つ結合された暗号文を作成します：

```
[1バイトフラグ]
[4バイトウィットネス暗号文長]
[ウィットネス暗号文]
[4バイト公開入力長]  （フラグ = 1の場合）
[公開入力JSON]        （フラグ = 1の場合）
[暗号化されたメッセージ]
```

**フラグバイト：**

- `1` = 公開入力を含む（デフォルト）
- `0` = 公開入力を含まない（`--no-public-input`を使用）

この形式はzkenc-jsと互換性があり、以下を可能にします：

- 自己完結型暗号文（すべての必要なデータを含む）
- クロスツール互換性
- オプションの公開入力埋め込み

## バイナリファイルの処理

### バイナリファイルの暗号化

高レベルAPIを使用して任意のファイルタイプを暗号化できます：

```bash
# ワンステップで画像を暗号化
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message photo.jpg \
  --output encrypted_photo.bin

# ウィットネスを持つ人がワンステップで画像を復号化
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted_photo.bin \
  --output decrypted_photo.jpg
```

### 低レベルAPIを使用したバイナリファイルの処理

高度な使用例の場合：

```bash
# ステップ1：回路から鍵を生成
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness_ct.bin \
  --key key.bin

# ステップ2：外部ツールまたはカスタム方法で暗号化
# （key.binはAES-256に適した32バイトの鍵です）

# ステップ3：受信者が鍵を復元
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness_ct.bin \
  --key recovered_key.bin

# ステップ4：ステップ2で使用したのと同じ方法で復号化
```

## 高度な使用法

### 公開入力を埋め込まない暗号化

デフォルトでは、`encrypt`は暗号文に公開入力を埋め込みます。それらを除外するには：

```bash
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin \
  --no-public-input
```

**`--no-public-input`を使用する場合：**

- 公開入力が非常に大きい
- 公開入力を別々に配布する予定
- より小さな暗号文ファイルが必要

**注意：** 受信者はウィットネスを検証するために公開入力が必要です。

### バッチ処理

同じ回路と公開入力で複数のメッセージを暗号化：

```bash
# 埋め込まれた公開入力で複数のファイルを暗号化
for file in documents/*.txt; do
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input public.json \
    --message "$file" \
    --output "encrypted/$(basename $file).enc"
done
```

各暗号化ファイルは自己完結型で、独立して復号化できます。

### クロスツール互換性

zkenc-cliはzkenc-jsと**完全に互換性があります**！1つのツールで暗号化し、もう1つのツールで復号化できます：

**CLI → JS：**

```bash
# CLIで暗号化
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin

# Node.jsまたはブラウザでzkenc-jsを使用して復号化
# encrypted.binはzkenc-js decrypt()で読み取ることができます
```

**JS → CLI：**

```bash
# zkenc-js encrypt()で暗号化した後...
# CLIで復号化
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

両方のツールは同じ結合された暗号文形式を使用し、シームレスな相互運用性を確保します。

[クロスツールワークフローの詳細 →](/docs/guides/cross-tool-workflow)

## パフォーマンスのヒント

1. **高レベルAPIを使用**：`encrypt`/`decrypt`コマンドがすべてを効率的に処理
2. **公開入力を埋め込む**：暗号文を自己完結型に保つ（デフォルトの動作）
3. **回路を事前コンパイル**：回路を一度コンパイルし、何度も再利用
4. **回路サイズを考慮**：大きな回路 = 遅いencap/decap操作
5. **バイナリ形式**：すべてのファイルが効率的なバイナリシリアライゼーションを使用

## 一般的なパターン

### 条件付きアクセス制御

```bash
# パズルを解決したユーザーのみが復号化できる
zkenc encrypt \
  --circuit puzzle.r1cs \
  --input question.json \
  --message "Secret answer: 42" \
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
# 埋め込まれた公開入力で暗号化
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message secret.txt \
  --output package.bin

# package.binを公開で共有
# 有効なウィットネスを生成できる人のみが復号化できる
```

## 次のステップ

- **[APIリファレンス →](/docs/api/zkenc-cli)** - 完全なCLIコマンドリファレンス
- **[クロスツールワークフロー →](/docs/guides/cross-tool-workflow)** - CLIをzkenc-jsと一緒に使用
- **[zkenc-js入門 →](/docs/getting-started/zkenc-js)** - JavaScript代替手段

## トラブルシューティング

### "Circuit file not found"（回路ファイルが見つかりません）

R1CSファイルのパスが正しいことを確認してください：

```bash
# ファイルが存在するか確認
ls -lh circuit.r1cs
```

### "Invalid inputs"（無効な入力）

JSONファイルを確認してください：

- 有効なJSON形式である
- すべての必要なシグナル名を含む
- すべての数値に文字列値を使用

```bash
# JSONを検証
cat inputs.json | jq .
```

### "Invalid ciphertext: too short"（無効な暗号文：短すぎます）

これは暗号文ファイルが破損しているか、有効なzkenc暗号文でないことを意味します。確認してください：

- ファイルがzkenc-cli `encrypt`またはzkenc-js `encrypt()`によって作成された
- ファイルが変更または切り捨てられていない
- 正しいファイルを使用している

### "Decap failed"（Decapに失敗しました）

これは通常以下を意味します：

- ウィットネスが回路制約を満たさない
- ウィットネスファイルが破損している
- 間違った回路ファイルを使用している
- ウィットネスが暗号化に使用された公開入力と一致しない

まずウィットネスを検証してください：

```bash
snarkjs wtns check circuit.r1cs witness.wtns
```

### "Decryption failed" または "Message decryption failed"（復号化に失敗しました）

確認してください：

- ウィットネスが回路制約を満たす
- 暗号文ファイルが破損していない
- 正しい回路ファイルを使用している
- ウィットネスが暗号化時の公開入力と一致する

## サポート

問題や質問については：

1. [APIリファレンス](/docs/api/zkenc-cli)を確認
2. [ワークフロー例](/docs/guides/cross-tool-workflow)をレビュー
3. [GitHub](https://github.com/flyinglimao/zkenc)で問題を開く
