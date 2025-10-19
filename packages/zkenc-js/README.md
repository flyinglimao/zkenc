# zkenc-js

TypeScript/JavaScript bindings for zkenc - Witness Encryption using Circom circuits.

## Overview

zkenc-js provides a high-level API for witness encryption, allowing you to encrypt data that can only be decrypted by someone who knows a valid witness (solution) to a computational statement defined by a Circom circuit.

**Key Features:**

- 🔐 Witness-based encryption using zero-knowledge circuits
- 🌐 Works in both Node.js and browsers
- 🚀 Powered by WASM for high performance
- 📦 TypeScript support with full type definitions
- 🧪 Comprehensive test suite (24 tests passing)

## Installation

```bash
npm install zkenc-js
# or
pnpm add zkenc-js
# or
yarn add zkenc-js
```

## Quick Start

```typescript
import { encap, decap, encrypt, decrypt } from "zkenc-js";
import { readFileSync } from "fs";

// Load your Circom circuit files
const r1csBuffer = readFileSync("circuit.r1cs");
const wasmBuffer = readFileSync("circuit.wasm");

// Define your public inputs (the statement)
const publicInputs = {
  puzzle: [
    /* your puzzle data */
  ],
  solution: [
    /* your solution data */
  ],
};

// 1. Encapsulation: Generate ciphertext and encryption key
const { ciphertext, key } = await encap(
  { r1csBuffer, wasmBuffer },
  publicInputs
);

// 2. Encrypt your message
const message = new TextEncoder().encode("Secret message");
const encrypted = await encrypt(key, message);

// 3. Decapsulation: Recover key with valid witness
const recoveredKey = await decap(
  { r1csBuffer, wasmBuffer },
  ciphertext,
  publicInputs // Must include valid witness
);

// 4. Decrypt the message
const decrypted = await decrypt(recoveredKey, encrypted);
const plaintext = new TextDecoder().decode(decrypted);
console.log(plaintext); // "Secret message"
```

## Complete Example: Sudoku Witness Encryption

This example demonstrates encrypting a message that can only be decrypted by someone who knows a valid Sudoku solution.

```typescript
import { encap, decap, encrypt, decrypt } from "zkenc-js";
import { readFileSync } from "fs";

// Load Sudoku circuit (defines the computational statement)
const r1csBuffer = readFileSync("sudoku.r1cs");
const wasmBuffer = readFileSync("sudoku.wasm");

// The puzzle (public) and solution (witness)
const puzzle = [
  5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0,
  // ... 81 cells total
];

const solution = [
  5, 3, 4, 6, 7, 8, 9, 1, 2, 6, 7, 2, 1, 9, 5, 3, 4, 8,
  // ... complete valid solution
];

// Step 1: Encap - anyone can do this with just the puzzle
const inputs = { puzzle, solution };
const { ciphertext, key } = await encap({ r1csBuffer, wasmBuffer }, inputs);

// Step 2: Encrypt message
const secret = new TextEncoder().encode("Prize: $1000");
const encrypted = await encrypt(key, secret);

// Step 3: Decap - only works with valid solution
try {
  const recoveredKey = await decap({ r1csBuffer, wasmBuffer }, ciphertext, {
    puzzle,
    solution,
  });

  // Step 4: Decrypt
  const decrypted = await decrypt(recoveredKey, encrypted);
  console.log(new TextDecoder().decode(decrypted)); // "Prize: $1000"
} catch (error) {
  console.error("Invalid witness!", error);
}
```

## API Reference

### `encap(circuitFiles, publicInputs)`

Generate ciphertext and encryption key from circuit and public inputs.

**Parameters:**

- `circuitFiles: CircuitFiles` - Circuit files
  - `r1csBuffer: Uint8Array` - R1CS circuit file
  - `wasmBuffer: Uint8Array` - Circom WASM file
- `publicInputs: Record<string, any>` - Public inputs as JSON object

**Returns:** `Promise<EncapResult>`

- `ciphertext: Uint8Array` - Ciphertext for witness verification
- `key: Uint8Array` - Symmetric encryption key (32 bytes)

**Example:**

```typescript
const { ciphertext, key } = await encap(
  { r1csBuffer, wasmBuffer },
  { puzzle: puzzleData, solution: solutionData }
);
```

---

### `decap(circuitFiles, ciphertext, fullInputs)`

Recover encryption key using valid witness.

**Parameters:**

- `circuitFiles: CircuitFiles` - Circuit files
- `ciphertext: Uint8Array` - Ciphertext from encap
- `fullInputs: Record<string, any>` - Complete inputs including witness

**Returns:** `Promise<Uint8Array>` - Recovered encryption key (32 bytes)

