---
sidebar_position: 2
---

# zkenc-cli API 參考

zkenc-cli 的完整命令列參考，這是基於 Rust 的見證加密工具。

## 安裝

### 從原始碼建構

```bash
git clone https://github.com/flyinglimao/zkenc.git
cd zkenc
cargo build --release --package zkenc-cli

# 二進位檔案位於: target/release/zkenc
```

### 新增至 PATH

```bash
# Linux/macOS
export PATH=$PATH:$(pwd)/target/release

# 或是安裝至系統
sudo cp target/release/zkenc /usr/local/bin/
```

## 命令概覽

zkenc-cli 提供四個主要命令：

| 命令      | 用途             | 輸入               | 輸出        |
| --------- | ---------------- | ------------------ | ----------- |
| `encap`   | 使用電路產生金鑰 | R1CS + 公開輸入    | 密文 + 金鑰 |
| `decap`   | 使用見證恢復金鑰 | R1CS + 見證 + 密文 | 金鑰        |
| `encrypt` | 使用金鑰加密訊息 | 金鑰 + 訊息        | 加密檔案    |
| `decrypt` | 使用金鑰解密訊息 | 金鑰 + 加密檔案    | 解密檔案    |

## 命令

### `zkenc encap`

從電路和公開輸入產生見證加密的金鑰和密文。

```bash
zkenc encap [OPTIONS]
```

**必要選項：**

- `-c, --circuit <FILE>` - R1CS 電路檔案路徑 (.r1cs)
- `-i, --input <FILE>` - 包含公開輸入的 JSON 檔案路徑
- `--ciphertext <FILE>` - 密文的輸出路徑
- `-k, --key <FILE>` - 加密金鑰的輸出路徑

**範例：**

```bash
zkenc encap \
  --circuit sudoku.r1cs \
  --input public_inputs.json \
  --ciphertext witness.ct \
  --key encryption.key
```

**輸入 JSON 格式：**

```json
{
  "puzzle": [5, 3, 0, 0, 7, 0, 0, 0, 0],
  "difficulty": 1
}
```

**輸出：**

- **密文檔案**: 約 1576 位元組（見證加密密文）
- **金鑰檔案**: 約 32 位元組（AES-256 加密金鑰）

**範例輸出：**

```
📂 Loading R1CS circuit...
   - Constraints: 12847
   - Public inputs: 81
   - Wires: 13129

📋 Loading public inputs from JSON...
   - Parsed 81 field elements

🔐 Running Encap...

💾 Saving ciphertext...
   ✅ Ciphertext saved (1576 bytes)

🔑 Saving key...
   ✅ Key saved (32 bytes)
```

### `zkenc decap`

使用有效見證恢復加密金鑰。

```bash
zkenc decap [OPTIONS]
```

**必要選項：**

- `-c, --circuit <FILE>` - R1CS 電路檔案路徑 (.r1cs)
- `-w, --witness <FILE>` - 見證檔案路徑（來自 snarkjs 的 .wtns）
- `--ciphertext <FILE>` - 密文檔案路徑（來自 encap）
- `-k, --key <FILE>` - 恢復金鑰的輸出路徑

**產生見證：**

首先，使用 snarkjs 產生見證：

```bash
# 建立完整輸入 JSON（公開 + 私密）
cat > full_inputs.json <<EOF
{
  "puzzle": [5, 3, 0, 0, 7, 0, 0, 0, 0],
  "solution": [5, 3, 4, 6, 7, 8, 9, 1, 2]
}
EOF

# 使用 snarkjs 產生見證
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns
```

**範例：**

```bash
zkenc decap \
  --circuit sudoku.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key
```

**輸出：**

```
📂 Loading R1CS circuit...
   - Constraints: 12847
   - Public inputs: 81
   - Wires: 13129

📂 Loading witness...
   - Wires: 13129

📂 Loading ciphertext...
   - Size: 1576 bytes

🔓 Running Decap...
   ✅ Key recovered successfully

💾 Saving recovered key...
   ✅ Key saved (32 bytes)
```

### `zkenc encrypt`

使用加密金鑰加密訊息。

```bash
zkenc encrypt [OPTIONS]
```

**必要選項：**

- `-k, --key <FILE>` - 加密金鑰檔案路徑（來自 encap 或 decap）
- `-i, --input <FILE>` - 明文檔案路徑
- `-o, --output <FILE>` - 輸出加密檔案路徑

**範例：**

