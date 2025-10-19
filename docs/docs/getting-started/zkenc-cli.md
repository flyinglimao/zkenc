---
sidebar_position: 2
---

# Getting Started with zkenc-cli

zkenc-cli is a command-line tool for witness encryption operations. It provides a simple interface for encrypting and decrypting messages using Circom circuits.

## Installation

### From Source

Clone the repository and build from source:

```bash
git clone https://github.com/flyinglimao/zkenc-handmade.git
cd zkenc-handmade
cargo build --release

# The binary will be at target/release/zkenc
```

### Add to PATH

For convenient access, add the binary to your PATH:

```bash
# Linux/macOS
export PATH=$PATH:$(pwd)/target/release

# Or copy to a system directory
sudo cp target/release/zkenc /usr/local/bin/
```

## Prerequisites

Before using zkenc-cli, you need:

1. **A compiled Circom circuit** with:

   - `.r1cs` file (circuit constraints)
   - `.wasm` file (witness generator)

2. **Input files** in JSON format

## Quick Example

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

### 4. Encrypt a Message

```bash
zkenc encrypt \
  --r1cs circuit_output/example.r1cs \
  --wasm circuit_output/example_js/example.wasm \
  --inputs public_inputs.json \
  --message "Hello, zkenc!" \
  --output encrypted.bin
```

Output:

```
Encryption successful!
Ciphertext saved to: encrypted.bin
Ciphertext size: 1636 bytes
```

### 5. Decrypt the Message

```bash
zkenc decrypt \
  --r1cs circuit_output/example.r1cs \
  --wasm circuit_output/example_js/example.wasm \
  --inputs full_inputs.json \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

Output:

```
Decryption successful!
Message saved to: decrypted.txt
Message: Hello, zkenc!
```

## Command Reference

### `zkenc encrypt`

Encrypt a message using a circuit and public inputs.

```bash
zkenc encrypt [OPTIONS]
```

**Required Options:**

- `--r1cs <FILE>` - Path to the R1CS file
- `--wasm <FILE>` - Path to the WASM file
- `--inputs <FILE>` - Path to JSON file with public inputs
- `--message <TEXT>` or `--message-file <FILE>` - Message to encrypt
- `--output <FILE>` - Output file for ciphertext

**Example:**

```bash
# Encrypt text message
zkenc encrypt \
  --r1cs circuit.r1cs \
  --wasm circuit.wasm \
  --inputs public.json \
  --message "Secret message" \
  --output encrypted.bin

# Encrypt file contents
zkenc encrypt \
  --r1cs circuit.r1cs \
  --wasm circuit.wasm \
  --inputs public.json \
  --message-file secret.txt \
  --output encrypted.bin
```

### `zkenc decrypt`

Decrypt a ciphertext using a circuit and full witness inputs.

```bash
zkenc decrypt [OPTIONS]
```

**Required Options:**

- `--r1cs <FILE>` - Path to the R1CS file
- `--wasm <FILE>` - Path to the WASM file
- `--inputs <FILE>` - Path to JSON file with complete witness
- `--ciphertext <FILE>` - Path to ciphertext file
- `--output <FILE>` - Output file for decrypted message

**Example:**

```bash
zkenc decrypt \
  --r1cs circuit.r1cs \
  --wasm circuit.wasm \
  --inputs witness.json \
  --ciphertext encrypted.bin \
  --output decrypted.txt
```

## Input File Format

Input files must be in JSON format with signal names as keys:

```json
{
  "signalName1": "value1",
  "signalName2": "value2",
  "arraySignal": ["1", "2", "3"]
}
```

**Important Notes:**

- All values must be strings (even numbers)
- Array signals are supported
- Signal names must match those defined in your circuit
- For encryption, only provide public inputs
- For decryption, provide all inputs (public + private)

## Working with Files

### Encrypting Binary Files

```bash
# Encrypt an image
zkenc encrypt \
  --r1cs circuit.r1cs \
  --wasm circuit.wasm \
  --inputs public.json \
  --message-file image.png \
  --output encrypted.bin

