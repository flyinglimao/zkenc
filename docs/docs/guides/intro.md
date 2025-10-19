---
sidebar_position: 1
---

# Guides Overview

Welcome to the zkenc guides! These step-by-step tutorials will help you integrate witness encryption into your projects.

## What You'll Learn

These guides provide complete, practical examples of using zkenc in real-world applications:

### 📦 Node.js Integration

Learn how to build a complete Node.js application with witness encryption.

- Load and compile Circom circuits
- Encrypt and decrypt files
- Handle circuit inputs properly
- Error handling and best practices

[Start Node.js Guide →](/docs/guides/nodejs-integration)

### ⚛️ React Integration

Build an interactive React application with witness encryption.

- Set up Vite + React + TypeScript
- Handle circuit files in the browser
- Create encryption/decryption UI
- Optimize performance with Web Workers

[Start React Guide →](/docs/guides/react-integration)

### 🔄 Cross-Tool Workflows

Use zkenc-cli and zkenc-js together for maximum flexibility.

- Encrypt with CLI, decrypt with JavaScript
- Share ciphertexts across environments
- Combine tool strengths for your workflow
- Batch processing and automation

[Start Cross-Tool Guide →](/docs/guides/cross-tool-workflow)

## Prerequisites

Before starting these guides, you should:

1. **Have basic knowledge of:**

   - JavaScript/TypeScript (for JS guides)
   - Command line tools (for CLI guide)
   - Circom circuits (basic understanding)

2. **Install required tools:**

   ```bash
   # Node.js (18+)
   node --version

   # Circom
   circom --version

   # zkenc-cli (for cross-tool guide)
   zkenc --help
   ```

3. **Have a circuit ready:**
   - `.circom` source file
   - Or pre-compiled `.r1cs` and `.wasm` files

## Guide Structure

Each guide follows this structure:

1. **Setup** - Project initialization and dependencies
2. **Circuit Preparation** - Compile and load your circuit
3. **Implementation** - Step-by-step code examples
4. **Testing** - Verify everything works
5. **Optimization** - Performance improvements
6. **Deployment** - Production considerations

## Example Circuits

The guides use these example circuits:

### Simple Example Circuit

A basic circuit for learning:

```circom
pragma circom 2.0.0;

template Example() {
    signal input publicValue;
    signal input privateValue;
    signal output result;

    result <== publicValue + privateValue;
}

component main = Example();
```

### Sudoku Circuit

A practical example used in the playground:

```circom
pragma circom 2.0.0;

template Sudoku() {
    signal input puzzle[81];      // Public: the puzzle
    signal input solution[81];    // Private: the solution

    // Verify solution is valid
    // ... constraints ...
}

component main = Sudoku();
```

## Common Patterns

### Encryption Pattern

```typescript
// 1. Load circuit files
const circuitFiles = {
  r1csBuffer: await loadFile('circuit.r1cs'),
  wasmBuffer: await loadFile('circuit.wasm'),
};

// 2. Prepare public inputs
const publicInputs = { puzzle: [...] };

// 3. Encrypt
const { ciphertext } = await zkenc.encrypt(
  circuitFiles,
  publicInputs,
  message
);
```

### Decryption Pattern

```typescript
// 1. Load ciphertext
const ciphertext = await loadFile('encrypted.bin');

// 2. Prepare full inputs (public + private)
const fullInputs = {
  puzzle: [...],
  solution: [...],
};

// 3. Decrypt
const decrypted = await zkenc.decrypt(
  circuitFiles,
  ciphertext,
  fullInputs
);
```

## Getting Help

If you get stuck:

1. **Check the API Reference:**

   - [zkenc-js API](/docs/api/zkenc-js)
   - [zkenc-cli API](/docs/api/zkenc-cli)
   - [zkenc-core API](/docs/api/zkenc-core)

2. **Try the Playground:**

   - [Interactive Sudoku Example](/playground)

3. **Review Example Code:**

   - Each guide includes complete, runnable examples

4. **Open an Issue:**
   - [GitHub Issues](https://github.com/flyinglimao/zkenc-handmade/issues)

## Choose Your Guide

<div className="guides-grid">

### For Node.js Developers

Perfect if you're building:

- CLI tools
- Backend services
- File encryption tools
- Batch processors

[Node.js Integration →](/docs/guides/nodejs-integration)

### For React Developers

Perfect if you're building:

- Web applications
- Interactive UIs
- Browser-based tools
- Progressive Web Apps

[React Integration →](/docs/guides/react-integration)

### For Automation

Perfect if you're:

- Using multiple tools
- Batch processing files
- Building pipelines
- Cross-platform workflows

[Cross-Tool Workflow →](/docs/guides/cross-tool-workflow)

</div>

## What's Next

Ready to start? Pick a guide above, or:

- **New to zkenc?** Start with [zkenc-js Getting Started](/docs/getting-started/zkenc-js)
- **Want to experiment?** Try the [Playground](/playground)
- **Need API details?** Check the [API Reference](/docs/api/zkenc-js)

Happy coding! 🚀
