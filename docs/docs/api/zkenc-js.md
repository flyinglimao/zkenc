---
sidebar_position: 3
---

# zkenc-js API Reference

Complete API reference for zkenc-js, the JavaScript/TypeScript library for witness encryption.

## Installation

```bash
npm install zkenc-js
```

## Import

```typescript
import { zkenc, CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";
```

## Types

### `CircuitFiles`

Circuit files required for witness encryption operations.

```typescript
interface CircuitFiles {
  /** R1CS circuit file buffer (.r1cs) */
  r1csBuffer: Uint8Array;
  /** Circom WASM file buffer (.wasm) for witness calculation */
  wasmBuffer: Uint8Array;
}
```

**Example:**

```typescript
import fs from "fs/promises";

const circuitFiles: CircuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};
```

### `EncapResult`

Result from encapsulation containing ciphertext and key.

```typescript
interface EncapResult {
  /** Ciphertext that can be decrypted with valid witness (1576 bytes) */
  ciphertext: Uint8Array;
  /** Symmetric encryption key (32 bytes, AES-256) */
  key: Uint8Array;
}
```

### `EncryptResult`

Result from encryption containing combined ciphertext and key.

```typescript
interface EncryptResult {
  /** Combined ciphertext: [4B length][witness CT][AES CT] */
  ciphertext: Uint8Array;
  /** Encryption key for advanced users (32 bytes) */
  key: Uint8Array;
}
```

## High-Level API

The high-level API provides complete witness encryption functionality in single function calls.

### `encrypt()`

Encrypt a message using witness encryption. Combines key generation with AES-256-GCM encryption.

```typescript
async function encrypt(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>,
  message: Uint8Array
): Promise<EncryptResult>;
```

**Parameters:**

- `circuitFiles` - Circuit files (R1CS and WASM)
- `publicInputs` - Public inputs as a JSON object (only public signals)
- `message` - Message to encrypt as Uint8Array

**Returns:**

- `Promise<EncryptResult>` - Combined ciphertext and encryption key

**Ciphertext Format:**

```
[4 bytes: witness CT length][witness ciphertext][AES-encrypted message]
│                           │                   │
│                           │                   └─ AES-256-GCM encrypted
│                           └─ Witness encryption (1576 bytes)
└─ Big-endian length (always 1576)
```

**Example:**

```typescript
const { ciphertext, key } = await zkenc.encrypt(
  {
    r1csBuffer: await fs.readFile("sudoku.r1cs"),
    wasmBuffer: await fs.readFile("sudoku.wasm"),
  },
  {
    puzzle: [5, 3, 0, 0, 7, 0, 0, 0, 0 /* ... */],
  },
  new TextEncoder().encode("Secret message")
);

console.log("Ciphertext size:", ciphertext.length);
// Ciphertext size: 1608 bytes (4 + 1576 + 28)
```

**Performance:**

- First call: ~50-100ms (WASM initialization)
- Subsequent calls: ~30-50ms

### `decrypt()`

Decrypt a message using witness decryption. Recovers the key and decrypts the message.

```typescript
async function decrypt(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array>;
```

**Parameters:**

- `circuitFiles` - Circuit files (R1CS and WASM)
- `ciphertext` - Combined ciphertext from `encrypt()`
- `inputs` - Complete inputs as JSON object (public + private signals)

**Returns:**

- `Promise<Uint8Array>` - Decrypted message

**Throws:**

- `Error` - If ciphertext is invalid or witness doesn't satisfy circuit

**Example:**

```typescript
const decrypted = await zkenc.decrypt(
  {
    r1csBuffer: await fs.readFile("sudoku.r1cs"),
    wasmBuffer: await fs.readFile("sudoku.wasm"),
  },
  ciphertext,
  {
    puzzle: [5, 3, 0, 0, 7, 0, 0, 0, 0 /* ... */],
    solution: [5, 3, 4, 6, 7, 8, 9, 1, 2 /* ... */],
  }
);

const message = new TextDecoder().decode(decrypted);
console.log("Decrypted:", message);
```

