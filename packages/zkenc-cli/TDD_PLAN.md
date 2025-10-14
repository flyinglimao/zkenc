# zkenc-cli TDD Plan

## Phase 1: Basic Circom Loading (Test-First) ✅

**Goal**: Load and parse circom R1CS and WASM files

### Test 1.1: Load circom files ✅

- [x] Test: `test_load_circom_circuit`
- [x] Load signature.r1cs and signature.wasm
- [x] Verify circuit info (num constraints, variables)
- **Result**: 2294 constraints, 3 public inputs, 4588 variables

### Implementation 1.1: Circom module ✅

- [x] Add lib.rs for testing support
- [x] Create `circom.rs` module
- [x] Implement `load_circom_circuit()` - minimal version
- Note: Full ark-circom integration deferred to Phase 3

---

## Phase 2: Input Parsing (Test-First) ✅

**Goal**: Parse JSON input files to field elements

### Test 2.1: Parse simple inputs ✅

- [x] Test: `test_parse_simple_input`
- [x] Parse signature_basic.json
- [x] Verify field element conversion
- **Result**: Successfully parsed 5 fields (message, publicKeys, R8, S, signerIndices)

### Implementation 2.1: Input parsing ✅

- [x] Implement `parse_inputs()` - returns HashMap<String, Vec<String>>
- [x] Handle strings, numbers, arrays
- [x] Recursive flattening of nested arrays
- Note: Flattens 2D arrays (e.g., publicKeys: 3×2 → 6 values)

---

## Phase 3: Encap/Decap Integration (Test-First) ✅ Complete

**Goal**: Integrate with zkenc-core without ark-circom

**Status**: ✅ **Complete** - Bypassed ark-circom by parsing R1CS directly

### Solution: Direct R1CS Parsing

**Approach**: Instead of using ark-circom, we:

1. Parse R1CS binary format directly (following iden3 spec)
2. Implement `ConstraintSynthesizer<Fr>` trait ourselves
3. Use zkenc-core's BLS12-381 curve directly

**Benefits**:

- ✅ No dependency conflicts
- ✅ Full control over parsing
- ✅ Lighter weight
- ✅ Works with zkenc-core's git arkworks versions

### Implementation Complete ✅

- [x] **R1CS Parser** (`src/r1cs.rs`)

  - Parses magic, version, sections
  - Extracts header (field size, prime, wires, constraints)
  - Parses constraints (A, B, C linear combinations)
  - Handles wire2label mapping
  - Test: 8443 constraints, 7 public inputs ✅

- [x] **CircomCircuit** (`src/circuit.rs`)
  - Implements `ConstraintSynthesizer<Fr>` for BLS12-381
  - Allocates variables (public inputs + private witnesses)
  - Converts R1CS constraints to gr1cs format
  - Enforces A\*B=C constraints via closures
  - Test: Synthesis successful ✅

### Test 3.1: Load and synthesize circuit ✅

- [x] Test: `test_load_circom_circuit`
- [x] Load R1CS file
- [x] Create CircomCircuit wrapper
- [x] Verify circuit info

### Test 3.2: Circuit synthesis ✅

- [x] Test: `test_circuit_synthesis`
- [x] Set witness values
- [x] Generate constraints via ConstraintSynthesizer
- [x] Verify constraint system created

### Integration with zkenc-core

- [x] Match arkworks git versions (0.5)
- [x] Use BLS12-381 curve
- [x] Implement ConstraintSynthesizer trait
- [ ] Test encap with CircomCircuit (next step)
- [ ] Test decap with CircomCircuit (next step)

---

## Phase 4: AES Encryption (Test-First)

**Status**: ✅ Complete

**Goal**: Encrypt/decrypt with derived key

### Test 4.1: AES-GCM roundtrip

- [x] Test: `test_aes_gcm_roundtrip`
- [x] Encrypt plaintext
- [x] Decrypt and verify

### Test 4.2: AES-CTR roundtrip

- [x] Test: `test_aes_ctr_roundtrip`

### Implementation 4.1: Crypto module

- [x] Implement `encrypt()` and `decrypt()`
- [x] Support GCM and CTR modes

**Results**: 4 unit tests + 2 integration tests passing

- `crypto::tests::test_gcm_roundtrip` ✅
- `crypto::tests::test_ctr_roundtrip` ✅
- `crypto::tests::test_gcm_wrong_key` ✅
- `crypto::tests::test_invalid_key_length` ✅
- `test_aes_gcm_roundtrip` ✅
- `test_aes_ctr_roundtrip` ✅

---

## Phase 5: CLI Commands (Test-First)

**Goal**: Complete CLI integration

### Test 5.1: Encap command

- [ ] Test: End-to-end encap with file I/O

### Test 5.2: Decap command

- [ ] Test: End-to-end decap with file I/O

### Test 5.3: Encrypt/Decrypt commands

- [ ] Test: Full encrypt/decrypt flow

### Implementation 5.1: Commands module

- [ ] Implement all 4 commands
- [ ] File I/O handling

---

## Progress Tracker

- Phase 1: ✅ Complete (Circom loading - file validation)
- Phase 2: ✅ Complete (Input parsing - JSON flattening)
- Phase 3: ✅ Complete (R1CS parsing + CircomCircuit wrapper)
- Phase 4: ✅ Complete (AES encryption - GCM/CTR modes)
- Phase 5: 🔄 Ready (CLI commands with zkenc-core integration)

## Current Status

**Major Breakthrough**: Successfully bypassed ark-circom dependency conflict!

**Completed**:

- ✅ R1CS binary parser (12 tests passing)
- ✅ CircomCircuit ConstraintSynthesizer
- ✅ Full arkworks git version compatibility
- ✅ BLS12-381 integration

**Ready for**:

- Phase 5: CLI commands (encap, decap, encrypt, decrypt)
- Integration tests with zkenc-core
- End-to-end workflow

**Test Results**: 12/12 passing

- 8 unit tests (r1cs, circuit, crypto, circom)
- 4 integration tests (Phase 1,2,4)
