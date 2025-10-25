---
sidebar_position: 3
---

# React 整合指南

使用 zkenc-js 建立具有见证加密功能的交互式 React 应用程序。

## 我们要建立什么

一个 React 应用程序：

- 在浏览器中加密和解密消息
- 使用数独谜题作为电路
- 提供直观的 UI
- 处理文件上传和下载

## 前置需求

- Node.js 18+
- 基本的 React 和 TypeScript 知识
- Circom 编译的电路文件

## 步骤 1：项目设置

建立新的 Vite + React + TypeScript 项目：

```bash
npm create vite@latest zkenc-react-app -- --template react-ts
cd zkenc-react-app
npm install
```

安装 zkenc-js：

```bash
npm install zkenc-js
```

## 步骤 2：添加电路文件

将编译好的电路文件复制到 `public/circuits/`：

```
public/
└── circuits/
    ├── simple.r1cs
    └── simple.wasm
```

这让浏览器可以透过 fetch 载入它们。

## 步骤 3：建立电路载入器

建立 `src/utils/circuit.ts`：

```typescript
import { CircuitFiles } from "zkenc-js";

export async function loadCircuitFiles(): Promise<CircuitFiles> {
  const [r1csResponse, wasmResponse] = await Promise.all([
    fetch("/circuits/simple.r1cs"),
    fetch("/circuits/simple.wasm"),
  ]);

  if (!r1csResponse.ok || !wasmResponse.ok) {
    throw new Error("无法载入电路文件");
  }

  const [r1csBuffer, wasmBuffer] = await Promise.all([
    r1csResponse.arrayBuffer(),
    wasmResponse.arrayBuffer(),
  ]);

  return {
    r1csBuffer: new Uint8Array(r1csBuffer),
    wasmBuffer: new Uint8Array(wasmBuffer),
  };
}
```

## 步骤 4：建立加密组件

建立 `src/components/EncryptionForm.tsx`：

```typescript
import { useState } from "react";
import { zkenc } from "zkenc-js";
import { loadCircuitFiles } from "../utils/circuit";

export function EncryptionForm() {
  const [message, setMessage] = useState("");
  const [publicValue, setPublicValue] = useState(30);
  const [loading, setLoading] = useState(false);
  const [ciphertext, setCiphertext] = useState<Uint8Array | null>(null);
  const [error, setError] = useState("");

  const handleEncrypt = async () => {
    setLoading(true);
    setError("");

    try {
      // 载入电路文件
      const circuitFiles = await loadCircuitFiles();

      // 准备输入
      const publicInputs = { publicValue };
      const messageBytes = new TextEncoder().encode(message);

      // 加密
      const result = await zkenc.encrypt(
        circuitFiles,
        publicInputs,
        messageBytes
      );

      setCiphertext(result.ciphertext);
    } catch (err) {
      setError(err instanceof Error ? err.message : "加密失败");
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = () => {
    if (!ciphertext) return;

    const blob = new Blob([ciphertext], { type: "application/octet-stream" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "encrypted.bin";
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="encryption-form">
      <h2>加密消息</h2>

      <div className="form-group">
        <label>消息：</label>
        <textarea
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          placeholder="输入你的秘密消息..."
          rows={4}
        />
      </div>

      <div className="form-group">
        <label>公开值：</label>
        <input
          type="number"
          value={publicValue}
          onChange={(e) => setPublicValue(Number(e.target.value))}
        />
        <small>注意：私密值必须是 {100 - publicValue} 才能解密</small>
      </div>

      <button onClick={handleEncrypt} disabled={loading || !message}>
        {loading ? "加密中..." : "加密"}
      </button>

      {error && <div className="error">{error}</div>}

      {ciphertext && (
        <div className="success">
          <p>✅ 加密成功！</p>
          <p>密文大小：{ciphertext.length} 字节</p>
          <button onClick={handleDownload}>下载密文</button>
        </div>
      )}
    </div>
  );
}
```