**Performance:**

- First call: ~150-200ms (WASM initialization + witness calculation)
- Subsequent calls: ~100-150ms

## Low-Level API

The low-level API provides fine-grained control over the witness encryption process. Use this for research or custom encryption schemes.

### `encap()`

Generate encryption key using witness encryption (encapsulation).

```typescript
async function encap(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>
): Promise<EncapResult>;
```

**Parameters:**

- `circuitFiles` - Circuit files (R1CS and WASM)
- `publicInputs` - Public inputs as JSON object

**Returns:**

- `Promise<EncapResult>` - Witness ciphertext (1576 bytes) and key (32 bytes)

**Example:**

```typescript
const { ciphertext: witnessCiphertext, key } = await zkenc.encap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"),
  },
  { publicValue: 42 }
);

// Now use the key for your own encryption
const encryptedMessage = await customEncrypt(key, message);
```

**Use Cases:**

- Custom encryption schemes
- Separate key management
- Research and experimentation

### `decap()`

Recover encryption key using valid witness (decapsulation).

```typescript
async function decap(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array>;
```

**Parameters:**

- `circuitFiles` - Circuit files (R1CS and WASM)
- `ciphertext` - Witness ciphertext from `encap()` (1576 bytes)
- `inputs` - Complete inputs as JSON object (must satisfy circuit)

**Returns:**

- `Promise<Uint8Array>` - Recovered encryption key (32 bytes)

**Throws:**

- `Error` - If witness doesn't satisfy circuit constraints

**Example:**

```typescript
const recoveredKey = await zkenc.decap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"),
  },
  witnessCiphertext,
  {
    publicValue: 42,
    privateValue: 123,
  }
);

// Now use the recovered key
const decryptedMessage = await customDecrypt(recoveredKey, encryptedMessage);
```

## Usage Patterns

### Basic Text Encryption

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};

// Encrypt
const message = new TextEncoder().encode("Hello, World!");
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  { publicInput: 42 },
  message
);

// Decrypt
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, {
  publicInput: 42,
  privateInput: 123,
});

console.log(new TextDecoder().decode(decrypted));
```

### Binary Data Encryption

```typescript
// Encrypt a file
const fileData = await fs.readFile("document.pdf");
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  fileData
);

await fs.writeFile("document.pdf.enc", ciphertext);

// Decrypt the file
const encryptedData = await fs.readFile("document.pdf.enc");
const decryptedData = await zkenc.decrypt(
  circuitFiles,
  encryptedData,
  fullInputs
);

await fs.writeFile("document_decrypted.pdf", decryptedData);
```

### Storing Circuit Files Once

```typescript
// Load once
const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};

// Reuse for multiple operations
const results = await Promise.all([
  zkenc.encrypt(circuitFiles, inputs1, message1),
  zkenc.encrypt(circuitFiles, inputs2, message2),
  zkenc.encrypt(circuitFiles, inputs3, message3),
]);
```

### Advanced: Low-Level with Custom Encryption

```typescript
// Generate key
const { ciphertext: witnessCt, key } = await zkenc.encap(
  circuitFiles,
  publicInputs
);

// Use your own encryption
import { customEncrypt, customDecrypt } from "./my-crypto";
const encrypted = await customEncrypt(key, message);

// Store both parts
await fs.writeFile("witness.ct", witnessCt);
await fs.writeFile("message.ct", encrypted);

// Later: decrypt
const witnessCt = await fs.readFile("witness.ct");
const encrypted = await fs.readFile("message.ct");

