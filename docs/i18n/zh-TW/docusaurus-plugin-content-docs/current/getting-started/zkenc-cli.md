---
sidebar_position: 2
---

# zkenc-cli 快速開始

zkenc-cli 是用於見證加密操作的命令列工具。它提供簡單的介面，用於使用 Circom 電路加密和解密訊息。

## 安裝

### 從原始碼安裝

複製儲存庫並從原始碼建置：

```bash
git clone https://github.com/flyinglimao/zkenc.git
cd zkenc/packages/zkenc-cli
cargo install --path .
```

## 前置需求

使用 zkenc-cli 之前，你需要：

1. **已編譯的 Circom 電路**，包含：

   - `.r1cs` 檔案（電路約束）
   - `.wasm` 檔案（見證產生器）
   - `.sym` 檔案（信號到線路的映射）**← 加密時必需**

2. **輸入檔案**採用 JSON 格式

使用 `--sym` 旗標編譯你的電路以產生所有必需的檔案。

## 快速開始

### 1. 建立簡單電路

建立檔案 `example.circom`：

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

### 2. 編譯電路

```bash
circom example.circom --r1cs --wasm --sym --output circuit_output
```

這會建立：

- `circuit_output/example.r1cs`
- `circuit_output/example_js/example.wasm`
- `circuit_output/example.sym`（zkenc-cli 的符號檔案）

### 3. 準備輸入檔案

建立 `public_inputs.json`（加密時已知）：

```json
{
  "publicValue": "42"
}
```

建立 `full_inputs.json`（解密所需）：

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

### 4. 加密你的秘密訊息

使用 `encrypt` 一步完成見證加密：

```bash
echo "Hello, zkenc!" > message.txt
zkenc encrypt \
  --circuit circuit_output/example.r1cs \
  --sym circuit_output/example.sym \
  --input public_inputs.json \
  --message message.txt \
  --output encrypted.bin
```

此命令會：

- 從公開輸入使用 .sym 檔案產生見證加密金鑰（encap）
- 使用 AES-256-GCM 加密你的訊息
- 將所有內容組合成單一密文檔案
- 預設將公開輸入嵌入密文中

輸出：

```
🔐 Step 1: Running Encap...
📂 Loading R1CS circuit...
   - Constraints: 2
   - Public inputs: 1
   - Wires: 4

📂 Loading symbol file...
   - Signal mapping loaded

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

### 5. 產生見證檔案

解密前，接收者需要產生見證來證明他們有有效的解答：

```bash
snarkjs wtns calculate \
  circuit_output/example_js/example.wasm \
  full_inputs.json \
  witness.wtns
```

### 6. 解密訊息

使用 `decrypt` 一步恢復並解密訊息：

```bash
zkenc decrypt \
  --circuit circuit_output/example.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

此命令會：

- 解析組合密文
- 使用見證恢復金鑰（decap）
- 使用 AES-256-GCM 解密訊息

輸出：

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

驗證結果：

```bash
cat decrypted.txt
# 輸出：Hello, zkenc!
```

## 命令參考

### `zkenc encap`

從電路和公開輸入產生密文和加密金鑰。

```bash
zkenc encap \
  --circuit <R1CS_FILE> \
  --sym <SYM_FILE> \
  --input <JSON_FILE> \
  --ciphertext <OUTPUT_CT> \
  --key <OUTPUT_KEY>
```

**參數：**

- `--circuit <FILE>` - R1CS 電路檔案路徑（Circom 產生的 `.r1cs`）
- `--sym <FILE>` - 符號檔案路徑（Circom 產生的 `.sym`）**← 必需**
- `--input <FILE>` - 包含公開輸入的 JSON 檔案路徑
- `--ciphertext <FILE>` - 密文的輸出路徑
- `--key <FILE>` - 加密金鑰的輸出路徑

**範例：**

```bash
zkenc encap \
  --circuit sudoku.r1cs \
  --sym sudoku.sym \
  --input puzzle.json \
  --ciphertext ciphertext.bin \
  --key key.bin
```

---

### `zkenc decap`

使用有效的見證和密文恢復加密金鑰。

```bash
zkenc decap \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --key <OUTPUT_KEY>
```

**參數：**

- `--circuit <FILE>` - R1CS 電路檔案路徑
- `--witness <FILE>` - 見證檔案路徑（snarkjs 產生的 `.wtns`）
- `--ciphertext <FILE>` - 密文檔案路徑
- `--key <FILE>` - 恢復金鑰的輸出路徑

**範例：**

```bash
zkenc decap \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext ciphertext.bin \
  --key recovered_key.bin
```

---

### `zkenc encrypt`

使用見證加密來加密訊息（高階、一步驟操作）。

```bash
zkenc encrypt \
  --circuit <R1CS_FILE> \
  --sym <SYM_FILE> \
  --input <JSON_FILE> \
  --message <MESSAGE_FILE> \
  --output <OUTPUT_FILE> \
  [--no-public-input]
```

**參數：**