## 步骤 5：建立解密组件

建立 `src/components/DecryptionForm.tsx`：

```typescript
import { useState } from "react";
import { zkenc } from "zkenc-js";
import { loadCircuitFiles } from "../utils/circuit";

export function DecryptionForm() {
  const [publicValue, setPublicValue] = useState(30);
  const [privateValue, setPrivateValue] = useState(70);
  const [ciphertext, setCiphertext] = useState<Uint8Array | null>(null);
  const [loading, setLoading] = useState(false);
  const [decrypted, setDecrypted] = useState("");
  const [error, setError] = useState("");

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      const arrayBuffer = reader.result as ArrayBuffer;
      setCiphertext(new Uint8Array(arrayBuffer));
    };
    reader.readAsArrayBuffer(file);
  };

  const handleDecrypt = async () => {
    if (!ciphertext) {
      setError("请上传密文文件");
      return;
    }

    if (publicValue + privateValue !== 100) {
      setError(`无效的见证：${publicValue} + ${privateValue} ≠ 100`);
      return;
    }

    setLoading(true);
    setError("");
    setDecrypted("");

    try {
      // 载入电路文件
      const circuitFiles = await loadCircuitFiles();

      // 准备完整输入
      const fullInputs = {
        publicValue,
        privateValue,
      };

      // 解密
      const decryptedBytes = await zkenc.decrypt(
        circuitFiles,
        ciphertext,
        fullInputs
      );

      const message = new TextDecoder().decode(decryptedBytes);
      setDecrypted(message);
    } catch (err) {
      setError(err instanceof Error ? err.message : "解密失败");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="decryption-form">
      <h2>解密消息</h2>

      <div className="form-group">
        <label>密文文件：</label>
        <input type="file" onChange={handleFileUpload} accept=".bin" />
        {ciphertext && <small>已载入：{ciphertext.length} 字节</small>}
      </div>

      <div className="form-group">
        <label>公开值：</label>
        <input
          type="number"
          value={publicValue}
          onChange={(e) => setPublicValue(Number(e.target.value))}
        />
      </div>

      <div className="form-group">
        <label>私密值：</label>
        <input
          type="number"
          value={privateValue}
          onChange={(e) => setPrivateValue(Number(e.target.value))}
        />
        <small>
          必须满足：{publicValue} + {privateValue} = 100
        </small>
      </div>

      <button onClick={handleDecrypt} disabled={loading || !ciphertext}>
        {loading ? "解密中..." : "解密"}
      </button>

      {error && <div className="error">{error}</div>}

      {decrypted && (
        <div className="success">
          <h3>✅ 解密消息：</h3>
          <pre>{decrypted}</pre>
        </div>
      )}
    </div>
  );
}
```
## 步骤 6：建立主应用程序

更新 `src/App.tsx`：

```typescript
import { useState } from "react";
import { EncryptionForm } from "./components/EncryptionForm";
import { DecryptionForm } from "./components/DecryptionForm";
import "./App.css";

function App() {
  const [mode, setMode] = useState<"encrypt" | "decrypt">("encrypt");

  return (
    <div className="app">
      <header>
        <h1>zkenc 示范</h1>
        <p>浏览器中的见证加密</p>
      </header>

      <div className="mode-selector">
        <button
          className={mode === "encrypt" ? "active" : ""}
          onClick={() => setMode("encrypt")}
        >
          加密
        </button>
        <button
          className={mode === "decrypt" ? "active" : ""}
          onClick={() => setMode("decrypt")}
        >
          解密
        </button>
      </div>

      <main>
        {mode === "encrypt" ? <EncryptionForm /> : <DecryptionForm />}
      </main>

      <footer>
        <p>由 zkenc-js 驱动</p>
      </footer>
    </div>
  );
}

export default App;
```

在 `src/App.css` 中添加基本样式：

