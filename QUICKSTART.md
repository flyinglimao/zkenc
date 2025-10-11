# 快速開始指南

## 前置需求

```bash
# 安裝 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 安裝 wasm-pack（用於 WASM 編譯）
cargo install wasm-pack

# 驗證安裝
rustc --version
cargo --version
wasm-pack --version
```

## 驗證設定

```bash
# 檢查所有套件編譯
cargo check --workspace

# 執行所有測試
cargo test --workspace

# 應該看到：
# test result: ok. 3 passed; 0 failed; 0 ignored
```

## 開始使用

### 1. zkenc-core（核心庫）

```rust
// 在其他 Rust 專案中使用
// Cargo.toml:
// zkenc-core = { path = "../zkenc-handmade/packages/zkenc-core" }

use zkenc_core::{ZkEncAlgorithm, PublicParameters};

fn main() {
    let mut rng = rand::thread_rng();

    // 生成參數
    let params = ZkEncAlgorithm::<Bls12_381>::setup(&mut rng);

    println!("參數已生成！");
}
```

### 2. zkenc-cli（命令列工具）

```bash
# 顯示幫助
cargo run -p zkenc-cli -- --help

# 生成公開參數
cargo run -p zkenc-cli -- setup --output params.bin

# 查看 setup 幫助
cargo run -p zkenc-cli -- setup --help

# 生成證明
cargo run -p zkenc-cli -- prove \
  --params params.bin \
  --witness witness.bin \
  --output proof.bin

# 驗證證明
cargo run -p zkenc-cli -- verify \
  --params params.bin \
  --proof proof.bin \
  --inputs "abc" "def"
```

### 3. zkenc-js（WASM/JavaScript）

```bash
# 進入目錄
cd packages/zkenc-js

# 編譯為 WASM（瀏覽器）
wasm-pack build --target web

# 編譯為 WASM（Node.js）
wasm-pack build --target nodejs

# 產生的檔案在 pkg/ 目錄
ls pkg/
```

**在 HTML 中使用**:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>zkenc Demo</title>
  </head>
  <body>
    <h1>zkenc WASM Demo</h1>
    <script type="module">
      import init, { greet, WasmEncryptor } from "./pkg/zkenc_js.js";

      async function run() {
        await init();

        console.log(greet("World"));

        const encryptor = new WasmEncryptor();
        const data = new Uint8Array([1, 2, 3, 4]);
        const encrypted = encryptor.encrypt(data);

        console.log("Encrypted:", encrypted);
      }

      run();
    </script>
  </body>
</html>
```

**在 Node.js 中使用**:

```javascript
const { greet, WasmEncryptor } = require("./pkg/zkenc_js");

console.log(greet("World"));

const encryptor = new WasmEncryptor();
const data = Buffer.from([1, 2, 3, 4]);
const encrypted = encryptor.encrypt(data);

console.log("Encrypted:", encrypted);
```

## 編譯選項

### Native（完整功能）

```bash
# 標準編譯
cargo build -p zkenc-core

# 啟用並行計算
cargo build -p zkenc-core --features "parallel"

# Release 模式（優化）
cargo build -p zkenc-core --release --features "parallel"
```

### WASM（瀏覽器/Node.js）

```bash
# 進入 zkenc-js 目錄
cd packages/zkenc-js

# 開發模式
wasm-pack build --dev --target web

# 生產模式（優化）
wasm-pack build --release --target web

# 不同目標
wasm-pack build --target web      # 瀏覽器
wasm-pack build --target nodejs   # Node.js
wasm-pack build --target bundler  # Webpack/Rollup
```

### No-std（嵌入式）

```bash
# 編譯為 no_std
cargo build -p zkenc-core --no-default-features

# 加入 WASM 支援
cargo build -p zkenc-core \
  --no-default-features \
  --features "wasm" \
  --target wasm32-unknown-unknown
```

## 開發工作流程

### 1. 修改核心演算法

```bash
# 編輯檔案
vim packages/zkenc-core/src/algorithm.rs

# 檢查編譯
cargo check -p zkenc-core

# 執行測試
cargo test -p zkenc-core

# 查看文件
cargo doc -p zkenc-core --open
```

### 2. 更新 CLI

```bash
# 編輯 CLI
vim packages/zkenc-cli/src/main.rs

# 執行 CLI
cargo run -p zkenc-cli -- --help

# 測試特定功能
cargo run -p zkenc-cli -- setup --output test.bin
```

### 3. 更新 WASM 綁定

```bash
# 編輯 WASM 綁定
vim packages/zkenc-js/src/lib.rs

# 重新編譯
cd packages/zkenc-js
wasm-pack build --target web

# 測試
wasm-pack test --node
```

## 常用指令

```bash
# 檢查所有套件
cargo check --workspace

# 格式化程式碼
cargo fmt --all

# 執行 linter
cargo clippy --workspace

# 執行所有測試
cargo test --workspace

# 產生文件
cargo doc --workspace --open

# 清理建置
cargo clean

# 更新依賴
cargo update
```

## 除錯技巧

### Rust 程式碼

```rust
// 使用 dbg! 宏
let result = dbg!(some_function());

// 使用 println!
println!("Debug: {:?}", value);

// 條件編譯（僅在 debug 模式）
#[cfg(debug_assertions)]
println!("Debug info");
```

### WASM 程式碼

```rust
// 在 WASM 中輸出到 console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// 使用
log(&format!("Debug: {:?}", value));
```

### CLI 程式碼

```bash
# 執行時顯示更多資訊
RUST_LOG=debug cargo run -p zkenc-cli -- setup --output test.bin

# 使用 backtrace
RUST_BACKTRACE=1 cargo run -p zkenc-cli -- setup --output test.bin
```

## 效能分析

```bash
# 建立 benchmark
cargo bench --workspace

# 使用 flamegraph
cargo install flamegraph
cargo flamegraph --bench benchmark_name

# 檢查二進位大小
cargo build --release
ls -lh target/release/zkenc

# WASM 大小優化
wasm-pack build --release --target web
ls -lh pkg/*.wasm
```

## 問題排解

### 編譯很慢

```bash
# 使用 sccache 加速
cargo install sccache
export RUSTC_WRAPPER=sccache

# 減少並行度（節省記憶體）
cargo build -j 2
```

### WASM 編譯失敗

```bash
# 確認有安裝 wasm32 target
rustup target add wasm32-unknown-unknown

# 確認 wasm-pack 版本
wasm-pack --version

# 重新安裝 wasm-pack
cargo install wasm-pack --force
```

### 依賴衝突

```bash
# 清理並重建
cargo clean
cargo build --workspace

# 查看依賴樹
cargo tree
```

## 下一步

1. 閱讀 [WORKSPACE_SETUP.md](./WORKSPACE_SETUP.md) 了解詳細配置
2. 閱讀 [COMPLETION_REPORT.md](./COMPLETION_REPORT.md) 了解專案狀態
3. 開始實作演算法邏輯（參考 Groth16）
4. 撰寫測試和文件

## 需要幫助？

- 查看 [arkworks 文件](https://docs.arkworks.rs/)
- 參考 [Groth16 實作](https://github.com/arkworks-rs/groth16)
- 閱讀各套件的 README.md
