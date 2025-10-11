# zkenc-workspace

這是一個包含多個子專案的工作區，旨在實現零知識編碼（Zero-Knowledge Encoding）相關的演算法和工具。

## 子專案

### zkenc-core
- **描述**：核心演算法的實作，包含主要邏輯和功能。
- **位置**：`packages/zkenc-core`
- **使用說明**：請參閱 [zkenc-core/README.md](packages/zkenc-core/README.md)。

### zkenc-cli
- **描述**：將核心演算法包裝成命令行介面（CLI），方便用戶使用。
- **位置**：`packages/zkenc-cli`
- **使用說明**：請參閱 [zkenc-cli/README.md](packages/zkenc-cli/README.md)。

### zkenc-js
- **描述**：將核心演算法包裝成 WebAssembly（WASM），並提供 JavaScript 介面。
- **位置**：`packages/zkenc-js`
- **使用說明**：請參閱 [zkenc-js/README.md](packages/zkenc-js/README.md)。

## 安裝與使用

請根據各子專案的說明文件進行安裝和使用。整體工作區的配置和依賴項已在根目錄的 `package.json` 和 `Cargo.toml` 中定義。

## 貢獻

歡迎任何形式的貢獻！請參閱各子專案的說明文件以了解如何參與。

## 授權

本專案遵循 MIT 授權條款。詳情請參閱 LICENSE 文件。