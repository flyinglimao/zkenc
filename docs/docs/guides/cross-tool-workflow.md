---
sidebar_position: 4
---

# Cross-Tool Workflow Guide

Learn how to use zkenc-cli and zkenc-js together for maximum flexibility and power.

## Why Use Both Tools?

Combining zkenc-cli and zkenc-js enables powerful workflows:

- **Encrypt on server, decrypt in browser**
- **CLI for batch processing, JS for UI**
- **Different environments, same ciphertexts**
- **Leverage strengths of each tool**

## Compatibility

zkenc-cli and zkenc-js are **fully compatible** and use the same combined ciphertext format:

✅ Files encrypted with CLI can be decrypted with JS
✅ Files encrypted with JS can be decrypted with CLI  
✅ Same circuit files work with both tools
✅ Same input format for both tools
✅ No file format conversion needed

**Both tools use the same combined format:**

```
[1 byte flag][4 bytes witness CT length][witness ciphertext]
[4 bytes public input length (if flag=1)][public input JSON (if flag=1)]
[encrypted message]
```

## Workflow 1: CLI Encrypt → JS Decrypt

**Use case:** Encrypt sensitive files on a server, decrypt in a web application.

### Step 1: Prepare Circuit (CLI)

```bash
# Compile circuit
circom circuit.circom --r1cs --wasm -o build

# You'll need:
# - build/circuit.r1cs (for both CLI and JS)
# - build/circuit_js/circuit.wasm (for both CLI and JS)
```

### Step 2: Create Public Inputs (CLI)

Create `public_inputs.json`:

```json
{
  "publicValue": "42"
}
```

### Step 3: Encrypt with CLI

```bash
# One-step encryption (recommended)
zkenc encrypt \
  --circuit build/circuit.r1cs \
  --input public_inputs.json \
  --message secret.txt \
  --output encrypted.bin
```

The output `encrypted.bin` is a combined ciphertext that includes:

- Witness encryption ciphertext
- Public inputs (embedded by default)
- AES-encrypted message

**File sizes:**

- `encrypted.bin` (combined) ≈ witness CT (1576 bytes) + public inputs + message + overhead

### Step 4: Decrypt with JS

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

// Load combined ciphertext
const ciphertext = await fs.readFile("encrypted.bin");

// Load circuit files
const circuitFiles = {
  r1csBuffer: await fs.readFile("build/circuit.r1cs"),
  wasmBuffer: await fs.readFile("build/circuit_js/circuit.wasm"),
};

// Prepare full inputs (public + private)
const fullInputs = {
  publicValue: 42,
  privateValue: 123, // The witness
};

// Decrypt in one step
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

console.log(new TextDecoder().decode(decrypted));
// Output: (contents of secret.txt)
```

**That's it!** No file conversion needed.

## Workflow 2: JS Encrypt → CLI Decrypt

**Use case:** Encrypt in browser, process/decrypt on server.

### Step 1: Encrypt with JS

```typescript
import { zkenc } from "zkenc-js";

const circuitFiles = {
  r1csBuffer: await fetch("/circuits/circuit.r1cs")
    .then((r) => r.arrayBuffer())
    .then((b) => new Uint8Array(b)),
  wasmBuffer: await fetch("/circuits/circuit.wasm")
    .then((r) => r.arrayBuffer())
    .then((b) => new Uint8Array(b)),
};

const publicInputs = { publicValue: "42" };
const message = new TextEncoder().encode("Secret from browser");

// One-step encryption
const { ciphertext } = await zkenc.encrypt(circuitFiles, publicInputs, message);

// Download ciphertext
const blob = new Blob([ciphertext]);
const url = URL.createObjectURL(blob);
const a = document.createElement("a");
a.href = url;
a.download = "encrypted.bin";
a.click();
```

The `ciphertext` is already in the combined format that CLI can read directly.

### Step 2: Generate Witness (CLI)

Create full inputs `full_inputs.json`:

```json
{
  "publicValue": "42",
  "privateValue": "123"
}
```

Generate witness using snarkjs:

```bash
snarkjs wtns calculate \
  build/circuit_js/circuit.wasm \
  full_inputs.json \
  witness.wtns
```

### Step 3: Decrypt with CLI

```bash
# One-step decryption
zkenc decrypt \
  --circuit build/circuit.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin \
  --output decrypted.txt

cat decrypted.txt
# Output: Secret from browser
```

**That's it!** The CLI can read the JS-encrypted file directly.

## Workflow 3: Hybrid Processing

**Use case:** Use CLI for batch operations, JS for interactive UI.

### Example: Photo Encryption Service

**Server (CLI):**

```bash
#!/bin/bash
# encrypt-photos.sh

