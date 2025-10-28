# zkenc-js

TypeScript/JavaScript bindings for zkenc - Witness Encryption using Circom circuits.

## Overview

zkenc-js provides a high-level API for witness encryption, allowing you to encrypt data that can only be decrypted by someone who knows a valid witness (solution) to a computational statement defined by a Circom circuit.

**Key Features:**

- 🔐 Witness-based encryption using zero-knowledge circuits
- 🌐 Works in both Node.js and browsers
- 🚀 Powered by WASM for high performance
- 📦 TypeScript support with full type definitions
- 🧪 Comprehensive test suite (29 tests passing)

## Installation

```bash
npm install zkenc-js
# or
pnpm add zkenc-js
# or
yarn add zkenc-js
```

## Quick Start

### High-Level API (Recommended)

The simplest way to use zkenc-js - directly encrypt and decrypt messages:

```typescript
import { encrypt, decrypt, getPublicInput } from "zkenc-js";
import { readFileSync } from "fs";

// Load your Circom circuit files
const r1csBuffer = readFileSync("circuit.r1cs");
const wasmBuffer = readFileSync("circuit.wasm");
const symBuffer = readFileSync("circuit.sym");

// Public inputs (the statement)
const publicInputs = {
  puzzle: [
    /* puzzle data */
  ],
};

// Complete inputs (public + witness)
const completeInputs = {
  puzzle: [
    /* puzzle data */
  ],
  solution: [
    /* solution data */
  ],
};

// 1. Encrypt: Automatically handles encap + AES encryption
const message = new TextEncoder().encode("Secret message");
const { ciphertext, key } = await encrypt(
  { r1csBuffer, wasmBuffer, symBuffer },
  publicInputs,
  message
  // Optional: { includePublicInput: false } to exclude public inputs
);
// key is available for advanced users

// 2. Extract public inputs from ciphertext (if included)
const extractedInputs = getPublicInput(ciphertext);
console.log(extractedInputs); // { puzzle: [...] }

// 3. Decrypt: Automatically handles decap + AES decryption
const decrypted = await decrypt(
  { r1csBuffer, wasmBuffer, symBuffer },
  ciphertext,
  completeInputs // Must include valid witness
);

const plaintext = new TextDecoder().decode(decrypted);
console.log(plaintext); // "Secret message"
```

### Low-Level API (Advanced)

For research or custom encryption schemes, use the paper-aligned API:

```typescript
import { encap, decap } from "zkenc-js";

// 1. Encap: Generate witness-encrypted key
const { ciphertext, key } = await encap(
  { r1csBuffer, wasmBuffer, symBuffer },
  publicInputs
);

// 2. Use key for custom encryption
// ... your custom encryption logic ...

// 3. Decap: Recover key with valid witness
const recoveredKey = await decap(
  { r1csBuffer, wasmBuffer, symBuffer },
  ciphertext,
  completeInputs
);

// 4. Use recovered key for decryption
// ... your custom decryption logic ...
```

## Complete Example: Sudoku Witness Encryption

This example demonstrates encrypting a message that can only be decrypted by someone who knows a valid Sudoku solution.

```typescript
import { encrypt, decrypt } from "zkenc-js";
import { readFileSync } from "fs";

// Load Sudoku circuit (defines the computational statement)
const r1csBuffer = readFileSync("sudoku.r1cs");
const wasmBuffer = readFileSync("sudoku.wasm");
const symBuffer = readFileSync("sudoku.sym");
const circuitFiles = { r1csBuffer, wasmBuffer, symBuffer };

// The puzzle (public input)
const puzzle = [
  5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0,
  // ... 81 cells total
];

const solution = [
  5, 3, 4, 6, 7, 8, 9, 1, 2, 6, 7, 2, 1, 9, 5, 3, 4, 8,
  // ... complete valid solution
];

// Step 1: Encrypt - anyone can do this with just the puzzle
const secret = new TextEncoder().encode("Prize: $1000");
const { ciphertext, key } = await encrypt(
  circuitFiles,
  { puzzle }, // Only public inputs needed
  secret
);
// ciphertext now contains both witness encryption and AES encryption

// Step 2: Decrypt - only works with valid solution
try {
  const decrypted = await decrypt(
    circuitFiles,
    ciphertext,
    { puzzle, solution } // Need both public and private inputs
  );

  console.log(new TextDecoder().decode(decrypted)); // "Prize: $1000"
} catch (error) {
  console.error("Invalid witness!", error);
}
```

## API Reference

### High-Level API

#### `encrypt(circuitFiles, publicInputs, message)`

Encrypt message using witness encryption (combines encap + AES encryption).

**Parameters:**

- `circuitFiles: CircuitFiles` - Circuit files
  - `r1csBuffer: Uint8Array` - R1CS circuit file
  - `wasmBuffer: Uint8Array` - Circom WASM file
  - `symBuffer: Uint8Array` - Circom symbol file
- `publicInputs: Record<string, any>` - Public inputs as JSON object
- `message: Uint8Array` - Message to encrypt
- `options?: EncryptOptions` - Optional encryption options
  - `includePublicInput?: boolean` - Include public inputs in ciphertext (default: true)

