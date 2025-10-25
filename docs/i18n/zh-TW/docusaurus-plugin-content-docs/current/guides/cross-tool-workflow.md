---
sidebar_position: 4
---

# 跨工具工作流程指南

學習如何結合使用 zkenc-cli 和 zkenc-js，以獲得最大的靈活性和功能。

## 為什麼要同時使用兩種工具？

結合 zkenc-cli 和 zkenc-js 可實現強大的工作流程：

- **伺服器加密，瀏覽器解密**
- **CLI 用於批次處理，JS 用於 UI**
- **不同環境，相同密文**
- **發揮每個工具的優勢**

## 相容性

zkenc-cli 和 zkenc-js **完全相容**，使用相同的組合密文格式：

✅ 使用 CLI 加密的檔案可以用 JS 解密
✅ 使用 JS 加密的檔案可以用 CLI 解密
✅ 相同的電路檔案可用於兩種工具
✅ 兩種工具使用相同的輸入格式
✅ 不需要檔案格式轉換

**兩種工具都使用相同的組合格式：**

```
[1 位元組旗標][4 位元組見證 CT 長度][見證密文]
[4 位元組公開輸入長度（如果旗標=1）][公開輸入 JSON（如果旗標=1）]
[加密訊息]
```

## 工作流程 1：CLI 加密 → JS 解密

**使用情境：**在伺服器上加密敏感檔案，在 Web 應用程式中解密。

### 步驟 1：準備電路 (CLI)

```bash
# 編譯電路
circom circuit.circom --r1cs --wasm -o build

# 你需要：
# - build/circuit.r1cs（CLI 和 JS 都需要）
# - build/circuit_js/circuit.wasm（CLI 和 JS 都需要）
```

### 步驟 2：建立公開輸入 (CLI)

建立 `public_inputs.json`：

```json
{
  "publicValue": "42"
}
```

### 步驟 3：使用 CLI 加密

```bash
# 一步完成加密（建議）
zkenc encrypt \
  --circuit build/circuit.r1cs \
  --input public_inputs.json \
  --message secret.txt \
  --output encrypted.bin
```

輸出的 `encrypted.bin` 是組合密文，包含：

- 見證加密密文
- 公開輸入（預設嵌入）
- AES 加密訊息

**檔案大小：**

- `encrypted.bin`（組合）≈ 見證 CT（1576 位元組）+ 公開輸入 + 訊息 + 額外開銷

### 步驟 4：使用 JS 解密

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

// 載入組合密文
const ciphertext = await fs.readFile("encrypted.bin");

// 載入電路檔案
const circuitFiles = {
  r1csBuffer: await fs.readFile("build/circuit.r1cs"),
  wasmBuffer: await fs.readFile("build/circuit_js/circuit.wasm"),
};

// 準備完整輸入（公開 + 私密）
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // 見證
};

// 一步完成解密
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

console.log(new TextDecoder().decode(decrypted));
// 輸出：（secret.txt 的內容）
```

**就這樣！**不需要檔案轉換。

## 工作流程 2：JS 加密 → CLI 解密

**使用情境：**在瀏覽器中加密，在伺服器上處理/解密。

### 步驟 1：使用 JS 加密

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
const message = new TextEncoder().encode("來自瀏覽器的秘密");

// 一步完成加密
const { ciphertext } = await zkenc.encrypt(circuitFiles, publicInputs, message);

// 下載密文
const blob = new Blob([ciphertext]);
const url = URL.createObjectURL(blob);
const a = document.createElement("a");
a.href = url;
a.download = "encrypted.bin";
a.click();
```

`ciphertext` 已經是 CLI 可以直接讀取的組合格式。

### 步驟 2：產生見證 (CLI)

建立完整輸入 `full_inputs.json`：

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

使用 snarkjs 產生見證：

```bash
snarkjs wtns calculate \
  build/circuit_js/circuit.wasm \
  full_inputs.json \
  witness.wtns
```

### 步驟 3：使用 CLI 解密

```bash
# 一步完成解密
zkenc decrypt \
  --circuit build/circuit.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt

cat decrypted.txt
# 輸出：來自瀏覽器的秘密
```

**就這樣！**CLI 可以直接讀取 JS 加密的檔案。

## 工作流程 3：混合處理

**使用情境：**使用 CLI 進行批次操作，使用 JS 進行互動式 UI。

### 範例：照片加密服務

**伺服器 (CLI)：**

```bash
#!/bin/bash
# encrypt-photos.sh

for photo in uploads/*.jpg; do
  echo "正在處理 $photo..."

  # 產生唯一的公開輸入
  PUBLIC_VALUE=$(date +%s)
  echo "{\"timestamp\": \"$PUBLIC_VALUE\"}" > inputs.json

  # 一步完成加密
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input inputs.json \
    --message "$photo" \
    --output "${photo}.enc"

  # 儲存中繼資料
  echo "$photo,$PUBLIC_VALUE" >> metadata.csv

  rm inputs.json
done
```

**客戶端 (JS)：**

```typescript
// 解密選定的照片
async function decryptPhoto(photoId: string, privateValue: number) {
  // 取得加密照片（組合格式）
  const response = await fetch(`/api/photos/${photoId}/encrypted`);
  const ciphertext = new Uint8Array(await response.arrayBuffer());

  // 從中繼資料取得公開值
  const metadata = await fetch(`/api/photos/${photoId}/metadata`).then((r) =>
    r.json()
  );

  // 一步完成解密
  const fullInputs = {
    timestamp: metadata.timestamp,
    userSecret: privateValue,
  };

  const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

  // 顯示照片
  const blob = new Blob([decrypted], { type: "image/jpeg" });
  const url = URL.createObjectURL(blob);
  imageElement.src = url;
}
```

