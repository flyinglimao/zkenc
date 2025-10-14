# zkenc-js Implementation Plan

## Current Progress

### ✅ Phase 1: Project Setup (Complete)

- TypeScript configuration with ESM output
- Vitest test framework setup
- Public API defined: encap, decap, encrypt, decrypt
- Initial TDD tests

### ✅ Phase 2: Encrypt/Decrypt (Complete)

- Implemented AES-256-GCM encryption using Web Crypto API
- Tests passing for:
  - Round-trip encryption/decryption
  - Wrong key rejection
  - Random nonce generation
- Test fixtures copied from zkenc-cli

### 🔄 Phase 3: Witness Calculator Integration (Next)

**Goal**: Integrate witness_calculator.js to compute witnesses from Circom WASM

**Tasks**:

1. Create TypeScript wrapper for witness_calculator.js
2. Load WASM file and compute witness from inputs
3. Test witness computation with sudoku circuit

**Implementation**:

```typescript
// src/witness.ts
import witnessCalculatorBuilder from "./witness_calculator.js";

export async function calculateWitness(
  wasmBuffer: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array> {
  const witnessCalculator = await witnessCalculatorBuilder(wasmBuffer);
  const witnessArray = await witnessCalculator.calculateWitness(inputs);

  // Convert BigInt array to bytes
  return serializeWitness(witnessArray);
}
```

### 🔄 Phase 4: WASM Bindings (Next)

**Goal**: Create Rust WASM bindings for zkenc-core

**Update lib.rs**:

```rust
use wasm_bindgen::prelude::*;
use zkenc_core::{encap, decap};
use ark_bn254::{Bn254, Fr};

#[wasm_bindgen]
pub struct WasmZkenc;

#[wasm_bindgen]
impl WasmZkenc {
    #[wasm_bindgen]
    pub fn encap(
        r1cs_bytes: &[u8],
        public_inputs: Vec<String>,
    ) -> Result<JsValue, JsValue> {
        // Parse R1CS
        // Create CircomCircuit with public inputs only
        // Call zkenc_core::encap
        // Return (ciphertext, key) as JsValue
    }

    #[wasm_bindgen]
    pub fn decap(
        r1cs_bytes: &[u8],
        witness_bytes: &[u8],
        ciphertext_bytes: &[u8],
    ) -> Result<Vec<u8>, JsValue> {
        // Parse R1CS and witness
        // Create CircomCircuit with full witness
        // Call zkenc_core::decap
        // Return key
    }
}
```

### 🔄 Phase 5: TypeScript Integration

**Goal**: Connect all pieces together

**Update zkenc.ts**:

```typescript
import { calculateWitness } from "./witness.js";
import { WasmZkenc } from "../pkg/zkenc_js.js";

export async function encap(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>
): Promise<EncapResult> {
  const wasm = new WasmZkenc();

  // Convert publicInputs to string array
  const publicInputsArray = flattenInputs(publicInputs);

  // Call WASM encap
  const result = wasm.encap(circuitFiles.r1csBuffer, publicInputsArray);

  return {
    ciphertext: result.ciphertext,
    key: result.key,
  };
}

export async function decap(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array> {
  // Calculate witness from WASM circuit
  const witnessBytes = await calculateWitness(circuitFiles.wasmBuffer, inputs);

  const wasm = new WasmZkenc();

  // Call WASM decap
  return wasm.decap(circuitFiles.r1csBuffer, witnessBytes, ciphertext);
}
```

### 🔄 Phase 6: E2E Tests

**Goal**: Full integration test with Sudoku circuit

```typescript
import { readFile } from "fs/promises";
import { encap, decap, encrypt, decrypt } from "./zkenc.js";

describe("E2E: Sudoku encryption", () => {
  it("should encrypt message that only correct solver can decrypt", async () => {
    // Load circuit files
    const r1csBuffer = await readFile("tests/fixtures/sudoku.r1cs");
    const wasmBuffer = await readFile("tests/fixtures/sudoku.wasm");
    const inputs = JSON.parse(
      await readFile("tests/fixtures/sudoku_basic.json", "utf-8")
    );

    const circuitFiles = { r1csBuffer, wasmBuffer };
    const publicInputs = { puzzle: inputs.puzzle };

    // 1. Encap: generate ciphertext and key
    const { ciphertext, key: key1 } = await encap(circuitFiles, publicInputs);

    // 2. Encrypt message
    const message = new TextEncoder().encode("Secret message");
    const encrypted = await encrypt(key1, message);

    // 3. Decap: recover key with full solution
    const key2 = await decap(circuitFiles, ciphertext, inputs);

    // Keys should match
    expect(key2).toEqual(key1);

    // 4. Decrypt message
    const decrypted = await decrypt(key2, encrypted);
    expect(decrypted).toEqual(message);
  });

  it("should fail with wrong solution", async () => {
    // Similar test with wrong solution
    // Should fail to recover same key
  });
});
```

## Dependencies Needed

### Cargo.toml updates:

```toml
[dependencies]
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
zkenc-core = { path = "../zkenc-core", features = ["with_curves"] }
# Import R1CS/Circuit parsing from zkenc-cli
zkenc-cli = { path = "../zkenc-cli" }

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

### package.json updates:

```json
{
  "scripts": {
    "prebuild": "cargo build --target wasm32-unknown-unknown --release",
    "build:wasm": "wasm-pack build --target web --out-dir pkg",
    "build": "npm run build:wasm && tsc"
  }
}
```

## Testing Strategy

1. **Unit Tests**: Each function tested independently
2. **Integration Tests**: WASM bindings + TypeScript integration
3. **E2E Tests**: Full workflow with real circuit files
4. **Compatibility Tests**: Verify interop with zkenc-cli outputs

## Remaining Work

- [ ] Implement witness calculator wrapper
- [ ] Create WASM bindings in lib.rs
- [ ] Build WASM module with wasm-pack
- [ ] Integrate everything in TypeScript
- [ ] Add E2E tests
- [ ] Update documentation

## Notes

- Use same R1CS parser from zkenc-cli (avoid duplication)
- Ensure BN254 curve is used (Circom compatibility)
- Match CLI's input/output formats for interoperability
- Consider bundle size (WASM can be large)

## Timeline Estimate

- Phase 3: 2-3 hours (witness calculator)
- Phase 4: 4-5 hours (WASM bindings, complex)
- Phase 5: 2-3 hours (TypeScript integration)
- Phase 6: 2-3 hours (E2E tests)
- **Total**: ~12-15 hours of focused work

Current Status: **2/6 phases complete (33%)**
