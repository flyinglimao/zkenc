---
sidebar_position: 3
---

# React Integration Guide

Build an interactive React application with witness encryption using zkenc-js.

## What We'll Build

A React app that:

- Encrypts and decrypts messages in the browser
- Uses a Sudoku puzzle as the circuit
- Provides an intuitive UI
- Handles file uploads and downloads

## Prerequisites

- Node.js 18+
- Basic React and TypeScript knowledge
- Circom compiled circuit files

## Step 1: Project Setup

Create a new Vite + React + TypeScript project:

```bash
npm create vite@latest zkenc-react-app -- --template react-ts
cd zkenc-react-app
npm install
```

Install zkenc-js:

```bash
npm install zkenc-js
```

## Step 2: Add Circuit Files

Copy your compiled circuit files to `public/circuits/`:

```
public/
└── circuits/
    ├── simple.r1cs
    └── simple.wasm
```

This allows the browser to load them via fetch.

## Step 3: Create Circuit Loader

Create `src/utils/circuit.ts`:

```typescript
import { CircuitFiles } from "zkenc-js";

export async function loadCircuitFiles(): Promise<CircuitFiles> {
  const [r1csResponse, wasmResponse] = await Promise.all([
    fetch("/circuits/simple.r1cs"),
    fetch("/circuits/simple.wasm"),
  ]);

  if (!r1csResponse.ok || !wasmResponse.ok) {
    throw new Error("Failed to load circuit files");
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

## Step 4: Create Encryption Component

Create `src/components/EncryptionForm.tsx`:

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
      // Load circuit files
      const circuitFiles = await loadCircuitFiles();

      // Prepare inputs
      const publicInputs = { publicValue };
      const messageBytes = new TextEncoder().encode(message);

      // Encrypt
      const result = await zkenc.encrypt(
        circuitFiles,
        publicInputs,
        messageBytes
      );

      setCiphertext(result.ciphertext);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Encryption failed");
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
      <h2>Encrypt Message</h2>

      <div className="form-group">
        <label>Message:</label>
        <textarea
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          placeholder="Enter your secret message..."
          rows={4}
        />
      </div>

      <div className="form-group">
        <label>Public Value:</label>
        <input
          type="number"
          value={publicValue}
          onChange={(e) => setPublicValue(Number(e.target.value))}
        />
        <small>
          Note: Private value must be {100 - publicValue} to decrypt
        </small>
      </div>

      <button onClick={handleEncrypt} disabled={loading || !message}>
        {loading ? "Encrypting..." : "Encrypt"}
      </button>

      {error && <div className="error">{error}</div>}

      {ciphertext && (
        <div className="success">
          <p>✅ Encrypted successfully!</p>
          <p>Ciphertext size: {ciphertext.length} bytes</p>
          <button onClick={handleDownload}>Download Ciphertext</button>
        </div>
      )}
    </div>
  );
}
```

## Step 5: Create Decryption Component

Create `src/components/DecryptionForm.tsx`:

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
      setError("Please upload a ciphertext file");
      return;
    }

    if (publicValue + privateValue !== 100) {
      setError(`Invalid witness: ${publicValue} + ${privateValue} ≠ 100`);
      return;
    }

    setLoading(true);
    setError("");
    setDecrypted("");

    try {
      // Load circuit files
      const circuitFiles = await loadCircuitFiles();

      // Prepare full inputs
      const fullInputs = {
        publicValue,
        privateValue,
      };

      // Decrypt
      const decryptedBytes = await zkenc.decrypt(
        circuitFiles,
        ciphertext,
        fullInputs
      );

      const message = new TextDecoder().decode(decryptedBytes);
      setDecrypted(message);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Decryption failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="decryption-form">
      <h2>Decrypt Message</h2>

      <div className="form-group">
        <label>Ciphertext File:</label>
        <input type="file" onChange={handleFileUpload} accept=".bin" />
        {ciphertext && <small>Loaded: {ciphertext.length} bytes</small>}
      </div>

      <div className="form-group">
        <label>Public Value:</label>
        <input
          type="number"
          value={publicValue}
          onChange={(e) => setPublicValue(Number(e.target.value))}
        />
      </div>

      <div className="form-group">
        <label>Private Value:</label>
        <input
          type="number"
          value={privateValue}
          onChange={(e) => setPrivateValue(Number(e.target.value))}
        />
        <small>
          Must satisfy: {publicValue} + {privateValue} = 100
        </small>
      </div>

      <button onClick={handleDecrypt} disabled={loading || !ciphertext}>
        {loading ? "Decrypting..." : "Decrypt"}
      </button>

      {error && <div className="error">{error}</div>}

      {decrypted && (
        <div className="success">
          <h3>✅ Decrypted Message:</h3>
          <pre>{decrypted}</pre>
        </div>
      )}
    </div>
  );
}
```

## Step 6: Create Main App

Update `src/App.tsx`:

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
        <h1>zkenc Demo</h1>
        <p>Witness Encryption in the Browser</p>
      </header>

      <div className="mode-selector">
        <button
          className={mode === "encrypt" ? "active" : ""}
          onClick={() => setMode("encrypt")}
        >
          Encrypt
        </button>
        <button
          className={mode === "decrypt" ? "active" : ""}
          onClick={() => setMode("decrypt")}
        >
          Decrypt
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

Add basic styles in `src/App.css`:

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

## Step 7: Configure Vite

Update `vite.config.ts` to handle WASM:

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

## Step 8: Run and Test

Start the development server:

```bash
npm run dev
```

Visit `http://localhost:5173` and try:

1. **Encrypt:**

   - Enter message: "Hello, zkenc!"
   - Public value: 30
   - Click "Encrypt"
   - Download the ciphertext

2. **Decrypt:**
   - Upload the ciphertext file
   - Public value: 30
   - Private value: 70
   - Click "Decrypt"
   - See the decrypted message!

## Advanced: Web Workers

For better performance, use Web Workers to avoid blocking the main thread.

Create `src/workers/zkenc.worker.ts`:

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
      error: error instanceof Error ? error.message : "Unknown error",
    });
  }
};
```

Use the worker in components:

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

## Production Build

Build for production:

```bash
npm run build
```

The `dist/` folder contains your production-ready app.

## Deployment

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

Add to `vite.config.ts`:

```typescript
export default defineConfig({
  base: "/zkenc-react-app/",
  // ... rest of config
});
```

Build and deploy:

```bash
npm run build
npx gh-pages -d dist
```

## Next Steps

- **[Node.js Integration →](/docs/guides/nodejs-integration)** - Server-side encryption
- **[Cross-Tool Workflow →](/docs/guides/cross-tool-workflow)** - Combine CLI and JS
- **[Playground →](/playground)** - See a complete example
- **[API Reference →](/docs/api/zkenc-js)** - Explore all functions

## Troubleshooting

**Circuit files 404:**

- Ensure files are in `public/circuits/`
- Check network tab in browser DevTools
- Verify file names match exactly

**WASM initialization fails:**

- Add `zkenc-js` to `optimizeDeps.exclude` in vite.config.ts
- Clear Vite cache: `rm -rf node_modules/.vite`

**Performance is slow:**

- Use Web Workers for heavy operations
- Cache circuit files after first load
- Consider circuit complexity

**Memory issues:**

- Large circuits use more memory
- Use streaming for large files
- Consider server-side encryption for very large files