- `--circuit <FILE>` - R1CS 電路檔案路徑（Circom 產生的 `.r1cs`）
- `--sym <FILE>` - 符號檔案路徑（Circom 產生的 `.sym`）**← 必需**
- `--input <FILE>` - 包含公開輸入的 JSON 檔案路徑
- `--message <FILE>` - 明文訊息檔案路徑
- `--output <FILE>` - 組合密文的輸出路徑
- `--no-public-input` - 不在密文中嵌入公開輸入（選用）

**功能：**

此命令將 encap 和 AES 加密結合成單一步驟：

1. 從公開輸入使用 .sym 檔案產生見證加密金鑰以正確對應輸入
2. 使用 AES-256-GCM 加密訊息
3. 建立組合密文，格式為：`[旗標][見證長度][見證密文][公開輸入長度][公開輸入][加密訊息]`

**範例：**

```bash
zkenc encrypt \
  --circuit sudoku.r1cs \
  --sym sudoku.sym \
  --input puzzle.json \
  --message secret.txt \
  --output encrypted.bin
```

**相容性：**輸出與 zkenc-js 的 `decrypt()` 函式完全相容。

---

### `zkenc decrypt`

使用見證解密來解密訊息（高階、一步驟操作）。

```bash
zkenc decrypt \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --output <OUTPUT_FILE>
```

**參數：**

- `--circuit <FILE>` - R1CS 電路檔案路徑
- `--witness <FILE>` - 見證檔案路徑（snarkjs 產生的 `.wtns`）
- `--ciphertext <FILE>` - 組合密文檔案路徑
- `--output <FILE>` - 解密訊息的輸出路徑

**功能：**

此命令將 decap 和 AES 解密結合成單一步驟：

1. 解析組合密文
2. 使用見證恢復金鑰
3. 使用 AES-256-GCM 解密訊息

**範例：**

```bash
zkenc decrypt \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

**相容性：** 可以解密由 zkenc-js `encrypt()` 函式建立的檔案。

---

## 了解工作流程

zkenc-cli 提供兩個層級的 API：

### 高階 API（建議使用）

簡單的兩步驟流程：

1. **`encrypt`** - 在一個命令中結合 encap + AES 加密

   - 輸入：電路、公開輸入、訊息
   - 輸出：組合密文（與 zkenc-js 相容）

2. **`decrypt`** - 在一個命令中結合 decap + AES 解密
   - 輸入：電路、見證、組合密文
   - 輸出：解密訊息

**優點：**

- 更簡單的工作流程（2 步驟 vs 4 步驟）
- 只需管理單一密文檔案
- 與 zkenc-js 完全相容
- 可將公開輸入嵌入密文

### 低階 API（進階）

提供細粒度控制的四步驟流程：

1. **`encap`** - 從公開輸入產生見證加密密文和金鑰
2. 分別加密訊息（使用任何 AES 工具）
3. **`decap`** - 使用有效見證恢復金鑰
4. 分別解密訊息（使用任何 AES 工具）

**使用情境：**

- 自訂加密方案
- 跨多個訊息重複使用金鑰
- 與現有加密流程整合
- 教育目的以理解協定

**注意：**對於大多數使用情境，建議使用高階 API，因為它確保相容性並簡化工作流程。

**注意：**對於大多數使用情境，建議使用高階 API，因為它確保相容性並簡化工作流程。

## 輸入檔案格式

### R1CS 電路檔案（`.r1cs`）

由 Circom 編譯器產生：

```bash
circom circuit.circom --r1cs --wasm --sym
```

### 見證檔案（`.wtns`）

由 snarkjs 從你的完整輸入產生：

```bash
# 從輸入計算見證
snarkjs wtns calculate circuit.wasm input.json witness.wtns

# 驗證見證（選用）
snarkjs wtns check circuit.r1cs witness.wtns
```

### 輸入 JSON 檔案

以訊號名稱為鍵的 JSON 物件：

```json
{
  "publicValue": "42",
  "privateValue": "123",
  "arraySignal": ["1", "2", "3"]
}
```

**重要注意事項：**

- 所有值必須是字串（即使是數字）
- 支援陣列訊號
- 訊號名稱必須與電路中定義的名稱相符
- 對於 `encrypt`，只提供公開輸入
- 對於 `decrypt`，提供從完整輸入（公開 + 私密）產生的見證檔案

## 組合密文格式

`encrypt` 命令建立具有以下結構的組合密文：

```
[1 位元組旗標]
[4 位元組見證密文長度]
[見證密文]
[4 位元組公開輸入長度]  （如果旗標 = 1）
[公開輸入 JSON]          （如果旗標 = 1）
[加密訊息]
```

**旗標位元組：**

- `1` = 包含公開輸入（預設）
- `0` = 不包含公開輸入（使用 `--no-public-input`）

此格式與 zkenc-js 相容，並允許：

- 自包含的密文（包含所有必要資料）
- 跨工具相容性
- 選用的公開輸入嵌入

## 處理二進位檔案

### 加密二進位檔案

你可以使用高階 API 加密任何檔案類型：

```bash
# 一步驟加密圖片
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message photo.jpg \
  --output encrypted_photo.bin

