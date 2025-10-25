---
sidebar_position: 1
---

# zkenc 介绍

zkenc 是一个用于 Circom 电路的见证加密库。它可以让您加密消息，使其只能通过提供指定电路的有效见证（解答）来解密。

## 什么是见证加密？

见证加密允许您在电路约束下加密消息。加密的消息只能由知道满足电路的有效见证（输入）的人解密。这对以下场景特别有用：

- **条件式访问**：加密只有在满足特定条件时才能访问的数据
- **零知识谜题**：创建在解决时揭示秘密的加密谜题
- **时间锁加密**：加密只有在执行特定计算后才能解密的消息

## 项目状态

zkenc 实现已经过初步安全验证。虽然该库提供了 Circom 电路见证加密的实用实现，且在大多数情况下已验证安全性，但我们目前正在处理一些需要额外安全考量的边缘案例。我们的论文仍在改进中，随着我们继续强化实现。

## 可用包

zkenc 由三个主要组件组成：

### zkenc-core (Rust)

实现见证加密密码学原语的核心 Rust 库。

- 低级加密/解密操作
- 电路处理和见证验证
- CLI 和 JavaScript 绑定的基础

[深入了解 →](/docs/api/zkenc-core)

### zkenc-cli (Rust)

用于见证加密操作的命令行界面。

- 从命令行加密消息
- 使用有效见证解密密文
- 与 zkenc-js 可互通

[深入了解 →](/docs/api/zkenc-cli)

### zkenc-js (JavaScript/TypeScript)

使用 WebAssembly 从 Rust 编译的 JavaScript/TypeScript 绑定。

- 在 Node.js 和浏览器中工作
- 高级和低级 API
- 完整的 TypeScript 支持

[深入了解 →](/docs/api/zkenc-js)

## 快速开始

选择您偏好的包开始：

- **对于 JavaScript/TypeScript 项目**：[zkenc-js 快速开始 →](/docs/getting-started/zkenc-js)
- **对于命令行使用**：[zkenc-cli 快速开始 →](/docs/getting-started/zkenc-cli)

## 交互式游乐场

在浏览器中使用我们的交互式数独谜题游乐场试用 zkenc：

[打开游乐场 →](/playground)

## 架构

zkenc 采用双层架构构建：

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

**核心层：** zkenc-core 使用 arkworks 提供密码学基础，处理 R1CS 到 QAP 转换、加密/解密原语和密钥派生。

**应用层：** zkenc-cli（命令行工具）和 zkenc-js（WASM 绑定）都建立在 zkenc-core 之上，为相同的底层功能提供不同的接口。

## 跨工具兼容性

zkenc-cli 和 zkenc-js 完全可互通。您可以：

- 使用 zkenc-cli 加密，使用 zkenc-js 解密
- 使用 zkenc-js 加密，使用 zkenc-cli 解密
- 在不同环境间共享密文

[深入了解跨工具工作流程 →](/docs/guides/cross-tool-workflow)

## 下一步

1. **[开始使用](/docs/getting-started/zkenc-js)** - 安装并尝试您的第一次加密
2. **[API 参考](/docs/api/zkenc-js)** - 探索完整的 API
3. **[指南](/docs/guides/intro)** - 遵循逐步集成指南
4. **[游乐场](/playground)** - 使用数独谜题范例进行实验
