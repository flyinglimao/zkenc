---
sidebar_position: 1
---

# 指南总览

欢迎来到 zkenc 指南！这些逐步教程将帮助你将见证加密整合到你的项目中。

## 你将学到什么

这些指南提供完整、实用的范例，说明如何在实际应用中使用 zkenc：

### 📦 Node.js 整合

学习如何使用见证加密建立完整的 Node.js 应用程序。

- 载入和编译 Circom 电路
- 加密和解密文件
- 正确处理电路输入
- 错误处理和最佳实践

[开始 Node.js 指南 →](/docs/guides/nodejs-integration)

### ⚛️ React 整合

使用见证加密建立交互式 React 应用程序。

- 设置 Vite + React + TypeScript
- 在浏览器中处理电路文件
- 建立加密/解密 UI
- 使用 Web Workers 优化效能

[开始 React 指南 →](/docs/guides/react-integration)

### 🔄 跨工具工作流程

结合使用 zkenc-cli 和 zkenc-js 以获得最大灵活性。

- 使用 CLI 加密，用 JavaScript 解密
- 跨环境共享密文
- 结合工具优势以适应你的工作流程
- 批处理和自动化

[开始跨工具指南 →](/docs/guides/cross-tool-workflow)

## 前置需求

在开始这些指南之前，你应该：

1. **具备基本知识：**

   - JavaScript/TypeScript（用于 JS 指南）
   - 命令行工具（用于 CLI 指南）
   - Circom 电路（基本理解）

2. **安装所需工具：**

   ```bash
   # Node.js (18+)
   node --version

   # Circom
   circom --version

   # zkenc-cli（用于跨工具指南）
   zkenc --help
   ```

3. **准备好电路：**
   - `.circom` 源码文件
   - 或预先编译的 `.r1cs` 和 `.wasm` 文件

## 指南结构

每个指南遵循以下结构：

1. **设置** - 项目初始化和依赖项
2. **电路准备** - 编译和载入你的电路
3. **实现** - 逐步代码范例
4. **测试** - 验证一切正常运作
5. **优化** - 效能改进
6. **部署** - 生产环境考量

## 范例电路

指南使用这些范例电路：

### 简单范例电路

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

实验场中使用的实用范例：

```circom
pragma circom 2.0.0;

template Sudoku() {
    signal input puzzle[81];      // 公开：谜题
    signal input solution[81];    // 私密：解答

    // 验证解答有效
    // ... 约束 ...
}

component main = Sudoku();
```

## 常见模式

### 加密模式

```typescript
// 1. 载入电路文件
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
// 1. 载入密文
const ciphertext = await loadFile('encrypted.bin');

// 2. 准备完整输入（公开 + 私密）
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

## 取得协助

如果遇到困难：

1. **查看 API 参考：**

   - [zkenc-js API](/docs/api/zkenc-js)
   - [zkenc-cli API](/docs/api/zkenc-cli)
   - [zkenc-core API](/docs/api/zkenc-core)

2. **试用实验场：**

   - [交互式数独范例](/playground)

3. **检阅范例代码：**

   - 每个指南都包含完整、可执行的范例

4. **开启 Issue：**
   - [GitHub Issues](https://github.com/flyinglimao/zkenc/issues)

## 选择你的指南

<div className="guides-grid">

### 给 Node.js 开发者

适合用于建立：

- CLI 工具
- 后端服务
- 文件加密工具
- 批处理器

[Node.js 整合 →](/docs/guides/nodejs-integration)

### 给 React 开发者

适合用于建立：

- Web 应用程序
- 交互式 UI
- 基于浏览器的工具
- Progressive Web Apps

[React 整合 →](/docs/guides/react-integration)

### 给自动化需求

适合用于：

- 使用多种工具
- 批处理文件
- 建立流程
- 跨平台工作流程

[跨工具工作流程 →](/docs/guides/cross-tool-workflow)

</div>

## 下一步

准备好开始了吗？选择上方的指南，或者：

- **刚接触 zkenc？** 从 [zkenc-js 快速开始](/docs/getting-started/zkenc-js) 开始
- **想要实验？** 试试 [实验场](/playground)
- **需要 API 详情？** 查看 [API 参考](/docs/api/zkenc-js)

祝你编码愉快！🚀