**注意：**如果嵌入了公開輸入，可以使用 `getPublicInput()` 從密文中提取：

```typescript
import { getPublicInput } from "zkenc-js";

// 提取嵌入的公開輸入
const publicInputs = getPublicInput(ciphertext);
console.log(publicInputs.timestamp); // 不需要取得中繼資料！
```

## 工作流程 4：多平台分發

**使用情境：**加密一次，在任何平台上解密。

### 設定

```bash
# 編譯電路
circom puzzle.circom --r1cs --wasm -o dist

# 建立分發套件
mkdir -p package/circuits
cp dist/puzzle.r1cs package/circuits/
cp dist/puzzle_js/puzzle.wasm package/circuits/
cp README.md package/
```

### 加密一次

```bash
# 建立謎題
cat > puzzle.json <<EOF
{
  "puzzle": ["5", "3", "0", "0", "7", "0", "0", "0", "0"]
}
EOF

# 加密訊息（建立組合格式）
zkenc encrypt \
  --circuit package/circuits/puzzle.r1cs \
  --input puzzle.json \
  --message treasure.txt \
  --output package/treasure.enc
```

### 分發

```
package/
├── circuits/
│   ├── puzzle.r1cs     # 電路檔案
│   └── puzzle.wasm      # 見證產生器
├── treasure.enc         # 組合密文（兩種工具都能用！）
└── README.md            # 使用說明
```

### 使用者可以用任一工具解密

**CLI 使用者：**

```bash
# 產生解答見證
cat > solution.json <<EOF
{
  "puzzle": ["5", "3", "0", ...],
  "solution": ["5", "3", "4", "6", "7", "8", "9", "1", "2", ...]
}
EOF

snarkjs wtns calculate puzzle.wasm solution.json solution.wtns

# 直接解密
zkenc decrypt \
  --circuit puzzle.r1cs \
  --witness solution.wtns \
  --ciphertext treasure.enc \
  --output treasure.txt
```

**JS 使用者：**

```typescript
// 載入相同的加密檔案
const ciphertext = await fetch('treasure.enc')
  .then(r => r.arrayBuffer())
  .then(b => new Uint8Array(b));

const solution = {
  puzzle: ["5", "3", "0", ...],
  solution: ["5", "3", "4", "6", "7", "8", "9", "1", "2", ...],
};

// 直接解密
const treasure = await zkenc.decrypt(circuitFiles, ciphertext, solution);
```

**不需要轉換！**兩種工具讀取相同的檔案格式。

## 進階：使用低階 API

對於進階使用情境，你仍可以單獨使用低階 `encap`/`decap` 命令：

### CLI 低階命令

```bash
# 步驟 1：產生見證密文和金鑰
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key key.bin

# 步驟 2：使用任何 AES 工具或自訂實作加密
# （key.bin 是適用於 AES-256 的 32 位元組金鑰）

# 步驟 3：解密 - 恢復金鑰
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness.ct \
  --key recovered_key.bin

# 步驟 4：使用步驟 2 中使用的相同方法解密
```

### 何時使用低階 API

- 自訂加密方案
- 與現有加密管線整合
- 教學目的
- 單獨除錯加密/解密

**注意：**對於大多數使用情境，建議使用高階 `encrypt`/`decrypt` 命令。

## 最佳實踐

1. **使用高階 API**：使用 `encrypt`/`decrypt` 命令以確保簡單性和相容性
2. **保持電路檔案一致**：跨工具使用相同的編譯電路檔案
3. **記錄公開輸入**：清楚記錄哪些輸入是公開的，哪些是私密的
4. **嵌入公開輸入**：使用預設行為（嵌入）以獲得自包含的密文
5. **版本控制你的電路**：追蹤電路版本以確保相容性
6. **雙向測試**：始終測試 CLI→JS 和 JS→CLI 工作流程

## 疑難排解

**解密時出現「Invalid ciphertext」：**

- 確保檔案是有效的 zkenc 密文（由 `encrypt` 命令建立）
- 驗證檔案在傳輸過程中沒有損毀
- 檢查你使用的是正確的電路檔案

**「Witness doesn't satisfy constraints」：**

- 驗證加密和解密之間的公開輸入是否匹配
- 檢查私密輸入是否滿足電路約束
- 確保使用相同的電路版本
- 使用 `snarkjs wtns check` 驗證見證

**檔案格式問題：**

- 檔案已經相容 - 不需要轉換！
- 所有檔案操作都使用二進位模式
- 避免使用可能損毀二進位檔案的文字編輯器
- 如需檢查檔案，使用 `xxd` 或 `hexdump`

**公開輸入不匹配：**

- CLI 和 JS 預設都會嵌入公開輸入
- 在 JS 中使用 `getPublicInput()` 從密文提取
- CLI 在解密時會顯示公開輸入（如果已嵌入）

## 下一步

- **[Node.js 指南 →](/docs/guides/nodejs-integration)** - 建立 CLI 工具
- **[React 指南 →](/docs/guides/react-integration)** - 建立 Web UI
- **[API 參考 →](/docs/api/zkenc-js)** - 詳細文件
- **[實驗場 →](/playground)** - 在瀏覽器中試用