```bash
# 加密文字檔案
zkenc encrypt \
  --key encryption.key \
  --input message.txt \
  --output message.txt.enc

# 加密二進位檔案
zkenc encrypt \
  --key encryption.key \
  --input document.pdf \
  --output document.pdf.enc
```

**輸出：**

```
🔑 Loading key...
📄 Loading plaintext...
   - Plaintext size: 1234 bytes

🔒 Encrypting...
   ✅ Encrypted file saved (1266 bytes)
```

**注意：** 輸出大小 = 輸入大小 + 28 位元組（GCM nonce + tag）

### `zkenc decrypt`

使用加密金鑰解密訊息。

```bash
zkenc decrypt [OPTIONS]
```

**必要選項：**

- `-k, --key <FILE>` - 加密金鑰檔案路徑
- `-i, --input <FILE>` - 加密檔案路徑
- `-o, --output <FILE>` - 輸出解密檔案路徑

**範例：**

```bash
zkenc decrypt \
  --key recovered.key \
  --input message.txt.enc \
  --output decrypted.txt
```

**輸出：**

```
🔑 Loading key...
📦 Loading encrypted data...
   - Encrypted size: 1266 bytes

🔓 Decrypting...
   ✅ Decrypted file saved (1234 bytes)
```

## 完整工作流程

### 完整的加密/解密流程

```bash
# 1. 封裝：使用電路產生金鑰
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key encryption.key

# 2. 加密：使用金鑰加密訊息
zkenc encrypt \
  --key encryption.key \
  --input secret.txt \
  --output secret.txt.enc

# 3. 使用 snarkjs 產生見證
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns

# 4. 解封裝：使用見證恢復金鑰
zkenc decap \
  --circuit circuit.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key

# 5. 解密：使用恢復的金鑰解密訊息
zkenc decrypt \
  --key recovered.key \
  --input secret.txt.enc \
  --output decrypted.txt
```

### 簡化流程（單步）

為方便起見，您可以結合 encap + encrypt：

```bash
# 加密（在一個腳本中執行 encap + encrypt）
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key temp.key

zkenc encrypt \
  --key temp.key \
  --input message.txt \
  --output message.enc

# 分發: witness.ct + message.enc

# 解密（在一個腳本中執行 decap + decrypt）
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns

zkenc decap \
  --circuit circuit.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key temp.key

zkenc decrypt \
  --key temp.key \
  --input message.enc \
  --output decrypted.txt
```

## 檔案格式

### 輸入 JSON 格式

**公開輸入（用於 encap）：**

```json
{
  "signalName1": 42,
  "signalName2": [1, 2, 3],
  "signalName3": "123"
}
```

**完整輸入（用於見證產生）：**

```json
{
  "publicSignal": [5, 3, 0],
  "privateSignal": [5, 3, 4]
}
```

**規則：**

- 數字可以是整數或字串
- 陣列會自動展平
- 按排序順序處理鍵
- 所有值必須是有效的欄位元素

### 電路檔案

**必要檔案：**

- `.r1cs` - R1CS 電路檔案（來自 circom 編譯）
- `.wasm` - WASM 見證產生器（用於 snarkjs）

**編譯電路：**

```bash
circom circuit.circom --r1cs --wasm --output build
# 產生: build/circuit.r1cs, build/circuit_js/circuit.wasm
```

### 見證檔案

**格式：** `.wtns`（snarkjs 二進位格式）

**產生：**

```bash
snarkjs wtns calculate circuit.wasm inputs.json witness.wtns
```

### 輸出檔案

- **密文** (`.ct`): 約 1576 位元組，見證加密密文
- **金鑰** (`.key`): 約 32 位元組，AES-256 加密金鑰
- **加密檔案** (`.enc`): 原始大小 + 28 位元組，AES-256-GCM 密文

## 整合範例

### Bash 腳本

```bash
#!/bin/bash
set -e

CIRCUIT="sudoku.r1cs"
WASM="sudoku.wasm"
PUBLIC="public.json"
FULL="full_inputs.json"
MESSAGE="secret.txt"

echo "加密中..."
zkenc encap -c "$CIRCUIT" -i "$PUBLIC" --ciphertext ct.bin -k key.bin
zkenc encrypt -k key.bin -i "$MESSAGE" -o encrypted.bin

echo "解密中..."
snarkjs wtns calculate "$WASM" "$FULL" witness.wtns
zkenc decap -c "$CIRCUIT" -w witness.wtns --ciphertext ct.bin -k recovered.bin
zkenc decrypt -k recovered.bin -i encrypted.bin -o decrypted.txt

echo "驗證..."
diff "$MESSAGE" decrypted.txt && echo "✅ 成功！"
```

