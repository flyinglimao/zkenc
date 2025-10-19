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

zkenc-cli and zkenc-js are **fully compatible**:

✅ Ciphertexts from CLI can be decrypted with JS
✅ Ciphertexts from JS can be decrypted with CLI  
✅ Same circuit files work with both tools
✅ Same input format for both tools

## Workflow 1: CLI Encrypt → JS Decrypt

**Use case:** Encrypt sensitive files on a server, decrypt in a web application.

### Step 1: Prepare Circuit (CLI)

```bash
# Compile circuit
circom circuit.circom --r1cs --wasm -o build

# You'll need:
# - build/circuit.r1cs (for CLI)
# - build/circuit_js/circuit.wasm (for both)
```

### Step 2: Create Public Inputs (CLI)

Create `public_inputs.json`:

```json
{
  "publicValue": 42
}
```

### Step 3: Encrypt with CLI

```bash
# Encapsulate: generate witness-encrypted key
zkenc encap \
  --circuit build/circuit.r1cs \
  --input public_inputs.json \
  --ciphertext witness.ct \
  --key encryption.key

# Encrypt: encrypt message with key
zkenc encrypt \
  --key encryption.key \
  --input secret.txt \
  --output message.enc
```

You now have two files:

- `witness.ct` (1576 bytes) - witness encryption ciphertext
- `message.enc` (message size + 28 bytes) - AES-encrypted message

### Step 4: Combine Files for JS

zkenc-js expects a combined format. Create it:

```bash
# Calculate witness ciphertext length (always 1576 for BN254)
WITNESS_LENGTH=1576

# Create length prefix (4 bytes, big-endian)
printf '\x00\x00\x06\x28' > combined.bin

# Append witness ciphertext
cat witness.ct >> combined.bin

# Append encrypted message
cat message.enc >> combined.bin

echo "Combined ciphertext created: combined.bin"
```

Or use this helper script `combine.sh`:

```bash
#!/bin/bash
WITNESS_CT=$1
MESSAGE_ENC=$2
OUTPUT=$3

# Write 4-byte length (1576 = 0x00000628)
printf '\x00\x00\x06\x28' > "$OUTPUT"
cat "$WITNESS_CT" >> "$OUTPUT"
cat "$MESSAGE_ENC" >> "$OUTPUT"

echo "✅ Created $OUTPUT"
```

Usage:

```bash
chmod +x combine.sh
./combine.sh witness.ct message.enc combined.bin
```

### Step 5: Decrypt with JS

```typescript
import { zkenc } from "zkenc-js";
import fs from "fs/promises";

// Load combined ciphertext
const ciphertext = await fs.readFile("combined.bin");

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

// Decrypt
const decrypted = await zkenc.decrypt(circuitFiles, ciphertext, fullInputs);

console.log(new TextDecoder().decode(decrypted));
```

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

const publicInputs = { publicValue: 42 };
const message = new TextEncoder().encode("Secret from browser");

const { ciphertext } = await zkenc.encrypt(circuitFiles, publicInputs, message);

// Download ciphertext
const blob = new Blob([ciphertext]);
const url = URL.createObjectURL(blob);
const a = document.createElement("a");
a.href = url;
a.download = "encrypted.bin";
a.click();
```

### Step 2: Split Combined Ciphertext (CLI)

zkenc-cli expects separate files. Split them:

```bash
# Extract witness ciphertext (skip 4 bytes, read 1576 bytes)
dd if=encrypted.bin of=witness.ct bs=1 skip=4 count=1576

# Extract encrypted message (skip 4+1576 bytes, read rest)
dd if=encrypted.bin of=message.enc bs=1 skip=1580

echo "✅ Split into witness.ct and message.enc"
```

Or use this helper script `split.sh`:

```bash
#!/bin/bash
INPUT=$1
WITNESS_CT="${INPUT%.bin}_witness.ct"
MESSAGE_ENC="${INPUT%.bin}_message.enc"

# Skip 4-byte header, extract 1576 bytes
dd if="$INPUT" of="$WITNESS_CT" bs=1 skip=4 count=1576 2>/dev/null