**Returns:** `Promise<EncryptResult>`

- `ciphertext: Uint8Array` - Combined ciphertext (witness CT + AES CT)
- `key: Uint8Array` - Encryption key (32 bytes, for advanced users)

**Example:**

```typescript
const message = new TextEncoder().encode("Secret");
const { ciphertext, key } = await encrypt(
  { r1csBuffer, wasmBuffer, symBuffer },
  { puzzle: puzzleData },
  message,
  { includePublicInput: true } // Optional, true by default
);
```

---

#### `decrypt(circuitFiles, ciphertext, inputs)`

Decrypt message using witness decryption (combines decap + AES decryption).

**Parameters:**

- `circuitFiles: CircuitFiles` - Circuit files
- `ciphertext: Uint8Array` - Combined ciphertext from encrypt
- `inputs: Record<string, any>` - Complete inputs (public + witness)

**Returns:** `Promise<Uint8Array>` - Decrypted message

**Throws:** Error if witness is invalid or doesn't satisfy constraints

**Example:**

```typescript
const decrypted = await decrypt(
  { r1csBuffer, wasmBuffer, symBuffer },
  ciphertext,
  {
    puzzle: puzzleData,
    solution: solutionData,
  }
);
const message = new TextDecoder().decode(decrypted);
```

---

#### `getPublicInput(ciphertext)`

Extract public inputs from ciphertext (if they were included during encryption).

**Parameters:**

- `ciphertext: Uint8Array` - Combined ciphertext from encrypt

**Returns:** `Record<string, any>` - Public inputs as JSON object

**Throws:** Error if public inputs were not included in the ciphertext

**Example:**

```typescript
// After encrypting with default options (includePublicInput: true)
const publicInputs = getPublicInput(ciphertext);
console.log(publicInputs.puzzle); // [5, 3, 0, ...]
```

**Note:** This only works if the ciphertext was created with `includePublicInput: true` (default).

---

### Low-Level API

#### `encap(circuitFiles, publicInputs)`

Generate witness-encrypted key (low-level, paper-aligned API).

**Parameters:**

- `circuitFiles: CircuitFiles` - Circuit files
- `publicInputs: Record<string, any>` - Public inputs as JSON object

**Returns:** `Promise<EncapResult>`

- `ciphertext: Uint8Array` - Witness ciphertext (1576 bytes)
- `key: Uint8Array` - Symmetric encryption key (32 bytes)

**Example:**

```typescript
const { ciphertext, key } = await encap(
  { r1csBuffer, wasmBuffer, symBuffer },
  { puzzle: puzzleData }
);
// Use key for custom encryption...
```

---

#### `decap(circuitFiles, ciphertext, inputs)`

Recover encryption key using valid witness (low-level, paper-aligned API).

**Parameters:**

- `circuitFiles: CircuitFiles` - Circuit files
- `ciphertext: Uint8Array` - Witness ciphertext from encap
- `inputs: Record<string, any>` - Complete inputs (public + witness)

**Returns:** `Promise<Uint8Array>` - Recovered encryption key (32 bytes)

**Throws:** Error if witness is invalid

**Example:**

```typescript
const key = await decap({ r1csBuffer, wasmBuffer, symBuffer }, ciphertext, {
  puzzle: puzzleData,
  solution: solutionData,
});
// Use key for custom decryption...
```

## Browser Usage

zkenc-js works in browsers with bundlers like Vite, Webpack, or Rollup:

```typescript
// Fetch circuit files
const r1csResponse = await fetch("/circuits/sudoku.r1cs");
const wasmResponse = await fetch("/circuits/sudoku.wasm");
const symResponse = await fetch("/circuits/sudoku.sym");

const r1csBuffer = new Uint8Array(await r1csResponse.arrayBuffer());
const wasmBuffer = new Uint8Array(await wasmResponse.arrayBuffer());
const symBuffer = new Uint8Array(await symResponse.arrayBuffer());

// Use normally
const { ciphertext, key } = await encap(
  { r1csBuffer, wasmBuffer, symBuffer },
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

zkenc-js is built on top of zkenc-core and compiled to WebAssembly:

```
┌─────────────────────────────────────┐
│         zkenc-js                    │
│  (WASM + TypeScript Application)    │
│                                     │
│  - TypeScript API                   │
│  - AES-256-GCM encryption           │
│  - Witness calculator               │
│  - R1CS/WTNS parsers                │
└───────────────┬─────────────────────┘
                │
  ┌─────────────▼──────────────┐
  │      zkenc-core            │
  │  (Cryptographic Foundation)│
  └────────────────────────────┘
```

The package consists of three internal layers:

1. **TypeScript API Layer**: High-level `encrypt()`/`decrypt()` functions with witness calculator integration
2. **WASM Bindings Layer**: Circom R1CS parser, snarkjs witness parser, and `CircomCircuit` implementation
3. **Core Layer**: zkenc-core providing `encap()`/`decap()` cryptographic primitives

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
- ✅ 7 WASM integration tests
- ✅ 8 end-to-end workflow tests
- ✅ 5 zkenc API tests
- **Total: 29/29 passing**

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