**Throws:** Error if witness is invalid or doesn't satisfy constraints

**Example:**

```typescript
const key = await decap({ r1csBuffer, wasmBuffer }, ciphertext, {
  puzzle: puzzleData,
  solution: solutionData,
});
```

---

### `encrypt(key, message)`

Encrypt message with symmetric key using AES-256-GCM.

**Parameters:**

- `key: Uint8Array` - Encryption key (32 bytes)
- `message: Uint8Array` - Message to encrypt

**Returns:** `Promise<Uint8Array>` - Encrypted message with nonce

**Example:**

```typescript
const message = new TextEncoder().encode("Hello");
const encrypted = await encrypt(key, message);
```

---

### `decrypt(key, encrypted)`

Decrypt message with symmetric key using AES-256-GCM.

**Parameters:**

- `key: Uint8Array` - Decryption key (32 bytes)
- `encrypted: Uint8Array` - Encrypted message with nonce

**Returns:** `Promise<Uint8Array>` - Decrypted message

**Throws:** Error if decryption fails (wrong key or corrupted data)

**Example:**

```typescript
const decrypted = await decrypt(key, encrypted);
const message = new TextDecoder().decode(decrypted);
```

## Browser Usage

zkenc-js works in browsers with bundlers like Vite, Webpack, or Rollup:

```typescript
// Fetch circuit files
const r1csResponse = await fetch("/circuits/sudoku.r1cs");
const wasmResponse = await fetch("/circuits/sudoku.wasm");

const r1csBuffer = new Uint8Array(await r1csResponse.arrayBuffer());
const wasmBuffer = new Uint8Array(await wasmResponse.arrayBuffer());

// Use normally
const { ciphertext, key } = await encap(
  { r1csBuffer, wasmBuffer },
  publicInputs
);
```

**Note:** Make sure your bundler is configured to handle WASM files. For Vite:

```typescript
// vite.config.ts
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
});
```

## Architecture

zkenc-js consists of three layers:

1. **zkenc-core** (Rust): Core witness encryption algorithms using arkworks
2. **WASM bindings** (Rust): WebAssembly interface with R1CS/witness parsers
3. **TypeScript API** (this package): High-level API with witness calculator

```
┌─────────────────────────────────────┐
│  TypeScript API (zkenc-js)          │
│  - encap/decap wrappers             │
│  - AES-256-GCM encrypt/decrypt      │
│  - Witness calculator integration   │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  WASM Bindings (lib.rs)             │
│  - R1CS parser (Circom format)      │
│  - Witness parser (snarkjs format)  │
│  - CircomCircuit implementation     │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  zkenc-core (Rust)                  │
│  - Encap: Generate key from QAP     │
│  - Decap: Recover key with witness  │
│  - Pairing-based cryptography       │
└─────────────────────────────────────┘
```

## How It Works

Witness encryption allows encrypting data to a computational statement rather than a public key:

1. **Statement (Public)**: A puzzle or problem defined by a Circom circuit
2. **Witness (Private)**: A solution that satisfies the circuit constraints
3. **Encryption**: Anyone can encrypt to the statement
4. **Decryption**: Only works with a valid witness

### Security Properties

- **Correctness**: Valid witness always recovers the correct key
- **Soundness**: Invalid witness cannot recover the key (with high probability)
- **Witness Privacy**: The ciphertext doesn't reveal the witness
- **CRS Security**: Based on pairing-friendly elliptic curves (BN254)

## Testing

```bash
# Run all tests
pnpm test

# Run specific test suites
pnpm test e2e        # End-to-end tests
pnpm test witness    # Witness calculator tests
pnpm test zkenc      # Crypto tests

# Watch mode
pnpm test --watch
```

Test coverage:

- ✅ 9 witness calculator tests
- ✅ 3 AES-256-GCM encryption tests
- ✅ 4 WASM integration tests
- ✅ 8 end-to-end workflow tests
- **Total: 24/24 passing**

## Development

Build the WASM module:

```bash
pnpm run build:wasm
```

Compile TypeScript:

```bash
pnpm run build
```

## Troubleshooting

### "R1CS parse error"

- Ensure your `.r1cs` file is generated by Circom
- Check file is not corrupted

### "Witness size mismatch"

- Verify your inputs match the circuit's expected format

### "Invalid witness" during decap

- The provided solution doesn't satisfy the circuit constraints
- Double-check your witness values

### WASM not loading in browser

- Configure your bundler to handle WASM files
- Check CORS headers if loading from CDN

## Examples

See the `tests/` directory for more examples:

- `e2e.test.ts` - Complete encryption workflows
- `witness.test.ts` - Witness calculator usage
- `zkenc-wasm.test.ts` - Low-level WASM API

## License

MIT