# Skip header + witness, extract rest
dd if="$INPUT" of="$MESSAGE_ENC" bs=1 skip=1580 2>/dev/null

echo "✅ Created $WITNESS_CT and $MESSAGE_ENC"
```

Usage:

```bash
chmod +x split.sh
./split.sh encrypted.bin
```

### Step 3: Generate Witness (CLI)

Create full inputs `full_inputs.json`:

```json
{
  "publicValue": 42,
  "privateValue": 123
}
```

Generate witness using snarkjs:

```bash
snarkjs wtns calculate \
  build/circuit_js/circuit.wasm \
  full_inputs.json \
  witness.wtns
```

### Step 4: Decrypt with CLI

```bash
# Decapsulate: recover key
zkenc decap \
  --circuit build/circuit.r1cs \
  --witness witness.wtns \
  --ciphertext encrypted.bin_witness.ct \
  --key recovered.key

# Decrypt: decrypt message
zkenc decrypt \
  --key recovered.key \
  --input encrypted.bin_message.enc \
  --output decrypted.txt

cat decrypted.txt
```

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
  echo "{\"timestamp\": $PUBLIC_VALUE}" > inputs.json

  # Encrypt
  zkenc encap -c circuit.r1cs -i inputs.json --ciphertext ct.bin -k key.bin
  zkenc encrypt -k key.bin -i "$photo" -o "${photo}.enc"

  # Store metadata
  echo "$photo,$PUBLIC_VALUE" >> metadata.csv

  rm key.bin ct.bin inputs.json
done
```

**Client (JS):**

```typescript
// Decrypt selected photo
async function decryptPhoto(photoId: string, privateValue: number) {
  // Fetch encrypted photo
  const response = await fetch(`/api/photos/${photoId}/encrypted`);
  const ciphertext = new Uint8Array(await response.arrayBuffer());

  // Get public value from metadata
  const metadata = await fetch(`/api/photos/${photoId}/metadata`).then((r) =>
    r.json()
  );

  // Decrypt
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
  "puzzle": [5, 3, 0, 0, 7, 0, 0, 0, 0]
}
EOF

# Encrypt message
zkenc encap -c package/circuits/puzzle.r1cs -i puzzle.json \
  --ciphertext package/puzzle.ct -k temp.key
zkenc encrypt -k temp.key -i treasure.txt -o package/treasure.enc
rm temp.key

# Combine for JS users
./combine.sh package/puzzle.ct package/treasure.enc package/treasure-combined.bin
```

### Distribute

```
package/
├── circuits/
│   ├── puzzle.r1cs     # For CLI users
│   └── puzzle.wasm      # For all users
├── puzzle.ct            # For CLI users
├── treasure.enc         # For CLI users
├── treasure-combined.bin # For JS users
└── README.md            # Instructions for both
```

### Users Can Decrypt With Either Tool

**CLI User:**

```bash
# Generate solution
cat > solution.json <<EOF
{
  "puzzle": [5, 3, 0, ...],
  "solution": [5, 3, 4, 6, 7, 8, 9, 1, 2, ...]
}
EOF

snarkjs wtns calculate puzzle.wasm solution.json solution.wtns
zkenc decap -c puzzle.r1cs -w solution.wtns --ciphertext puzzle.ct -k key.bin
zkenc decrypt -k key.bin -i treasure.enc -o treasure.txt
```

**JS User:**

```typescript
const ciphertext = await fetch('treasure-combined.bin')
  .then(r => r.arrayBuffer())
  .then(b => new Uint8Array(b));

const solution = {
  puzzle: [5, 3, 0, ...],
  solution: [5, 3, 4, 6, 7, 8, 9, 1, 2, ...],
};

const treasure = await zkenc.decrypt(circuitFiles, ciphertext, solution);
```

## Helper Scripts

### Complete Combine Script

`tools/combine-ciphertext.sh`:

