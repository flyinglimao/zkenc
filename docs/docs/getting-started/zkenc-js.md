---
sidebar_position: 1
---

# Getting Started with zkenc-js

zkenc-js is a JavaScript/TypeScript library for witness encryption that works in both Node.js and browser environments.

## Installation

Install zkenc-js using your preferred package manager:

```bash
npm install zkenc-js
# or
yarn add zkenc-js
# or
pnpm add zkenc-js
```

## Prerequisites

Before using zkenc-js, you need:

1. **A compiled Circom circuit** with the following files:
   - `.r1cs` file (circuit constraints)
   - `.wasm` file (witness generator)
   - `.sym` file (signal-to-wire mapping) **← Required for encap**
2. **Circuit files** can be obtained by compiling a Circom circuit:

```bash
circom your_circuit.circom --r1cs --wasm --sym
```

**Note:** The `--sym` flag is essential for proper encapsulation. The `.sym` file maps signal names to wire indices, ensuring your JSON inputs are correctly processed regardless of key order.

## Quick Example

Here's a simple example using zkenc-js:

```typescript
import { encrypt, decrypt } from "zkenc-js";
import fs from "fs/promises";

// Load your circuit files
const r1csBuffer = await fs.readFile("circuit.r1cs");
const wasmBuffer = await fs.readFile("circuit.wasm");
const symContent = await fs.readFile("circuit.sym", "utf-8"); // Symbol file for encap

// Define public inputs for the circuit
const publicInputs = {
  publicValue: 42,
};

// Message to encrypt
const message = new TextEncoder().encode("Hello, zkenc!");

// Encrypt the message (uses r1csBuffer and symContent)
const { ciphertext, key } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  message
);

console.log("Encrypted successfully!");
console.log("Ciphertext size:", ciphertext.length);

// To decrypt, you need the full witness (including private inputs)
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // This is the secret witness
};

// Decrypt the message (uses r1csBuffer and wasmBuffer)
const decrypted = await decrypt(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  fullInputs
);

const decryptedMessage = new TextDecoder().decode(decrypted);
console.log("Decrypted message:", decryptedMessage);
```

## High-Level vs Low-Level API

zkenc-js provides two APIs:

### High-Level API (Recommended)

The high-level API (`encrypt` and `decrypt`) handles the complete witness encryption flow:

```typescript
// Encrypt: combines witness encryption with AES (uses r1csBuffer + symContent)
const { ciphertext, key } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  message
);

// Decrypt: recovers key and decrypts message (uses r1csBuffer + wasmBuffer)
const message = await decrypt(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  fullInputs
);
```

**Use cases:**

- General encryption/decryption needs
- When you want everything handled automatically
- When you don't need separate key management

### Low-Level API (Advanced)

The low-level API (`encap` and `decap`) provides fine-grained control:

```typescript
// Encapsulate: generate key based on circuit
// Note: encap requires r1csBuffer and symContent for input mapping
const { ciphertext: witnessCiphertext, key } = await encap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    symContent: await fs.readFile("circuit.sym", "utf-8"), // Symbol file required
  },
  publicInputs
);

// Manually encrypt message with AES
const encryptedMessage = await aesEncrypt(key, message);

// Decapsulate: recover key with valid witness
// Note: decap requires r1csBuffer and wasmBuffer for witness calculation
const recoveredKey = await decap(
  {
    r1csBuffer: await fs.readFile("circuit.r1cs"),
    wasmBuffer: await fs.readFile("circuit.wasm"), // WASM file required
  },
  witnessCiphertext,
  fullInputs
);

// Manually decrypt message
const decryptedMessage = await aesDecrypt(recoveredKey, encryptedMessage);
```

**Use cases:**

- Research and experimentation
- Custom encryption schemes
- When you need separate key management

## Environment-Specific Setup

### Node.js

zkenc-js works out of the box in Node.js:

```typescript
import { encrypt, decrypt } from "zkenc-js";
import fs from "fs/promises";

const r1csBuffer = await fs.readFile("circuit.r1cs");
const wasmBuffer = await fs.readFile("circuit.wasm");
const symContent = await fs.readFile("circuit.sym", "utf-8"); // UTF-8 string for symbol file
```

### Browser

In browser environments, you need to load files differently:

```typescript
import { encrypt, decrypt } from "zkenc-js";

// Load files using fetch
const [r1csResponse, wasmResponse, symResponse] = await Promise.all([
  fetch("/circuits/circuit.r1cs"),
  fetch("/circuits/circuit.wasm"),
  fetch("/circuits/circuit.sym"),
]);

const r1csBuffer = new Uint8Array(await r1csResponse.arrayBuffer());
const wasmBuffer = new Uint8Array(await wasmResponse.arrayBuffer());
const symContent = await symResponse.text(); // Symbol file as UTF-8 text

// Use for encryption (r1csBuffer + symContent)
const { ciphertext } = await encrypt(
  { r1csBuffer, symContent },
  publicInputs,
  message
);

// Use for decryption (r1csBuffer + wasmBuffer)
const decrypted = await decrypt(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  fullInputs
);
```

### React

For React applications, see our [React Integration Guide →](/docs/guides/react-integration)

## Common Circuit Pattern

Here's a typical Circom circuit structure that works with zkenc:

```circom
pragma circom 2.0.0;

template MyCircuit() {
    // Public inputs (known to encryptor)
    signal input publicValue;

    // Private inputs (witness, needed for decryption)
    signal input privateValue;

    // Output (must be computed correctly)
    signal output result;

    // Constraints
    result <== publicValue + privateValue;
}

component main = MyCircuit();
```

**Key Points:**

- **Public inputs**: Known when encrypting, part of the encryption condition
- **Private inputs**: The "witness" needed to decrypt
- **Constraints**: Define the conditions that must be satisfied

## Next Steps

- **[API Reference →](/docs/api/zkenc-js)** - Explore the complete zkenc-js API
- **[Node.js Integration →](/docs/guides/nodejs-integration)** - Step-by-step Node.js guide
- **[React Integration →](/docs/guides/react-integration)** - Step-by-step React guide
- **[Try the Playground →](/playground)** - Interactive Sudoku example

## Troubleshooting

### Module not found errors

If you encounter module resolution errors, ensure your `tsconfig.json` includes:

```json
{
  "compilerOptions": {
    "moduleResolution": "bundler",
    "esModuleInterop": true
  }
}
```

### WebAssembly errors in browser

Make sure your bundler is configured to handle WASM files. For Vite:

```javascript
// vite.config.js
export default {
  optimizeDeps: {
    exclude: ["zkenc-js"],
  },
};
```

### Performance considerations

- Circuit compilation is CPU-intensive
- First encryption/decryption is slower due to WASM initialization
- Consider caching circuit files in production
- Use Web Workers for browser applications to avoid blocking the main thread

## Support

If you encounter issues:

1. Check the [API Reference](/docs/api/zkenc-js) for detailed documentation
2. Review the [guides](/docs/guides/intro) for common patterns
3. Open an issue on [GitHub](https://github.com/flyinglimao/zkenc)
