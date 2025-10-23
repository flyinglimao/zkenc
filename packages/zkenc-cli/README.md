# zkenc-cli

Command-line interface for zkenc witness encryption with Circom circuit support.

## Architecture

zkenc-cli is built on top of zkenc-core, providing a command-line interface for witness encryption operations:

```
┌─────────────────────────────────┐
│         zkenc-cli               │
│  (Command-line Application)     │
└───────────────┬─────────────────┘
                │
  ┌─────────────▼──────────────┐
  │      zkenc-core            │
  │  (Cryptographic Foundation)│
  └────────────────────────────┘
```

## Installation

````bash
git clone https://github.com/flyinglimao/zkenc
cd zkenc/packages/zkenc-cli

### Install to PATH

```bash
cargo install --path .
````

## 📖 Usage

### Command Overview

zkenc provides two levels of commands:

**High-Level API (Recommended):**

1. **`encrypt`** - One-step encryption (encap + AES)
2. **`decrypt`** - One-step decryption (decap + AES)

**Low-Level API (Advanced):** 3. **`encap`** - Generate ciphertext and encryption key 4. **`decap`** - Recover decryption key using witness

### High-Level Commands

#### `zkenc encrypt`

Encrypt a message using witness encryption (one-step operation, compatible with zkenc-js).

```bash
zkenc encrypt \
  --circuit <R1CS_FILE> \
  --input <JSON_FILE> \
  --message <MESSAGE_FILE> \
  --output <OUTPUT_FILE> \
  [--no-public-input]
```

**Arguments:**

- `--circuit <FILE>` - Path to R1CS circuit file (`.r1cs` from Circom)
- `--input <FILE>` - Path to JSON file with public inputs
- `--message <FILE>` - Path to plaintext message file
- `--output <FILE>` - Output path for combined ciphertext
- `--no-public-input` - Don't embed public inputs in ciphertext (optional)

**Example:**

```bash
zkenc encrypt \
  --circuit tests/r1cs/sudoku.r1cs \
  --input tests/inputs/sudoku_general.json \
  --message secret.txt \
  --output encrypted.bin
```

**Output:** Creates a combined ciphertext file compatible with zkenc-js `decrypt()`.

#### `zkenc decrypt`

Decrypt a message using witness decryption (one-step operation, compatible with zkenc-js).

```bash
zkenc decrypt \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --output <OUTPUT_FILE>
```

**Arguments:**

- `--circuit <FILE>` - Path to R1CS circuit file
- `--witness <FILE>` - Path to witness file (`.wtns` from snarkjs)
- `--ciphertext <FILE>` - Path to combined ciphertext file
- `--output <FILE>` - Output path for decrypted message

**Example:**

```bash
zkenc decrypt \
  --circuit tests/r1cs/sudoku.r1cs \
  --witness tests/inputs/sudoku_solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

**Input:** Can decrypt files created by zkenc-js `encrypt()`.

### Low-Level Commands

For advanced use cases or custom encryption schemes:

#### `zkenc encap`

Generate ciphertext and encryption key from circuit and public inputs.

```bash
zkenc encap \
  --circuit <R1CS_FILE> \
  --input <JSON_FILE> \
  --ciphertext <OUTPUT_CT> \
  --key <OUTPUT_KEY>
```

**Arguments:**

- `--circuit <FILE>` - Path to R1CS circuit file (`.r1cs` from Circom)
- `--input <FILE>` - Path to JSON file with public inputs
- `--ciphertext <FILE>` - Output path for ciphertext
- `--key <FILE>` - Output path for encryption key

**Example:**

```bash
zkenc encap \
  --circuit tests/r1cs/sudoku.r1cs \
  --input tests/inputs/sudoku_basic.json \
  --ciphertext ciphertext.bin \
  --key key.bin
```

#### `zkenc decap`

Recover the encryption key using a valid witness and ciphertext.

```bash
zkenc decap \
  --circuit <R1CS_FILE> \
  --witness <WTNS_FILE> \
  --ciphertext <CT_FILE> \
  --key <OUTPUT_KEY>
```

**Arguments:**

- `--circuit <FILE>` - Path to R1CS circuit file
- `--witness <FILE>` - Path to witness file (`.wtns` from snarkjs)
- `--ciphertext <FILE>` - Path to ciphertext file
- `--key <FILE>` - Output path for recovered key

**Example:**

```bash
zkenc decap \
  --circuit tests/r1cs/sudoku.r1cs \
  --witness tests/inputs/sudoku_sudoku_basic.wtns \
  --ciphertext ciphertext.bin \
  --key recovered_key.bin
```

#### `zkenc encrypt`

Encrypt a message using the encryption key (AES-256-GCM).

