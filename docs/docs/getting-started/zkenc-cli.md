---
sidebar_position: 2
---

# Getting Started with zkenc-cli

zkenc-cli is a command-line tool for witness encryption operations. It provides a simple interface for encrypting and decrypting messages using Circom circuits.

## Installation

### From Source

Clone the repository and build from source:

```bash
git clone https://github.com/flyinglimao/zkenc.git
cd zkenc/packages/zkenc-cli
cargo install --path .
```

## Prerequisites

Before using zkenc-cli, you need:

1. **A compiled Circom circuit** with:

   - `.r1cs` file (circuit constraints)
   - `.wasm` file (witness generator)

2. **Input files** in JSON format

## Quick Start

### 1. Create a Simple Circuit

Create a file `example.circom`:

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

### 2. Compile the Circuit

```bash
circom example.circom --r1cs --wasm --output circuit_output
```

This creates:

- `circuit_output/example.r1cs`
- `circuit_output/example_js/example.wasm`

### 3. Prepare Input Files

Create `public_inputs.json` (known when encrypting):

```json
{
  "publicValue": "42"
}
```

Create `full_inputs.json` (needed for decryption):

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

### 4. Encrypt Your Secret Message

Use `encrypt` to perform witness encryption in one step:

```bash
echo "Hello, zkenc!" > message.txt
zkenc encrypt \
  --circuit circuit_output/example.r1cs \
  --input public_inputs.json \
  --message message.txt \
  --output encrypted.bin
```

This command:

- Generates a witness-encrypted key from public inputs (encap)
- Encrypts your message with AES-256-GCM
- Combines everything into a single ciphertext file
- Embeds public inputs in the ciphertext (by default)

Output:

```
🔐 Step 1: Running Encap...
📂 Loading R1CS circuit...
   - Constraints: 2
   - Public inputs: 1
   - Wires: 4

📋 Loading public inputs from JSON...
   - Parsed 1 field elements

   ✅ Witness ciphertext generated (123 bytes)

🔒 Step 2: Encrypting message...
   - Message size: 14 bytes
   ✅ Message encrypted (42 bytes)

📦 Step 3: Creating combined ciphertext...
   ✅ Combined ciphertext saved (218 bytes)

✨ Encryption complete! Public inputs are embedded in the ciphertext.
```

### 5. Generate Witness File

Before decrypting, the recipient needs to generate a witness proving they have a valid solution:

```bash
snarkjs wtns calculate \
  circuit_output/example_js/example.wasm \
  full_inputs.json \
  witness.wtns
```

### 6. Decrypt the Message

Use `decrypt` to recover and decrypt the message in one step:

```bash
zkenc decrypt \
  --circuit circuit_output/example.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

This command:

- Parses the combined ciphertext
- Recovers the key using the witness (decap)
- Decrypts the message with AES-256-GCM

Output:

```
📦 Step 1: Parsing combined ciphertext...
   - Flag: 1
   - Witness ciphertext: 123 bytes
   - Public input: {"publicValue":"42"}
   - Encrypted message: 42 bytes

🔓 Step 2: Running Decap...
📂 Loading R1CS circuit...
   - Constraints: 2
   - Public inputs: 1

📋 Loading witness from snarkjs...
   - Witness elements: 4

   ✅ Key recovered from witness

🔓 Step 3: Decrypting message...
   ✅ Decrypted message saved (14 bytes)

✨ Decryption complete!
```

Verify the result:

```bash
cat decrypted.txt
# Output: Hello, zkenc!
```

## Command Reference

### `zkenc encap`

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
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --ciphertext ciphertext.bin \
  --key key.bin
```

---

### `zkenc decap`

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
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext ciphertext.bin \
  --key recovered_key.bin
```

---

### `zkenc encrypt`

Encrypt a message using witness encryption (high-level, one-step operation).

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

**What it does:**

This command combines encap and AES encryption into a single step:

1. Generates witness-encrypted key from public inputs
2. Encrypts message with AES-256-GCM
3. Creates combined ciphertext with format: `[flag][witnessLen][witnessCT][publicLen][publicInput][encryptedMsg]`

**Example:**

```bash
zkenc encrypt \
  --circuit sudoku.r1cs \
  --input puzzle.json \
  --message secret.txt \
  --output encrypted.bin
