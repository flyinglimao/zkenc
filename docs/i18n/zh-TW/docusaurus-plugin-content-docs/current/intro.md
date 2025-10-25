---
sidebar_position: 1
---

# zkenc 簡介

zkenc 是一個用於 Circom 電路的見證加密函式庫。它讓你能夠加密訊息，使得訊息只能在提供有效見證（解答）給指定電路時才能解密。

## 什麼是見證加密？

見證加密允許你在電路約束條件下加密訊息。加密的訊息只能由知道滿足電路的有效見證（輸入）的人解密。這特別適用於：

- **條件存取**：加密只有在特定條件滿足時才能存取的資料
- **零知識謎題**：建立在解開時才會揭露秘密的加密謎題
- **時間鎖加密**：加密只有在執行特定計算後才能解密的訊息

## 專案狀態

zkenc 實作已經經過初步的安全驗證。雖然此函式庫提供了 Circom 電路見證加密的實用實作，並在大多數情境下驗證了安全性，我們目前正在處理需要額外安全考量的邊緣案例。我們的論文仍在持續改進實作的過程中進行修訂。

## 可用套件

zkenc 包含三個主要元件：

### zkenc-core (Rust)

實作見證加密密碼學原語的核心 Rust 函式庫。

- 低階加密/解密操作
- 電路處理與見證驗證
- CLI 和 JavaScript 綁定的基礎

[了解更多 →](/docs/api/zkenc-core)

### zkenc-cli (Rust)

用於見證加密操作的命令列介面。

- 從命令列加密訊息
- 使用有效見證解密密文
- 與 zkenc-js 互通

[了解更多 →](/docs/api/zkenc-cli)

### zkenc-js (JavaScript/TypeScript)

使用 WebAssembly 從 Rust 編譯的 JavaScript/TypeScript 綁定。

- 可在 Node.js 和瀏覽器中運作
- 高階和低階 API
- 完整的 TypeScript 支援

[了解更多 →](/docs/api/zkenc-js)

## 快速開始

選擇你偏好的套件開始使用：

- **JavaScript/TypeScript 專案**：[zkenc-js 快速開始 →](/docs/getting-started/zkenc-js)
- **命令列使用**：[zkenc-cli 快速開始 →](/docs/getting-started/zkenc-cli)

## 互動式實驗場

在我們的互動式數獨謎題實驗場中，於瀏覽器試用 zkenc：

[開啟實驗場 →](/playground)

## 架構

zkenc 採用雙層架構建置：

```
┌─────────────────────────────────────────────────────┐
│              應用層                                  │
│                                                     │
│  ┌───────────────────┐    ┌───────────────────┐   │
│  │   zkenc-cli       │    │    zkenc-js       │   │
│  │   (Rust)          │    │    (WASM)         │   │
│  │                   │    │                   │   │
│  │ • 命令列          │    │ • 瀏覽器與        │   │
│  │ • 批次處理        │    │   Node.js         │   │
│  │                   │    │ • TypeScript API  │   │
│  └─────────┬─────────┘    └─────────┬─────────┘   │
│            │                        │             │
│            └────────────┬───────────┘             │
└─────────────────────────┼─────────────────────────┘
                          │
            ┌─────────────▼──────────────┐
            │      zkenc-core            │
            │      (Rust)                │
            │                            │
            │ • 密碼學原語               │
            │ • R1CS → QAP 轉換          │
            │ • BN254 曲線支援           │
            │ • Blake3 KDF               │
            └────────────────────────────┘
```

**核心層：** zkenc-core 使用 arkworks 提供密碼學基礎，處理 R1CS 到 QAP 的轉換、加密/解密原語，以及金鑰衍生。

**應用層：** zkenc-cli（命令列工具）和 zkenc-js（WASM 綁定）都建立在 zkenc-core 之上，為相同的底層功能提供不同的介面。

## 跨工具相容性

zkenc-cli 和 zkenc-js 完全互通。你可以：

- 使用 zkenc-cli 加密，用 zkenc-js 解密
- 使用 zkenc-js 加密，用 zkenc-cli 解密
- 在不同環境之間共享密文

[了解跨工具工作流程 →](/docs/guides/cross-tool-workflow)

## 下一步

1. **[開始使用](/docs/getting-started/zkenc-js)** - 安裝並嘗試你的第一次加密
2. **[API 參考](/docs/api/zkenc-js)** - 探索完整的 API
3. **[指南](/docs/guides/intro)** - 跟隨逐步整合指南
4. **[實驗場](/playground)** - 用數獨謎題範例進行實驗
