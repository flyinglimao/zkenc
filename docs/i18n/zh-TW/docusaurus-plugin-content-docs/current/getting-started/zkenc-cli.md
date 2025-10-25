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

