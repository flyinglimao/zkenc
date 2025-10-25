---
sidebar_position: 2
---

# zkenc-cli API リファレンス

zkenc-cli の完全なコマンドラインリファレンス、Rust ベースの証拠暗号化ツールです。

## インストール

### ソースからビルド

```bash
git clone https://github.com/flyinglimao/zkenc.git
cd zkenc
cargo build --release --package zkenc-cli

# バイナリの場所: target/release/zkenc
```

### PATH に追加

```bash
# Linux/macOS
export PATH=$PATH:$(pwd)/target/release

# またはシステム全体にインストール
sudo cp target/release/zkenc /usr/local/bin/
```

## コマンド概要

zkenc-cli は 4 つの主要なコマンドを提供します:

| コマンド  | 目的                     | 入力                         | 出力           |
| --------- | ------------------------ | ---------------------------- | -------------- |
| `encap`   | 回路を使用してキーを生成 | R1CS + 公開入力              | 暗号文 + キー  |
| `decap`   | ウィットネスでキーを回復 | R1CS + ウィットネス + 暗号文 | キー           |
| `encrypt` | キーでメッセージを暗号化 | キー + メッセージ            | 暗号化ファイル |
| `decrypt` | キーでメッセージを復号化 | キー + 暗号化ファイル        | 復号化ファイル |

## コマンド

### `zkenc encap`

回路と公開入力から証拠暗号化されたキーと暗号文を生成します。

```bash
zkenc encap [OPTIONS]
```

**必須オプション:**

- `-c, --circuit <FILE>` - R1CS 回路ファイルへのパス(.r1cs)
- `-i, --input <FILE>` - 公開入力を含む JSON ファイルへのパス
- `--ciphertext <FILE>` - 暗号文の出力パス
- `-k, --key <FILE>` - 暗号化キーの出力パス

**例:**

```bash
zkenc encap \
  --circuit sudoku.r1cs \
  --input public_inputs.json \
  --ciphertext witness.ct \
  --key encryption.key
```

**入力 JSON 形式:**

```json
{
  "puzzle": [5, 3, 0, 0, 7, 0, 0, 0, 0],
  "difficulty": 1
}
```

**出力:**

- **暗号文ファイル**: 約 1576 バイト(証拠暗号化暗号文)
- **キーファイル**: 約 32 バイト(AES-256 暗号化キー)

**出力例:**

```
📂 R1CS回路をロード中...
   - 制約: 12847
   - 公開入力: 81
   - ワイヤ: 13129

📋 JSONから公開入力をロード中...
   - 81個のフィールド要素を解析

🔐 Encapを実行中...

💾 暗号文を保存中...
   ✅ 暗号文を保存しました(1576バイト)

🔑 キーを保存中...
   ✅ キーを保存しました(32バイト)
```

### `zkenc decap`

有効なウィットネスを使用して暗号化キーを回復します。

```bash
zkenc decap [OPTIONS]
```

**必須オプション:**

- `-c, --circuit <FILE>` - R1CS 回路ファイルへのパス(.r1cs)
- `-w, --witness <FILE>` - ウィットネスファイルへのパス(snarkjs からの.wtns)
- `--ciphertext <FILE>` - 暗号文ファイルへのパス(encap から)
- `-k, --key <FILE>` - 回復されたキーの出力パス

**ウィットネスの生成:**

まず、snarkjs を使用してウィットネスを生成します:

```bash
# 完全な入力JSONを作成(公開 + 秘密)
cat > full_inputs.json <<EOF
{
  "puzzle": [5, 3, 0, 0, 7, 0, 0, 0, 0],
  "solution": [5, 3, 4, 6, 7, 8, 9, 1, 2]
}
EOF

# snarkjsを使用してウィットネスを生成
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns
```

**例:**

```bash
zkenc decap \
  --circuit sudoku.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key
```

**出力:**

```
📂 R1CS回路をロード中...
   - 制約: 12847
   - 公開入力: 81
   - ワイヤ: 13129

📋 snarkjsからウィットネスをロード中...
   - ウィットネス要素: 13129

📦 暗号文をロード中...
   - 暗号文サイズ: 1576バイト

🔓 Decapを実行中...

🔑 回復されたキーを保存中...
   ✅ キーを保存しました(32バイト)
```

### `zkenc encrypt`

暗号化キーを使用してメッセージを暗号化します。

```bash
zkenc encrypt [OPTIONS]
```

**必須オプション:**

- `-k, --key <FILE>` - 暗号化キーファイルへのパス(encap または decap から)
- `-i, --input <FILE>` - 平文ファイルへのパス
- `-o, --output <FILE>` - 出力暗号化ファイルへのパス

**例:**

