---
sidebar_position: 2
---

# zkenc-cli API Reference

Complete command-line reference for zkenc-cli, the Rust-based witness encryption tool.

## Installation

### Build from Source

```bash
git clone https://github.com/flyinglimao/zkenc-handmade.git
cd zkenc-handmade
cargo build --release --package zkenc-cli

# Binary located at: target/release/zkenc
```

### Add to PATH

```bash
# Linux/macOS
export PATH=$PATH:$(pwd)/target/release

# Or install system-wide
sudo cp target/release/zkenc /usr/local/bin/
```

## Commands Overview

zkenc-cli provides four main commands:

| Command   | Purpose                    | Input                       | Output           |
| --------- | -------------------------- | --------------------------- | ---------------- |
| `encap`   | Generate key using circuit | R1CS + public inputs        | Ciphertext + Key |
| `decap`   | Recover key with witness   | R1CS + witness + ciphertext | Key              |
| `encrypt` | Encrypt message with key   | Key + message               | Encrypted file   |
| `decrypt` | Decrypt message with key   | Key + encrypted file        | Decrypted file   |

## Commands

### `zkenc encap`

Generate witness-encrypted key and ciphertext from circuit and public inputs.

```bash
zkenc encap [OPTIONS]
```

**Required Options:**

- `-c, --circuit <FILE>` - Path to R1CS circuit file (.r1cs)
- `-i, --input <FILE>` - Path to JSON file with public inputs
- `--ciphertext <FILE>` - Output path for ciphertext
- `-k, --key <FILE>` - Output path for encryption key

**Example:**

```bash
zkenc encap \
  --circuit sudoku.r1cs \
  --input public_inputs.json \
  --ciphertext witness.ct \
  --key encryption.key
```

**Input JSON Format:**

```json
{
  "puzzle": [5, 3, 0, 0, 7, 0, 0, 0, 0],
  "difficulty": 1
}
```

**Output:**

- **Ciphertext file**: ~1576 bytes (witness encryption ciphertext)
- **Key file**: ~32 bytes (AES-256 encryption key)

**Example Output:**

```
📂 Loading R1CS circuit...
   - Constraints: 12847
   - Public inputs: 81
   - Wires: 13129

📋 Loading public inputs from JSON...
   - Parsed 81 field elements

🔐 Running Encap...

💾 Saving ciphertext...
   ✅ Ciphertext saved (1576 bytes)

🔑 Saving key...
   ✅ Key saved (32 bytes)
```

### `zkenc decap`

Recover encryption key using valid witness.

```bash
zkenc decap [OPTIONS]
```

**Required Options:**

- `-c, --circuit <FILE>` - Path to R1CS circuit file (.r1cs)
- `-w, --witness <FILE>` - Path to witness file (.wtns from snarkjs)
- `--ciphertext <FILE>` - Path to ciphertext file (from encap)
- `-k, --key <FILE>` - Output path for recovered key

**Generating Witness:**

First, generate witness using snarkjs:

```bash
# Create full input JSON (public + private)
cat > full_inputs.json <<EOF
{
  "puzzle": [5, 3, 0, 0, 7, 0, 0, 0, 0],
  "solution": [5, 3, 4, 6, 7, 8, 9, 1, 2]
}
EOF

# Generate witness using snarkjs
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns
```

**Example:**

```bash
zkenc decap \
  --circuit sudoku.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key
```

**Output:**

```
📂 Loading R1CS circuit...
   - Constraints: 12847
   - Public inputs: 81
   - Wires: 13129

📋 Loading witness from snarkjs...
   - Witness elements: 13129

📦 Loading ciphertext...
   - Ciphertext size: 1576 bytes

🔓 Running Decap...

🔑 Saving recovered key...
   ✅ Key saved (32 bytes)
```

### `zkenc encrypt`

Encrypt a message using an encryption key.

```bash
zkenc encrypt [OPTIONS]
```

**Required Options:**

- `-k, --key <FILE>` - Path to encryption key file (from encap or decap)
- `-i, --input <FILE>` - Path to plaintext file
- `-o, --output <FILE>` - Path to output encrypted file

**Example:**

```bash
# Encrypt text file
zkenc encrypt \
  --key encryption.key \
  --input message.txt \
  --output message.txt.enc

# Encrypt binary file
zkenc encrypt \
  --key encryption.key \
  --input document.pdf \
  --output document.pdf.enc
```

