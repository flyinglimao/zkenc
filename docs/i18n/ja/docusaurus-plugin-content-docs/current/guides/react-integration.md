---
sidebar_position: 3
---

# React 統合ガイド

zkenc-js を使用して証拠暗号化を実装したインタラクティブな React アプリケーションを構築します。

## 構築するもの

以下の機能を持つ React アプリ:

- ブラウザでメッセージを暗号化・復号化
- 数独パズルを回路として使用
- 直感的な UI
- ファイルのアップロードとダウンロードの処理

## 前提条件

- Node.js 18+
- React と TypeScript の基本知識
- Circom でコンパイルされた回路ファイル

## ステップ 1:プロジェクトセットアップ

新しい Vite + React + TypeScript プロジェクトを作成:

```bash
npm create vite@latest zkenc-react-app -- --template react-ts
cd zkenc-react-app
npm install
```

zkenc-js をインストール:

```bash
npm install zkenc-js
```

## ステップ 2:回路ファイルの追加

コンパイル済みの回路ファイルを`public/circuits/`にコピー:

```
public/
└── circuits/
    ├── simple.r1cs
    └── simple.wasm
```

これにより、ブラウザが fetch 経由でファイルをロードできるようになります。

## ステップ 3:回路ローダーの作成

`src/utils/circuit.ts`を作成:

```typescript
import { CircuitFiles } from "zkenc-js";

export async function loadCircuitFiles(): Promise<CircuitFiles> {
  const [r1csResponse, wasmResponse] = await Promise.all([
    fetch("/circuits/simple.r1cs"),
    fetch("/circuits/simple.wasm"),
  ]);

  if (!r1csResponse.ok || !wasmResponse.ok) {
    throw new Error("回路ファイルのロードに失敗しました");
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

## ステップ 4:暗号化コンポーネントの作成

`src/components/EncryptionForm.tsx`を作成:

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
      // 回路ファイルをロード
      const circuitFiles = await loadCircuitFiles();

      // 入力を準備
      const publicInputs = { publicValue };
      const messageBytes = new TextEncoder().encode(message);

      // 暗号化
      const result = await zkenc.encrypt(
        circuitFiles,
        publicInputs,
        messageBytes
      );

      setCiphertext(result.ciphertext);
    } catch (err) {
      setError(err instanceof Error ? err.message : "暗号化に失敗しました");
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
      <h2>メッセージを暗号化</h2>

      <div className="form-group">
        <label>メッセージ:</label>
        <textarea
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          placeholder="秘密のメッセージを入力..."
          rows={4}
        />
      </div>

      <div className="form-group">
        <label>公開値:</label>
        <input
          type="number"
          value={publicValue}
          onChange={(e) => setPublicValue(Number(e.target.value))}
        />
        <small>
          注意: 復号化するには秘密値は{100 - publicValue}でなければなりません
        </small>
      </div>

      <button onClick={handleEncrypt} disabled={loading || !message}>
        {loading ? "暗号化中..." : "暗号化"}
      </button>

      {error && <div className="error">{error}</div>}

      {ciphertext && (
        <div className="success">
          <p>✅ 暗号化に成功しました!</p>
          <p>暗号文サイズ: {ciphertext.length} バイト</p>
          <button onClick={handleDownload}>暗号文をダウンロード</button>
        </div>
      )}
    </div>
  );
}
```

## ステップ 5:復号化コンポーネントの作成