```bash
# テキストファイルを暗号化
zkenc encrypt \
  --key encryption.key \
  --input message.txt \
  --output message.txt.enc

# バイナリファイルを暗号化
zkenc encrypt \
  --key encryption.key \
  --input document.pdf \
  --output document.pdf.enc
```

**出力:**

```
🔑 キーをロード中...
📄 平文をロード中...
   - 平文サイズ: 1234バイト

🔒 暗号化中...
   ✅ 暗号化ファイルを保存しました(1266バイト)
```

**注意:** 出力サイズ = 入力サイズ + 28 バイト(GCM ノンス + タグ)

### `zkenc decrypt`

暗号化キーを使用してメッセージを復号化します。

```bash
zkenc decrypt [OPTIONS]
```

**必須オプション:**

- `-k, --key <FILE>` - 暗号化キーファイルへのパス
- `-i, --input <FILE>` - 暗号化ファイルへのパス
- `-o, --output <FILE>` - 出力復号化ファイルへのパス

**例:**

```bash
zkenc decrypt \
  --key recovered.key \
  --input message.txt.enc \
  --output decrypted.txt
```

**出力:**

```
🔑 キーをロード中...
📦 暗号化データをロード中...
   - 暗号化サイズ: 1266バイト

🔓 復号化中...
   ✅ 復号化ファイルを保存しました(1234バイト)
```

## 完全なワークフロー

### 完全な暗号化/復号化フロー

```bash
# 1. カプセル化: 回路でキーを生成
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key encryption.key

# 2. 暗号化: キーでメッセージを暗号化
zkenc encrypt \
  --key encryption.key \
  --input secret.txt \
  --output secret.txt.enc

# 3. snarkjsでウィットネスを生成
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns

# 4. デカプセル化: ウィットネスでキーを回復
zkenc decap \
  --circuit circuit.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key

# 5. 復号化: 回復されたキーでメッセージを復号化
zkenc decrypt \
  --key recovered.key \
  --input secret.txt.enc \
  --output decrypted.txt
```

### 簡略化されたフロー(ワンステップ)

便宜上、encap + encrypt を組み合わせることができます:

```bash
# 暗号化(1つのスクリプトでencap + encrypt)
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key temp.key

zkenc encrypt \
  --key temp.key \
  --input message.txt \
  --output message.enc

# 配布: witness.ct + message.enc

# 復号化(1つのスクリプトでdecap + decrypt)
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns

zkenc decap \
  --circuit circuit.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key

zkenc decrypt \
  --key recovered.key \
  --input message.enc \
  --output decrypted.txt
```

## ファイル形式

### 入力 JSON 形式

**公開入力(encap 用):**

```json
{
  "signalName1": 42,
  "signalName2": [1, 2, 3],
  "signalName3": "123"
}
```

**完全な入力(ウィットネス生成用):**

```json
{
  "publicSignal": [5, 3, 0],
  "privateSignal": [5, 3, 4]
}
```

**ルール:**

- 数値は整数または文字列
- 配列は自動的にフラット化
- キーはソート順に処理
- すべての値は有効なフィールド要素である必要

### 回路ファイル

**必要なファイル:**

- `.r1cs` - R1CS 回路ファイル(circom コンパイルから)
- `.wasm` - WASM ウィットネス生成器(snarkjs 用)

**回路をコンパイル:**

```bash
circom circuit.circom --r1cs --wasm --output build
# 作成: build/circuit.r1cs, build/circuit_js/circuit.wasm
```

### ウィットネスファイル

**形式:** `.wtns`(snarkjs バイナリ形式)

**生成:**

```bash
snarkjs wtns calculate circuit.wasm inputs.json witness.wtns
```

### 出力ファイル

- **暗号文**(`.ct`): 約 1576 バイト、証拠暗号化暗号文
- **キー**(`.key`): 約 32 バイト、AES-256 暗号化キー
- **暗号化済み**(`.enc`): 元のサイズ + 28 バイト、AES-256-GCM 暗号文

## 統合例

### Bash スクリプト

```bash
#!/bin/bash
set -e

CIRCUIT="sudoku.r1cs"
WASM="sudoku.wasm"
PUBLIC="public.json"
FULL="full_inputs.json"
MESSAGE="secret.txt"

echo "暗号化中..."
zkenc encap -c "$CIRCUIT" -i "$PUBLIC" --ciphertext ct.bin -k key.bin
zkenc encrypt -k key.bin -i "$MESSAGE" -o encrypted.bin

echo "復号化中..."
snarkjs wtns calculate "$WASM" "$FULL" witness.wtns
zkenc decap -c "$CIRCUIT" -w witness.wtns --ciphertext ct.bin -k recovered.bin
zkenc decrypt -k recovered.bin -i encrypted.bin -o decrypted.txt

echo "検証中..."
diff "$MESSAGE" decrypted.txt && echo "✅ 成功!"
```

