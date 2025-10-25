---
sidebar_position: 2
---

# zkenc-cli 入門

zkenc-cli 是一個用於見證加密操作的命令列工具。它提供了一個簡單的介面，可使用 Circom 電路加密和解密訊息。

## 安裝

### 從原始碼安裝

複製儲存庫並從原始碼建置：

```bash
git clone https://github.com/flyinglimao/zkenc.git
cd zkenc/packages/zkenc-cli
cargo install --path .
```

## 前置需求

使用 zkenc-cli 之前，您需要：

1. **已編譯的 Circom 電路**，包含：

   - `.r1cs` 檔案（電路約束）
   - `.wasm` 檔案（見證產生器）

2. **輸入檔案**，格式為 JSON

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
circom example.circom --r1cs --wasm --output circuit_output
```

這會建立：

- `circuit_output/example.r1cs`
- `circuit_output/example_js/example.wasm`

### 3. 準備輸入檔案

建立 `public_inputs.json`（加密時已知）：

```json
{
  "publicValue": "42"
}
```

建立 `full_inputs.json`（解密時需要）：

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

### 4. 加密您的秘密訊息

使用 `encrypt` 一步完成見證加密：

```bash
echo "Hello, zkenc!" > message.txt
zkenc encrypt \
  --circuit circuit_output/example.r1cs \
  --input public_inputs.json \
  --message message.txt \
  --output encrypted.bin
```

此命令會：

- 從公開輸入生成見證加密金鑰（encap）
- 使用 AES-256-GCM 加密您的訊息
- 將所有內容合併為單一密文檔案
- 在密文中嵌入公開輸入（預設）

輸出：

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

### 5. 生成見證檔案

解密之前，接收者需要生成一個見證來證明他們擁有有效的解答：

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

- 解析合併的密文
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

從電路和公開輸入生成密文和加密金鑰。

```bash
zkenc encap \
  --circuit <R1CS_FILE> \
  --input <JSON_FILE> \
  --ciphertext <OUTPUT_CT> \
  --key <OUTPUT_KEY>
```

**參數：**

- `--circuit <FILE>` - R1CS 電路檔案的路徑（來自 Circom 的 `.r1cs`）
- `--input <FILE>` - 包含公開輸入的 JSON 檔案路徑
- `--ciphertext <FILE>` - 密文的輸出路徑
- `--key <FILE>` - 加密金鑰的輸出路徑

**範例：**

```bash
zkenc encap \
  --circuit sudoku.r1cs \
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

- `--circuit <FILE>` - R1CS 電路檔案的路徑
- `--witness <FILE>` - 見證檔案的路徑（來自 snarkjs 的 `.wtns`）
- `--ciphertext <FILE>` - 密文檔案的路徑
- `--key <FILE>` - 恢復的金鑰輸出路徑

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

使用見證加密來加密訊息（高階、一步操作）。

```bash
zkenc encrypt \
  --circuit <R1CS_FILE> \
  --input <JSON_FILE> \
  --message <MESSAGE_FILE> \
  --output <OUTPUT_FILE> \
  [--no-public-input]
```

**參數：**

- `--circuit <FILE>` - R1CS 電路檔案的路徑（來自 Circom 的 `.r1cs`）
- `--input <FILE>` - 包含公開輸入的 JSON 檔案路徑
- `--message <FILE>` - 明文訊息檔案的路徑
- `--output <FILE>` - 合併密文的輸出路徑
- `--no-public-input` - 不在密文中嵌入公開輸入（選用）

**功能：**

此命令將 encap 和 AES 加密合併為單一步驟：

1. 從公開輸入生成見證加密金鑰
2. 使用 AES-256-GCM 加密訊息
3. 建立合併密文，格式為：`[flag][witnessLen][witnessCT][publicLen][publicInput][encryptedMsg]`

**範例：**

```bash
zkenc encrypt \
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --message secret.txt \
  --output encrypted.bin
```

**相容性：** 輸出與 zkenc-js 的 `decrypt()` 函數完全相容。

