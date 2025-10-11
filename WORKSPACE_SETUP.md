# zkenc Workspace 設定指南

這個 workspace 包含三個套件，分別用於不同的用途：

## 套件結構

### 1. zkenc-core

**用途**: 核心演算法實作

**特性**:

- 支援 `no_std` 環境
- 可編譯為 WASM
- 使用 arkworks 生態系的數學庫

**Features**:

- `std` (default): 標準庫支援
- `parallel`: 並行計算支援（僅 native）
- `r1cs`: R1CS gadgets 支援
- `wasm`: WASM 環境支援
- `print-trace`: 除錯追蹤

**編譯方式**:

```bash
# Native (with parallel)
cargo build -p zkenc-core --features "std,parallel"

# WASM
cargo build -p zkenc-core --no-default-features --features "wasm" --target wasm32-unknown-unknown

# No-std
cargo build -p zkenc-core --no-default-features
```

### 2. zkenc-cli

**用途**: 命令列介面工具

**特性**:

- 包裝 zkenc-core 的功能成 CLI
- 使用 clap 做參數解析
- 支援檔案輸入/輸出

**編譯方式**:

```bash
cargo build -p zkenc-cli --release
```

**使用方式**:

```bash
zkenc --help
```

### 3. zkenc-js

**用途**: JavaScript/WASM 綁定

**特性**:

- 將 zkenc-core 包裝成 WASM 模組
- 提供 JavaScript 介面
- 支援 Node.js 和瀏覽器環境

**編譯方式**:

```bash
# 安裝 wasm-pack (如果尚未安裝)
cargo install wasm-pack

# 編譯為 WASM
cd packages/zkenc-js
wasm-pack build --target web
wasm-pack build --target nodejs
wasm-pack build --target bundler
```

**使用方式** (TypeScript/JavaScript):

```typescript
import init, { WasmEncryptor, greet } from "./zkenc-js";

await init();

const encryptor = new WasmEncryptor();
const encrypted = encryptor.encrypt(data);
```

## 依賴說明

### Arkworks 生態系

所有套件都使用 arkworks 的數學庫：

- `ark-ff`: 有限域運算
- `ark-ec`: 橢圓曲線運算
- `ark-poly`: 多項式運算
- `ark-relations`: R1CS 約束系統
- `ark-snark`: SNARK 抽象層
- `ark-crypto-primitives`: 密碼學原語

這些依賴都設定為 `default-features = false`，以支援 `no_std` 環境。

### Feature 組合說明

**Native 開發** (最大功能):

```bash
cargo build --features "std,parallel,r1cs"
```

**WASM 編譯** (最小化):

```bash
cargo build --no-default-features --features "wasm" --target wasm32-unknown-unknown
```

**測試**:

```bash
# 所有測試
cargo test --workspace

# 特定套件
cargo test -p zkenc-core
cargo test -p zkenc-cli

# WASM 測試
cd packages/zkenc-js
wasm-pack test --node
```

## 開發工作流程

1. **實作核心演算法** (`zkenc-core`):

   - 在 `src/algorithm.rs` 中實作主要邏輯
   - 在 `src/data_structures.rs` 中定義資料結構
   - 使用 arkworks 的 API 進行數學運算

2. **包裝 CLI** (`zkenc-cli`):

   - 在 `src/main.rs` 中定義命令列介面
   - 呼叫 zkenc-core 的函數
   - 處理檔案 I/O 和錯誤

3. **建立 WASM 綁定** (`zkenc-js`):
   - 在 `src/lib.rs` 中使用 `#[wasm_bindgen]` 標註
   - 將 zkenc-core 的型別轉換為 JS 相容型別
   - 在 `src/index.ts` 中提供 TypeScript 介面

## 常見問題

### Q: 如何在 WASM 中使用亂數生成？

A: 已經加入 `getrandom = { version = "0.2", features = ["js"] }`，會使用瀏覽器的 `crypto.getRandomValues()`。

### Q: 為什麼編譯很慢？

A: arkworks 依賴使用 git，而非 crates.io。可以考慮：

- 使用 `cargo build --release` 的優化版本
- 啟用 `parallel` feature 加速運算
- 使用 `sccache` 加速重複編譯

### Q: 如何除錯 WASM 程式碼？

A:

1. 使用 `console_error_panic_hook` (已包含)
2. 使用 `wasm-bindgen-test` 進行單元測試
3. 在瀏覽器 DevTools 中查看 console

## 下一步

1. 實作具體的演算法邏輯
2. 撰寫單元測試與整合測試
3. 建立完整的 API 文件
4. 效能優化與基準測試
