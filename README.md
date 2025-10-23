# zkenc

A witness encryption implementation for QAP (Quadratic Arithmetic Programs) with Circom circuit support.

## 📦 Project Structure

```
zkenc/
├── packages/
│   ├── zkenc-core/      # Core algorithm implementation (Rust)
│   ├── zkenc-cli/       # Command-line interface tool
│   └── zkenc-js/        # WASM/JavaScript bindings
├── Cargo.toml           # Rust Workspace configuration
└── package.json         # Node.js/pnpm configuration
```

## 🎯 Key Features

### Witness Encryption for QAP

This project implements a witness encryption scheme based on QAP (Quadratic Arithmetic Programs):

1. **Encap** (Encapsulation): Anyone can generate ciphertext using a circuit and public inputs
2. **Decap** (Decapsulation): Only those with a valid witness satisfying the circuit constraints can recover the key
3. **End-to-End Encryption**: Complete message protection by combining with symmetric encryption

### Circom Integration

- ✅ Direct loading of Circom-compiled `.r1cs` circuit files
- ✅ Support for snarkjs-generated `.wtns` witness files
- ✅ Handles complex circuits (tested: Sudoku 162 constraints, Signature 8443 constraints)
- ✅ Uses BN254 curve for Circom ecosystem compatibility

### Security Features

- ✅ Invalid witnesses cannot recover keys (constraint verification)
- ✅ AES-256-GCM symmetric encryption
- ✅ Fresh random parameters for each Encap
- ✅ Comprehensive E2E test coverage

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- (Optional) Circom 2.0+ and snarkjs (for generating circuits and witnesses)
- (Optional) pnpm (for JavaScript bindings)

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack (for WASM compilation)
cargo install wasm-pack

# Install pnpm (if using Node.js parts)
npm install -g pnpm
```

### Building

```bash
# Build all Rust packages
cargo build --workspace --release

# Build core library only
cargo build -p zkenc-core --release

# Build CLI tool
cargo build -p zkenc-cli --release

# Build WASM module
cd packages/zkenc-js
wasm-pack build --target web
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Test specific package
cargo test -p zkenc-core
```

## 💡 Usage Examples

### Complete Sudoku Encryption Example (High-Level API)

Suppose you have a Sudoku circuit and want to encrypt a message that only someone who knows the correct solution can decrypt:

```bash
# Prepare files
# - sudoku.r1cs: Circom-compiled circuit
# - puzzle.json: Sudoku puzzle (public inputs)
# - solution.wtns: snarkjs-generated correct solution witness

# Step 1: Encrypt your message (one-step encryption with embedded public inputs)
echo "Secret message for Sudoku solver" > message.txt
zkenc encrypt \
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --message message.txt \
  --output encrypted_package.bin

# Step 2: (Performed by someone with the solution) Decrypt the message in one step
zkenc decrypt \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted_package.bin \
  --output decrypted_message.txt

# Verify
cat decrypted_message.txt
# Output: Secret message for Sudoku solver
```

**Key Benefits:**

- ✅ Single command for encryption (no intermediate files needed)
- ✅ Single command for decryption (fully automatic)
- ✅ Public inputs embedded in ciphertext (self-contained)
- ✅ Compatible with zkenc-js (no format conversion needed)

### Generating Inputs with Circom and snarkjs

```bash
# 1. Compile Circom circuit
circom sudoku.circom --r1cs --wasm --sym

# 2. Prepare input JSON
cat > input.json << EOF
{
  "puzzle": ["5","3","0","0","7","0","0","0","0"],
  "solution": ["5","3","4","6","7","8","9","1","2"]
}
EOF

# 3. Calculate witness
snarkjs wtns calculate sudoku.wasm input.json witness.wtns

# 4. Verify witness (optional)
snarkjs wtns check sudoku.r1cs witness.wtns

# 5. Now you can use zkenc!
```

### Cross-Tool Compatibility

zkenc-cli and zkenc-js are fully compatible and use the same ciphertext format:

```bash
# Encrypt with CLI
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message secret.txt \
  --output encrypted.bin
