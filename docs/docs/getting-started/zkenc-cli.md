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

### 4. Generate Witness-Encrypted Key

Use `encap` to generate a ciphertext and encryption key from public inputs:

```bash
zkenc encap \
  --circuit circuit_output/example.r1cs \
  --input public_inputs.json \
  --ciphertext witness_ct.bin \
  --key encryption_key.bin
```

Output:

```
Encap successful!
Ciphertext saved to: witness_ct.bin
Key saved to: encryption_key.bin
```

### 5. Encrypt Your Secret Message

Use the generated key to encrypt your message with AES-256-GCM:

```bash
echo "Hello, zkenc!" > message.txt
zkenc encrypt \
  --key encryption_key.bin \
  --input message.txt \
  --output encrypted_message.bin
```

Output:

```
Encryption successful!
Encrypted message saved to: encrypted_message.bin
```

### 6. Generate Witness File

Before decrypting, the recipient needs to generate a witness proving they have a valid solution:

```bash
snarkjs wtns calculate \
  circuit_output/example_js/example.wasm \
  full_inputs.json \
  witness.wtns
```

### 7. Recover the Key

Use `decap` with the witness to recover the encryption key:

```bash
zkenc decap \
  --circuit circuit_output/example.r1cs \
  --witness witness.wtns \
  --ciphertext witness_ct.bin \
  --key recovered_key.bin
```

Output:

```
Decap successful!
Key recovered and saved to: recovered_key.bin
```

### 8. Decrypt the Message

Use the recovered key to decrypt the message:

```bash
zkenc decrypt \
  --key recovered_key.bin \
  --input encrypted_message.bin \
  --output decrypted.txt
```

Output:

```
Decryption successful!
Message saved to: decrypted.txt
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

Encrypt a message using an encryption key (AES-256-GCM).

```bash
zkenc encrypt \
  --key <KEY_FILE> \
  --input <MESSAGE_FILE> \
  --output <OUTPUT_FILE>
```

**Arguments:**

- `--key <FILE>` - Path to encryption key file (from encap)
- `--input <FILE>` - Path to plaintext message file
- `--output <FILE>` - Output path for encrypted file

**Example:**

```bash
zkenc encrypt \
  --key key.bin \
  --input message.txt \
  --output encrypted.bin
```

---

### `zkenc decrypt`

Decrypt a message using a decryption key.

```bash
zkenc decrypt \
  --key <KEY_FILE> \
  --input <ENCRYPTED_FILE> \
  --output <OUTPUT_FILE>
```

**Arguments:**

- `--key <FILE>` - Path to decryption key file (from decap)
- `--input <FILE>` - Path to encrypted file
- `--output <FILE>` - Output path for decrypted message

**Example:**

```bash
zkenc decrypt \
  --key recovered_key.bin \
  --input encrypted.bin \
  --output decrypted.txt
```

## Understanding the Workflow

zkenc-cli uses a four-step process for witness encryption:

1. **`encap`** - Generate witness-encrypted ciphertext and key from public inputs
2. **`encrypt`** - Encrypt your message using the generated key (AES-256-GCM)
3. **`decap`** - Recover the key using a valid witness (solution)
4. **`decrypt`** - Decrypt the message using the recovered key

This separation allows you to:

- Share the witness ciphertext and encrypted message publicly
- Only those with a valid witness can recover the key
- The actual message remains secure with symmetric encryption

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
- For `encap`, only provide public inputs
- For `decap`, provide witness file with all signals

## Working with Binary Files

### Encrypting Binary Files

You can encrypt any file type:

```bash
# Step 1: Generate key from circuit
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness_ct.bin \
  --key key.bin

# Step 2: Encrypt an image
zkenc encrypt \
  --key key.bin \
  --input photo.jpg \
  --output encrypted_photo.bin

# Step 3: Someone with witness recovers the key
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness_ct.bin \
  --key recovered_key.bin

# Step 4: Decrypt the image
zkenc decrypt \
  --key recovered_key.bin \
  --input encrypted_photo.bin \
  --output decrypted_photo.jpg
```

## Advanced Usage

### Batch Processing

Encrypt multiple messages with the same witness ciphertext:

```bash
# Generate key once
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext shared_witness_ct.bin \
  --key key.bin

# Encrypt multiple files
for file in documents/*.txt; do
  zkenc encrypt \
    --key key.bin \
    --input "$file" \
    --output "encrypted/$(basename $file).enc"
done
```

### Cross-Tool Compatibility

Files created with zkenc-cli work with zkenc-js:

```bash
# Encrypt with CLI
zkenc encap --circuit circuit.r1cs --input public.json --ciphertext ct.bin --key k.bin
zkenc encrypt --key k.bin --input message.txt --output encrypted.bin

# The encrypted.bin can be used in zkenc-js
# Load and decrypt in Node.js or browser
```

[Learn more about cross-tool workflows →](/docs/guides/cross-tool-workflow)

## Performance Tips

1. **Reuse witness ciphertext**: Generate once, use for multiple messages
2. **Pre-compile circuits**: Compile circuits once, reuse many times
3. **Consider circuit size**: Larger circuits = slower encap/decap operations
4. **Use binary format**: Files are already in efficient binary format

## Common Patterns

### Conditional Access Control

```bash
# Only users who solve the puzzle can decrypt
zkenc encap --circuit puzzle.r1cs --input question.json --ciphertext puzzle_ct.bin --key key.bin
zkenc encrypt --key key.bin --input "Secret answer: 42" --output secret.bin
```

### Time-Lock Encryption

```bash
# Requires computational work to generate witness
zkenc encap --circuit timelock.r1cs --input params.json --ciphertext timelock_ct.bin --key key.bin
zkenc encrypt --key key.bin --input future_message.txt --output locked.bin
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

### "Decap failed"

This usually means:

- The witness doesn't satisfy the circuit constraints
- The witness file is corrupted
- Using wrong circuit files

Verify your witness first:

```bash
snarkjs wtns check circuit.r1cs witness.wtns
```

### "Decryption failed"

Ensure:

- You're using the correct key (from decap)
- The encrypted file isn't corrupted
- Key and ciphertext match

## Support

For issues or questions:

1. Check the [API Reference](/docs/api/zkenc-cli)
2. Review [example workflows](/docs/guides/cross-tool-workflow)
3. Open an issue on [GitHub](https://github.com/flyinglimao/zkenc)