`src/components/DecryptionForm.tsx`を作成:

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
      setError("暗号文ファイルをアップロードしてください");
      return;
    }

    if (publicValue + privateValue !== 100) {
      setError(`無効なウィットネス: ${publicValue} + ${privateValue} ≠ 100`);
      return;
    }

    setLoading(true);
    setError("");
    setDecrypted("");

    try {
      // 回路ファイルをロード
      const circuitFiles = await loadCircuitFiles();

      // 完全な入力を準備
      const fullInputs = {
        publicValue,
        privateValue,
      };

      // 復号化
      const decryptedBytes = await zkenc.decrypt(
        circuitFiles,
        ciphertext,
        fullInputs
      );

      const message = new TextDecoder().decode(decryptedBytes);
      setDecrypted(message);
    } catch (err) {
      setError(err instanceof Error ? err.message : "復号化に失敗しました");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="decryption-form">
      <h2>メッセージを復号化</h2>

      <div className="form-group">
        <label>暗号文ファイル:</label>
        <input type="file" onChange={handleFileUpload} accept=".bin" />
        {ciphertext && <small>ロード済み: {ciphertext.length} バイト</small>}
      </div>

      <div className="form-group">
        <label>公開値:</label>
        <input
          type="number"
          value={publicValue}
          onChange={(e) => setPublicValue(Number(e.target.value))}
        />
      </div>

      <div className="form-group">
        <label>秘密値:</label>
        <input
          type="number"
          value={privateValue}
          onChange={(e) => setPrivateValue(Number(e.target.value))}
        />
        <small>
          満たす必要があります: {publicValue} + {privateValue} = 100
        </small>
      </div>

      <button onClick={handleDecrypt} disabled={loading || !ciphertext}>
        {loading ? "復号化中..." : "復号化"}
      </button>

      {error && <div className="error">{error}</div>}

      {decrypted && (
        <div className="success">
          <h3>✅ 復号化されたメッセージ:</h3>
          <pre>{decrypted}</pre>
        </div>
      )}
    </div>
  );
}
```

## ステップ 6:メインアプリの作成

`src/App.tsx`を更新:

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
        <h1>zkencデモ</h1>
        <p>ブラウザでの証拠暗号化</p>
      </header>

      <div className="mode-selector">
        <button
          className={mode === "encrypt" ? "active" : ""}
          onClick={() => setMode("encrypt")}
        >
          暗号化
        </button>
        <button
          className={mode === "decrypt" ? "active" : ""}
          onClick={() => setMode("decrypt")}
        >
          復号化
        </button>
      </div>

      <main>
        {mode === "encrypt" ? <EncryptionForm /> : <DecryptionForm />}
      </main>

      <footer>
        <p>Powered by zkenc-js</p>
      </footer>
    </div>
  );
}

export default App;
```

`src/App.css`に基本的なスタイルを追加:

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

## ステップ 7:Vite の設定

WASM を処理するために`vite.config.ts`を更新:

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

## ステップ 8:実行とテスト

開発サーバーを起動:

```bash
npm run dev
```

`http://localhost:5173`にアクセスして試してみましょう:

1. **暗号化:**

   - メッセージを入力: "Hello, zkenc!"
   - 公開値: 30
   - 「暗号化」をクリック
   - 暗号文をダウンロード

2. **復号化:**
   - 暗号文ファイルをアップロード
   - 公開値: 30
   - 秘密値: 70
   - 「復号化」をクリック
   - 復号化されたメッセージを確認!

## 高度:Web Workers

パフォーマンスを向上させるために、Web Workers を使用してメインスレッドのブロックを避けます。

`src/workers/zkenc.worker.ts`を作成:

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
      error: error instanceof Error ? error.message : "不明なエラー",
    });
  }
};
```

コンポーネントで worker を使用:

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

## 本番環境ビルド

本番環境用にビルド:

```bash
npm run build
```

`dist/`フォルダに本番環境用のアプリが含まれます。

## デプロイ

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

`vite.config.ts`に追加:

```typescript
export default defineConfig({
  base: "/zkenc-react-app/",
  // ... 残りの設定
});
```

ビルドとデプロイ:

```bash
npm run build
npx gh-pages -d dist
```

## 次のステップ

- **[Node.js 統合 →](/docs/guides/nodejs-integration)** - サーバーサイド暗号化
- **[クロスツールワークフロー →](/docs/guides/cross-tool-workflow)** - CLI と JS を組み合わせる
- **[プレイグラウンド →](/playground)** - 完全な例を見る
- **[API リファレンス →](/docs/api/zkenc-js)** - すべての関数を探索

## トラブルシューティング

**回路ファイルが 404:**

- ファイルが`public/circuits/`にあることを確認
- ブラウザ DevTools のネットワークタブを確認
- ファイル名が正確に一致することを確認

**WASM の初期化に失敗する:**

- vite.config.ts の`optimizeDeps.exclude`に`zkenc-js`を追加
- Vite キャッシュをクリア: `rm -rf node_modules/.vite`

**パフォーマンスが遅い:**

- 重い操作には Web Workers を使用
- 最初のロード後に回路ファイルをキャッシュ
- 回路の複雑さを考慮

**メモリの問題:**

- 大きな回路はより多くのメモリを使用
- 大きなファイルにはストリーミングを使用
- 非常に大きなファイルにはサーバーサイド暗号化を検討
