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
2. **輸入檔案**採用 JSON 格式

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
  --input public_inputs.json \
  --message message.txt \
  --output encrypted.bin
```

此命令會：
- 從公開輸入產生見證加密金鑰（encap）
- 使用 AES-256-GCM 加密你的訊息
- 將所有內容組合成單一密文檔案
- 預設將公開輸入嵌入密文中

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

驗證結果：

```bash
cat decrypted.txt
# 輸出：Hello, zkenc!
```

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

## 支援

若有問題或疑問：

1. 查看 [API 參考](/docs/api/zkenc-cli)
2. 檢閱 [範例工作流程](/docs/guides/cross-tool-workflow)
3. 在 [GitHub](https://github.com/flyinglimao/zkenc) 上開啟 issue
