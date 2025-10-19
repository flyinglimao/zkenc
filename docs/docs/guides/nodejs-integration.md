---
sidebar_position: 2
---

# Node.js Integration Guide

This guide shows you how to build a complete Node.js application using zkenc-js for witness encryption.

## What We'll Build

A Node.js CLI tool that:

- Encrypts files using a Sudoku circuit
- Decrypts files with a valid Sudoku solution
- Handles errors gracefully
- Provides a clean command-line interface

## Prerequisites

- Node.js 18 or higher
- Basic TypeScript knowledge
- Circom installed (`circom --version`)

## Step 1: Project Setup

Create a new project:

```bash
mkdir zkenc-node-example
cd zkenc-node-example
npm init -y
```

Install dependencies:

```bash
npm install zkenc-js
npm install --save-dev typescript @types/node tsx
```

Create `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "esModuleInterop": true,
    "strict": true,
    "skipLibCheck": true,
    "outDir": "./dist"
  },
  "include": ["src/**/*"]
}
```

Update `package.json`:

```json
{
  "type": "module",
  "scripts": {
    "dev": "tsx src/index.ts",
    "build": "tsc",
    "start": "node dist/index.js"
  }
}
```

## Step 2: Prepare Circuit Files

Create a simple circuit `circuits/simple.circom`:

```circom
pragma circom 2.0.0;

template Simple() {
    signal input publicValue;
    signal input privateValue;
    signal output result;

    // Constraint: publicValue + privateValue must equal 100
    result <== publicValue + privateValue;
    result === 100;
}

component main {public [publicValue]} = Simple();
```

Compile the circuit:

```bash
mkdir -p circuits/build
circom circuits/simple.circom --r1cs --wasm -o circuits/build
```

This creates:

- `circuits/build/simple.r1cs`
- `circuits/build/simple_js/simple.wasm`

## Step 3: Load Circuit Files

Create `src/circuit.ts`:

```typescript
import fs from "fs/promises";
import path from "path";
import { CircuitFiles } from "zkenc-js";

export async function loadCircuitFiles(): Promise<CircuitFiles> {
  const circuitsDir = path.join(process.cwd(), "circuits", "build");

  const [r1csBuffer, wasmBuffer] = await Promise.all([
    fs.readFile(path.join(circuitsDir, "simple.r1cs")),
    fs.readFile(path.join(circuitsDir, "simple_js", "simple.wasm")),
  ]);

  return {
    r1csBuffer,
    wasmBuffer,
  };
}
```

## Step 4: Implement Encryption

Create `src/encrypt.ts`:

```typescript
import fs from "fs/promises";
import { zkenc } from "zkenc-js";
import { loadCircuitFiles } from "./circuit.js";

export async function encryptFile(
  inputFile: string,
  outputFile: string,
  publicValue: number
): Promise<void> {
  console.log("🔐 Starting encryption...");

  // Load circuit files
  console.log("📂 Loading circuit...");
  const circuitFiles = await loadCircuitFiles();

  // Read message file
  console.log("📄 Reading message...");
  const message = await fs.readFile(inputFile);
  console.log(`   Message size: ${message.length} bytes`);

  // Prepare public inputs
  const publicInputs = {
    publicValue: publicValue,
  };

  // Encrypt
  console.log("🔒 Encrypting...");
  const startTime = Date.now();

  const { ciphertext, key } = await zkenc.encrypt(
    circuitFiles,
    publicInputs,
    message
  );

  const duration = Date.now() - startTime;
  console.log(`   Encryption took ${duration}ms`);

  // Save ciphertext
  await fs.writeFile(outputFile, ciphertext);
  console.log(`✅ Ciphertext saved to: ${outputFile}`);
  console.log(`   Ciphertext size: ${ciphertext.length} bytes`);

  // Optionally save key for debugging
  const keyFile = outputFile + ".key";
  await fs.writeFile(keyFile, key);
  console.log(`🔑 Key saved to: ${keyFile} (for debugging)`);
}
```

## Step 5: Implement Decryption

Create `src/decrypt.ts`:

```typescript
import fs from "fs/promises";
import { zkenc } from "zkenc-js";
import { loadCircuitFiles } from "./circuit.js";

export async function decryptFile(
  inputFile: string,
  outputFile: string,
  publicValue: number,
  privateValue: number
): Promise<void> {
  console.log("🔓 Starting decryption...");

  // Load circuit files
  console.log("📂 Loading circuit...");
  const circuitFiles = await loadCircuitFiles();

  // Read ciphertext
  console.log("📦 Reading ciphertext...");
  const ciphertext = await fs.readFile(inputFile);
  console.log(`   Ciphertext size: ${ciphertext.length} bytes`);

  // Prepare full inputs (public + private)
  const fullInputs = {
    publicValue: publicValue,
    privateValue: privateValue,
  };

  // Verify inputs satisfy constraint
  if (publicValue + privateValue !== 100) {
    throw new Error(`Invalid witness: ${publicValue} + ${privateValue} ≠ 100`);
  }

  // Decrypt
  console.log("🔓 Decrypting...");
  const startTime = Date.now();

  try {
    const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

    const duration = Date.now() - startTime;
    console.log(`   Decryption took ${duration}ms`);

    // Save decrypted message
    await fs.writeFile(outputFile, decrypted);
    console.log(`✅ Message decrypted to: ${outputFile}`);
    console.log(`   Message size: ${decrypted.length} bytes`);
  } catch (error) {
    console.error("❌ Decryption failed!");
    if (error instanceof Error) {
      console.error(`   Error: ${error.message}`);
    }
    throw error;
  }
}
```

## Step 6: Create CLI Interface

Create `src/index.ts`:

```typescript
#!/usr/bin/env node

import { Command } from "commander";
import { encryptFile } from "./encrypt.js";
import { decryptFile } from "./decrypt.js";

const program = new Command();

program
  .name("zkenc-example")
  .description("Witness encryption example using zkenc-js")
  .version("1.0.0");

program
  .command("encrypt")
  .description("Encrypt a file")
  .requiredOption("-i, --input <file>", "Input file to encrypt")
  .requiredOption("-o, --output <file>", "Output encrypted file")
  .requiredOption("-p, --public <value>", "Public value (number)", parseInt)
  .action(async (options) => {
    try {
      await encryptFile(options.input, options.output, options.public);
    } catch (error) {
      console.error("Encryption failed:", error);
      process.exit(1);
    }
  });

program
  .command("decrypt")
  .description("Decrypt a file")
  .requiredOption("-i, --input <file>", "Input encrypted file")
  .requiredOption("-o, --output <file>", "Output decrypted file")
  .requiredOption("-p, --public <value>", "Public value (number)", parseInt)
  .requiredOption("--private <value>", "Private value (number)", parseInt)
  .action(async (options) => {
    try {
      await decryptFile(
        options.input,
        options.output,
        options.public,
        options.private
      );
    } catch (error) {
      console.error("Decryption failed:", error);
      process.exit(1);
    }
  });

program.parse();
```

Install commander for CLI:

```bash
npm install commander
```

## Step 7: Test the Application

Create a test message:

```bash
echo "This is a secret message!" > message.txt
```

Encrypt the message:

```bash
npm run dev encrypt -- \
  --input message.txt \
  --output encrypted.bin \
  --public 30
```

Output:

```
🔐 Starting encryption...
📂 Loading circuit...
📄 Reading message...
   Message size: 26 bytes
🔒 Encrypting...
   Encryption took 45ms
✅ Ciphertext saved to: encrypted.bin
   Ciphertext size: 1630 bytes
🔑 Key saved to: encrypted.bin.key (for debugging)
```

Decrypt the message (using correct witness: 30 + 70 = 100):

```bash
npm run dev decrypt -- \
  --input encrypted.bin \
  --output decrypted.txt \
  --public 30 \
  --private 70
```

Output:

```
🔓 Starting decryption...
📂 Loading circuit...
📦 Reading ciphertext...
   Ciphertext size: 1630 bytes
🔓 Decrypting...
   Decryption took 156ms
✅ Message decrypted to: decrypted.txt
   Message size: 26 bytes
```

Verify:

```bash
diff message.txt decrypted.txt
echo "Success!"
```

Try with wrong witness (will fail):

```bash
npm run dev decrypt -- \
  --input encrypted.bin \
  --output decrypted.txt \
  --public 30 \
  --private 50
```

Output:

```
❌ Decryption failed!
   Error: Invalid witness: 30 + 50 ≠ 100
```