# Decrypt the image
zkenc decrypt \
  --r1cs circuit.r1cs \
  --wasm circuit.wasm \
  --inputs witness.json \
  --ciphertext encrypted.bin \
  --output decrypted.png
```

### Batch Operations

```bash
# Encrypt multiple messages
for file in messages/*.txt; do
  zkenc encrypt \
    --r1cs circuit.r1cs \
    --wasm circuit.wasm \
    --inputs public.json \
    --message-file "$file" \
    --output "encrypted/$(basename $file).enc"
done
```

## Cross-Tool Compatibility

Ciphertexts created with zkenc-cli can be decrypted with zkenc-js and vice versa:

```bash
# Encrypt with CLI
zkenc encrypt \
  --r1cs circuit.r1cs \
  --wasm circuit.wasm \
  --inputs public.json \
  --message "Hello" \
  --output encrypted.bin

# The encrypted.bin file can be loaded in zkenc-js:
# const ciphertext = await fs.readFile('encrypted.bin');
# const message = await zkenc.decrypt(circuitFiles, ciphertext, inputs);
```

[Learn more about cross-tool workflows →](/docs/guides/cross-tool-workflow)

## Advanced Usage

### Using Different Circuits

```bash
# Encrypt with one circuit
zkenc encrypt \
  --r1cs circuit1.r1cs \
  --wasm circuit1.wasm \
  --inputs inputs1.json \
  --message "Message" \
  --output encrypted1.bin

# Decrypt with a different (but compatible) circuit
zkenc decrypt \
  --r1cs circuit2.r1cs \
  --wasm circuit2.wasm \
  --inputs inputs2.json \
  --ciphertext encrypted1.bin \
  --output decrypted.txt
```

### Piping Operations

```bash
# Generate message programmatically
echo "Dynamic message $(date)" | zkenc encrypt \
  --r1cs circuit.r1cs \
  --wasm circuit.wasm \
  --inputs public.json \
  --message-file - \
  --output encrypted.bin
```

## Performance Tips

1. **Keep circuit files accessible**: Avoid remote/slow storage
2. **Use binary format**: The ciphertext is already in efficient binary format
3. **Pre-compile circuits**: Compile circuits once, reuse multiple times
4. **Consider circuit complexity**: Larger circuits = slower operations

## Common Patterns

### Conditional Access

```bash
# Only users who know the answer can decrypt
zkenc encrypt \
  --r1cs puzzle.r1cs \
  --wasm puzzle.wasm \
  --inputs question.json \
  --message "The secret answer is 42" \
  --output puzzle.enc
```

### Time-Lock Encryption

```bash
# Message that requires computational work to decrypt
zkenc encrypt \
  --r1cs timelock.r1cs \
  --wasm timelock.wasm \
  --inputs parameters.json \
  --message "Future message" \
  --output timelock.enc
```

## Next Steps

- **[API Reference →](/docs/api/zkenc-cli)** - Complete CLI reference
- **[Cross-Tool Workflow →](/docs/guides/cross-tool-workflow)** - Use CLI with zkenc-js
- **[zkenc-js Getting Started →](/docs/getting-started/zkenc-js)** - JavaScript alternative

## Troubleshooting

### "Circuit file not found"

Ensure the paths to R1CS and WASM files are correct:

```bash
# Check files exist
ls -lh circuit.r1cs circuit.wasm
```

### "Invalid inputs"

Check that your JSON file:

- Is valid JSON
- Contains all required signals
- Uses string values for all numbers

```bash
# Validate JSON
cat inputs.json | jq .
```

### "Decryption failed"

This usually means:

- The inputs don't satisfy the circuit constraints
- The ciphertext is corrupted
- Using wrong circuit files

Verify that your inputs produce a valid witness:

```bash
# Test witness generation separately
node circuit_js/generate_witness.js circuit.wasm inputs.json witness.wtns
```

## Support

For issues or questions:

1. Check the [API Reference](/docs/api/zkenc-cli)
2. Review [cross-tool workflows](/docs/guides/cross-tool-workflow)
3. Open an issue on [GitHub](https://github.com/flyinglimao/zkenc-handmade)