---

### `zkenc decrypt`

使用見證解密來解密訊息（高階、一步操作）。

```bash
zkenc decrypt \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --output <OUTPUT_FILE>
```

**參數：**

- `--circuit <FILE>` - R1CS 電路檔案的路徑
- `--witness <FILE>` - 見證檔案的路徑（來自 snarkjs 的 `.wtns`）
- `--ciphertext <FILE>` - 合併密文檔案的路徑
- `--output <FILE>` - 解密訊息的輸出路徑

**功能：**

此命令將 decap 和 AES 解密合併為單一步驟：

1. 解析合併的密文
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

**相容性：** 可以解密由 zkenc-js `encrypt()` 函數建立的檔案。

---

### 低階命令

對於進階使用案例，您可以單獨使用低階 encap/decap 命令：

#### `zkenc encap`

## 了解工作流程

zkenc-cli 提供兩種層級的 API：

### 高階 API（推薦）

簡單的兩步驟流程：

1. **`encrypt`** - 在一個命令中合併 encap + AES 加密

   - 輸入：電路、公開輸入、訊息
   - 輸出：合併密文（與 zkenc-js 相容）

2. **`decrypt`** - 在一個命令中合併 decap + AES 解密
   - 輸入：電路、見證、合併密文
   - 輸出：解密訊息

**優點：**

- 更簡單的工作流程（2 步驟相對於 4 步驟）
- 單一密文檔案管理
- 與 zkenc-js 完全相容
- 公開輸入可嵌入密文中

### 低階 API（進階）

四步驟流程以進行精細控制：

1. **`encap`** - 從公開輸入生成見證加密的密文和金鑰
2. 單獨加密訊息（使用任何 AES 工具）
3. **`decap`** - 使用有效見證恢復金鑰
4. 單獨解密訊息（使用任何 AES 工具）

**使用案例：**

- 自訂加密方案
- 跨多個訊息重複使用金鑰
- 與現有加密管道整合
- 教育目的以了解協定

**注意：** 對於大多數使用案例，建議使用高階 API，因為它確保相容性並簡化工作流程。

## 輸入檔案格式

### R1CS 電路檔案（`.r1cs`）

由 Circom 編譯器生成：

```bash
circom circuit.circom --r1cs --wasm --sym
```

### 見證檔案（`.wtns`）

由 snarkjs 從您的完整輸入生成：

```bash
# 從輸入計算見證
snarkjs wtns calculate circuit.wasm input.json witness.wtns

# 驗證見證（選用）
snarkjs wtns check circuit.r1cs witness.wtns
```

### 輸入 JSON 檔案

JSON 物件，以訊號名稱作為鍵：

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
- 訊號名稱必須與電路中定義的名稱匹配
- 對於 `encrypt`，僅提供公開輸入
- 對於 `decrypt`，提供從完整輸入（公開 + 私密）生成的見證檔案

## 合併密文格式

`encrypt` 命令建立具有以下結構的合併密文：

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

- 自包含密文（包含所有必要資料）
- 跨工具相容性
- 選用的公開輸入嵌入

## 處理二進位檔案

### 加密二進位檔案

您可以使用高階 API 加密任何檔案類型：

```bash
# 一步加密圖片
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message photo.jpg \
  --output encrypted_photo.bin

# 擁有見證的人一步解密圖片
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted_photo.bin \
  --output decrypted_photo.jpg
```

### 使用低階 API 處理二進位檔案

對於進階使用案例：