```

**Compatibility:** The output is fully compatible with zkenc-js `decrypt()` function.

---

### `zkenc decrypt`

Decrypt a message using witness decryption (high-level, one-step operation).

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

**What it does:**

This command combines decap and AES decryption into a single step:

1. Parses the combined ciphertext
2. Recovers key using the witness
3. Decrypts message with AES-256-GCM

**Example:**

```bash
zkenc decrypt \
  --circuit sudoku.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

**Compatibility:** Can decrypt files created by zkenc-js `encrypt()` function.

---

### Low-Level Commands

For advanced use cases, you can use the low-level encap/decap commands separately:

#### `zkenc encap`

## Understanding the Workflow

zkenc-cli provides two levels of API:

### High-Level API (Recommended)

A simple two-step process:

1. **`encrypt`** - Combines encap + AES encryption in one command

   - Input: circuit, public inputs, message
   - Output: combined ciphertext (compatible with zkenc-js)

2. **`decrypt`** - Combines decap + AES decryption in one command
   - Input: circuit, witness, combined ciphertext
   - Output: decrypted message

**Benefits:**

- Simpler workflow (2 steps vs 4)
- Single ciphertext file to manage
- Full compatibility with zkenc-js
- Public inputs can be embedded in ciphertext

### Low-Level API (Advanced)

A four-step process for fine-grained control:

1. **`encap`** - Generate witness-encrypted ciphertext and key from public inputs
2. Encrypt message separately (use any AES tool)
3. **`decap`** - Recover the key using a valid witness
4. Decrypt message separately (use any AES tool)

**Use cases:**

- Custom encryption schemes
- Key reuse across multiple messages
- Integration with existing encryption pipelines
- Educational purposes to understand the protocol

**Note:** For most use cases, the high-level API is recommended as it ensures compatibility and simplifies the workflow.

## Input File Formats

### R1CS Circuit File (`.r1cs`)

Generated by Circom compiler:

```bash
circom circuit.circom --r1cs --wasm --sym
```

### Witness File (`.wtns`)

Generated by snarkjs from your complete inputs:

```bash
# Calculate witness from input
snarkjs wtns calculate circuit.wasm input.json witness.wtns

# Verify witness (optional)
snarkjs wtns check circuit.r1cs witness.wtns
```

### Input JSON File

JSON object with signal names as keys:

```json
{
  "publicValue": "42",
  "privateValue": "123",
  "arraySignal": ["1", "2", "3"]
}
```

**Important Notes:**

- All values must be strings (even numbers)
- Array signals are supported
- Signal names must match those defined in your circuit
- For `encrypt`, only provide public inputs
- For `decrypt`, provide witness file generated from full inputs (public + private)

## Combined Ciphertext Format

The `encrypt` command creates a combined ciphertext with the following structure:

```
[1 byte flag]
[4 bytes witness CT length]
[witness ciphertext]
[4 bytes public input length]  (if flag = 1)
[public input JSON]             (if flag = 1)
[encrypted message]
```

**Flag byte:**

- `1` = Public inputs included (default)
- `0` = Public inputs not included (use `--no-public-input`)

This format is compatible with zkenc-js and allows:

- Self-contained ciphertext (includes all necessary data)
- Cross-tool compatibility
- Optional public input embedding

## Working with Binary Files

### Encrypting Binary Files

You can encrypt any file type with the high-level API:

```bash
# Encrypt an image in one step
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message photo.jpg \
  --output encrypted_photo.bin

# Someone with witness decrypts the image in one step
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted_photo.bin \
  --output decrypted_photo.jpg
```

### Using Low-Level API for Binary Files

For advanced use cases:

```bash
# Step 1: Generate key from circuit
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness_ct.bin \
  --key key.bin

# Step 2: Encrypt with external tool or custom method
# (The key.bin is a 32-byte key suitable for AES-256)

# Step 3: Recipient recovers the key
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness_ct.bin \
  --key recovered_key.bin

# Step 4: Decrypt with the same method used in Step 2
```

## Advanced Usage

### Encrypting Without Embedding Public Inputs

