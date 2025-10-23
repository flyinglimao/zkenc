---
sidebar_position: 1
---

# Introduction to zkenc

zkenc is a witness encryption library for Circom circuits. It enables you to encrypt messages such that they can only be decrypted by providing a valid witness (solution) to a specified circuit.

## What is Witness Encryption?

Witness Encryption allows you to encrypt a message under a circuit constraint. The encrypted message can only be decrypted by someone who knows a valid witness (input) that satisfies the circuit. This is particularly useful for:

- **Conditional Access**: Encrypt data that can only be accessed when certain conditions are met
- **Zero-Knowledge Puzzles**: Create encrypted puzzles that reveal secrets when solved
- **Time-Lock Encryption**: Encrypt messages that can be decrypted only after performing specific computations

## Project Status

The zkenc implementation has undergone initial security verification. While the library provides a practical implementation of witness encryption for Circom circuits with security validated in most scenarios, we are currently addressing edge cases where additional security considerations apply. Our paper is still being refined as we continue to strengthen the implementation.

## Available Packages

zkenc consists of three main components:

### zkenc-core (Rust)

The core Rust library that implements the cryptographic primitives for witness encryption.

- Low-level encryption/decryption operations
- Circuit handling and witness verification
- Foundation for CLI and JavaScript bindings

[Learn more →](/docs/api/zkenc-core)

### zkenc-cli (Rust)

A command-line interface for witness encryption operations.

- Encrypt messages from the command line
- Decrypt ciphertexts with valid witnesses
- Interoperable with zkenc-js

[Learn more →](/docs/api/zkenc-cli)

### zkenc-js (JavaScript/TypeScript)

JavaScript/TypeScript bindings compiled from Rust using WebAssembly.

- Works in Node.js and browsers
- High-level and low-level APIs
- Full TypeScript support

[Learn more →](/docs/api/zkenc-js)

## Quick Start

Choose your preferred package to get started:

- **For JavaScript/TypeScript projects**: [zkenc-js Quick Start →](/docs/getting-started/zkenc-js)
- **For command-line usage**: [zkenc-cli Quick Start →](/docs/getting-started/zkenc-cli)

## Interactive Playground

Try zkenc in your browser with our interactive Sudoku puzzle playground:

[Open Playground →](/playground)

## Architecture

zkenc is built with a two-layer architecture:

```
┌─────────────────────────────────────────────────────┐
│              Application Layer                      │
│                                                     │
│  ┌───────────────────┐    ┌───────────────────┐   │
│  │   zkenc-cli       │    │    zkenc-js       │   │
│  │   (Rust)          │    │    (WASM)         │   │
│  │                   │    │                   │   │
│  │ • Command-line    │    │ • Browser &       │   │
│  │ • Batch           │    │   Node.js         │   │
│  │   processing      │    │ • TypeScript API  │   │
│  └─────────┬─────────┘    └─────────┬─────────┘   │
│            │                        │             │
│            └────────────┬───────────┘             │
└─────────────────────────┼─────────────────────────┘
                          │
            ┌─────────────▼──────────────┐
            │      zkenc-core            │
            │      (Rust)                │
            │                            │
            │ • Cryptographic primitives │
            │ • R1CS → QAP conversion    │
            │ • BN254 curve support      │
            │ • Blake3 KDF               │
            └────────────────────────────┘
```

**Core Layer:** zkenc-core provides the cryptographic foundation using arkworks, handling R1CS to QAP conversion, encryption/decryption primitives, and key derivation.

**Application Layer:** Both zkenc-cli (command-line tool) and zkenc-js (WASM bindings) are built on top of zkenc-core, providing different interfaces for the same underlying functionality.

## Cross-Tool Compatibility

zkenc-cli and zkenc-js are fully interoperable. You can:

- Encrypt with zkenc-cli and decrypt with zkenc-js
- Encrypt with zkenc-js and decrypt with zkenc-cli
- Share ciphertexts between different environments

[Learn about cross-tool workflows →](/docs/guides/cross-tool-workflow)

## Next Steps

1. **[Getting Started](/docs/getting-started/zkenc-js)** - Install and try your first encryption
2. **[API Reference](/docs/api/zkenc-js)** - Explore the complete API
3. **[Guides](/docs/guides/intro)** - Follow step-by-step integration guides
4. **[Playground](/playground)** - Experiment with the Sudoku puzzle example