**Output:**

```
🔑 Loading key...
📄 Loading plaintext...
   - Plaintext size: 1234 bytes

🔒 Encrypting...
   ✅ Encrypted file saved (1266 bytes)
```

**Note:** Output size = input size + 28 bytes (GCM nonce + tag)

### `zkenc decrypt`

Decrypt a message using an encryption key.

```bash
zkenc decrypt [OPTIONS]
```

**Required Options:**

- `-k, --key <FILE>` - Path to encryption key file
- `-i, --input <FILE>` - Path to encrypted file
- `-o, --output <FILE>` - Path to output decrypted file

**Example:**

```bash
zkenc decrypt \
  --key recovered.key \
  --input message.txt.enc \
  --output decrypted.txt
```

**Output:**

```
🔑 Loading key...
📦 Loading encrypted data...
   - Encrypted size: 1266 bytes

🔓 Decrypting...
   ✅ Decrypted file saved (1234 bytes)
```

## Complete Workflows

### Full Encryption/Decryption Flow

```bash
# 1. Encapsulate: Generate key with circuit
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key encryption.key

# 2. Encrypt: Encrypt message with key
zkenc encrypt \
  --key encryption.key \
  --input secret.txt \
  --output secret.txt.enc

# 3. Generate witness with snarkjs
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns

# 4. Decapsulate: Recover key with witness
zkenc decap \
  --circuit circuit.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key

# 5. Decrypt: Decrypt message with recovered key
zkenc decrypt \
  --key recovered.key \
  --input secret.txt.enc \
  --output decrypted.txt
```

### Simplified Flow (One-Step)

For convenience, you can combine encap + encrypt:

```bash
# Encrypt (encap + encrypt in one script)
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key temp.key

zkenc encrypt \
  --key temp.key \
  --input message.txt \
  --output message.enc

# Distribute: witness.ct + message.enc

# Decrypt (decap + decrypt in one script)
snarkjs wtns calculate circuit.wasm full_inputs.json witness.wtns

zkenc decap \
  --circuit circuit.r1cs \
  --witness witness.wtns \
  --ciphertext witness.ct \
  --key recovered.key

zkenc decrypt \
  --key recovered.key \
  --input message.enc \
  --output decrypted.txt
```

## File Formats

### Input JSON Format

**Public Inputs (for encap):**

```json
{
  "signalName1": 42,
  "signalName2": [1, 2, 3],
  "signalName3": "123"
}
```

**Full Inputs (for witness generation):**

```json
{
  "publicSignal": [5, 3, 0],
  "privateSignal": [5, 3, 4]
}
```

**Rules:**

- Numbers can be integers or strings
- Arrays are automatically flattened
- Keys are processed in sorted order
- All values must be valid field elements

### Circuit Files

**Required files:**

- `.r1cs` - R1CS circuit file (from circom compilation)
- `.wasm` - WASM witness generator (for snarkjs)

**Compile circuit:**

```bash
circom circuit.circom --r1cs --wasm --output build
# Creates: build/circuit.r1cs, build/circuit_js/circuit.wasm
```

### Witness Files

**Format:** `.wtns` (snarkjs binary format)

**Generate:**

```bash
snarkjs wtns calculate circuit.wasm inputs.json witness.wtns
```

### Output Files

- **Ciphertext** (`.ct`): ~1576 bytes, witness encryption ciphertext
- **Key** (`.key`): ~32 bytes, AES-256 encryption key
- **Encrypted** (`.enc`): original size + 28 bytes, AES-256-GCM ciphertext

## Integration Examples

### Bash Script

```bash
#!/bin/bash
set -e

CIRCUIT="sudoku.r1cs"
WASM="sudoku.wasm"
PUBLIC="public.json"
FULL="full_inputs.json"
MESSAGE="secret.txt"

echo "Encrypting..."
zkenc encap -c "$CIRCUIT" -i "$PUBLIC" --ciphertext ct.bin -k key.bin
zkenc encrypt -k key.bin -i "$MESSAGE" -o encrypted.bin

echo "Decrypting..."
snarkjs wtns calculate "$WASM" "$FULL" witness.wtns
zkenc decap -c "$CIRCUIT" -w witness.wtns --ciphertext ct.bin -k recovered.bin
zkenc decrypt -k recovered.bin -i encrypted.bin -o decrypted.txt

echo "Verification..."
diff "$MESSAGE" decrypted.txt && echo "✅ Success!"
```

