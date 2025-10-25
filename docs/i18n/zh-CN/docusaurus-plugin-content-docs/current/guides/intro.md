---
sidebar_position: 1
---

# 指南概览

欢迎来到 zkenc 指南！这些逐步教程将帮助您将见证加密集成到您的项目中。

## 您将学到什么

这些指南提供在实际应用中使用 zkenc 的完整实用示例：

### 📦 Node.js 集成

学习如何构建完整的 Node.js 应用程序与见证加密。

- 加载和编译 Circom 电路
- 加密和解密文件
- 正确处理电路输入
- 错误处理和最佳实践

[开始 Node.js 指南 →](/docs/guides/nodejs-integration)

### ⚛️ React 集成

构建具有见证加密的交互式 React 应用程序。

- 设置 Vite + React + TypeScript
- 在浏览器中处理电路文件
- 创建加密/解密 UI
- 使用 Web Workers 优化性能

[开始 React 指南 →](/docs/guides/react-integration)

### 🔄 跨工具工作流程

结合使用 zkenc-cli 和 zkenc-js 以获得最大灵活性。

- 使用 CLI 加密，使用 JavaScript 解密
- 跨环境共享密文
- 结合工具优势以适应您的工作流程
- 批处理和自动化

[开始跨工具指南 →](/docs/guides/cross-tool-workflow)

## 前置需求

开始这些指南之前，您应该：

1. **具备基本知识：**

   - JavaScript/TypeScript（用于 JS 指南）
   - 命令行工具（用于 CLI 指南）
   - Circom 电路（基本理解）

2. **安装必要工具：**

   ```bash
   # Node.js (18+)
   node --version

   # Circom
   circom --version

   # zkenc-cli（用于跨工具指南）
   zkenc --help
   ```

3. **准备好电路：**
   - `.circom` 源文件
   - 或预编译的 `.r1cs` 和 `.wasm` 文件

## 指南结构

每个指南遵循以下结构：

1. **设置** - 项目初始化和依赖项
2. **电路准备** - 编译和加载您的电路
3. **实现** - 逐步代码示例
4. **测试** - 验证一切正常工作
5. **优化** - 性能改进
6. **部署** - 生产环境考量

## 示例电路

指南使用这些示例电路：

### 简单示例电路

用于学习的基本电路：

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

### 数独电路

游乐场中使用的实用示例：

```circom
pragma circom 2.0.0;

template Sudoku() {
    signal input puzzle[81];      // 公开：谜题
    signal input solution[81];    // 私有：解答

    // 验证解答有效
    // ... 约束 ...
}

component main = Sudoku();
```

## 常见模式

### 加密模式

```typescript
// 1. 加载电路文件
const circuitFiles = {
  r1csBuffer: await loadFile('circuit.r1cs'),
  wasmBuffer: await loadFile('circuit.wasm'),
};

// 2. 准备公开输入
const publicInputs = { puzzle: [...] };

// 3. 加密
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);
```

### 解密模式

```typescript
// 1. 加载密文
const ciphertext = await loadFile('encrypted.bin');

// 2. 准备完整输入（公开 + 私有）
const fullInputs = {
  puzzle: [...],
  solution: [...],
};

// 3. 解密
const decrypted = await zkenc.decrypt(
  circuitFiles,
  ciphertext,
  fullInputs
);
```

## 获取帮助

如果遇到困难：

1. **查看 API 参考：**

   - [zkenc-js API](/docs/api/zkenc-js)
   - [zkenc-cli API](/docs/api/zkenc-cli)
   - [zkenc-core API](/docs/api/zkenc-core)

2. **试用游乐场：**

   - [交互式数独示例](/playground)

3. **查看示例代码：**

   - 每个指南都包含完整、可运行的示例

4. **开启问题：**
   - [GitHub Issues](https://github.com/flyinglimao/zkenc/issues)

## 选择您的指南

<div className="guides-grid">

### 对于 Node.js 开发者

适合您正在构建：

- CLI 工具
- 后端服务
- 文件加密工具
- 批处理器

[Node.js 集成 →](/docs/guides/nodejs-integration)

### 对于 React 开发者

适合您正在构建：

- Web 应用程序
- 交互式 UI
- 基于浏览器的工具
- 渐进式 Web 应用程序

[React 集成 →](/docs/guides/react-integration)

### 对于自动化

适合您：

- 使用多种工具
- 批处理文件
- 构建管道
- 跨平台工作流程

[跨工具工作流程 →](/docs/guides/cross-tool-workflow)

</div>

## 下一步

准备好开始了吗？选择上面的指南，或：

- **新手？** 从 [zkenc-js 入门](/docs/getting-started/zkenc-js) 开始
- **想要实验？** 试试 [游乐场](/playground)
- **需要 API 细节？** 查看 [API 参考](/docs/api/zkenc-js)

编码愉快！🚀