```css
.app {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}

header {
  text-align: center;
  margin-bottom: 2rem;
}

.mode-selector {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin-bottom: 2rem;
}

.mode-selector button {
  padding: 0.5rem 2rem;
  font-size: 1rem;
  cursor: pointer;
}

.mode-selector button.active {
  background: #007bff;
  color: white;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: bold;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 0.5rem;
  font-size: 1rem;
}

.error {
  color: red;
  margin-top: 1rem;
}

.success {
  background: #d4edda;
  border: 1px solid #c3e6cb;
  padding: 1rem;
  margin-top: 1rem;
  border-radius: 4px;
}
```

## 步骤 7：配置 Vite

更新 `vite.config.ts` 以处理 WASM：

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  optimizeDeps: {
    exclude: ["zkenc-js"],
  },
});
```

## 步骤 8：执行和测试

启动开发服务器：

```bash
npm run dev
```

访问 `http://localhost:5173` 并试用：

1. **加密：**

   - 输入消息："Hello, zkenc!"
   - 公开值：30
   - 点击「加密」
   - 下载密文

2. **解密：**
   - 上传密文文件
   - 公开值：30
   - 私密值：70
   - 点击「解密」
   - 查看解密的消息！

## 进阶：Web Workers

为了更好的效能，使用 Web Workers 来避免阻塞主执行绪。

建立 `src/workers/zkenc.worker.ts`：

```typescript
import { zkenc, CircuitFiles } from "zkenc-js";

self.onmessage = async (e) => {
  const { type, data } = e.data;

  try {
    if (type === "encrypt") {
      const { circuitFiles, publicInputs, message } = data;
      const result = await zkenc.encrypt(circuitFiles, publicInputs, message);
      self.postMessage({ success: true, result });
    } else if (type === "decrypt") {
      const { circuitFiles, ciphertext, inputs } = data;
      const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, inputs);
      self.postMessage({ success: true, decrypted });
    }
  } catch (error) {
    self.postMessage({
      success: false,
      error: error instanceof Error ? error.message : "未知错误",
    });
  }
};
```

在组件中使用 worker：

```typescript
const worker = new Worker(
  new URL("../workers/zkenc.worker.ts", import.meta.url),
  { type: "module" }
);

worker.postMessage({
  type: "encrypt",
  data: { circuitFiles, publicInputs, message },
});

worker.onmessage = (e) => {
  if (e.data.success) {
    setCiphertext(e.data.result.ciphertext);
  } else {
    setError(e.data.error);
  }
};
```

## 生产建置

为生产环境建置：

```bash
npm run build
```

`dist/` 文件夹包含你的生产就绪应用程序。
## 部署

### Vercel

```bash
npm install -g vercel
vercel deploy
```

### Netlify

```bash
npm install -g netlify-cli
netlify deploy --prod
```

### GitHub Pages

在 `vite.config.ts` 中添加：

```typescript
export default defineConfig({
  base: "/zkenc-react-app/",
  // ... 其余配置
});
```

建置和部署：

```bash
npm run build
npx gh-pages -d dist
```

## 下一步

- **[Node.js 整合 →](/docs/guides/nodejs-integration)** - 服务器端加密
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 结合 CLI 和 JS
- **[实验场 →](/playground)** - 查看完整范例
- **[API 参考 →](/docs/api/zkenc-js)** - 探索所有函数

## 疑难排解

**电路文件 404：**

- 确保文件在 `public/circuits/` 中
- 检查浏览器 DevTools 中的网络标签
- 验证文件名称完全相符

**WASM 初始化失败：**

- 在 vite.config.ts 的 `optimizeDeps.exclude` 中添加 `zkenc-js`
- 清除 Vite 缓存：`rm -rf node_modules/.vite`

**效能缓慢：**

- 对繁重操作使用 Web Workers
- 在首次载入后缓存电路文件
- 考虑电路复杂度

**内存问题：**

- 大型电路使用更多内存
- 对大型文件使用串流
- 对非常大的文件考虑服务器端加密