```bash
#!/bin/bash
set -e

if [ "$#" -ne 3 ]; then
  echo "Usage: $0 <witness.ct> <message.enc> <output.bin>"
  exit 1
fi

WITNESS_CT=$1
MESSAGE_ENC=$2
OUTPUT=$3

# Verify inputs exist
[ -f "$WITNESS_CT" ] || { echo "Error: $WITNESS_CT not found"; exit 1; }
[ -f "$MESSAGE_ENC" ] || { echo "Error: $MESSAGE_ENC not found"; exit 1; }

# Check witness ciphertext size
SIZE=$(stat -f%z "$WITNESS_CT" 2>/dev/null || stat -c%s "$WITNESS_CT" 2>/dev/null)
if [ "$SIZE" -ne 1576 ]; then
  echo "Warning: Witness ciphertext should be 1576 bytes, got $SIZE"
fi

# Create combined file
printf '\x00\x00\x06\x28' > "$OUTPUT"
cat "$WITNESS_CT" >> "$OUTPUT"
cat "$MESSAGE_ENC" >> "$OUTPUT"

echo "✅ Created combined ciphertext: $OUTPUT"
echo "   Witness CT: 1576 bytes"
echo "   Message CT: $(stat -f%z "$MESSAGE_ENC" 2>/dev/null || stat -c%s "$MESSAGE_ENC") bytes"
echo "   Total: $(stat -f%z "$OUTPUT" 2>/dev/null || stat -c%s "$OUTPUT") bytes"
```

### Complete Split Script

`tools/split-ciphertext.sh`:

```bash
#!/bin/bash
set -e

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <combined.bin> [output-prefix]"
  exit 1
fi

INPUT=$1
PREFIX=${2:-${INPUT%.bin}}
WITNESS_CT="${PREFIX}_witness.ct"
MESSAGE_ENC="${PREFIX}_message.enc"

# Verify input exists
[ -f "$INPUT" ] || { echo "Error: $INPUT not found"; exit 1; }

# Extract witness ciphertext
dd if="$INPUT" of="$WITNESS_CT" bs=1 skip=4 count=1576 2>/dev/null

# Extract encrypted message
dd if="$INPUT" of="$MESSAGE_ENC" bs=1 skip=1580 2>/dev/null

echo "✅ Split ciphertext:"
echo "   Witness CT: $WITNESS_CT ($(stat -f%z "$WITNESS_CT" 2>/dev/null || stat -c%s "$WITNESS_CT") bytes)"
echo "   Message CT: $MESSAGE_ENC ($(stat -f%z "$MESSAGE_ENC" 2>/dev/null || stat -c%s "$MESSAGE_ENC") bytes)"
```

## Best Practices

1. **Keep Circuit Files Consistent**: Use the same compiled circuit files across tools
2. **Document Public Inputs**: Clearly document which inputs are public vs private
3. **Version Your Circuits**: Track circuit versions to ensure compatibility
4. **Test Both Directions**: Always test CLI→JS and JS→CLI workflows
5. **Automate Conversion**: Use scripts to convert between formats

## Troubleshooting

**"Invalid ciphertext" when decrypting:**

- Ensure combined format is correct (4-byte length prefix)
- Verify witness ciphertext is exactly 1576 bytes
- Check files weren't corrupted during transfer

**"Witness doesn't satisfy constraints":**

- Verify public inputs match between encryption and decryption
- Check private inputs satisfy circuit constraints
- Ensure using same circuit version

**File format issues:**

- Use binary mode for all file operations
- Avoid text editors that might corrupt binary files
- Use `xxd` or `hexdump` to inspect files

## Next Steps

- **[Node.js Guide →](/docs/guides/nodejs-integration)** - Build CLI tools
- **[React Guide →](/docs/guides/react-integration)** - Build web UIs
- **[API References →](/docs/api/zkenc-js)** - Detailed documentation
- **[Playground →](/playground)** - Try it in browser

## Examples Repository

Complete working examples: `examples/cross-tool-workflow/`

```
examples/cross-tool-workflow/
├── cli-to-js/
│   ├── encrypt.sh
│   ├── combine.sh
│   └── decrypt.ts
├── js-to-cli/
│   ├── encrypt.ts
│   ├── split.sh
│   └── decrypt.sh
└── hybrid/
    ├── server/
    └── client/
```