### Make 整合

```makefile
.PHONY: encrypt decrypt clean

CIRCUIT := circuit.r1cs
WASM := circuit.wasm
PUBLIC := public.json
FULL := full.json

encrypt: message.txt
	zkenc encap -c $(CIRCUIT) -i $(PUBLIC) --ciphertext witness.ct -k encrypt.key
	zkenc encrypt -k encrypt.key -i message.txt -o message.enc
	@echo "已加密: witness.ct + message.enc"

decrypt: witness.ct message.enc
	snarkjs wtns calculate $(WASM) $(FULL) witness.wtns
	zkenc decap -c $(CIRCUIT) -w witness.wtns --ciphertext witness.ct -k decrypt.key
	zkenc decrypt -k decrypt.key -i message.enc -o decrypted.txt
	@echo "已解密: decrypted.txt"

clean:
	rm -f *.ct *.key *.enc *.wtns decrypted.txt
```

## 跨工具相容性

zkenc-cli 與 zkenc-js 完全相容。檔案可以在它們之間共享。

### CLI 加密 → JS 解密

```bash
# CLI: 加密
zkenc encap -c circuit.r1cs -i public.json --ciphertext witness.ct -k key.bin
zkenc encrypt -k key.bin -i message.txt -o message.enc

# 為 zkenc-js 合併檔案
cat <(head -c 4 <(printf '\x00\x00\x06(\n')) witness.ct message.enc > combined.bin
```

```javascript
// JS: 解密
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const ciphertext = await fs.readFile("combined.bin");
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

[深入了解跨工具工作流程 →](/docs/guides/cross-tool-workflow)

## 效能

### Encap 效能

| 電路大小   | 約束數量         | 時間       |
| ---------- | ---------------- | ---------- |
| 小型       | < 1,000          | < 100ms    |
| 中型       | 1,000 - 10,000   | 100ms - 1s |
| 大型       | 10,000 - 100,000 | 1s - 10s   |
| 超大型     | > 100,000        | > 10s      |

### Decap 效能

類似於 encap，加上見證計算開銷（約 50-200ms）

### Encrypt/Decrypt 效能

非常快（< 10ms）- 僅 AES 操作，與電路大小無關

## 疑難排解

### "Failed to load R1CS circuit"

- 檢查檔案路徑是否正確
- 確保檔案是有效的 R1CS 格式（使用 circom 編譯）
- 嘗試重新編譯電路

```bash
circom circuit.circom --r1cs --output build
```

### "Failed to parse input JSON"

- 驗證 JSON 語法
- 確保所有值是數字或字串
- 檢查訊號名稱是否符合電路

```bash
# 驗證 JSON
cat inputs.json | jq .
```

### "Decap failed"

- 見證不滿足電路約束
- 錯誤的電路檔案
- 密文已損壞

**除錯：**

```bash
# 測試見證產生
snarkjs wtns calculate circuit.wasm inputs.json test.wtns

# 檢查電路
snarkjs r1cs info circuit.r1cs
```

### "Encryption/Decryption failed"

- 錯誤的金鑰檔案
- 加密檔案已損壞
- 檔案格式不符

**驗證金鑰：**

```bash
# 金鑰應該剛好是 32 位元組
ls -l *.key
```

## 最佳實踐

1. **保持電路檔案安全**：加密和解密都需要 R1CS 檔案
2. **分離公開/私密輸入**：僅與加密者共享公開輸入
3. **驗證見證有效性**：解密前測試見證產生
4. **使用一致的檔案命名**：遵循慣例（`.ct`、`.key`、`.enc`）
5. **臨時備份金鑰**：金鑰僅在加密階段需要

## 安全性考量

- **金鑰管理**：金鑰是臨時的 - 應安全保管見證
- **電路完整性**：確保 R1CS 檔案未被篡改
- **見證隱私**：永遠不要共享見證檔案 - 它們就像私鑰
- **傳輸安全**：使用安全通道分發密文

## 下一步

- **[快速入門 →](/docs/getting-started/zkenc-cli)** - 快速入門指南
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 與 zkenc-js 一起使用
- **[zkenc-core API →](/docs/api/zkenc-core)** - Rust 函式庫參考