```bash
zkenc encrypt \
  --key <KEY_FILE> \
  --input <MESSAGE_FILE> \
  --output <OUTPUT_FILE>
```

**Arguments:**

- `--key <FILE>` - Path to encryption key file
- `--input <FILE>` - Path to plaintext message file
- `--output <FILE>` - Output path for encrypted file

**Example:**

```bash
zkenc decap \
  --circuit tests/r1cs/sudoku.r1cs \
  --witness tests/inputs/sudoku_sudoku_basic.wtns \
  --ciphertext ciphertext.bin \
  --key recovered_key.bin
```

## 🎯 Complete Workflow Examples

### Simple Workflow (Recommended)

Using high-level commands for maximum compatibility:

```bash
# Encrypt
zkenc encrypt \
  --circuit tests/r1cs/sudoku.r1cs \
  --input tests/inputs/sudoku_general.json \
  --message secret.txt \
  --output encrypted.bin

# Decrypt (with valid witness)
zkenc decrypt \
  --circuit tests/r1cs/sudoku.r1cs \
  --witness tests/inputs/sudoku_solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt

# Verify
diff secret.txt decrypted.txt
```

**Benefits:**

- ✅ Compatible with zkenc-js
- ✅ Single ciphertext file
- ✅ Public inputs embedded (self-contained)
- ✅ Simpler workflow

### Advanced Workflow

Using low-level commands for custom encryption:

```bash
# Step 1: Generate witness ciphertext and key
zkenc encap \
  --circuit tests/r1cs/sudoku.r1cs \
  --input tests/inputs/sudoku_general.json \
  --ciphertext c.bin \
  --key k.bin

# Step 2: Use the key with any encryption method
# (k.bin is a 32-byte key suitable for AES-256 or other schemes)

# Step 3: (Someone with valid witness) Recover the key
zkenc decap \
  --circuit tests/r1cs/sudoku.r1cs \
  --witness tests/inputs/sudoku_solution.wtns \
  --ciphertext c.bin \
  --key recovered_key.bin

# Step 4: Decrypt with the same method used in Step 2
```

**Use cases:**

- Custom encryption schemes
- Integration with existing encryption pipelines
- Educational purposes
- Key reuse for multiple messages

## 🔧 Input File Formats

### R1CS Circuit File (`.r1cs`)

Generated by Circom compiler:

```bash
circom circuit.circom --r1cs --wasm --sym
```

### Witness File (`.wtns`)

Generated by snarkjs:

```bash
# Calculate witness from input
snarkjs wtns calculate circuit.wasm input.json witness.wtns

# Verify witness (optional)
snarkjs wtns check circuit.r1cs witness.wtns
```

### Input JSON File

JSON object with public inputs (flattened in alphabetical key order):

```json
{
  "puzzle": ["5", "3", "0", "0", "7", "0", "..."],
  "solution": ["5", "3", "4", "6", "7", "8", "..."]
}
```

**Note:** All values must be strings.

## 🔄 Cross-Tool Compatibility

zkenc-cli is **fully compatible** with zkenc-js!

### CLI → JS

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

const ciphertext = await fs.readFile("encrypted.bin");
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

### JS → CLI

```typescript
// Encrypt with zkenc-js
const { ciphertext } = await zkenc.encrypt(circuitFiles, publicInputs, message);
await fs.writeFile("encrypted.bin", ciphertext);
```

```bash
# Decrypt with CLI
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

**Both tools use the same combined ciphertext format - no conversion needed!**

Learn more: [Cross-Tool Workflow Guide](../../docs/docs/guides/cross-tool-workflow.md)

## 🧪 Testing

Run the test suite:

```bash
# Unit tests
cargo test

# E2E integration tests
cargo test --test e2e_test

# Run specific test with output
cargo test test_sudoku_e2e -- --nocapture
```

## 📝 Technical Details

### Supported Curves

- **BN254 (alt_bn128)** - Default curve used by Circom

### Encryption Algorithm

- **AES-256-GCM** - Authenticated encryption for message protection
- **Blake3** - Key derivation function (32-byte output)

### Combined Ciphertext Format

The `encrypt` command creates a combined format:

```
[1 byte flag]
[4 bytes witness CT length]
[witness ciphertext]
[4 bytes public input length]  (if flag = 1)
[public input JSON]             (if flag = 1)
[encrypted message]
```

This format is compatible with zkenc-js and allows self-contained ciphertexts.

### File Sizes

**High-level commands:**

- Combined ciphertext ≈ 1576 bytes (witness CT) + public inputs + message + overhead

**Low-level commands:**

- Witness ciphertext: ~1576 bytes (for BN254)
- Key: 32 bytes
- Witness: Varies by circuit size

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
