# zkenc-handmade

A witness encryption implementation for QAP (Quadratic Arithmetic Programs) with Circom circuit support.

## 📦 Project Structure

```
zkenc-handmade/
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

### Complete Sudoku Encryption Example

Suppose you have a Sudoku circuit and want to encrypt a message that only someone who knows the correct solution can decrypt:

```bash
# Prepare files
# - sudoku.r1cs: Circom-compiled circuit
# - puzzle.json: Sudoku puzzle (public inputs)
# - solution.wtns: snarkjs-generated correct solution witness

# Step 1: Generate ciphertext and encryption key
zkenc encap \
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --ciphertext ciphertext.bin \
  --key encryption_key.bin

# Step 2: Encrypt your message
echo "Secret message for Sudoku solver" > message.txt
zkenc encrypt \
  --key encryption_key.bin \
  --input message.txt \
  --output encrypted_message.bin

# Step 3: (Performed by someone with the solution) Recover key from ciphertext
zkenc decap \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext ciphertext.bin \
  --key decryption_key.bin

# Step 4: Decrypt the message
zkenc decrypt \
  --key decryption_key.bin \
  --input encrypted_message.bin \
  --output decrypted_message.txt

# Verify
cat decrypted_message.txt
# Output: Secret message for Sudoku solver
```

### Generating Inputs with Circom and snarkjs

```bash
# 1. Compile Circom circuit
circom sudoku.circom --r1cs --wasm --sym

# 2. Prepare input JSON
cat > input.json << EOF
{
  "puzzle": [5,3,0,0,7,0,0,0,0, ...],
  "solution": [5,3,4,6,7,8,9,1,2, ...]
}
EOF

EOF

# 3. Calculate witness
snarkjs wtns calculate sudoku.wasm input.json witness.wtns

# 4. Verify witness (optional)
snarkjs wtns check sudoku.r1cs witness.wtns

# 5. Now you can use zkenc!
```

## 📚 Package Documentation

### zkenc-core

Core witness encryption algorithm implementation using the arkworks ecosystem.

**Core Features**:

- ✅ **Encap**: Generate ciphertext and key from circuit and public inputs
- ✅ **Decap**: Recover key from ciphertext using witness
- ✅ R1CS to QAP conversion
- ✅ BN254 (alt_bn128) curve support - Circom's default curve
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

See the [CLI README](./packages/zkenc-cli/README.md) for detailed command documentation.

### zkenc-js

JavaScript/WASM bindings for browser and Node.js environments.

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