for photo in uploads/*.jpg; do
  echo "Processing $photo..."

  # Generate unique public input
  PUBLIC_VALUE=$(date +%s)
  echo "{\"timestamp\": \"$PUBLIC_VALUE\"}" > inputs.json

  # Encrypt in one step
  zkenc encrypt \
    --circuit circuit.r1cs \
    --input inputs.json \
    --message "$photo" \
    --output "${photo}.enc"

  # Store metadata
  echo "$photo,$PUBLIC_VALUE" >> metadata.csv

  rm inputs.json
done
```

**Client (JS):**

```typescript
// Decrypt selected photo
async function decryptPhoto(photoId: string, privateValue: number) {
  // Fetch encrypted photo (combined format)
  const response = await fetch(`/api/photos/${photoId}/encrypted`);
  const ciphertext = new Uint8Array(await response.arrayBuffer());

  // Get public value from metadata
  const metadata = await fetch(`/api/photos/${photoId}/metadata`).then((r) =>
    r.json()
  );

  // Decrypt in one step
  const fullInputs = {
    timestamp: metadata.timestamp,
    userSecret: privateValue,
  };

  const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

  // Display photo
  const blob = new Blob([decrypted], { type: "image/jpeg" });
  const url = URL.createObjectURL(blob);
  imageElement.src = url;
}
```

**Note:** Public inputs can be extracted from ciphertext using `getPublicInput()` if embedded:

```typescript
import { getPublicInput } from "zkenc-js";

// Extract embedded public inputs
const publicInputs = getPublicInput(ciphertext);
console.log(publicInputs.timestamp); // No need to fetch metadata!
```

## Workflow 4: Multi-Platform Distribution

**Use case:** Encrypt once, decrypt on any platform.

### Setup

```bash
# Compile circuit
circom puzzle.circom --r1cs --wasm -o dist

# Create distribution package
mkdir -p package/circuits
cp dist/puzzle.r1cs package/circuits/
cp dist/puzzle_js/puzzle.wasm package/circuits/
cp README.md package/
```

### Encrypt Once

```bash
# Create puzzle
cat > puzzle.json <<EOF
{
  "puzzle": ["5", "3", "0", "0", "7", "0", "0", "0", "0"]
}
EOF

# Encrypt message (creates combined format)
zkenc encrypt \
  --circuit package/circuits/puzzle.r1cs \
  --input puzzle.json \
  --message treasure.txt \
  --output package/treasure.enc
```

### Distribute

```
package/
├── circuits/
│   ├── puzzle.r1cs     # Circuit file
│   └── puzzle.wasm      # Witness generator
├── treasure.enc         # Combined ciphertext (works with both tools!)
└── README.md            # Instructions
```

### Users Can Decrypt With Either Tool

**CLI User:**

```bash
# Generate solution witness
cat > solution.json <<EOF
{
  "puzzle": ["5", "3", "0", ...],
  "solution": ["5", "3", "4", "6", "7", "8", "9", "1", "2", ...]
}
EOF

snarkjs wtns calculate puzzle.wasm solution.json solution.wtns

# Decrypt directly
zkenc decrypt \
  --circuit puzzle.r1cs \
  --witness solution.wtns \
  --ciphertext treasure.enc \
  --output treasure.txt
```

**JS User:**

```typescript
// Load the same encrypted file
const ciphertext = await fetch('treasure.enc')
  .then(r => r.arrayBuffer())
  .then(b => new Uint8Array(b));

const solution = {
  puzzle: ["5", "3", "0", ...],
  solution: ["5", "3", "4", "6", "7", "8", "9", "1", "2", ...],
};

// Decrypt directly
const treasure = await zkenc.decrypt(circuitFiles, ciphertext, solution);
```

**No conversion needed!** Both tools read the same file format.

## Advanced: Using Low-Level API

For advanced use cases, you can still use the low-level `encap`/`decap` commands separately:

### CLI Low-Level Commands

```bash
# Step 1: Generate witness ciphertext and key
zkenc encap \
  --circuit circuit.r1cs \
  --input public.json \
  --ciphertext witness.ct \
  --key key.bin

# Step 2: Encrypt with any AES tool or custom implementation
# (key.bin is a 32-byte key suitable for AES-256)

# Step 3: Decrypt - recover key
zkenc decap \
  --circuit circuit.r1cs \
  --witness solution.wtns \
  --ciphertext witness.ct \
  --key recovered_key.bin

# Step 4: Decrypt with the same method used in Step 2
```

### When to Use Low-Level API

- Custom encryption schemes
- Integration with existing encryption pipelines
- Educational purposes
- Debugging encryption/decryption separately

**Note:** For most use cases, the high-level `encrypt`/`decrypt` commands are recommended.

## Best Practices

1. **Use High-Level API**: Use `encrypt`/`decrypt` commands for simplicity and compatibility
2. **Keep Circuit Files Consistent**: Use the same compiled circuit files across tools
3. **Document Public Inputs**: Clearly document which inputs are public vs private
4. **Embed Public Inputs**: Use default behavior (embedded) for self-contained ciphertexts
5. **Version Your Circuits**: Track circuit versions to ensure compatibility
6. **Test Both Directions**: Always test CLI→JS and JS→CLI workflows

## Troubleshooting

**"Invalid ciphertext" when decrypting:**

- Ensure the file is a valid zkenc ciphertext (created by `encrypt` command)
- Verify file wasn't corrupted during transfer
- Check you're using the correct circuit file

**"Witness doesn't satisfy constraints":**

- Verify public inputs match between encryption and decryption
- Check private inputs satisfy circuit constraints
- Ensure using same circuit version
- Use `snarkjs wtns check` to validate witness

**File format issues:**

- Files are already compatible - no conversion needed!
- Use binary mode for all file operations
- Avoid text editors that might corrupt binary files
- Use `xxd` or `hexdump` to inspect files if needed

**Public inputs mismatch:**

- CLI and JS both embed public inputs by default
- Use `getPublicInput()` in JS to extract from ciphertext
- CLI displays public inputs when decrypting (if embedded)

## Next Steps

- **[Node.js Guide →](/docs/guides/nodejs-integration)** - Build CLI tools
- **[React Guide →](/docs/guides/react-integration)** - Build web UIs
- **[API References →](/docs/api/zkenc-js)** - Detailed documentation
- **[Playground →](/playground)** - Try it in browser
