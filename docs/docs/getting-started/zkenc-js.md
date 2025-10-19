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
2. **Circuit files** can be obtained by compiling a Circom circuit:

```bash
circom your_circuit.circom --r1cs --wasm
```

## Quick Example

Here's a simple example using zkenc-js:

```typescript
import { zkenc, CircuitFiles } from "zkenc-js";

// Load your circuit files
const circuitFiles: CircuitFiles = {
  r1cs: await fs.readFile("circuit.r1cs"),
  wasm: await fs.readFile("circuit.wasm"),
};

// Define public inputs for the circuit
const publicInputs = {
  publicValue: 42,
};

// Message to encrypt
const message = new TextEncoder().encode("Hello, zkenc!");

// Encrypt the message
const { ciphertext, key } = await zkenc.encrypt(
  circuitFiles,
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

// Decrypt the message
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

const decryptedMessage = new TextDecoder().decode(decrypted);
console.log("Decrypted message:", decryptedMessage);
```

## High-Level vs Low-Level API

zkenc-js provides two APIs:

### High-Level API (Recommended)

The high-level API (`encrypt` and `decrypt`) handles the complete witness encryption flow:

```typescript
// Encrypt: combines witness encryption with AES
const { ciphertext, key } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);

// Decrypt: recovers key and decrypts message
const message = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

**Use cases:**

- General encryption/decryption needs
- When you want everything handled automatically
- When you don't need separate key management

### Low-Level API (Advanced)

The low-level API (`encap` and `decap`) provides fine-grained control:

```typescript
// Encapsulate: generate key based on circuit
const { ciphertext: witnessCiphertext, key } = await zkenc.encap(
  circuitFiles,
  publicInputs
);

// Manually encrypt message with AES
const encryptedMessage = await aesEncrypt(key, message);

// Decapsulate: recover key with valid witness
const recoveredKey = await zkenc.decap(
  circuitFiles,
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
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const circuitFiles = {
  r1cs: await fs.readFile("circuit.r1cs"),
  wasm: await fs.readFile("circuit.wasm"),
};
```

### Browser

In browser environments, you need to load files differently:

```typescript
import { zkenc } from "zkenc-js";

// Load files using fetch
const [r1csResponse, wasmResponse] = await Promise.all([
  fetch("/circuits/circuit.r1cs"),
  fetch("/circuits/circuit.wasm"),
]);

const circuitFiles = {
  r1cs: new Uint8Array(await r1csResponse.arrayBuffer()),
  wasm: new Uint8Array(await wasmResponse.arrayBuffer()),
};
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
3. Open an issue on [GitHub](https://github.com/flyinglimao/zkenc-handmade)
