---
sidebar_position: 3
---

# React 整合指南

使用 zkenc-js 建立具有見證加密功能的互動式 React 應用程式。

## 我們要建立什麼

一個 React 應用程式：

- 在瀏覽器中加密和解密訊息
- 使用數獨謎題作為電路
- 提供直觀的 UI
- 處理檔案上傳和下載

## 前置需求

- Node.js 18+
- 基本的 React 和 TypeScript 知識
- Circom 編譯的電路檔案

## 步驟 1：專案設定

建立新的 Vite + React + TypeScript 專案：

```bash
npm create vite@latest zkenc-react-app -- --template react-ts
cd zkenc-react-app
npm install
```

安裝 zkenc-js：

```bash
npm install zkenc-js
```

## 步驟 2：添加電路檔案

將編譯好的電路檔案複製到 `public/circuits/`：

```
public/
└── circuits/
    ├── simple.r1cs
    └── simple.wasm
```

這讓瀏覽器可以透過 fetch 載入它們。

## 步驟 3：建立電路載入器

建立 `src/utils/circuit.ts`：

```typescript
import { CircuitFiles } from "zkenc-js";

export async function loadCircuitFiles(): Promise<CircuitFiles> {
  const [r1csResponse, wasmResponse] = await Promise.all([
    fetch("/circuits/simple.r1cs"),
    fetch("/circuits/simple.wasm"),
  ]);

  if (!r1csResponse.ok || !wasmResponse.ok) {
    throw new Error("無法載入電路檔案");
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

## 步驟 4：建立加密元件

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
      // 載入電路檔案
      const circuitFiles = await loadCircuitFiles();

      // 準備輸入
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
      setError(err instanceof Error ? err.message : "加密失敗");
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
      <h2>加密訊息</h2>

      <div className="form-group">
        <label>訊息：</label>
        <textarea
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          placeholder="輸入你的秘密訊息..."
          rows={4}
        />
      </div>

      <div className="form-group">
        <label>公開值：</label>
        <input
          type="number"
          value={publicValue}
          onChange={(e) => setPublicValue(Number(e.target.value))}
        />
        <small>
          注意：私密值必須是 {100 - publicValue} 才能解密
        </small>
      </div>

      <button onClick={handleEncrypt} disabled={loading || !message}>
        {loading ? "加密中..." : "加密"}
      </button>

      {error && <div className="error">{error}</div>}

      {ciphertext && (
        <div className="success">
          <p>✅ 加密成功！</p>
          <p>密文大小：{ciphertext.length} 位元組</p>
          <button onClick={handleDownload}>下載密文</button>
        </div>
      )}
    </div>
  );
}
```

## 步驟 5：建立解密元件

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
      setError("請上傳密文檔案");
      return;
    }

    if (publicValue + privateValue !== 100) {
      setError(`無效的見證：${publicValue} + ${privateValue} ≠ 100`);
      return;
    }

    setLoading(true);
    setError("");
    setDecrypted("");

    try {
      // 載入電路檔案
      const circuitFiles = await loadCircuitFiles();

      // 準備完整輸入
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
      setError(err instanceof Error ? err.message : "解密失敗");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="decryption-form">
      <h2>解密訊息</h2>

      <div className="form-group">
        <label>密文檔案：</label>
        <input type="file" onChange={handleFileUpload} accept=".bin" />
        {ciphertext && <small>已載入：{ciphertext.length} 位元組</small>}
      </div>

      <div className="form-group">
        <label>公開值：</label>
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
          必須滿足：{publicValue} + {privateValue} = 100
        </small>
      </div>

      <button onClick={handleDecrypt} disabled={loading || !ciphertext}>
        {loading ? "解密中..." : "解密"}
      </button>

      {error && <div className="error">{error}</div>}

      {decrypted && (
        <div className="success">
          <h3>✅ 解密訊息：</h3>
          <pre>{decrypted}</pre>
        </div>
      )}
    </div>
  );
}
```

## 步驟 6：建立主應用程式

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
        <h1>zkenc 示範</h1>
        <p>瀏覽器中的見證加密</p>
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
        <p>由 zkenc-js 驅動</p>
      </footer>
    </div>
  );
}

export default App;
```

在 `src/App.css` 中添加基本樣式：

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

## 步驟 7：配置 Vite

更新 `vite.config.ts` 以處理 WASM：

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

## 步驟 8：執行和測試

啟動開發伺服器：

```bash
npm run dev
```

造訪 `http://localhost:5173` 並試用：

1. **加密：**

   - 輸入訊息："Hello, zkenc!"
   - 公開值：30
   - 點擊「加密」
   - 下載密文

2. **解密：**
   - 上傳密文檔案
   - 公開值：30
   - 私密值：70
   - 點擊「解密」
   - 查看解密的訊息！

## 進階：Web Workers

為了更好的效能，使用 Web Workers 來避免阻塞主執行緒。

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
      error: error instanceof Error ? error.message : "未知錯誤",
    });
  }
};
```

在元件中使用 worker：

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

## 生產建置

為生產環境建置：

```bash
npm run build
```

`dist/` 資料夾包含你的生產就緒應用程式。

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
  // ... 其餘配置
});
```

建置和部署：

```bash
npm run build
npx gh-pages -d dist
```

## 下一步

- **[Node.js 整合 →](/docs/guides/nodejs-integration)** - 伺服器端加密
- **[跨工具工作流程 →](/docs/guides/cross-tool-workflow)** - 結合 CLI 和 JS
- **[實驗場 →](/playground)** - 查看完整範例
- **[API 參考 →](/docs/api/zkenc-js)** - 探索所有函式

## 疑難排解

**電路檔案 404：**

- 確保檔案在 `public/circuits/` 中
- 檢查瀏覽器 DevTools 中的網路標籤
- 驗證檔案名稱完全相符

**WASM 初始化失敗：**

- 在 vite.config.ts 的 `optimizeDeps.exclude` 中添加 `zkenc-js`
- 清除 Vite 快取：`rm -rf node_modules/.vite`

**效能緩慢：**

- 對繁重操作使用 Web Workers
- 在首次載入後快取電路檔案
- 考慮電路複雜度

**記憶體問題：**

- 大型電路使用更多記憶體
- 對大型檔案使用串流
- 對非常大的檔案考慮伺服器端加密