By default, `encrypt` embeds public inputs in the ciphertext. To exclude them:

```bash
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin \
  --no-public-input
```

**When to use `--no-public-input`:**

- Public inputs are very large
- You'll distribute public inputs separately
- You want smaller ciphertext files

**Note:** Recipients will need the public inputs to verify the witness.

### Batch Processing

Encrypt multiple messages for the same circuit and public inputs:

```bash
# Encrypt multiple files with embedded public inputs
for file in documents/*.txt; do
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input public.json \
    --message "$file" \
    --output "encrypted/$(basename $file).enc"
done
```

Each encrypted file is self-contained and can be decrypted independently.

### Cross-Tool Compatibility

zkenc-cli is **fully compatible** with zkenc-js! You can encrypt with one tool and decrypt with the other:

**CLI → JS:**

```bash
# Encrypt with CLI
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message message.txt \
  --output encrypted.bin

# Decrypt with zkenc-js in Node.js or browser
# The encrypted.bin can be read by zkenc-js decrypt()
```

**JS → CLI:**

```bash
# After encrypting with zkenc-js encrypt()...
# Decrypt with CLI
zkenc decrypt \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

Both tools use the same combined ciphertext format, ensuring seamless interoperability.

[Learn more about cross-tool workflows →](/docs/guides/cross-tool-workflow)

## Performance Tips

1. **Use high-level API**: `encrypt`/`decrypt` commands handle everything efficiently
2. **Embed public inputs**: Keeps ciphertext self-contained (default behavior)
3. **Pre-compile circuits**: Compile circuits once, reuse many times
4. **Consider circuit size**: Larger circuits = slower encap/decap operations
5. **Binary format**: All files use efficient binary serialization

## Common Patterns

### Conditional Access Control

```bash
# Only users who solve the puzzle can decrypt
zkenc encrypt \
  --circuit puzzle.r1cs \
  --input question.json \
  --message "Secret answer: 42" \
  --output secret.bin
```

### Time-Lock Encryption

```bash
# Requires computational work to generate witness
zkenc encrypt \
  --circuit timelock.r1cs \
  --input params.json \
  --message future_message.txt \
  --output locked.bin
```

### Distributing Encrypted Files

```bash
# Encrypt with embedded public inputs
zkenc encrypt \
  --circuit circuit.r1cs \
  --input public.json \
  --message secret.txt \
  --output package.bin

# Share package.bin publicly
# Only those who can generate valid witness can decrypt
```

## Next Steps

- **[API Reference →](/docs/api/zkenc-cli)** - Complete CLI command reference
- **[Cross-Tool Workflow →](/docs/guides/cross-tool-workflow)** - Use CLI with zkenc-js
- **[zkenc-js Getting Started →](/docs/getting-started/zkenc-js)** - JavaScript alternative

## Troubleshooting

### "Circuit file not found"

Ensure the R1CS file path is correct:

```bash
# Check file exists
ls -lh circuit.r1cs
```

### "Invalid inputs"

Check that your JSON file:

- Is valid JSON format
- Contains all required signal names
- Uses string values for all numbers

```bash
# Validate JSON
cat inputs.json | jq .
```

### "Invalid ciphertext: too short"

This means the ciphertext file is corrupted or not a valid zkenc ciphertext. Ensure:

- The file was created by zkenc-cli `encrypt` or zkenc-js `encrypt()`
- The file wasn't modified or truncated
- You're using the correct file

### "Decap failed"

This usually means:

- The witness doesn't satisfy the circuit constraints
- The witness file is corrupted
- Using wrong circuit files
- The witness doesn't match the public inputs used for encryption

Verify your witness first:

```bash
snarkjs wtns check circuit.r1cs witness.wtns
```

### "Decryption failed" or "Message decryption failed"

Ensure:

- The witness satisfies the circuit constraints
- The ciphertext file is not corrupted
- Using the correct circuit file
- The witness matches the public inputs from encryption

## Support

For issues or questions:

1. Check the [API Reference](/docs/api/zkenc-cli)
2. Review [example workflows](/docs/guides/cross-tool-workflow)
3. Open an issue on [GitHub](https://github.com/flyinglimao/zkenc)