# 擁有見證的人一步驟解密圖片
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted_photo.bin \
  --output decrypted_photo.jpg
```

### 對二進位檔案使用低階 API

對於進階使用情境：

```bash
# 步驟 1：從電路產生金鑰
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness_ct.bin \
  --key key.bin

# 步驟 2：使用外部工具或自訂方法加密
# （key.bin 是一個 32 位元組的金鑰，適用於 AES-256）

# 步驟 3：接收者恢復金鑰
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness_ct.bin \
  --key recovered_key.bin

# 步驟 4：使用步驟 2 相同的方法解密
```

## 進階用法

### 不嵌入公開輸入的加密

預設情況下，`encrypt` 會在密文中嵌入公開輸入。要排除它們：

```bash
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin \
  --no-public-input
```

**何時使用 `--no-public-input`：**

- 公開輸入非常大
- 你會分別分發公開輸入
- 你想要更小的密文檔案

**注意：**接收者將需要公開輸入來驗證見證。

### 批次處理

對於相同的電路和公開輸入加密多個訊息：

```bash
# 加密多個檔案並嵌入公開輸入
for file in documents/*.txt; do
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input public.json \
    --message "$file" \
    --output "encrypted/$(basename $file).enc"
done
```

每個加密的檔案都是自包含的，可以獨立解密。

### 跨工具相容性

zkenc-cli 與 zkenc-js **完全相容**！你可以使用一個工具加密，用另一個工具解密：

**CLI → JS：**

```bash
# 使用 CLI 加密
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin

# 在 Node.js 或瀏覽器中使用 zkenc-js 解密
# encrypted.bin 可以被 zkenc-js 的 decrypt() 讀取
```

**JS → CLI：**

```bash
# 使用 zkenc-js 的 encrypt() 加密後...
# 使用 CLI 解密
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

兩個工具使用相同的組合密文格式，確保無縫互通性。

[了解更多跨工具工作流程 →](/docs/guides/cross-tool-workflow)

## 效能提示

1. **使用高階 API**：`encrypt`/`decrypt` 命令高效地處理所有事情
2. **嵌入公開輸入**：保持密文自包含（預設行為）
3. **預先編譯電路**：編譯電路一次，重複使用多次
4. **考慮電路大小**：更大的電路 = 更慢的 encap/decap 操作
5. **二進位格式**：所有檔案使用高效的二進位序列化

## 常見模式

### 條件存取控制

```bash
# 只有解開謎題的使用者才能解密
zkenc encrypt \
  --circuit puzzle.r1cs \
  --input question.json \
  --message "秘密答案：42" \
  --output secret.bin
```

### 時間鎖加密

```bash
# 需要計算工作來產生見證
zkenc encrypt \
  --circuit timelock.r1cs \
  --input params.json \
  --message future_message.txt \
  --output locked.bin
```

### 分發加密檔案

```bash
# 加密並嵌入公開輸入
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message secret.txt \
  --output package.bin

# 公開分享 package.bin
# 只有能產生有效見證的人才能解密
```

## 下一步

- **[API 參考 →](/docs/api/zkenc-cli)** - 完整的 CLI 命令參考
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 搭配 zkenc-js 使用 CLI
- **[zkenc-js 快速開始 →](/docs/getting-started/zkenc-js)** - JavaScript 替代方案

## 疑難排解

### "Circuit file not found"（找不到電路檔案）

確保 R1CS 檔案路徑正確：

```bash
# 檢查檔案是否存在
ls -lh circuit.r1cs
```

### "Invalid inputs"（無效的輸入）

檢查你的 JSON 檔案：

- 是有效的 JSON 格式
- 包含所有必要的訊號名稱
- 所有數字使用字串值

```bash
# 驗證 JSON
cat inputs.json | jq .
```

### "Invalid ciphertext: too short"（無效的密文：太短）

這表示密文檔案損毀或不是有效的 zkenc 密文。確保：

- 檔案是由 zkenc-cli `encrypt` 或 zkenc-js `encrypt()` 建立的
- 檔案沒有被修改或截斷
- 你使用的是正確的檔案

### "Decap failed"（解封裝失敗）

這通常表示：

- 見證不滿足電路約束
- 見證檔案損毀
- 使用了錯誤的電路檔案
- 見證與加密時使用的公開輸入不匹配

先驗證你的見證：

```bash
snarkjs wtns check circuit.r1cs witness.wtns
```

### "Decryption failed" 或 "Message decryption failed"（解密失敗或訊息解密失敗）

確保：

- 見證滿足電路約束
- 密文檔案沒有損毀
- 使用正確的電路檔案
- 見證與加密時的公開輸入相符

## 支援

若有問題或疑問：

1. 查看 [API 參考](/docs/api/zkenc-cli)
2. 檢閱 [範例工作流程](/docs/guides/cross-tool-workflow)
3. 在 [GitHub](https://github.com/flyinglimao/zkenc) 上開啟 issue