## Step 8: Advanced Features

### Circuit File Caching

Create `src/circuit-cache.ts`:

```typescript
import { CircuitFiles } from "zkenc-js";
import { loadCircuitFiles } from "./circuit.js";

let circuitCache: CircuitFiles | null = null;

export async function getCachedCircuitFiles(): Promise<CircuitFiles> {
  if (!circuitCache) {
    console.log("💾 Caching circuit files...");
    circuitCache = await loadCircuitFiles();
  }
  return circuitCache;
}
```

### Progress Reporting

```typescript
export async function encryptFileWithProgress(
  inputFile: string,
  outputFile: string,
  publicValue: number
): Promise<void> {
  const steps = [
    "Loading circuit",
    "Reading message",
    "Encrypting",
    "Saving ciphertext",
  ];

  for (let i = 0; i < steps.length; i++) {
    console.log(`[${i + 1}/${steps.length}] ${steps[i]}...`);
    // ... perform step
  }
}
```

### Batch Processing

```typescript
export async function encryptMultiple(
  files: string[],
  outputDir: string,
  publicValue: number
): Promise<void> {
  const circuitFiles = await getCachedCircuitFiles();

  for (const file of files) {
    const outputFile = path.join(outputDir, path.basename(file) + ".enc");

    console.log(`\nProcessing: ${file}`);
    // ... encrypt file
  }
}
```

## Complete Example

Full source code is available at: `examples/nodejs-integration/`

Project structure:

```
zkenc-node-example/
├── circuits/
│   ├── simple.circom
│   └── build/
│       ├── simple.r1cs
│       └── simple_js/
│           └── simple.wasm
├── src/
│   ├── index.ts          # CLI interface
│   ├── circuit.ts        # Circuit loading
│   ├── encrypt.ts        # Encryption logic
│   └── decrypt.ts        # Decryption logic
├── package.json
└── tsconfig.json
```

## Performance Optimization

### 1. Cache Circuit Files

```typescript
// Load once, reuse many times
const circuitFiles = await loadCircuitFiles();

for (const file of files) {
  await zkenc.encrypt(circuitFiles, inputs, message);
}
```

### 2. Use Streams for Large Files

```typescript
import { createReadStream, createWriteStream } from "fs";

async function encryptLargeFile(input: string, output: string) {
  const chunks: Buffer[] = [];
  const stream = createReadStream(input);

  for await (const chunk of stream) {
    chunks.push(chunk);
  }

  const message = Buffer.concat(chunks);
  // ... encrypt
}
```

### 3. Parallel Processing

```typescript
await Promise.all(
  files.map((file) => encryptFile(file, `${file}.enc`, publicValue))
);
```

## Error Handling

```typescript
try {
  await decryptFile(input, output, pubVal, privVal);
} catch (error) {
  if (error instanceof Error) {
    if (error.message.includes("Invalid ciphertext")) {
      console.error("File is corrupted or not a valid ciphertext");
    } else if (error.message.includes("constraint")) {
      console.error("Witness does not satisfy circuit constraints");
    } else {
      console.error("Unexpected error:", error.message);
    }
  }
  process.exit(1);
}
```

## Production Deployment

### 1. Build for Production

```bash
npm run build
```

### 2. Install Globally

```bash
npm install -g .
zkenc-example --help
```

### 3. Create Binary (optional)

Using `pkg`:

```bash
npm install -g pkg
pkg . --targets node18-linux-x64,node18-macos-x64,node18-win-x64
```

## Next Steps

- **[React Integration →](/docs/guides/react-integration)** - Build a web UI
- **[Cross-Tool Workflow →](/docs/guides/cross-tool-workflow)** - Combine CLI and JS
- **[API Reference →](/docs/api/zkenc-js)** - Explore all functions
- **[Playground →](/playground)** - Try in browser

## Troubleshooting

**Circuit loading fails:**

- Check file paths are correct
- Verify circuit was compiled successfully
- Ensure R1CS and WASM files exist

**Encryption is slow:**

- First call initializes WASM (~20-50ms overhead)
- Cache circuit files for multiple operations
- Consider circuit complexity

**Decryption fails:**

- Verify witness satisfies constraints
- Check public inputs match encryption
- Ensure ciphertext isn't corrupted
