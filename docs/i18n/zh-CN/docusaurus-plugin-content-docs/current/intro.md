---
sidebar_position: 1
---

# zkenc 简介

zkenc 是一个用于 Circom 电路的见证加密函数库。它让你能够加密消息，使得消息只能在提供有效见证（解答）给指定电路时才能解密。

## 什么是见证加密？

见证加密允许你在电路约束条件下加密消息。加密的消息只能由知道满足电路的有效见证（输入）的人解密。这特别适用于：

- **条件访问**：加密只有在特定条件满足时才能访问的数据
- **零知识谜题**：建立在解开时才会揭露秘密的加密谜题
- **时间锁加密**：加密只有在执行特定计算后才能解密的消息

## 项目状态

zkenc 实现已经经过初步的安全验证。虽然此函数库提供了 Circom 电路见证加密的实用实现，并在大多数情境下验证了安全性，我们目前正在处理需要额外安全考量的边缘案例。我们的论文仍在持续改进实现的过程中进行修订。

## 可用套件

zkenc 包含三个主要组件：

### zkenc-core (Rust)

实现见证加密密码学原语的核心 Rust 函数库。

- 低级加密/解密操作
- 电路处理与见证验证
- CLI 和 JavaScript 绑定的基础

[了解更多 →](/docs/api/zkenc-core)

### zkenc-cli (Rust)

用于见证加密操作的命令行界面。

- 从命令行加密消息
- 使用有效见证解密密文
- 与 zkenc-js 互通

[了解更多 →](/docs/api/zkenc-cli)

### zkenc-js (JavaScript/TypeScript)

使用 WebAssembly 从 Rust 编译的 JavaScript/TypeScript 绑定。

- 可在 Node.js 和浏览器中运作
- 高级和低级 API
- 完整的 TypeScript 支持

[了解更多 →](/docs/api/zkenc-js)

## 快速开始

选择你偏好的套件开始使用：

- **JavaScript/TypeScript 项目**：[zkenc-js 快速开始 →](/docs/getting-started/zkenc-js)
- **命令行使用**：[zkenc-cli 快速开始 →](/docs/getting-started/zkenc-cli)

## 交互式实验场

在我们的交互式数独谜题实验场中，于浏览器试用 zkenc：

[开启实验场 →](/playground)

## 架构

zkenc 采用双层架构建置：

```
┌─────────────────────────────────────────────────────┐
│              应用层                                  │
│                                                     │
│  ┌───────────────────┐    ┌───────────────────┐   │
│  │   zkenc-cli       │    │    zkenc-js       │   │
│  │   (Rust)          │    │    (WASM)         │   │
│  │                   │    │                   │   │
│  │ • 命令行          │    │ • 浏览器与        │   │
│  │ • 批处理          │    │   Node.js         │   │
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
            │ • 密码学原语               │
            │ • R1CS → QAP 转换          │
            │ • BN254 曲线支持           │
            │ • Blake3 KDF               │
            └────────────────────────────┘
```

**核心层：** zkenc-core 使用 arkworks 提供密码学基础，处理 R1CS 到 QAP 的转换、加密/解密原语，以及密钥衍生。

**应用层：** zkenc-cli（命令行工具）和 zkenc-js（WASM 绑定）都建立在 zkenc-core 之上，为相同的底层功能提供不同的界面。

## 跨工具兼容性

zkenc-cli 和 zkenc-js 完全互通。你可以：

- 使用 zkenc-cli 加密，用 zkenc-js 解密
- 使用 zkenc-js 加密，用 zkenc-cli 解密
- 在不同环境之间共享密文

[了解跨工具工作流程 →](/docs/guides/cross-tool-workflow)

## 下一步

1. **[开始使用](/docs/getting-started/zkenc-js)** - 安装并尝试你的第一次加密
2. **[API 参考](/docs/api/zkenc-js)** - 探索完整的 API
3. **[指南](/docs/guides/intro)** - 跟随逐步整合指南
4. **[实验场](/playground)** - 用数独谜题范例进行实验