```

```typescript
// Decrypt with zkenc-js (Node.js or browser)
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const ciphertext = await fs.readFile("encrypted.bin");
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
console.log(new TextDecoder().decode(decrypted));
```

**No format conversion needed!** Learn more about [cross-tool workflows →](./docs/docs/guides/cross-tool-workflow.md)

## 📚 Package Documentation

### zkenc-core

Core witness encryption algorithm implementation using the arkworks ecosystem.

**Core Features**:

- ✅ **Encap**: Generate witness-encrypted ciphertext and key from circuit and public inputs
- ✅ **Decap**: Recover key from ciphertext using valid witness
- ✅ R1CS to QAP conversion
- ✅ BN254 (alt_bn128) curve support - Circom's default curve
- ✅ Blake3 KDF for key derivation
- ✅ Serializable circuit format
- ✅ `no_std` environment support

**Features**:

- `std` (default): Standard library support
- `parallel`: Parallel computation acceleration
- `r1cs`: R1CS gadgets support
- `with_curves`: Enable concrete curves (BN254, BLS12-381)
- `test_fixtures`: Test fixture support (serialized circuit loading)

### zkenc-cli

Command-line interface tool providing witness encryption for Circom circuits.

**Key Commands:**

- `zkenc encrypt` - One-step encryption (encap + AES-256-GCM)
- `zkenc decrypt` - One-step decryption (decap + AES-256-GCM)
- `zkenc encap` - Low-level: generate witness-encrypted ciphertext
- `zkenc decap` - Low-level: recover key from witness

**Features:**

- ✅ Combined ciphertext format (compatible with zkenc-js)
- ✅ Public inputs can be embedded in ciphertext
- ✅ Batch file processing support
- ✅ Comprehensive error handling

See the [CLI README](./packages/zkenc-cli/README.md) for detailed command documentation.

### zkenc-js

JavaScript/WASM bindings for browser and Node.js environments.

**Key Functions:**

- `encrypt()` - One-step encryption (encap + AES-256-GCM)
- `decrypt()` - One-step decryption (decap + AES-256-GCM)
- `encap()` - Low-level: generate witness-encrypted ciphertext
- `decap()` - Low-level: recover key from witness
- `getPublicInput()` - Extract public inputs from ciphertext

**Features:**

- ✅ Works in browser and Node.js
- ✅ Full TypeScript support
- ✅ Combined ciphertext format
- ✅ Optional public input embedding
- ✅ 29+ test cases

See the [zkenc-js README](./packages/zkenc-js/README.md) for detailed API documentation.

## 🛠️ Tech Stack

- **Language**: Rust (edition 2021)
- **Math Libraries**: [arkworks](https://github.com/arkworks-rs) ecosystem
  - `ark-ff`: Finite field arithmetic
  - `ark-ec`: Elliptic curve operations
  - `ark-poly`: Polynomial operations
  - `ark-relations`: R1CS constraint systems
  - `ark-snark`: SNARK abstractions
  - `ark-crypto-primitives`: Cryptographic primitives
- **WASM**: wasm-bindgen, wasm-pack
- **CLI**: clap 4.5

## 📖 Development Guide

For detailed package documentation, please refer to:

- [packages/zkenc-core/README.md](./packages/zkenc-core/README.md) - Core algorithm documentation
- [packages/zkenc-cli/README.md](./packages/zkenc-cli/README.md) - CLI usage guide
- [packages/zkenc-js/README.md](./packages/zkenc-js/README.md) - JavaScript API documentation

## 🔧 Common Commands

```bash
# Check compilation (no binary output)
cargo check --workspace

# Format code
cargo fmt --all

# Run linter
cargo clippy --workspace

# Generate documentation
cargo doc --workspace --open

# Build as WASM (optimized)
cargo build -p zkenc-core --no-default-features --features "wasm" --target wasm32-unknown-unknown --release
```

## 📝 License

MIT/Apache-2.0 dual license
