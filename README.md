# zkenc-handmade

一個基於 Groth16 風格的零知識證明演算法實作，支援 Native、CLI 和 WASM 環境。

## 📦 專案結構

```
zkenc-handmade/
├── packages/
│   ├── zkenc-core/      # 核心演算法實作（Rust）
│   ├── zkenc-cli/       # 命令列介面工具
│   └── zkenc-js/        # WASM/JavaScript 綁定
├── Cargo.toml           # Rust Workspace 配置
├── package.json         # Node.js/pnpm 配置
└── WORKSPACE_SETUP.md   # 詳細設定指南
```

## 🚀 快速開始

### 安裝依賴

```bash
# 安裝 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安裝 wasm-pack (用於 WASM 編譯)
cargo install wasm-pack

# 安裝 pnpm (如果使用 Node.js 部分)
npm install -g pnpm
```

### 編譯

```bash
# 編譯所有 Rust 套件
cargo build --workspace --release

# 僅編譯核心庫
cargo build -p zkenc-core --release

# 編譯 CLI 工具
cargo build -p zkenc-cli --release

# 編譯 WASM 模組
cd packages/zkenc-js
wasm-pack build --target web
```

### 測試

```bash
# 執行所有測試
cargo test --workspace

# 測試特定套件
cargo test -p zkenc-core
```

## 📚 套件說明

### zkenc-core

核心演算法實作，使用 arkworks 生態系的數學庫。

**特性**:

- ✅ 支援 `no_std` 環境
- ✅ 可編譯為 WASM
- ✅ 基於 Groth16 的數學結構
- ✅ 使用 arkworks 進行橢圓曲線運算

**Features**:

- `std` (預設): 標準庫支援
- `parallel`: 並行計算（Native only）
- `r1cs`: R1CS gadgets 支援
- `wasm`: WASM 環境支援

### zkenc-cli

命令列介面工具，包裝 zkenc-core 的功能。

**使用範例**:

```bash
zkenc --help
zkenc encrypt --input data.txt --output encrypted.bin
zkenc decrypt --input encrypted.bin --output decrypted.txt
```

### zkenc-js

JavaScript/WASM 綁定，可在瀏覽器和 Node.js 中使用。

**使用範例**:

```typescript
import init, { WasmEncryptor } from "./zkenc-js";

await init();
const encryptor = new WasmEncryptor();
const encrypted = encryptor.encrypt(data);
```

## 🛠️ 技術棧

- **語言**: Rust (edition 2021)
- **數學庫**: [arkworks](https://github.com/arkworks-rs) 生態系
  - `ark-ff`: 有限域運算
  - `ark-ec`: 橢圓曲線運算
  - `ark-poly`: 多項式運算
  - `ark-relations`: R1CS 約束系統
  - `ark-snark`: SNARK 抽象層
  - `ark-crypto-primitives`: 密碼學原語
- **WASM**: wasm-bindgen, wasm-pack
- **CLI**: clap 4.5

## 📖 開發指南

詳細的開發指南請參考：

- [WORKSPACE_SETUP.md](./WORKSPACE_SETUP.md) - 完整的設定與使用說明
- [packages/zkenc-core/README.md](./packages/zkenc-core/README.md) - 核心演算法說明
- [packages/zkenc-cli/README.md](./packages/zkenc-cli/README.md) - CLI 使用說明
- [packages/zkenc-js/README.md](./packages/zkenc-js/README.md) - JavaScript API 說明

## 🔧 常用指令

```bash
# 檢查編譯（不產生二進位檔）
cargo check --workspace

# 格式化程式碼
cargo fmt --all

# 執行 linter
cargo clippy --workspace

# 產生文件
cargo doc --workspace --open

# 編譯為 WASM（最小化）
cargo build -p zkenc-core --no-default-features --features "wasm" --target wasm32-unknown-unknown --release
```

## 📝 授權

MIT/Apache-2.0 雙授權