### Make Integration

```makefile
.PHONY: encrypt decrypt clean

CIRCUIT := circuit.r1cs
WASM := circuit.wasm
PUBLIC := public.json
FULL := full.json

encrypt: message.txt
	zkenc encap -c $(CIRCUIT) -i $(PUBLIC) --ciphertext witness.ct -k encrypt.key
	zkenc encrypt -k encrypt.key -i message.txt -o message.enc
	@echo "Encrypted: witness.ct + message.enc"

decrypt: witness.ct message.enc
	snarkjs wtns calculate $(WASM) $(FULL) witness.wtns
	zkenc decap -c $(CIRCUIT) -w witness.wtns --ciphertext witness.ct -k decrypt.key
	zkenc decrypt -k decrypt.key -i message.enc -o decrypted.txt
	@echo "Decrypted: decrypted.txt"

clean:
	rm -f *.ct *.key *.enc *.wtns decrypted.txt
```

## Cross-Tool Compatibility

zkenc-cli is fully compatible with zkenc-js. Files can be shared between them.

### CLI Encrypt → JS Decrypt

```bash
# CLI: Encrypt
zkenc encap -c circuit.r1cs -i public.json --ciphertext witness.ct -k key.bin
zkenc encrypt -k key.bin -i message.txt -o message.enc

# Combine files for zkenc-js
cat <(head -c 4 <(printf '\x00\x00\x06(\n')) witness.ct message.enc > combined.bin
```

```javascript
// JS: Decrypt
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

const ciphertext = await fs.readFile("combined.bin");
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);
```

[Learn more about cross-tool workflows →](/docs/guides/cross-tool-workflow)

## Performance

### Encap Performance

| Circuit Size | Constraints      | Time       |
| ------------ | ---------------- | ---------- |
| Small        | < 1,000          | < 100ms    |
| Medium       | 1,000 - 10,000   | 100ms - 1s |
| Large        | 10,000 - 100,000 | 1s - 10s   |
| Very Large   | > 100,000        | > 10s      |

### Decap Performance

Similar to encap, plus witness calculation overhead (~50-200ms)

### Encrypt/Decrypt Performance

Very fast (< 10ms) - only AES operations, independent of circuit size

## Troubleshooting

### "Failed to load R1CS circuit"

- Check file path is correct
- Ensure file is valid R1CS format (compiled with circom)
- Try recompiling the circuit

```bash
circom circuit.circom --r1cs --output build
```

### "Failed to parse input JSON"

- Validate JSON syntax
- Ensure all values are numbers or strings
- Check signal names match circuit

```bash
# Validate JSON
cat inputs.json | jq .
```

### "Decap failed"

- Witness doesn't satisfy circuit constraints
- Wrong circuit file
- Corrupted ciphertext

**Debug:**

```bash
# Test witness generation
snarkjs wtns calculate circuit.wasm inputs.json test.wtns

# Check circuit
snarkjs r1cs info circuit.r1cs
```

### "Encryption/Decryption failed"

- Wrong key file
- Corrupted encrypted file
- File format mismatch

**Verify key:**

```bash
# Key should be exactly 32 bytes
ls -l *.key
```

## Best Practices

1. **Keep circuit files secure**: R1CS files are needed for both encryption and decryption
2. **Separate public/private inputs**: Only share public inputs with encryptors
3. **Verify witness validity**: Test witness generation before decryption
4. **Use consistent file naming**: Follow conventions (`.ct`, `.key`, `.enc`)
5. **Backup keys temporarily**: Keys are only needed during encryption phase

## Security Considerations

- **Key Management**: Keys are temporary - secure the witness instead
- **Circuit Integrity**: Ensure R1CS file hasn't been tampered with
- **Witness Privacy**: Never share witness files - they're like private keys
- **Transport Security**: Use secure channels for ciphertext distribution

## Next Steps

- **[Getting Started →](/docs/getting-started/zkenc-cli)** - Quick start guide
- **[Cross-Tool Workflow →](/docs/guides/cross-tool-workflow)** - Use with zkenc-js
- **[zkenc-core API →](/docs/api/zkenc-core)** - Rust library reference