```bash
# 步驟 1：從電路生成金鑰
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness_ct.bin \
  --key key.bin

# 步驟 2：使用外部工具或自訂方法加密
# （key.bin 是適用於 AES-256 的 32 位元組金鑰）

# 步驟 3：接收者恢復金鑰
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness_ct.bin \
  --key recovered_key.bin

# 步驟 4：使用步驟 2 中使用的相同方法解密
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
- 您將單獨分發公開輸入
- 您想要更小的密文檔案

**注意：** 接收者需要公開輸入來驗證見證。

### 批次處理

為相同的電路和公開輸入加密多個訊息：

```bash
# 使用嵌入的公開輸入加密多個檔案
for file in documents/*.txt; do
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input public.json \
    --message "$file" \
    --output "encrypted/$(basename $file).enc"
done
```

每個加密檔案都是自包含的，可以獨立解密。

### 跨工具相容性

zkenc-cli 與 zkenc-js **完全相容**！您可以使用一個工具加密，用另一個工具解密：

**CLI → JS：**

```bash
# 使用 CLI 加密
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin

# 在 Node.js 或瀏覽器中使用 zkenc-js 解密
# encrypted.bin 可以由 zkenc-js decrypt() 讀取
```

**JS → CLI：**

```bash
# 使用 zkenc-js encrypt() 加密後...
# 使用 CLI 解密
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

兩個工具使用相同的合併密文格式，確保無縫互通性。

[了解更多關於跨工具工作流程 →](/docs/guides/cross-tool-workflow)

## 效能提示

1. **使用高階 API**：`encrypt`/`decrypt` 命令有效處理一切
2. **嵌入公開輸入**：保持密文自包含（預設行為）
3. **預先編譯電路**：編譯電路一次，多次重複使用
4. **考慮電路大小**：較大的電路 = 較慢的 encap/decap 操作
5. **二進位格式**：所有檔案使用高效的二進位序列化

## 常見模式

### 條件存取控制

```bash
# 只有解決謎題的使用者才能解密
zkenc encrypt \
  --circuit puzzle.r1cs \
  --input question.json \
  --message "Secret answer: 42" \
  --output secret.bin
```

### 時間鎖加密

```bash
# 需要計算工作來生成見證
zkenc encrypt \
  --circuit timelock.r1cs \
  --input params.json \
  --message future_message.txt \
  --output locked.bin
```

### 分發加密檔案

```bash
# 使用嵌入的公開輸入加密
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message secret.txt \
  --output package.bin

# 公開分享 package.bin
# 只有能夠生成有效見證的人才能解密
```

## 下一步

- **[API 參考 →](/docs/api/zkenc-cli)** - 完整的 CLI 命令參考
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 將 CLI 與 zkenc-js 一起使用
- **[zkenc-js 入門 →](/docs/getting-started/zkenc-js)** - JavaScript 替代方案

## 疑難排解

### "Circuit file not found"（找不到電路檔案）

確保 R1CS 檔案路徑正確：

```bash
# 檢查檔案是否存在
ls -lh circuit.r1cs
```

### "Invalid inputs"（無效輸入）

檢查您的 JSON 檔案：

- 是有效的 JSON 格式
- 包含所有必需的訊號名稱
- 對所有數字使用字串值

```bash
# 驗證 JSON
cat inputs.json | jq .
```

### "Invalid ciphertext: too short"（無效密文：太短）

這表示密文檔案已損壞或不是有效的 zkenc 密文。確保：

- 檔案由 zkenc-cli `encrypt` 或 zkenc-js `encrypt()` 建立
- 檔案未被修改或截斷
- 您使用的是正確的檔案

### "Decap failed"（Decap 失敗）

這通常表示：

- 見證不滿足電路約束
- 見證檔案已損壞
- 使用錯誤的電路檔案
- 見證與用於加密的公開輸入不匹配

首先驗證您的見證：

```bash
snarkjs wtns check circuit.r1cs witness.wtns
```

### "Decryption failed" 或 "Message decryption failed"（解密失敗或訊息解密失敗）

確保：

- 見證滿足電路約束
- 密文檔案未損壞
- 使用正確的電路檔案
- 見證與加密時的公開輸入匹配

## 支援

如有問題或疑問：

1. 查看 [API 參考](/docs/api/zkenc-cli)
2. 查看[範例工作流程](/docs/guides/cross-tool-workflow)
3. 在 [GitHub](https://github.com/flyinglimao/zkenc) 上開啟問題