### Makefile との統合

```makefile
.PHONY: encrypt decrypt clean

CIRCUIT := circuit.r1cs
WASM := circuit.wasm
PUBLIC := public.json
FULL := full.json

encrypt: message.txt
	zkenc encap -c $(CIRCUIT) -i $(PUBLIC) --ciphertext witness.ct -k encrypt.key
	zkenc encrypt -k encrypt.key -i message.txt -o message.enc
	@echo "暗号化: witness.ct + message.enc"

decrypt: witness.ct message.enc
	snarkjs wtns calculate $(WASM) $(FULL) witness.wtns
	zkenc decap -c $(CIRCUIT) -w witness.wtns --ciphertext witness.ct -k decrypt.key
	zkenc decrypt -k decrypt.key -i message.enc -o decrypted.txt
	@echo "復号化: decrypted.txt"

clean:
	rm -f *.ct *.key *.enc *.wtns decrypted.txt
```

## クロスツール互換性

zkenc-cli は zkenc-js と完全に互換性があります。ファイルは両者間で共有できます。

### CLI 暗号化 → JS 復号化

```bash
# CLI: 暗号化
zkenc encap -c circuit.r1cs -i public.json --ciphertext witness.ct -k key.bin
zkenc encrypt -k key.bin -i message.txt -o message.enc

# zkenc-js用にファイルを結合
cat <(head -c 4 <(printf '\x00\x00\x06(\n')) witness.ct message.enc > combined.bin
```

```javascript
// JS: 復号化
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const ciphertext = await fs.readFile("combined.bin");
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

[クロスツールワークフローの詳細 →](/docs/guides/cross-tool-workflow)

## パフォーマンス

### Encap パフォーマンス

| 回路サイズ   | 制約数           | 時間       |
| ------------ | ---------------- | ---------- |
| 小           | < 1,000          | < 100ms    |
| 中           | 1,000 - 10,000   | 100ms - 1s |
| 大           | 10,000 - 100,000 | 1s - 10s   |
| 非常に大きい | > 100,000        | > 10s      |

### Decap パフォーマンス

Encap と同様、加えてウィットネス計算のオーバーヘッド(約 50-200ms)

### Encrypt/Decrypt パフォーマンス

非常に高速(< 10ms) - AES 操作のみ、回路サイズに依存しない

## トラブルシューティング

### "Failed to load R1CS circuit"

- ファイルパスが正しいことを確認
- ファイルが有効な R1CS 形式であることを確認(circom でコンパイル)
- 回路を再コンパイルしてみる

```bash
circom circuit.circom --r1cs --output build
```

### "Failed to parse input JSON"

- JSON 構文を検証
- すべての値が数値または文字列であることを確認
- シグナル名が回路と一致することを確認

```bash
# JSONを検証
cat inputs.json | jq .
```

### "Decap failed"

- ウィットネスが回路の制約を満たしていない
- 間違った回路ファイル
- 破損した暗号文

**デバッグ:**

```bash
# ウィットネス生成をテスト
snarkjs wtns calculate circuit.wasm inputs.json test.wtns

# 回路を確認
snarkjs r1cs info circuit.r1cs
```

### "Encryption/Decryption failed"

- 間違ったキーファイル
- 破損した暗号化ファイル
- ファイル形式の不一致

**キーを確認:**

```bash
# キーは正確に32バイトである必要
ls -l *.key
```

## ベストプラクティス

1. **回路ファイルを安全に保管**: R1CS ファイルは暗号化と復号化の両方に必要
2. **公開/秘密入力を分離**: 暗号化者には公開入力のみを共有
3. **ウィットネスの有効性を検証**: 復号化前にウィットネス生成をテスト
4. **一貫したファイル命名を使用**: 規則に従う(`.ct`、`.key`、`.enc`)
5. **一時的にキーをバックアップ**: キーは暗号化フェーズでのみ必要

## セキュリティの考慮事項

- **キー管理**: キーは一時的 - 代わりにウィットネスを保護
- **回路の整合性**: R1CS ファイルが改ざんされていないことを確認
- **ウィットネスのプライバシー**: ウィットネスファイルを共有しない - 秘密鍵のようなもの
- **転送セキュリティ**: 暗号文配布には安全なチャネルを使用

## 次のステップ

- **[入門 →](/docs/getting-started/zkenc-cli)** - クイックスタートガイド
- **[クロスツールワークフロー →](/docs/guides/cross-tool-workflow)** - zkenc-js と併用
- **[zkenc-core API →](/docs/api/zkenc-core)** - Rust ライブラリリファレンス