const recoveredKey = await zkenc.decap(circuitFiles, witnessCt, fullInputs);
const decrypted = await customDecrypt(recoveredKey, encrypted);
```

## Input Format

### Public Inputs (for encryption)

Only include signals that are marked as public or are part of the constraint but not part of the witness:

```typescript
const publicInputs = {
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,
};
```

### Full Inputs (for decryption)

Include all signals (public + private):

```typescript
const fullInputs = {
  // Public inputs
  puzzleGrid: [5, 3, 0 /* ... */],
  difficulty: 1,

  // Private witness
  solutionGrid: [5, 3, 4, 6, 7, 8, 9, 1, 2 /* ... */],
};
```

### Array Signals

Arrays are supported:

```typescript
const inputs = {
  singleValue: 42,
  arrayValue: [1, 2, 3, 4, 5],
  matrix: [
    [1, 2],
    [3, 4],
  ].flat(), // Flatten 2D arrays
};
```

## Error Handling

```typescript
try {
  const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, inputs);
  console.log("Success:", new TextDecoder().decode(decrypted));
} catch (error) {
  if (error.message.includes("Invalid ciphertext")) {
    console.error("Ciphertext is corrupted or invalid");
  } else if (error.message.includes("constraint")) {
    console.error("Witness does not satisfy circuit constraints");
  } else {
    console.error("Decryption failed:", error.message);
  }
}
```

## Performance Considerations

### WASM Initialization

The first call to any function initializes the WASM module (~20-50ms). Subsequent calls are faster.

### Circuit Complexity

Performance scales with circuit size:

- Small circuits (< 1000 constraints): < 50ms
- Medium circuits (1000-10000 constraints): 50-200ms
- Large circuits (> 10000 constraints): 200ms+

### Caching

Cache circuit files in memory:

```typescript
let cachedCircuitFiles: CircuitFiles | null = null;

async function getCircuitFiles(): Promise<CircuitFiles> {
  if (!cachedCircuitFiles) {
    cachedCircuitFiles = {
      r1csBuffer: await fs.readFile("circuit.r1cs"),
      wasmBuffer: await fs.readFile("circuit.wasm"),
    };
  }
  return cachedCircuitFiles;
}
```

### Browser Optimization

Use Web Workers for non-blocking operations:

```typescript
// worker.ts
import { zkenc } from "zkenc-js";

self.onmessage = async (e) => {
  const { circuitFiles, ciphertext, inputs } = e.data;

  try {
    const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, inputs);
    self.postMessage({ success: true, decrypted });
  } catch (error) {
    self.postMessage({ success: false, error: error.message });
  }
};
```

## Browser vs Node.js

### Node.js

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1csBuffer: await fs.readFile("circuit.r1cs"),
  wasmBuffer: await fs.readFile("circuit.wasm"),
};
```

### Browser

```typescript
import { zkenc } from "zkenc-js";

const [r1cs, wasm] = await Promise.all([
  fetch("/circuits/circuit.r1cs").then((r) => r.arrayBuffer()),
  fetch("/circuits/circuit.wasm").then((r) => r.arrayBuffer()),
]);

const circuitFiles = {
  r1csBuffer: new Uint8Array(r1cs),
  wasmBuffer: new Uint8Array(wasm),
};
```

## TypeScript Support

zkenc-js is written in TypeScript and provides full type definitions:

```typescript
import type { CircuitFiles, EncapResult, EncryptResult } from "zkenc-js";

// Type-safe usage
async function encryptMessage(
  files: CircuitFiles,
  inputs: Record<string, any>,
  msg: string
): Promise<EncryptResult> {
  return zkenc.encrypt(files, inputs, new TextEncoder().encode(msg));
}
```

## Compatibility

- **Node.js**: >= 18.0.0
- **Browsers**: Modern browsers with WebAssembly support
  - Chrome/Edge >= 90
  - Firefox >= 88
  - Safari >= 15
- **Bundlers**: Vite, Webpack 5+, Rollup
- **Frameworks**: React, Vue, Svelte, Next.js

## Next Steps

- **[Getting Started →](/docs/getting-started/zkenc-js)** - Installation and basic usage
- **[Node.js Guide →](/docs/guides/nodejs-integration)** - Complete Node.js integration
- **[React Guide →](/docs/guides/react-integration)** - Complete React integration
- **[Playground →](/playground)** - Try it in your browser
